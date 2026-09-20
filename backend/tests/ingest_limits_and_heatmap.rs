use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use heatmap_backend::{app, db, AppConfig, test_app, TEST_PROJECT_KEY};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

async fn response_json(response: axum::response::Response) -> Value {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("collect body")
        .to_bytes();
    serde_json::from_slice(&bytes).expect("parse response json")
}

fn valid_click(batch_suffix: &str, x: f64, y: f64, layout: &str) -> Value {
    let mut screen = json!({ "id": "Checkout" });
    if !layout.is_empty() {
        screen["layout_version"] = json!(layout);
    }
    json!({
        "schema_version": 1,
        "session_id": format!("sess_{batch_suffix}"),
        "anonymous_id": format!("anon_{batch_suffix}"),
        "timestamp": 1726771205000.0,
        "sdk": { "name": "heatmap-web", "version": "0.1.0" },
        "platform": {
            "type": "web",
            "os_version": "macOS 14.6",
            "app_version": "1.0.0"
        },
        "screen": screen,
        "event": { "type": "click" },
        "position": { "x": x, "y": y }
    })
}

async fn post_batch(app: &axum::Router, batch_id: &str, events: Value) -> StatusCode {
    let body = json!({ "batch_id": batch_id, "events": events });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/batches")
                .header("X-Project-Key", TEST_PROJECT_KEY)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    response.status()
}

