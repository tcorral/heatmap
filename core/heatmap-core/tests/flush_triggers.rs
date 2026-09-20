use std::sync::{Arc, Mutex};

use heatmap_core::{
    batch_ingest_url, CaptureCandidate, FlushError, HeatmapBatch, HeatmapConfig, HeatmapSdk,
    InteractionEvent, ManualClock, PlatformInfo, PlatformType, Position, ScreenInfo, SdkInfo,
    Viewport, FLUSH_INTERVAL_MS, FLUSH_MAX_BYTES, FLUSH_MAX_EVENTS, INGEST_MAX_BYTES,
    INGEST_MAX_EVENTS,
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

    fn last_batch_event_count(&self) -> usize {
        self.batches
            .lock()
            .expect("lock")
            .last()
            .map(|batch| batch.events.len())
            .unwrap_or(0)
    }

    fn batch_event_timestamps(&self, index: usize) -> Vec<i64> {
        self.batches
            .lock()
            .expect("lock")
            .get(index)
            .map(|batch| batch.events.iter().map(|event| event.timestamp).collect())
            .unwrap_or_default()
    }

    fn last_batch_wire_bytes(&self) -> usize {
        self.batches
            .lock()
            .expect("lock")
            .last()
            .map(|batch| {
                serde_json::to_vec(batch)
                    .map(|bytes| bytes.len())
                    .unwrap_or(0)
            })
            .unwrap_or(0)
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
            TransportOutcome::Http(code, retry_after_secs) => {
                Err(FlushError::http_status(code, retry_after_secs))
            }
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

fn enqueue_until_event_flush_threshold(sdk: &mut HeatmapSdk) {
    let mut ts = 1_000_i64;
    for _ in 0..FLUSH_MAX_EVENTS * 2 {
        if sdk.queued_event_count_for_testing() >= FLUSH_MAX_EVENTS {
            return;
        }
        sdk.enqueue_candidate(click_candidate(ts, "checkout.pay"))
            .expect("enqueue");
        ts += 1;
    }
}

fn enqueue_until_byte_flush_threshold(sdk: &mut HeatmapSdk) {
    let mut ts = 1_000_i64;
    let padding = "x".repeat(900);
    for _ in 0..FLUSH_MAX_EVENTS * 2 {
        if sdk.flush_byte_limit_reached_for_testing() {
            return;
        }
        sdk.enqueue_candidate(click_candidate(ts, &padding))
            .expect("enqueue");
        ts += 1;
    }
}

#[test]
fn should_not_call_transport_when_enqueue_reaches_event_flush_threshold() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    enqueue_until_event_flush_threshold(&mut sdk);
    assert_eq!(transport.send_count(), 0);
    assert!(sdk.queued_event_count_for_testing() >= FLUSH_MAX_EVENTS);
}

#[test]
fn should_not_call_transport_when_enqueue_reaches_byte_flush_threshold() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    enqueue_until_byte_flush_threshold(&mut sdk);
    assert_eq!(transport.send_count(), 0);
    assert!(sdk.flush_byte_limit_reached_for_testing());
}

#[test]
fn should_not_call_transport_when_enqueue_after_flush_interval() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.enqueue_candidate(click_candidate(0, "checkout.pay"))
        .expect("enqueue");
    clock.advance_ms(FLUSH_INTERVAL_MS);
    sdk.enqueue_candidate(click_candidate(FLUSH_INTERVAL_MS as i64, "checkout.pay"))
        .expect("enqueue after interval");
    assert_eq!(transport.send_count(), 0);
}

#[test]
fn should_upload_on_poll_flush_after_event_flush_threshold() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    enqueue_until_event_flush_threshold(&mut sdk);
    sdk.poll_flush().expect("poll");
    assert_eq!(transport.send_count(), 1);
    assert!(transport.sent_batch_ids()[0].starts_with("batch_"));
}

#[test]
fn should_upload_on_explicit_flush_after_event_flush_threshold() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    enqueue_until_event_flush_threshold(&mut sdk);
    sdk.flush().expect("flush");
    assert_eq!(transport.send_count(), 1);
    assert!(sdk.build_batch().is_err());
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
fn should_not_auto_flush_when_defer_network_flush_is_enabled() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.set_defer_network_flush(true);
    let mut ts = 1_000_i64;
    while sdk.queued_event_count_for_testing() < FLUSH_MAX_EVENTS {
        sdk.enqueue_candidate(click_candidate(ts, "checkout.pay"))
            .expect("enqueue");
        ts += 1;
    }
    assert_eq!(transport.send_count(), 0);
    assert!(sdk.queued_event_count_for_testing() >= FLUSH_MAX_EVENTS);
}

