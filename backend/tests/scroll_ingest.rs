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

fn app_with_test_events() -> axum::Router {
    let db = db::open_in_memory().expect("db");
    let config = AppConfig::default().with_test_events(true);
    app(config, db)
}

fn fixtures_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../test/fixtures")
}

fn read_fixture(name: &str) -> Value {
    let path = fixtures_dir().join(name);
    let raw = std::fs::read_to_string(&path).expect("read fixture");
    serde_json::from_str(&raw).expect("parse fixture json")
}

async fn ingest_batch(app: &axum::Router, batch_id: &str, events: Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/batches")
                .header("X-Project-Key", TEST_PROJECT_KEY)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({ "batch_id": batch_id, "events": events }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);
}

async fn fetch_events(app: &axum::Router) -> Value {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/events")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    response_json(response).await
}

async fn fetch_checkout_elements(app: &axum::Router) -> Value {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/elements?platform=web&screen=Checkout&layout_version=2024-09&app_version=1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    response_json(response).await
}

fn checkout_screen_view() -> Value {
    json!({
        "schema_version": 1,
        "session_id": "sess_scroll_checkout",
        "anonymous_id": "anon_scroll_checkout",
        "timestamp": 1726771203000.0,
        "sdk": { "name": "heatmap-web", "version": "0.1.0" },
        "platform": {
            "type": "web",
            "os_version": "macOS 14.6",
            "app_version": "1.0.0"
        },
        "screen": { "id": "Checkout", "layout_version": "2024-09" },
        "event": { "type": "screen_view" },
        "viewport": { "width": 1280, "height": 720 }
    })
}

#[tokio::test]
async fn scroll_after_checkout_screen_is_stored_with_checkout_screen_id() {
    let app = app_with_test_events();
    let scroll = read_fixture("scroll-checkout-macos.json");
    ingest_batch(
        &app,
        "batch_checkout_then_scroll",
        json!([checkout_screen_view(), scroll]),
    )
    .await;

    let body = fetch_events(&app).await;
    let events = body["events"].as_array().expect("events array");
    let scroll_event = events
        .iter()
        .find(|event| event["event"]["type"] == "scroll")
        .expect("scroll event in raw store");
    assert_eq!(scroll_event["screen"]["id"], "Checkout");
}

#[tokio::test]
async fn scroll_on_checkout_does_not_create_or_change_element_totals() {
    let app = test_app();
    let pay_click = read_fixture("click-checkout-pay-web.json");
    ingest_batch(&app, "batch_pay_before_scroll", json!([pay_click.clone()])).await;

    let before = fetch_checkout_elements(&app).await;
    let pay_before = before["elements"]
        .as_array()
        .expect("elements")
        .iter()
        .find(|row| row["element_id"] == "checkout.pay")
        .expect("checkout.pay row before scroll");
    assert_eq!(pay_before["click_count"], 1);

    let scroll = json!({
        "schema_version": 1,
        "session_id": "sess_a1b2c3d4-e5f6-7890-abcd-ef1234567890",
        "anonymous_id": "anon_web_001",
        "timestamp": 1726771204000.0,
        "sdk": { "name": "heatmap-web", "version": "0.1.0" },
        "platform": {
            "type": "web",
            "os_version": "macOS 14.6",
            "app_version": "1.0.0"
        },
        "screen": { "id": "Checkout", "layout_version": "2024-09" },
        "event": { "type": "scroll" },
        "viewport": { "width": 1280, "height": 720 }
    });
    ingest_batch(&app, "batch_scroll_only", json!([scroll])).await;

    let after = fetch_checkout_elements(&app).await;
    let elements = after["elements"].as_array().expect("elements");
    assert_eq!(elements.len(), 1);
    let pay_after = elements
        .iter()
        .find(|row| row["element_id"] == "checkout.pay")
        .expect("checkout.pay row after scroll");
    assert_eq!(pay_after["click_count"], 1);
    assert_eq!(pay_after["unique_sessions"], pay_before["unique_sessions"]);
}
