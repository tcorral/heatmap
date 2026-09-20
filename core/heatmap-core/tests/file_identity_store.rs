use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use heatmap_core::{
    batch_ingest_url, CaptureCandidate, FileIdentityStore, HeatmapBatch, HeatmapConfig, HeatmapSdk,
    IdentityStore, InteractionEvent, PlatformInfo, PlatformType, Position, SdkInfo, Viewport,
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

fn minimal_click_candidate(timestamp: i64) -> CaptureCandidate {
    CaptureCandidate {
        timestamp,
        sdk: SdkInfo {
            name: "heatmap-android".into(),
            version: "0.1.0".into(),
        },
        platform: PlatformInfo {
            platform_type: PlatformType::Android,
            os_version: "Android 14".into(),
            app_version: "1.0.0".into(),
        },
        screen: None,
        interaction: InteractionEvent::click(Some("checkout.pay".into())).expect("click"),
        position: Some(Position::new(0.5, 0.5).expect("position")),
        viewport: Some(Viewport {
            width: 800,
            height: 600,
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
fn should_restore_queued_events_after_process_restart_with_file_store() {
    let dir = std::env::temp_dir().join(format!("heatmap-file-store-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let store = Arc::new(FileIdentityStore::new(dir.clone()).expect("create file identity store"));

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
    let mut restored = HeatmapSdk::init_with_transport_and_store(
        valid_config(),
        transport,
        Arc::new(FileIdentityStore::new(dir.clone()).expect("reopen store")),
    )
    .expect("restore init");
    restored.flush().expect("flush restored queue");
    assert_eq!(*restore_sends.lock().expect("lock"), 1);
    assert_eq!(*restore_events.lock().expect("lock"), 2);
}

#[test]
fn should_persist_key_value_round_trip() {
    let dir =
        PathBuf::from(std::env::temp_dir()).join(format!("heatmap-kv-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let store = FileIdentityStore::new(dir).expect("store");
    store.write(heatmap_core::STORE_KEY_QUEUE, Some("[{\"event\":{}}]"));
    assert!(store.read(heatmap_core::STORE_KEY_QUEUE).is_some());
    store.write(heatmap_core::STORE_KEY_QUEUE, None);
    assert!(store.read(heatmap_core::STORE_KEY_QUEUE).is_none());
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    PathBuf::from(std::env::temp_dir()).join(format!("{prefix}-{}-{nanos}", std::process::id()))
}

#[test]
fn should_replace_existing_file_on_repeated_writes() {
    let dir = unique_temp_dir("heatmap-replace");
    let _ = std::fs::remove_dir_all(&dir);
    let store = FileIdentityStore::new(dir).expect("store");
    store.write(heatmap_core::STORE_KEY_QUEUE, Some("first"));
    store.write(heatmap_core::STORE_KEY_QUEUE, Some("second"));
    store.write(heatmap_core::STORE_KEY_ANONYMOUS_ID, Some("anon_a"));
    store.write(heatmap_core::STORE_KEY_ANONYMOUS_ID, Some("anon_b"));
    assert_eq!(
        store.read(heatmap_core::STORE_KEY_QUEUE).as_deref(),
        Some("second")
    );
    assert_eq!(
        store.read(heatmap_core::STORE_KEY_ANONYMOUS_ID).as_deref(),
        Some("anon_b")
    );
}
