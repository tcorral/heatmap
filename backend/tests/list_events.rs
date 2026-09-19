use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use heatmap_backend::{test_app, AppConfig, TEST_PROJECT_KEY};
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
    let db = heatmap_backend::db::open_in_memory().expect("db");
    let config = AppConfig::default().with_test_events(true);
    heatmap_backend::app(config, db)
}

#[tokio::test]
async fn get_events_is_hidden_unless_test_events_enabled() {
    let app = test_app();
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/events")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_events_returns_ingested_event_sequence() {
    let app = app_with_test_events();
    let batch = json!({
        "batch_id": "batch_e2e",
        "events": [
            {
                "schema_version": 1,
                "session_id": "sess_e2e",
                "anonymous_id": "anon_e2e",
                "timestamp": 1.0,
                "sdk": { "name": "heatmap-web", "version": "0.1.0" },
                "platform": {
                    "type": "web",
                    "os_version": "browser",
                    "app_version": "1.0.0"
                },
                "screen": { "id": "Home" },
                "event": { "type": "session_start" },
                "viewport": { "width": 1280, "height": 720 }
            }
        ]
    });
    app.clone()
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

    let response = app
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
    let body = response_json(response).await;
    let events = body["events"].as_array().expect("events");
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["event"]["type"], "session_start");
}
