use std::sync::Arc;

use heatmap_core::{
    CaptureCandidate, Clock, HeatmapConfig, HeatmapSdk, InMemoryIdentityStore, InteractionEvent,
    InteractionType, ManualClock, PlatformInfo, PlatformType, Position, SdkInfo, Viewport,
};

fn valid_config_with_timeout(timeout_ms: u64) -> HeatmapConfig {
    HeatmapConfig {
        project_key: "pk_test".into(),
        endpoint: "https://ingest.example.com".into(),
        session_timeout_ms: timeout_ms,
        sample_rate: HeatmapConfig::DEFAULT_SAMPLE_RATE,
        exclude_elements: vec![],
        exclude_screens: vec![],
        mask_elements: vec![],
        app_version: heatmap_core::HeatmapConfig::DEFAULT_APP_VERSION.into(),
    }
}

fn click_candidate(timestamp: i64) -> CaptureCandidate {
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
        interaction: InteractionEvent::click(Some("checkout.pay".into())).expect("click"),
        position: Some(Position::new(0.5, 0.5).expect("position")),
        viewport: Some(Viewport {
            width: 1280,
            height: 720,
        }),
        window_id: None,
    }
}

fn sdk_with_clock(timeout_ms: u64, clock: Arc<ManualClock>) -> HeatmapSdk {
    let store = Arc::new(InMemoryIdentityStore::new());
    HeatmapSdk::init_with_transport_store_and_clock(
        valid_config_with_timeout(timeout_ms),
        heatmap_core::HttpBatchTransport::new(),
        store,
        clock,
    )
    .expect("init")
}

#[test]
fn should_default_session_timeout_to_thirty_minutes() {
    assert_eq!(HeatmapConfig::DEFAULT_SESSION_TIMEOUT_MS, 1_800_000);
}

#[test]
fn should_use_configured_session_timeout_ms() {
    let config = valid_config_with_timeout(5_000);
    assert_eq!(config.session_timeout_ms, 5_000);
}

#[test]
fn should_start_session_on_first_capture_not_at_init() {
    let clock = Arc::new(ManualClock::new(1_000));
    let mut sdk = sdk_with_clock(10_000, Arc::clone(&clock));
    assert!(sdk.build_batch().is_err(), "no events before first capture");
    sdk.enqueue_candidate(click_candidate(clock.now_ms()))
        .expect("first capture");
    let batch = sdk.build_batch().expect("batch");
    assert_eq!(batch.events.len(), 2);
    assert_eq!(
        batch.events[0].event.interaction_type,
        InteractionType::SessionStart
    );
    assert_eq!(
        batch.events[1].event.interaction_type,
        InteractionType::Click
    );
}

#[test]
fn should_reset_inactivity_window_when_event_is_captured() {
    let clock = Arc::new(ManualClock::new(0));
    let timeout = 1_000;
    let mut sdk = sdk_with_clock(timeout, Arc::clone(&clock));
    sdk.enqueue_candidate(click_candidate(clock.now_ms()))
        .expect("first click");
    let first_batch = sdk.build_batch().expect("first batch");
    let session_id = first_batch.events[1].session_id.clone();

    clock.advance_ms(900);
    sdk.enqueue_candidate(click_candidate(clock.now_ms()))
        .expect("second click within window");
    let second_batch = sdk.build_batch().expect("second batch");
    assert_eq!(second_batch.events.len(), 1);
    assert_eq!(second_batch.events[0].session_id, session_id);
}

#[test]
fn should_keep_session_id_after_background_then_foreground_before_timeout() {
    let clock = Arc::new(ManualClock::new(100));
    let timeout = 10_000;
    let mut sdk = sdk_with_clock(timeout, Arc::clone(&clock));
    sdk.enqueue_candidate(click_candidate(clock.now_ms()))
        .expect("click");
    let session_id = sdk
        .build_batch()
        .expect("batch")
        .events[1]
        .session_id
        .clone();

    clock.advance_ms(500);
    sdk.on_background();
    clock.advance_ms(500);
    sdk.on_foreground();
    sdk.enqueue_candidate(click_candidate(clock.now_ms()))
        .expect("click after resume");
    let batch = sdk.build_batch().expect("batch");
    assert_eq!(batch.events[0].session_id, session_id);
}

#[test]
fn should_start_new_session_after_inactivity_timeout() {
    let clock = Arc::new(ManualClock::new(0));
    let timeout = 1_000;
    let mut sdk = sdk_with_clock(timeout, Arc::clone(&clock));
    sdk.enqueue_candidate(click_candidate(clock.now_ms()))
        .expect("first click");
    let first_session = sdk
        .build_batch()
        .expect("batch")
        .events[1]
        .session_id
        .clone();

    clock.advance_ms(timeout + 1);
    sdk.enqueue_candidate(click_candidate(clock.now_ms()))
        .expect("click after timeout");
    let batch = sdk.build_batch().expect("batch");
    assert_eq!(batch.events.len(), 2);
    assert_eq!(
        batch.events[0].event.interaction_type,
        InteractionType::SessionStart
    );
    assert_ne!(batch.events[1].session_id, first_session);
}

#[test]
fn should_enqueue_single_session_start_when_bootstrap_is_first_capture() {
    let clock = Arc::new(ManualClock::new(42));
    let mut sdk = sdk_with_clock(10_000, Arc::clone(&clock));
    sdk.bootstrap_web_session(
        SdkInfo {
            name: "heatmap-web".into(),
            version: "0.1.0".into(),
        },
        PlatformInfo {
            platform_type: PlatformType::Web,
            os_version: "macOS 14.6".into(),
            app_version: "1.0.0".into(),
        },
        Viewport {
            width: 1280,
            height: 720,
        },
    )
    .expect("bootstrap");
    let batch = sdk.build_batch().expect("batch");
    assert_eq!(batch.events.len(), 1);
    assert_eq!(
        batch.events[0].event.interaction_type,
        InteractionType::SessionStart
    );
}