#[tokio::test]
async fn oversize_event_count_returns_413() {
    let app = test_app();
    let events: Vec<Value> = (0..201)
        .map(|i| valid_click(&format!("n{i}"), 0.1, 0.1, "2024-09"))
        .collect();
    let status = post_batch(&app, "batch_too_many_events", json!(events)).await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn oversize_body_bytes_returns_413() {
    let app = test_app();
    let pad = "x".repeat(262_000);
    let event = json!({
        "schema_version": 1,
        "session_id": "sess_pad",
        "anonymous_id": "anon_pad",
        "timestamp": 1.0,
        "sdk": { "name": "heatmap-web", "version": "0.1.0" },
        "platform": {
            "type": "web",
            "os_version": "macOS",
            "app_version": "1.0.0"
        },
        "event": { "type": "screen_view" },
        "extra_padding": pad
    });
    let body = json!({ "batch_id": "batch_too_large", "events": [event] });
    let raw = body.to_string();
    assert!(raw.len() > 262_144);
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/batches")
                .header("X-Project-Key", TEST_PROJECT_KEY)
                .header("content-type", "application/json")
                .body(Body::from(raw))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn malformed_batch_body_returns_400() {
    let app = test_app();
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/batches")
                .header("X-Project-Key", TEST_PROJECT_KEY)
                .header("content-type", "application/json")
                .body(Body::from("{not json"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let missing_batch = json!({ "events": [valid_click("a", 0.5, 0.5, "")] });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/batches")
                .header("X-Project-Key", TEST_PROJECT_KEY)
                .header("content-type", "application/json")
                .body(Body::from(missing_batch.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn unsupported_schema_version_rejects_event_not_whole_batch() {
    let app = test_app();
    let mut bad = valid_click("bad_schema", 0.5, 0.5, "2024-09");
    bad["schema_version"] = json!(2);
    let good = valid_click("good_schema", 0.5, 0.5, "2024-09");
    let batch = json!({ "batch_id": "batch_mixed_schema", "events": [bad, good] });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/batches")
                .header("X-Project-Key", TEST_PROJECT_KEY)
                .header("content-type", "application/json")
                .body(Body::from(batch.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);
    let body = response_json(response).await;
    assert_eq!(body["accepted_count"], 1);
    assert_eq!(body["rejected_count"], 1);
}

#[tokio::test]
async fn rate_limit_returns_429_after_window_capacity() {
    let config = AppConfig::default().with_batches_per_minute(3);
    let db = db::open_in_memory().expect("db");
    let app = app(config, db);
    for i in 0..3 {
        let status = post_batch(
            &app,
            &format!("batch_rl_{i}"),
            json!([valid_click(&format!("rl{i}"), 0.1, 0.1, "")]),
        )
        .await;
        assert_eq!(status, StatusCode::ACCEPTED, "batch {i} should succeed");
    }
    let status = post_batch(
        &app,
        "batch_rl_over",
        json!([valid_click("rl_over", 0.1, 0.1, "")]),
    )
    .await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
async fn rate_limit_429_includes_retry_after_header() {
    let config = AppConfig::default().with_batches_per_minute(1);
    let db = db::open_in_memory().expect("db");
    let app = app(config, db);
    let status = post_batch(
        &app,
        "batch_rl_header_1",
        json!([valid_click("h1", 0.1, 0.1, "")]),
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED);
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/batches")
                .header("X-Project-Key", TEST_PROJECT_KEY)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "batch_id": "batch_rl_header_2",
                        "events": [valid_click("h2", 0.1, 0.1, "")]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    let retry_after = response
        .headers()
        .get("retry-after")
        .expect("Retry-After header");
    assert!(retry_after.to_str().unwrap().parse::<u64>().unwrap() >= 1);
}

#[tokio::test]
async fn unknown_fields_on_valid_event_still_persist() {
    let app = test_app();
    let mut click = valid_click("extra", 0.5, 0.5, "2024-09");
    click["custom_field"] = json!("ignored");
    let status = post_batch(&app, "batch_extra_field", json!([click])).await;
    assert_eq!(status, StatusCode::ACCEPTED);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/heatmap?platform=web&screen=Checkout&layout_version=2024-09&app_version=1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let heatmap = response_json(response).await;
    let bins = heatmap["bins"].as_array().expect("bins");
    assert!(bins.iter().any(|b| b["bin_x"] == 50 && b["bin_y"] == 50));
}

#[tokio::test]
async fn invalid_viewport_rejects_event_siblings_still_persist() {
    let app = test_app();
    let mut bad = valid_click("bad_viewport", 0.2, 0.2, "2024-09");
    bad["viewport"] = json!({ "width": 0, "height": 720 });
    let good = valid_click("good_viewport", 0.8, 0.8, "2024-09");
    let batch = json!({ "batch_id": "batch_mixed_viewport", "events": [bad, good] });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/batches")
                .header("X-Project-Key", TEST_PROJECT_KEY)
                .header("content-type", "application/json")
                .body(Body::from(batch.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);
    let body = response_json(response).await;
    assert_eq!(body["accepted_count"], 1);
    assert_eq!(body["rejected_count"], 1);
}

#[tokio::test]
async fn missing_required_field_rejects_event_siblings_still_persist() {
    let app = test_app();
    let mut bad = valid_click("missing", 0.2, 0.2, "2024-09");
    bad.as_object_mut().unwrap().remove("session_id");
    let good = valid_click("sibling", 0.8, 0.8, "2024-09");
    let batch = json!({ "batch_id": "batch_partial_invalid", "events": [bad, good] });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/batches")
                .header("X-Project-Key", TEST_PROJECT_KEY)
                .header("content-type", "application/json")
                .body(Body::from(batch.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);
    let body = response_json(response).await;
    assert_eq!(body["accepted_count"], 1);
    assert_eq!(body["rejected_count"], 1);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/heatmap?platform=web&screen=Checkout&layout_version=2024-09&app_version=1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let heatmap = response_json(response).await;
    let bins = heatmap["bins"].as_array().expect("bins");
    assert!(bins.iter().any(|b| b["bin_x"] == 80 && b["bin_y"] == 80));
    assert!(!bins.iter().any(|b| b["bin_x"] == 20 && b["bin_y"] == 20));
}

#[tokio::test]
async fn click_bins_use_floor_times_100_capped_at_99() {
    let app = test_app();
    let corner = valid_click("corner", 1.0, 1.0, "layout-a");
    let status = post_batch(&app, "batch_corner_click", json!([corner])).await;
    assert_eq!(status, StatusCode::ACCEPTED);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/heatmap?platform=web&screen=Checkout&layout_version=layout-a&app_version=1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let heatmap = response_json(response).await;
    let bins = heatmap["bins"].as_array().expect("bins");
    let corner_bin = bins
        .iter()
        .find(|b| b["bin_x"] == 99 && b["bin_y"] == 99)
        .expect("1.0,1.0 maps to bin 99,99");
    assert_eq!(corner_bin["count"], 1);
}

#[tokio::test]
async fn heatmap_slice_uses_layout_not_window_id() {
    let app = test_app();
    let mut layout_a = valid_click("la", 0.1, 0.1, "layout-a");
    layout_a["window_id"] = json!("win_one");
    let mut layout_b = valid_click("lb", 0.9, 0.9, "layout-b");
    layout_b["window_id"] = json!("win_one");
    post_batch(&app, "batch_layout_a", json!([layout_a])).await;
    post_batch(&app, "batch_layout_b", json!([layout_b])).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/heatmap?platform=web&screen=Checkout&layout_version=layout-a&app_version=1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let heatmap_a = response_json(response).await;
    let bins_a = heatmap_a["bins"].as_array().expect("bins");
    assert!(bins_a.iter().any(|b| b["bin_x"] == 10 && b["bin_y"] == 10));
    assert!(!bins_a.iter().any(|b| b["bin_x"] == 90 && b["bin_y"] == 90));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/heatmap?platform=web&screen=Checkout&layout_version=layout-b&app_version=1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let heatmap_b = response_json(response).await;
    let bins_b = heatmap_b["bins"].as_array().expect("bins");
    assert!(bins_b.iter().any(|b| b["bin_x"] == 90 && b["bin_y"] == 90));
}

#[tokio::test]
async fn get_screens_lists_known_screens_with_layout() {
    let app = test_app();
    post_batch(
        &app,
        "batch_screen_list",
        json!([valid_click("sl", 0.5, 0.5, "2024-09")]),
    )
    .await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/screens")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response_json(response).await;
    let screens = body["screens"].as_array().expect("screens");
    let checkout = screens
        .iter()
        .find(|s| s["id"] == "Checkout" && s["platform"] == "web")
        .expect("Checkout screen listed");
    assert_eq!(checkout["layout_version"], "2024-09");
}

#[tokio::test]
async fn scroll_events_do_not_increment_click_bins() {
    let app = test_app();
    let scroll = json!({
        "schema_version": 1,
        "session_id": "sess_scroll",
        "anonymous_id": "anon_scroll",
        "timestamp": 1.0,
        "sdk": { "name": "heatmap-web", "version": "0.1.0" },
        "platform": {
            "type": "web",
            "os_version": "macOS",
            "app_version": "1.0.0"
        },
        "screen": { "id": "Checkout", "layout_version": "2024-09" },
        "event": { "type": "scroll" },
        "position": { "x": 0.5, "y": 0.5 }
    });
    post_batch(&app, "batch_scroll_bins", json!([scroll])).await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/heatmap?platform=web&screen=Checkout&layout_version=2024-09&app_version=1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let heatmap = response_json(response).await;
    let bins = heatmap["bins"].as_array().expect("bins");
    assert!(bins.is_empty());
}
