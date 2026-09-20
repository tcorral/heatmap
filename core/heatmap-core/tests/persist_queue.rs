use std::sync::{Arc, Mutex};

use heatmap_core::{
    batch_ingest_url, CaptureCandidate, FlushError, HeatmapBatch, HeatmapConfig, HeatmapSdk,
    InMemoryIdentityStore, InteractionEvent, PlatformInfo, PlatformType, Position, QueueLimits,
    SdkInfo, Viewport, QUEUE_MAX_BYTES, QUEUE_MAX_EVENTS,
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
        app_version: HeatmapConfig::DEFAULT_APP_VERSION.into(),
    }
}

fn minimal_click_candidate(timestamp: i64) -> CaptureCandidate {
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
        screen: None,
        interaction: InteractionEvent::click(None).expect("click"),
        position: Some(Position::new(0.5, 0.5).expect("position")),
        viewport: Some(Viewport {
            width: 1280,
            height: 720,
        }),
        window_id: None,
    }
}

struct CountingTransport {
    send_count: Arc<Mutex<u64>>,
    last_batch_events: Arc<Mutex<usize>>,
}

impl CountingTransport {
    fn new() -> (Self, Arc<Mutex<u64>>, Arc<Mutex<usize>>) {
        let send_count = Arc::new(Mutex::new(0));
        let last_batch_events = Arc::new(Mutex::new(0));
        (
            Self {
                send_count: Arc::clone(&send_count),
                last_batch_events: Arc::clone(&last_batch_events),
            },
            send_count,
            last_batch_events,
        )
    }
}

struct OfflineTransport;

impl heatmap_core::BatchTransport for OfflineTransport {
    fn send_batch(
        &self,
        _endpoint: &str,
        _project_key: &str,
        _batch: &HeatmapBatch,
    ) -> Result<(), heatmap_core::FlushError> {
        Err(heatmap_core::FlushError::Transport("offline".into()))
    }
}

struct RecordingTransport {
    batches: Arc<Mutex<Vec<HeatmapBatch>>>,
    outcome: Arc<Mutex<Result<(), FlushError>>>,
}

impl RecordingTransport {
    fn new(outcome: Result<(), FlushError>) -> Self {
        Self {
            batches: Arc::new(Mutex::new(Vec::new())),
            outcome: Arc::new(Mutex::new(outcome)),
        }
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
        self.outcome.lock().expect("lock").clone()
    }
}

impl heatmap_core::BatchTransport for CountingTransport {
    fn send_batch(
        &self,
        endpoint: &str,
        _project_key: &str,
        batch: &HeatmapBatch,
    ) -> Result<(), heatmap_core::FlushError> {
        assert_eq!(
            batch_ingest_url(endpoint),
            "https://ingest.example.com/v1/batches"
        );
        *self.send_count.lock().expect("lock") += 1;
        *self.last_batch_events.lock().expect("lock") = batch.events.len();
        Ok(())
    }
}

#[test]
fn should_upload_offline_events_once_after_restore_and_flush() {
    let store = Arc::new(InMemoryIdentityStore::new());
    {
        let mut sdk = HeatmapSdk::init_with_transport_and_store(
            valid_config(),
            OfflineTransport,
            store.clone(),
        )
        .expect("init");
        sdk.enqueue_candidate(minimal_click_candidate(1_000))
            .expect("enqueue");
        sdk.on_background();
    }
    let (transport, restore_sends, restore_events) = CountingTransport::new();
    let mut restored =
        HeatmapSdk::init_with_transport_and_store(valid_config(), transport, store.clone())
            .expect("restore init");
    restored.flush().expect("flush restored queue");
    assert_eq!(*restore_sends.lock().expect("lock"), 1);
    assert_eq!(*restore_events.lock().expect("lock"), 2);
    let (transport, third_sends, _) = CountingTransport::new();
    let mut third = HeatmapSdk::init_with_transport_and_store(valid_config(), transport, store)
        .expect("third init");
    third.flush().expect("noop flush");
    assert_eq!(*third_sends.lock().expect("lock"), 0);
    assert_eq!(third.diagnostics().queue_dropped, 0);
}

#[test]
fn should_restore_queued_events_after_enqueue_without_background() {
    let store = Arc::new(InMemoryIdentityStore::new());
    {
        let mut sdk = HeatmapSdk::init_with_transport_and_store(
            valid_config(),
            OfflineTransport,
            store.clone(),
        )
        .expect("init");
        sdk.enqueue_candidate(minimal_click_candidate(3_000))
            .expect("enqueue");
    }
    let (transport, _, _) = CountingTransport::new();
    let mut restored = HeatmapSdk::init_with_transport_and_store(valid_config(), transport, store)
        .expect("restore");
    let batch = restored.build_batch().expect("pending");
    assert_eq!(batch.events.len(), 2);
}

