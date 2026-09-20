use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

use heatmap_core::{
    batch_ingest_url, BatchTransport, CaptureCandidate, FlushError, HeatmapBatch, HeatmapConfig,
    HeatmapSdk, HttpBatchTransport, InteractionEvent, PlatformInfo, PlatformType, Position,
    ScreenInfo, SdkInfo, Viewport,
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
    assert_eq!(batch.events.len(), 2);
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

fn serve_one_http_response(status: &str, extra_headers: &str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr");
    let status = status.to_string();
    let extra_headers = extra_headers.to_string();
    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept");
        let mut buf = [0u8; 2048];
        let _ = stream.read(&mut buf);
        let body = "{}";
        let response = format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n{extra_headers}Connection: close\r\n\r\n{body}",
            body.len()
        );
        let _ = stream.write_all(response.as_bytes());
    });
    format!("http://{addr}")
}

fn empty_batch() -> HeatmapBatch {
    HeatmapBatch {
        batch_id: "0199batch000000000000000000000001".into(),
        events: vec![],
    }
}

#[test]
fn should_classify_http_401_as_http_status_not_transport() {
    let endpoint = serve_one_http_response("401 Unauthorized", "");
    let err = HttpBatchTransport::new()
        .send_batch(&endpoint, "pk_invalid", &empty_batch())
        .expect_err("401");
    assert_eq!(err, FlushError::http_status(401, None));
}

#[test]
fn should_preserve_retry_after_when_http_429() {
    let endpoint = serve_one_http_response("429 Too Many Requests", "Retry-After: 7\r\n");
    let err = HttpBatchTransport::new()
        .send_batch(&endpoint, "pk_test", &empty_batch())
        .expect_err("429");
    assert_eq!(err, FlushError::http_status(429, Some(7)));
}
