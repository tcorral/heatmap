use std::sync::Arc;

use heatmap_core::{
    CaptureCandidate, HeatmapConfig, HeatmapSdk, IdentityStore, InMemoryIdentityStore,
    InteractionEvent, PlatformInfo, PlatformType, Position, SdkInfo, Viewport,
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

fn minimal_click_candidate() -> CaptureCandidate {
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

fn init_with_store(store: Arc<dyn IdentityStore>) -> HeatmapSdk {
    HeatmapSdk::init_with_transport_and_store(valid_config(), StubTransport, store).expect("init")
}

struct StubTransport;

impl heatmap_core::BatchTransport for StubTransport {
    fn send_batch(
        &self,
        _endpoint: &str,
        _project_key: &str,
        _batch: &heatmap_core::HeatmapBatch,
    ) -> Result<(), heatmap_core::FlushError> {
        Ok(())
    }
}

fn enqueue_one(sdk: &mut HeatmapSdk) -> heatmap_core::HeatmapEvent {
    sdk.enqueue_candidate(minimal_click_candidate())
        .expect("enqueue");
    sdk.build_batch().expect("batch").events[0].clone()
}

#[test]
fn should_keep_same_anonymous_id_on_events_before_and_after_identify() {
    let store = Arc::new(InMemoryIdentityStore::new());
    let mut sdk = init_with_store(store.clone());
    let before = enqueue_one(&mut sdk);
    sdk.identify("user-42".into()).expect("identify");
    let after = enqueue_one(&mut sdk);
    assert_eq!(before.anonymous_id, after.anonymous_id);
    assert_eq!(after.user_id.as_deref(), Some("user-42"));
}

#[test]
fn should_include_user_id_on_events_after_identify() {
    let store = Arc::new(InMemoryIdentityStore::new());
    let mut sdk = init_with_store(store);
    sdk.identify("acct-99".into()).expect("identify");
    let event = enqueue_one(&mut sdk);
    assert_eq!(event.user_id.as_deref(), Some("acct-99"));
}

#[test]
fn should_keep_anonymous_id_and_omit_user_id_after_sign_out() {
    let store = Arc::new(InMemoryIdentityStore::new());
    let mut sdk = init_with_store(store.clone());
    sdk.identify("user-42".into()).expect("identify");
    let identified = enqueue_one(&mut sdk);
    sdk.sign_out();
    let after_sign_out = enqueue_one(&mut sdk);
    assert_eq!(identified.anonymous_id, after_sign_out.anonymous_id);
    assert!(after_sign_out.user_id.is_none());
}

#[test]
fn should_restore_user_id_when_second_init_uses_same_store_after_identify() {
    let store = Arc::new(InMemoryIdentityStore::new());
    let mut first = init_with_store(store.clone());
    first.identify("user-persist".into()).expect("identify");
    drop(first);

    let mut second = init_with_store(store);
    let event = enqueue_one(&mut second);
    assert_eq!(event.user_id.as_deref(), Some("user-persist"));
}

#[test]
fn should_reuse_anonymous_id_when_second_init_uses_same_store() {
    let store = Arc::new(InMemoryIdentityStore::new());
    let mut first = init_with_store(store.clone());
    let anon_from_first_init = enqueue_one(&mut first).anonymous_id;

    let mut second = init_with_store(store);
    let anon_from_second_init = enqueue_one(&mut second).anonymous_id;

    assert_eq!(anon_from_first_init, anon_from_second_init);
}