#[test]
fn should_not_crash_when_relaunching_with_pending_persisted_events() {
    let store = Arc::new(InMemoryIdentityStore::new());
    {
        let mut sdk = HeatmapSdk::init_with_transport_and_store(
            valid_config(),
            OfflineTransport,
            store.clone(),
        )
        .expect("init");
        sdk.enqueue_candidate(minimal_click_candidate(2_000))
            .expect("enqueue");
        sdk.enqueue_candidate(minimal_click_candidate(2_001))
            .expect("enqueue");
        sdk.on_background();
    }
    let (transport, _, _) = CountingTransport::new();
    let mut sdk = HeatmapSdk::init_with_transport_and_store(valid_config(), transport, store)
        .expect("relaunch");
    let batch = sdk.build_batch().expect("pending batch");
    assert_eq!(batch.events.len(), 3);
}

#[test]
fn should_drop_oldest_events_at_cap_and_keep_host_capturing() {
    let store = Arc::new(InMemoryIdentityStore::new());
    let (transport, _, _) = CountingTransport::new();
    let mut sdk =
        HeatmapSdk::init_with_transport_and_store(valid_config(), transport, store).expect("init");
    sdk.set_queue_limits_for_testing(QueueLimits {
        max_events: 2,
        max_bytes: usize::MAX,
    });
    sdk.enqueue_candidate(minimal_click_candidate(10))
        .expect("first");
    sdk.enqueue_candidate(minimal_click_candidate(11))
        .expect("second");
    sdk.enqueue_candidate(minimal_click_candidate(12))
        .expect("third after overflow");
    let batch = sdk.build_batch().expect("batch");
    assert_eq!(batch.events.len(), 2);
    assert_eq!(batch.events[0].timestamp, 11);
    assert_eq!(batch.events[1].timestamp, 12);
}

#[test]
fn should_increment_queue_dropped_diagnostic_when_overflow_drops_events() {
    let store = Arc::new(InMemoryIdentityStore::new());
    let (transport, _, _) = CountingTransport::new();
    let mut sdk =
        HeatmapSdk::init_with_transport_and_store(valid_config(), transport, store).expect("init");
    sdk.set_queue_limits_for_testing(QueueLimits {
        max_events: 1,
        max_bytes: usize::MAX,
    });
    sdk.enqueue_candidate(minimal_click_candidate(100))
        .expect("first");
    sdk.enqueue_candidate(minimal_click_candidate(101))
        .expect("overflow");
    assert_eq!(sdk.diagnostics().queue_dropped, 2);
}

#[test]
fn should_keep_unacked_batch_after_restart_between_take_and_complete() {
    let store = Arc::new(InMemoryIdentityStore::new());
    let taken = {
        let mut sdk = HeatmapSdk::init_with_transport_and_store(
            valid_config(),
            OfflineTransport,
            store.clone(),
        )
        .expect("init");
        sdk.enqueue_candidate(minimal_click_candidate(1_000))
            .expect("enqueue");
        let taken = sdk.take_batch_for_upload().expect("take in-flight batch");
        sdk.enqueue_candidate(minimal_click_candidate(2_000))
            .expect("enqueue during upload");
        taken
    };
    let transport = RecordingTransport::new(Ok(()));
    let batches = Arc::clone(&transport.batches);
    let mut restored =
        HeatmapSdk::init_with_transport_and_store(valid_config(), transport, store.clone())
            .expect("restore during in-flight");
    restored.flush().expect("retry persisted in-flight");
    let sent = batches.lock().expect("lock")[0].clone();
    assert_eq!(sent.batch_id, taken.batch_id);
    assert_eq!(
        sent.events
            .iter()
            .map(|event| event.timestamp)
            .collect::<Vec<_>>(),
        taken
            .events
            .iter()
            .map(|event| event.timestamp)
            .collect::<Vec<_>>()
    );
    restored.flush().expect("later events");
    let sent_batches = batches.lock().expect("lock");
    assert_eq!(sent_batches.len(), 2);
    assert_ne!(sent_batches[1].batch_id, taken.batch_id);
}

#[test]
fn should_reuse_retry_batch_identity_after_restart_on_transient_failure() {
    let store = Arc::new(InMemoryIdentityStore::new());
    let first_id = {
        let transport = RecordingTransport::new(Err(FlushError::http_status(503, None)));
        let batches = Arc::clone(&transport.batches);
        let mut sdk =
            HeatmapSdk::init_with_transport_and_store(valid_config(), transport, store.clone())
                .expect("init");
        sdk.enqueue_candidate(minimal_click_candidate(1_000))
            .expect("enqueue");
        assert!(sdk.flush().is_err());
        let first_id = batches.lock().expect("lock")[0].batch_id.clone();
        first_id
    };
    let transport = RecordingTransport::new(Ok(()));
    let batches = Arc::clone(&transport.batches);
    let mut restored = HeatmapSdk::init_with_transport_and_store(valid_config(), transport, store)
        .expect("restore");
    restored.flush().expect("retry after restart");
    let sent = batches.lock().expect("lock");
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].batch_id, first_id);
}

#[test]
fn production_queue_caps_match_ten_thousand_events_and_five_mib() {
    assert_eq!(QUEUE_MAX_EVENTS, 10_000);
    assert_eq!(QUEUE_MAX_BYTES, 5 * 1024 * 1024);
}
