use std::sync::{Arc, Mutex};

use heatmap_core::{
    batch_ingest_url, CaptureCandidate, FlushError, HeatmapBatch, HeatmapConfig, HeatmapSdk,
    InteractionEvent, PlatformInfo, PlatformType, Position, ScreenInfo, SdkInfo, Viewport,
};

fn valid_config() -> HeatmapConfig {
    HeatmapConfig {
        project_key: "pk_test".into(),
        endpoint: "https://ingest.example.com".into(),
        session_timeout_ms: HeatmapConfig::DEFAULT_SESSION_TIMEOUT_MS,
        sample_rate: HeatmapConfig::DEFAULT_SAMPLE_RATE,
        exclude_elements: vec![],
        exclude_screens: vec![],
        mask_elements: vec![],
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
        window_id: None,
    }
}

#[test]
fn should_normalize_trailing_slash_on_batch_ingest_url() {
    assert_eq!(
        batch_ingest_url("https://ingest.example.com"),
        "https://ingest.example.com/v1/batches"
    );
    assert_eq!(
        batch_ingest_url("https://ingest.example.com/"),
        "https://ingest.example.com/v1/batches"
    );
}

struct RecordingTransport {
    last_url: Arc<Mutex<Option<String>>>,
    succeed: bool,
}

impl heatmap_core::BatchTransport for RecordingTransport {
    fn send_batch(
        &self,
        endpoint: &str,
        _project_key: &str,
        _batch: &HeatmapBatch,
    ) -> Result<(), FlushError> {
        *self.last_url.lock().expect("lock") = Some(batch_ingest_url(endpoint));
        if self.succeed {
            Ok(())
        } else {
            Err(FlushError::Transport("down".into()))
        }
    }
}

#[test]
fn should_restore_queue_when_flush_transport_fails() {
    let last_url = Arc::new(Mutex::new(None));
    let transport = RecordingTransport {
        last_url: Arc::clone(&last_url),
        succeed: false,
    };
    let mut sdk = HeatmapSdk::init_with_transport(valid_config(), transport).expect("init");
    sdk.enqueue_candidate(checkout_pay_click_candidate())
        .expect("enqueue");
    assert!(sdk.flush().is_err());
    let batch = sdk.build_batch().expect("queue restored");
    assert_eq!(batch.events.len(), 1);
}

#[test]
fn should_clear_queue_after_successful_flush() {
    let transport = RecordingTransport {
        last_url: Arc::new(Mutex::new(None)),
        succeed: true,
    };
    let mut sdk = HeatmapSdk::init_with_transport(valid_config(), transport).expect("init");
    sdk.enqueue_candidate(checkout_pay_click_candidate())
        .expect("enqueue");
    sdk.flush().expect("flush");
    assert!(sdk.build_batch().is_err());
}
