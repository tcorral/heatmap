use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use heatmap_backend::{test_app, TEST_PROJECT_KEY};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

fn fixtures_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../test/fixtures")
}

fn read_fixture(name: &str) -> Value {
    let path = fixtures_dir().join(name);
    let raw = std::fs::read_to_string(&path).expect("read fixture");
    serde_json::from_str(&raw).expect("parse fixture json")
}

async fn response_json(response: axum::response::Response) -> Value {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("collect body")
        .to_bytes();
    serde_json::from_slice(&bytes).expect("parse response json")
}

fn pay_click_for_platform(platform: &str) -> Value {
    let mut click = read_fixture("click-checkout-pay-web.json");
    click["platform"]["type"] = json!(platform);
    click["session_id"] = json!(format!("sess_{platform}"));
    click["anonymous_id"] = json!(format!("anon_{platform}"));
    click
}

async fn ingest_pay_clicks(app: &axum::Router, platforms: &[&str]) {
    for platform in platforms {
        let batch = json!({
            "batch_id": format!("batch_pay_{platform}"),
            "events": [pay_click_for_platform(platform)]
        });
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
    }
}

#[tokio::test]
async fn should_group_checkout_pay_totals_by_platform_when_platform_omitted() {
    let app = test_app();
    ingest_pay_clicks(&app, &["web", "ios", "android", "macos", "windows"]).await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/elements?screen=Checkout&layout_version=2024-09&app_version=1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response_json(response).await;
    let elements = body["elements"].as_array().expect("elements");
    let mut platforms: Vec<String> = elements
        .iter()
        .filter(|row| row["element_id"] == "checkout.pay")
        .map(|row| row["platform"].as_str().unwrap_or_default().to_string())
        .collect();
    platforms.sort();
    assert_eq!(platforms, vec!["android", "ios", "macos", "web", "windows"]);
}

#[tokio::test]
async fn should_keep_single_platform_slice_when_platform_present() {
    let app = test_app();
    ingest_pay_clicks(&app, &["web", "ios"]).await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/elements?platform=web&screen=Checkout&layout_version=2024-09&app_version=1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response_json(response).await;
    let elements = body["elements"].as_array().expect("elements");
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0]["element_id"], "checkout.pay");
    assert_eq!(elements[0]["click_count"], 1);
}

#[tokio::test]
async fn should_return_empty_elements_not_error_when_comparison_has_no_rows() {
    let app = test_app();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/elements?screen=Checkout&layout_version=2024-09&app_version=1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response_json(response).await;
    assert_eq!(body["elements"].as_array().expect("elements").len(), 0);
}

#[tokio::test]
async fn should_require_platform_on_heatmap_query() {
    let app = test_app();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/heatmap?screen=Checkout&layout_version=2024-09&app_version=1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn should_require_platform_on_stills_query() {
    let app = test_app();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/stills?screen=Checkout&layout_version=2024-09")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
