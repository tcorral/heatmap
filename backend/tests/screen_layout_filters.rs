use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use heatmap_backend::{app, db, test_app, AppConfig, TEST_PROJECT_KEY};
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

fn checkout_pay_click(layout_version: Option<&str>) -> Value {
    let mut screen = json!({ "id": "Checkout" });
    if let Some(layout) = layout_version {
        screen["layout_version"] = json!(layout);
    }
    json!({
        "schema_version": 1,
        "session_id": "sess_layout_test",
        "anonymous_id": "anon_layout_test",
        "timestamp": 1000.0,
        "sdk": { "name": "heatmap-web", "version": "0.1.0" },
        "platform": {
            "type": "web",
            "os_version": "macOS 14.6",
            "app_version": "1.0.0"
        },
        "screen": screen,
        "event": { "type": "click", "element_id": "checkout.pay" },
        "viewport": { "width": 1280, "height": 720 }
    })
}

async fn fetch_element_totals(app: &axum::Router, layout_version: Option<&str>) -> Value {
    let uri = match layout_version {
        Some(layout) => format!(
            "/v1/elements?platform=web&screen=Checkout&layout_version={layout}&app_version=1.0.0"
        ),
        None => "/v1/elements?platform=web&screen=Checkout&app_version=1.0.0".into(),
    };
    let response = app
        .clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    response_json(response).await
}

fn pay_click_count(totals: &Value) -> i64 {
    totals["elements"]
        .as_array()
        .expect("elements")
        .iter()
        .find(|row| row["element_id"] == "checkout.pay")
        .map(|row| row["click_count"].as_i64().unwrap_or(0))
        .unwrap_or(0)
}

#[tokio::test]
async fn click_without_screen_is_stored_but_excluded_from_screen_filtered_query() {
    let app = app_with_test_events();
    let click = json!({
        "schema_version": 1,
        "session_id": "sess_no_screen",
        "anonymous_id": "anon_no_screen",
        "timestamp": 2000.0,
        "sdk": { "name": "heatmap-web", "version": "0.1.0" },
        "platform": {
            "type": "web",
            "os_version": "macOS 14.6",
            "app_version": "1.0.0"
        },
        "event": { "type": "click", "element_id": "orphan.cta" },
        "viewport": { "width": 1280, "height": 720 }
    });
    ingest_batch(&app, "batch_no_screen_click", json!([click.clone()])).await;

    let events_response = app
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
    assert_eq!(events_response.status(), StatusCode::OK);
    let stored = response_json(events_response).await;
    assert_eq!(stored["events"].as_array().expect("events").len(), 1);
    assert!(stored["events"][0]["screen"].is_null());

    let checkout_totals = fetch_element_totals(&app, Some("2024-09")).await;
    assert_eq!(pay_click_count(&checkout_totals), 0);
    let elements = checkout_totals["elements"].as_array().expect("elements");
    assert!(elements.is_empty());
}

#[tokio::test]
async fn checkout_layout_versions_do_not_share_element_totals() {
    let app = test_app();
    ingest_batch(
        &app,
        "batch_layout_a",
        json!([checkout_pay_click(Some("layout-a"))]),
    )
    .await;
    ingest_batch(
        &app,
        "batch_layout_b",
        json!([checkout_pay_click(Some("layout-b"))]),
    )
    .await;

    let totals_a = fetch_element_totals(&app, Some("layout-a")).await;
    let totals_b = fetch_element_totals(&app, Some("layout-b")).await;
    assert_eq!(pay_click_count(&totals_a), 1);
    assert_eq!(pay_click_count(&totals_b), 1);

    let elements_a = totals_a["elements"].as_array().expect("elements a");
    assert_eq!(elements_a.len(), 1);
    let elements_b = totals_b["elements"].as_array().expect("elements b");
    assert_eq!(elements_b.len(), 1);
}

#[tokio::test]
async fn missing_layout_version_query_uses_unspecified_bucket() {
    let app = test_app();
    let click = checkout_pay_click(None);
    ingest_batch(&app, "batch_no_layout", json!([click])).await;

    let unspecified = fetch_element_totals(&app, None).await;
    assert_eq!(pay_click_count(&unspecified), 1);

    let named_layout = fetch_element_totals(&app, Some("2024-09")).await;
    assert_eq!(pay_click_count(&named_layout), 0);
    assert!(named_layout["elements"]
        .as_array()
        .expect("elements")
        .is_empty());
}