#[test]
fn should_not_poll_flush_when_defer_network_flush_is_enabled() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.set_defer_network_flush(true);
    sdk.enqueue_candidate(click_candidate(0, "checkout.pay"))
        .expect("enqueue");
    clock.advance_ms(FLUSH_INTERVAL_MS);
    sdk.poll_flush().expect("poll");
    assert_eq!(transport.send_count(), 0);
    assert!(sdk.queued_event_count_for_testing() > 0);
}

#[test]
fn should_not_background_flush_when_defer_network_flush_is_enabled() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.set_defer_network_flush(true);
    sdk.enqueue_candidate(click_candidate(0, "checkout.pay"))
        .expect("enqueue");
    sdk.on_background();
    assert_eq!(transport.send_count(), 0);
    assert!(sdk.queued_event_count_for_testing() > 0);
}

#[test]
fn should_still_explicit_flush_when_defer_network_flush_is_enabled() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.set_defer_network_flush(true);
    sdk.enqueue_candidate(click_candidate(0, "checkout.pay"))
        .expect("enqueue");
    sdk.flush().expect("flush");
    assert_eq!(transport.send_count(), 1);
}

#[test]
fn flush_limits_constants_match_spec() {
    assert_eq!(FLUSH_MAX_EVENTS, 50);
    assert_eq!(FLUSH_MAX_BYTES, 65536);
    assert_eq!(FLUSH_INTERVAL_MS, 10_000);
    assert_eq!(INGEST_MAX_EVENTS, 200);
    assert_eq!(heatmap_core::INGEST_MAX_BYTES, 256 * 1024);
}

#[test]
fn should_retry_original_front_batch_when_queue_exceeds_ingest_ceiling() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Http(503, None));
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.set_defer_network_flush(true);
    let mut ts = 1_000_i64;
    while sdk.queued_event_count_for_testing() < INGEST_MAX_EVENTS + 5 {
        sdk.enqueue_candidate(click_candidate(ts, "checkout.pay"))
            .expect("enqueue");
        ts += 1;
    }
    assert!(sdk.flush().is_err());
    let first_id = transport.sent_batch_ids()[0].clone();
    let first_timestamps = transport.batch_event_timestamps(0);
    assert_eq!(first_timestamps.len(), INGEST_MAX_EVENTS);
    *transport.outcome.lock().expect("lock") = TransportOutcome::Ok;
    sdk.flush().expect("retry original batch");
    let ids = transport.sent_batch_ids();
    assert_eq!(ids[0], first_id);
    assert_eq!(ids[1], first_id);
    assert_eq!(transport.batch_event_timestamps(1), first_timestamps);
    assert!(sdk.queued_event_count_for_testing() > 0);
}

#[test]
fn should_upload_at_most_ingest_event_ceiling_per_batch() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Transport("down".into()));
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.set_defer_network_flush(true);
    let mut ts = 1_000_i64;
    while sdk.queued_event_count_for_testing() < INGEST_MAX_EVENTS + 5 {
        sdk.enqueue_candidate(click_candidate(ts, "checkout.pay"))
            .expect("enqueue");
        ts += 1;
    }
    *transport.outcome.lock().expect("lock") = TransportOutcome::Ok;
    sdk.flush().expect("flush");
    assert_eq!(transport.last_batch_event_count(), INGEST_MAX_EVENTS);
    assert!(sdk.queued_event_count_for_testing() > 0);
}

#[test]
fn should_keep_serialized_batch_within_ingest_byte_ceiling() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.set_defer_network_flush(true);
    let pad = 1_500;
    let mut ts = 1_000_i64;
    while sdk.queued_event_count_for_testing() < INGEST_MAX_EVENTS {
        sdk.enqueue_candidate(click_candidate(ts, &format!("pay.{}", "x".repeat(pad))))
            .expect("enqueue");
        ts += 1;
    }
    sdk.flush().expect("flush");
    assert!(transport.last_batch_event_count() > 0);
    assert!(transport.last_batch_wire_bytes() <= INGEST_MAX_BYTES);
    assert!(sdk.queued_event_count_for_testing() > 0);
}

#[test]
fn should_drop_individually_oversized_event_instead_of_uploading_it() {
    let clock = Arc::new(ManualClock::new(0));
    let transport = RecordingTransport::new(TransportOutcome::Ok);
    let (mut sdk, transport) = sdk_with_clock(transport, Arc::clone(&clock));
    sdk.set_defer_network_flush(true);
    sdk.enqueue_candidate(click_candidate(1_000, &"x".repeat(INGEST_MAX_BYTES)))
        .expect("enqueue");
    sdk.flush().expect("first flush session_start");
    sdk.flush().expect("second flush drops oversized click");
    assert!(transport.last_batch_wire_bytes() <= INGEST_MAX_BYTES);
    assert!(sdk.diagnostics().permanent_drop >= 1);
    assert_eq!(sdk.queued_event_count_for_testing(), 0);
}
