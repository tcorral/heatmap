use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use heatmap_backend::{test_app, TEST_PROJECT_KEY};
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

fn click_at(ts: f64, x: f64, y: f64, layout: &str) -> Value {
    json!({
        "schema_version": 1,
        "session_id": "sess_date",
        "anonymous_id": "anon_date",
        "timestamp": ts,
        "sdk": { "name": "heatmap-web", "version": "0.1.0" },
        "platform": {
            "type": "web",
            "os_version": "macOS",
            "app_version": "1.0.0"
        },
        "screen": { "id": "Checkout", "layout_version": layout },
        "event": { "type": "click" },
        "position": { "x": x, "y": y }
    })
}

async fn post_batch(app: &axum::Router, batch_id: &str, events: Value) {
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
    assert_eq!(response.status(), StatusCode::ACCEPTED);
}

const TINY_PNG_BASE64: &str =
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";

#[tokio::test]
async fn get_still_returns_404_when_missing() {
    let app = test_app();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/stills?platform=web&screen=Checkout&layout_version=2024-09")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn post_and_get_still_round_trip() {
    let app = test_app();
    let body = json!({
        "platform": "web",
        "screen": "Checkout",
        "layout_version": "2024-09",
        "content_type": "image/png",
        "image_base64": TINY_PNG_BASE64
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/stills")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/stills?platform=web&screen=Checkout&layout_version=2024-09")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let still = response_json(response).await;
    assert_eq!(still["content_type"], "image/png");
    assert!(still["image_base64"]
        .as_str()
        .expect("base64")
        .starts_with(TINY_PNG_BASE64));
}

#[tokio::test]
async fn heatmap_date_range_excludes_clicks_outside_slice() {
    let app = test_app();
    post_batch(
        &app,
        "batch_early",
        json!([click_at(1000.0, 0.1, 0.1, "2024-09")]),
    )
    .await;
    post_batch(
        &app,
        "batch_late",
        json!([click_at(9000.0, 0.9, 0.9, "2024-09")]),
    )
    .await;

    let uri = "/v1/heatmap?platform=web&screen=Checkout&layout_version=2024-09&app_version=1.0.0&from=5000&to=10000";
    let response = app
        .clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let heatmap = response_json(response).await;
    let bins = heatmap["bins"].as_array().expect("bins");
    assert!(bins.iter().any(|b| b["bin_x"] == 90 && b["bin_y"] == 90));
    assert!(!bins.iter().any(|b| b["bin_x"] == 10 && b["bin_y"] == 10));
}

#[tokio::test]
async fn heatmap_layout_versions_do_not_mix_bins() {
    let app = test_app();
    post_batch(
        &app,
        "batch_layout_a",
        json!([click_at(1000.0, 0.2, 0.2, "layout-a")]),
    )
    .await;
    post_batch(
        &app,
        "batch_layout_b",
        json!([click_at(1000.0, 0.8, 0.8, "layout-b")]),
    )
    .await;

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
    let heatmap = response_json(response).await;
    let bins = heatmap["bins"].as_array().expect("bins");
    assert!(bins.iter().any(|b| b["bin_x"] == 20 && b["bin_y"] == 20));
    assert!(!bins.iter().any(|b| b["bin_x"] == 80 && b["bin_y"] == 80));
}
