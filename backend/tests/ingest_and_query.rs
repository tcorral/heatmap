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

#[tokio::test]
async fn post_batches_valid_project_key_returns_202_with_accepted_count() {
    let app = test_app();
    let batch = read_fixture("batch-checkout-journey.json");
    let response = app
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
    assert_eq!(body["batch_id"], batch["batch_id"]);
    assert_eq!(body["accepted_count"], 3);
    assert_eq!(body["rejected_count"], 0);
}

#[tokio::test]
async fn post_batches_invalid_project_key_returns_unauthorized() {
    let app = test_app();
    let batch = read_fixture("batch-checkout-journey.json");
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/batches")
                .header("X-Project-Key", "not-a-valid-key")
                .header("content-type", "application/json")
                .body(Body::from(batch.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::UNAUTHORIZED || response.status() == StatusCode::FORBIDDEN
    );
}

#[tokio::test]
async fn post_batches_duplicate_batch_id_is_idempotent() {
    let app = test_app();
    let batch = read_fixture("batch-checkout-journey.json");
    let body = batch.to_string();

    let first = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/batches")
                .header("X-Project-Key", TEST_PROJECT_KEY)
                .header("content-type", "application/json")
                .body(Body::from(body.clone()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(first.status(), StatusCode::ACCEPTED);

    let totals_before = fetch_checkout_pay_totals(&app).await;

    let second = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/batches")
                .header("X-Project-Key", TEST_PROJECT_KEY)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(second.status(), StatusCode::ACCEPTED);
    let second_body = response_json(second).await;
    assert_eq!(second_body["accepted_count"], 3);
    assert_eq!(second_body["rejected_count"], 0);

    let totals_after = fetch_checkout_pay_totals(&app).await;
    assert_eq!(totals_before, totals_after);
}

async fn fetch_checkout_pay_totals(app: &axum::Router) -> Value {
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

#[tokio::test]
async fn click_with_element_id_upserts_element_totals() {
    let app = test_app();
    let click = read_fixture("click-checkout-pay-web.json");
    let batch = json!({
        "batch_id": "batch_pay_click_only",
        "events": [click]
    });

    let ingest = app
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
    assert_eq!(ingest.status(), StatusCode::ACCEPTED);

    let totals = fetch_checkout_pay_totals(&app).await;
    let elements = totals["elements"].as_array().expect("elements array");
    let pay = elements
        .iter()
        .find(|row| row["element_id"] == "checkout.pay")
        .expect("checkout.pay row");
    assert_eq!(pay["click_count"], 1);
    assert_eq!(pay["unique_sessions"], 1);
}

#[tokio::test]
async fn click_without_element_id_stores_raw_only_no_total_row() {
    let app = test_app();
    let click = read_fixture("click-unmarked-cancel-android.json");
    let batch = json!({
        "batch_id": "batch_unmarked_android_click",
        "events": [click]
    });

    let ingest = app
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
    assert_eq!(ingest.status(), StatusCode::ACCEPTED);
    assert_eq!(response_json(ingest).await["accepted_count"], 1);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/elements?platform=android&screen=Checkout&app_version=1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let totals = response_json(response).await;
    let elements = totals["elements"].as_array().expect("elements array");
    assert!(elements.is_empty());
}

async fn fetch_android_checkout_unspecified_layout(app: &axum::Router) -> Value {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/elements?platform=android&screen=Checkout&app_version=1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    response_json(response).await
}

#[tokio::test]
async fn slice_unique_sessions_counts_unmarked_click_and_screen_view() {
    let app = test_app();

    let unmarked = read_fixture("click-unmarked-cancel-android.json");
    let unmarked_batch = json!({
        "batch_id": "batch_slice_unique_unmarked",
        "events": [unmarked]
    });
    let ingest = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/batches")
                .header("X-Project-Key", TEST_PROJECT_KEY)
                .header("content-type", "application/json")
                .body(Body::from(unmarked_batch.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(ingest.status(), StatusCode::ACCEPTED);

    let android_totals = fetch_android_checkout_unspecified_layout(&app).await;
    assert_eq!(android_totals["unique_sessions"], 1);
    let android_elements = android_totals["elements"]
        .as_array()
        .expect("elements array");
    assert!(android_elements.is_empty());

    let screen_view = read_fixture("screen-view-home-ios.json");
    let screen_view_batch = json!({
        "batch_id": "batch_slice_unique_screen_view",
        "events": [screen_view]
    });
    let ingest_sv = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/batches")
                .header("X-Project-Key", TEST_PROJECT_KEY)
                .header("content-type", "application/json")
                .body(Body::from(screen_view_batch.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(ingest_sv.status(), StatusCode::ACCEPTED);

    let ios_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/elements?platform=ios&screen=Home&layout_version=2024-09&app_version=1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(ios_response.status(), StatusCode::OK);
    let ios_totals = response_json(ios_response).await;
    assert_eq!(ios_totals["unique_sessions"], 1);
    assert!(ios_totals["elements"]
        .as_array()
        .expect("elements array")
        .is_empty());
}

fn android_checkout_pay_click(session_id: &str) -> Value {
    json!({
        "schema_version": 1,
        "session_id": session_id,
        "anonymous_id": "anon_android_pay",
        "timestamp": 1726771204600.0,
        "sdk": { "name": "heatmap-android", "version": "0.1.0" },
        "platform": {
            "type": "android",
            "os_version": "Android 14",
            "app_version": "1.0.0"
        },
        "screen": { "id": "Checkout" },
        "event": { "type": "click", "element_id": "checkout.pay" },
        "position": { "x": 0.5, "y": 0.5 },
        "viewport": { "width": 412, "height": 915 }
    })
}

#[tokio::test]
async fn slice_unique_sessions_distinct_from_per_element_unique_sessions() {
    let app = test_app();

    let unmarked = read_fixture("click-unmarked-cancel-android.json");
    let pay = android_checkout_pay_click("sess_android_pay_identified");
    let batch = json!({
        "batch_id": "batch_slice_two_sessions",
        "events": [unmarked, pay]
    });
    let ingest = app
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
    assert_eq!(ingest.status(), StatusCode::ACCEPTED);

    let totals = fetch_android_checkout_unspecified_layout(&app).await;
    assert_eq!(totals["unique_sessions"], 2);
    let elements = totals["elements"].as_array().expect("elements array");
    let pay_row = elements
        .iter()
        .find(|row| row["element_id"] == "checkout.pay")
        .expect("checkout.pay row");
    assert_eq!(pay_row["unique_sessions"], 1);
}

#[tokio::test]
async fn post_batches_rejects_click_with_out_of_range_position() {
    let app = test_app();
    let mut click = read_fixture("click-checkout-pay-web.json");
    click["position"] = json!({ "x": 1.5, "y": 0.5 });
    let batch = json!({
        "batch_id": "batch_bad_position",
        "events": [click]
    });
    let response = app
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
    assert_eq!(body["accepted_count"], 0);
    assert_eq!(body["rejected_count"], 1);
}
