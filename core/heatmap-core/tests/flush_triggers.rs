use std::sync::{Arc, Mutex};

use heatmap_core::{
    batch_ingest_url, CaptureCandidate, FlushError, HeatmapBatch, HeatmapConfig, HeatmapSdk,
    InteractionEvent, ManualClock, PlatformInfo, PlatformType, Position, ScreenInfo, SdkInfo,
    FLUSH_INTERVAL_MS, FLUSH_MAX_BYTES, FLUSH_MAX_EVENTS, Viewport,
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

fn click_candidate(timestamp: i64, element: &str) -> CaptureCandidate {
    CaptureCandidate {
        timestamp,
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
            layout_version: None,
        }),
        interaction: InteractionEvent::click(Some(element.into())).expect("click"),
        position: Some(Position::new(0.5, 0.5).expect("position")),
        viewport: Some(Viewport {
            width: 1280,
            height: 720,
        }),
        window_id: None,
    }
}

struct RecordingTransport {
    batches: Arc<Mutex<Vec<HeatmapBatch>>>,
    outcome: Arc<Mutex<TransportOutcome>>,
}

#[derive(Clone)]
enum TransportOutcome {
    Ok,
    Http(u16, Option<u64>),
    Transport(String),
}

impl RecordingTransport {
    fn new(outcome: TransportOutcome) -> Self {
        Self {
            batches: Arc::new(Mutex::new(Vec::new())),
            outcome: Arc::new(Mutex::new(outcome)),
        }
    }

    fn sent_batch_ids(&self) -> Vec<String> {
        self.batches
            .lock()
            .expect("lock")
            .iter()
            .map(|batch| batch.batch_id.clone())
            .collect()
    }

    fn send_count(&self) -> usize {
        self.batches.lock().expect("lock").len()
    }
}

impl heatmap_core::BatchTransport for RecordingTransport {
    fn send_batch(
        &self,
        endpoint: &str,
        _project_key: &str,
        batch: &HeatmapBatch,
    ) -> Result<(), FlushError> {
        assert_eq!(
            batch_ingest_url(endpoint),
            "https://ingest.example.com/v1/batches"
        );
        self.batches.lock().expect("lock").push(batch.clone());
        match self.outcome.lock().expect("lock").clone() {
            TransportOutcome::Ok => Ok(()),
            TransportOutcome::Transport(message) => Err(FlushError::Transport(message)),
            TransportOutcome::Http(code, retry_after_secs) => Err(FlushError::http_status(
                code,
                retry_after_secs,
            )),
        }
    }
}

fn sdk_with_clock(
    transport: RecordingTransport,
    clock: Arc<ManualClock>,
) -> (HeatmapSdk, RecordingTransport) {
    let batches = Arc::clone(&transport.batches);
    let outcome = Arc::clone(&transport.outcome);
    let transport_for_sdk = RecordingTransport { batches, outcome };
    let sdk = HeatmapSdk::init_with_transport_store_and_clock(
        valid_config(),
        transport_for_sdk,
        Arc::new(heatmap_core::InMemoryIdentityStore::new()),
        clock,
    )
    .expect("init");
    (sdk, transport)
}


#[test]
fn should_auto_flush_when_event_count_reaches_limit_with_new_batch_id() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    let mut ts = 1_000_i64;
    while transport.send_count() == 0 {
        sdk.enqueue_candidate(click_candidate(ts, "checkout.pay"))
            .expect("enqueue");
        ts += 1;
    }
    assert_eq!(transport.send_count(), 1);
    let ids = transport.sent_batch_ids();
    assert_eq!(ids.len(), 1);
    assert!(ids[0].starts_with("batch_"));
    assert!(sdk.build_batch().is_err());
}

#[test]
fn should_auto_flush_when_serialized_bytes_reach_limit_with_new_batch_id() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    let mut ts = 1_000_i64;
    let padding = "x".repeat(900);
    while transport.send_count() == 0 {
        sdk.enqueue_candidate(click_candidate(ts, &padding))
            .expect("enqueue");
        ts += 1;
    }
    assert_eq!(transport.send_count(), 1);
    let ids = transport.sent_batch_ids();
    assert!(ids[0].starts_with("batch_"));
}

#[test]
fn should_auto_flush_after_interval_elapses_with_new_batch_id() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.enqueue_candidate(click_candidate(0, "checkout.pay"))
        .expect("enqueue");
    assert_eq!(transport.send_count(), 0);
    clock.advance_ms(FLUSH_INTERVAL_MS);
    sdk.enqueue_candidate(click_candidate(FLUSH_INTERVAL_MS as i64, "checkout.pay"))
        .expect("enqueue after interval");
    assert_eq!(transport.send_count(), 1);
    assert!(transport.sent_batch_ids()[0].starts_with("batch_"));
}

