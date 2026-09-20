use std::sync::{Arc, Mutex};

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use heatmap_backend::{test_app, TEST_PROJECT_KEY};
use heatmap_core::{
    batch_ingest_url, CaptureCandidate, FlushError, HeatmapBatch, HeatmapConfig, HeatmapSdk,
    InteractionEvent, PlatformInfo, PlatformType, Position, ScreenInfo, SdkInfo, Viewport,
};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

fn checkout_config(endpoint: &str) -> HeatmapConfig {
    HeatmapConfig {
        project_key: TEST_PROJECT_KEY.into(),
        endpoint: endpoint.into(),
        session_timeout_ms: HeatmapConfig::DEFAULT_SESSION_TIMEOUT_MS,
        sample_rate: HeatmapConfig::DEFAULT_SAMPLE_RATE,
        exclude_elements: vec![],
        exclude_screens: vec![],
        mask_elements: vec![],
        app_version: heatmap_core::HeatmapConfig::DEFAULT_APP_VERSION.into(),
    }
}

fn checkout_pay_click_candidate() -> CaptureCandidate {
    CaptureCandidate {
        timestamp: 1_726_771_205_000,
        sdk: SdkInfo {
            name: "heatmap-web".into(),
            version: "0.1.0".into(),
        },
        platform: PlatformInfo {
            platform_type: PlatformType::Web,
            os_version: "macOS 14.6".into(),
            app_version: "1.0.0".into(),
        },
        screen: Some(ScreenInfo {
            id: "Checkout".into(),
            layout_version: Some("2024-09".into()),
        }),
        interaction: InteractionEvent::click(Some("checkout.pay".into())).expect("click"),
        position: Some(Position::new(0.5, 0.5).expect("position")),
        viewport: Some(Viewport {
            width: 1280,
            height: 720,
        }),
        window_id: Some("win_browser_main".into()),
    }
}

struct RecordAndForwardTransport {
    app: axum::Router,
    last_request_url: Arc<Mutex<Option<String>>>,
}

impl heatmap_core::BatchTransport for RecordAndForwardTransport {
    fn send_batch(
        &self,
        endpoint: &str,
        project_key: &str,
        batch: &HeatmapBatch,
    ) -> Result<(), FlushError> {
        let url = batch_ingest_url(endpoint);
        *self.last_request_url.lock().expect("lock") = Some(url);
        let response = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.app
                    .clone()
                    .oneshot(
                        Request::builder()
                            .method("POST")
                            .uri("/v1/batches")
                            .header("X-Project-Key", project_key)
                            .header("content-type", "application/json")
                            .body(Body::from(
                                serde_json::to_string(batch).expect("serialize batch"),
                            ))
                            .expect("request"),
                    )
                    .await
            })
        })
        .map_err(|err| FlushError::Transport(err.to_string()))?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(FlushError::http_status(response.status().as_u16(), None))
        }
    }
}

async fn fetch_checkout_pay_click_count(app: &axum::Router) -> u64 {
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
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("collect")
        .to_bytes();
    let totals: Value = serde_json::from_slice(&bytes).expect("json");
    let elements = totals["elements"].as_array().expect("elements");
    elements
        .iter()
        .find(|row| row["element_id"] == "checkout.pay")
        .map(|row| row["click_count"].as_u64().unwrap_or(0))
        .unwrap_or(0)
}

#[tokio::test(flavor = "multi_thread")]
async fn should_show_checkout_pay_total_after_flush_uploads_synthetic_click_once() {
    let app = test_app();
    let last_url = Arc::new(Mutex::new(None));
    let transport = RecordAndForwardTransport {
        app: app.clone(),
        last_request_url: Arc::clone(&last_url),
    };
    let endpoint = "https://ingest.example.test";
    let mut sdk =
        HeatmapSdk::init_with_transport(checkout_config(endpoint), transport).expect("init");
    sdk.enqueue_candidate(checkout_pay_click_candidate())
        .expect("enqueue");
    sdk.flush().expect("flush");
    assert_eq!(fetch_checkout_pay_click_count(&app).await, 1);

    sdk.flush().expect("second flush empty queue");
    assert_eq!(fetch_checkout_pay_click_count(&app).await, 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn should_post_batch_to_configured_https_ingest_base() {
    let app = test_app();
    let last_url = Arc::new(Mutex::new(None));
    let transport = RecordAndForwardTransport {
        app,
        last_request_url: Arc::clone(&last_url),
    };
    let endpoint = "https://ingest.example.test/base/";
    let mut sdk =
        HeatmapSdk::init_with_transport(checkout_config(endpoint), transport).expect("init");
    sdk.enqueue_candidate(checkout_pay_click_candidate())
        .expect("enqueue");
    sdk.flush().expect("flush");

    let recorded = last_url.lock().expect("lock").clone().expect("recorded url");
    assert_eq!(
        recorded,
        "https://ingest.example.test/base/v1/batches"
    );
}

struct UnreachableTransport;

impl heatmap_core::BatchTransport for UnreachableTransport {
    fn send_batch(
        &self,
        _endpoint: &str,
        _project_key: &str,
        _batch: &HeatmapBatch,
    ) -> Result<(), FlushError> {
        Err(FlushError::Transport("ingest unreachable".into()))
    }
}

#[tokio::test]
async fn should_keep_host_usable_when_ingest_is_down() {
    let mut sdk = HeatmapSdk::init_with_transport(
        checkout_config("https://ingest.example.test"),
        UnreachableTransport,
    )
    .expect("init");
    sdk.enqueue_candidate(checkout_pay_click_candidate())
        .expect("enqueue");
    assert!(sdk.flush().is_err());

    sdk.enqueue_candidate(checkout_pay_click_candidate())
        .expect("enqueue after failed flush");
    let batch = sdk.build_batch().expect("events retained");
    assert_eq!(batch.events.len(), 3);
}