#[test]
fn should_auto_flush_after_interval_without_another_event() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.enqueue_candidate(click_candidate(0, "checkout.pay"))
        .expect("enqueue");
    assert_eq!(transport.send_count(), 0);
    clock.advance_ms(FLUSH_INTERVAL_MS);
    sdk.poll_flush().expect("poll");
    assert_eq!(transport.send_count(), 1);
    assert!(transport.sent_batch_ids()[0].starts_with("batch_"));
}

#[test]
fn should_flush_on_background_with_new_batch_id() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.enqueue_candidate(click_candidate(0, "checkout.pay"))
        .expect("enqueue");
    sdk.on_background();
    assert_eq!(transport.send_count(), 1);
    assert!(transport.sent_batch_ids()[0].starts_with("batch_"));
}

#[test]
fn should_flush_explicitly_with_new_batch_id() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.enqueue_candidate(click_candidate(0, "checkout.pay"))
        .expect("enqueue");
    sdk.flush().expect("flush");
    assert_eq!(transport.send_count(), 1);
    let first = transport.sent_batch_ids()[0].clone();
    sdk.enqueue_candidate(click_candidate(1, "checkout.pay"))
        .expect("enqueue again");
    sdk.flush().expect("second flush");
    let ids = transport.sent_batch_ids();
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1]);
    assert_eq!(ids[0], first);
}

#[test]
fn should_reuse_batch_id_on_transient_http_5xx_and_restore_queue() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Http(503, None));
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.enqueue_candidate(click_candidate(0, "checkout.pay"))
        .expect("enqueue");
    assert!(sdk.flush().is_err());
    *transport.outcome.lock().expect("lock") = TransportOutcome::Ok;
    sdk.flush().expect("retry");
    let ids = transport.sent_batch_ids();
    assert_eq!(ids.len(), 2);
    assert_eq!(ids[0], ids[1]);
}

#[test]
fn should_reuse_batch_id_on_http_429_and_honor_retry_after() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Http(429, Some(5)));
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.enqueue_candidate(click_candidate(0, "checkout.pay"))
        .expect("enqueue");
    assert!(sdk.flush().is_err());
    assert_eq!(transport.send_count(), 1);
    *transport.outcome.lock().expect("lock") = TransportOutcome::Ok;
    assert!(sdk.flush().is_ok());
    assert_eq!(transport.send_count(), 1);
    clock.advance_ms(5_000);
    sdk.flush().expect("retry after wait");
    let ids = transport.sent_batch_ids();
    assert_eq!(ids.len(), 2);
    assert_eq!(ids[0], ids[1]);
}

#[test]
fn should_drop_batch_on_permanent_http_401_without_retry_loop() {
    should_drop_permanent_status_without_retry(401);
}

#[test]
fn should_drop_batch_on_permanent_http_403_without_retry_loop() {
    should_drop_permanent_status_without_retry(403);
}

#[test]
fn should_drop_batch_on_permanent_http_400_without_retry_loop() {
    should_drop_permanent_status_without_retry(400);
}

#[test]
fn should_drop_batch_on_permanent_http_413_without_retry_loop() {
    should_drop_permanent_status_without_retry(413);
}

fn should_drop_permanent_status_without_retry(status: u16) {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Http(status, None));
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.enqueue_candidate(click_candidate(0, "checkout.pay"))
        .expect("enqueue");
    assert!(sdk.flush().is_err());
    assert_eq!(sdk.diagnostics().permanent_drop, 1);
    assert!(sdk.build_batch().is_err());
    assert!(sdk.flush().is_ok());
    assert_eq!(transport.send_count(), 1);
}

#[test]
fn should_reuse_batch_id_on_transport_failure() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Transport("down".into()));
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.enqueue_candidate(click_candidate(0, "checkout.pay"))
        .expect("enqueue");
    assert!(sdk.flush().is_err());
    *transport.outcome.lock().expect("lock") = TransportOutcome::Ok;
    sdk.flush().expect("retry");
    let ids = transport.sent_batch_ids();
    assert_eq!(ids[0], ids[1]);
}

#[test]
fn flush_limits_constants_match_spec() {
    assert_eq!(FLUSH_MAX_EVENTS, 50);
    assert_eq!(FLUSH_MAX_BYTES, 65536);
    assert_eq!(FLUSH_INTERVAL_MS, 10_000);
}
