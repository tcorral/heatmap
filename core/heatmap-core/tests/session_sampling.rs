use std::sync::Arc;

use heatmap_core::{
    BatchError, CaptureCandidate, CaptureError, Clock, HeatmapConfig, HeatmapSdk, IdentityStore,
    InMemoryIdentityStore, InteractionEvent, ManualClock, ManualRng, PlatformInfo, PlatformType,
    Position, SdkInfo, Viewport, STORE_KEY_SAMPLE_IN,
};

fn config_with_sample_rate(sample_rate: f64) -> HeatmapConfig {
    HeatmapConfig {
        project_key: "pk_test".into(),
        endpoint: "https://ingest.example.com".into(),
        session_timeout_ms: 60_000,
        sample_rate,
        exclude_elements: vec![],
        exclude_screens: vec![],
        mask_elements: vec![],
        app_version: heatmap_core::HeatmapConfig::DEFAULT_APP_VERSION.into(),
    }
}

fn click_at(timestamp: i64) -> CaptureCandidate {
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

fn sdk_with_clock_and_rng(
    sample_rate: f64,
    clock: Arc<ManualClock>,
    rng: Arc<ManualRng>,
) -> HeatmapSdk {
    let store: Arc<dyn IdentityStore> = Arc::new(InMemoryIdentityStore::new());
    HeatmapSdk::init_with_transport_store_clock_and_rng(
        config_with_sample_rate(sample_rate),
        StubTransport,
        store,
        clock,
        rng,
    )
    .expect("init")
}

#[test]
fn should_enqueue_session_when_default_sample_rate_is_one() {
    let clock = Arc::new(ManualClock::new(1_000));
    let rng = Arc::new(ManualRng::sequence(vec![0.99]));
    let mut sdk = sdk_with_clock_and_rng(1.0, clock.clone(), rng);
    sdk.enqueue_candidate(click_at(clock.now_ms()))
        .expect("sampled-in enqueue");
    let batch = sdk.build_batch().expect("batch");
    assert_eq!(batch.events.len(), 2);
}

#[test]
fn should_enqueue_nothing_when_session_is_sampled_out() {
    let clock = Arc::new(ManualClock::new(1_000));
    let rng = Arc::new(ManualRng::sequence(vec![0.99]));
    let mut sdk = sdk_with_clock_and_rng(0.0, clock.clone(), rng);
    let err = sdk
        .enqueue_candidate(click_at(clock.now_ms()))
        .expect_err("sampled out");
    assert!(matches!(err, CaptureError::SampledOut));
    assert!(matches!(sdk.build_batch(), Err(BatchError::EmptyQueue)));
}

#[test]
fn should_sample_in_when_draw_is_below_half_rate() {
    let clock = Arc::new(ManualClock::new(1_000));
    let rng = Arc::new(ManualRng::sequence(vec![0.49]));
    let mut sdk = sdk_with_clock_and_rng(0.5, clock.clone(), rng);
    sdk.enqueue_candidate(click_at(clock.now_ms()))
        .expect("sampled in");
    assert_eq!(sdk.build_batch().expect("batch").events.len(), 2);
}

#[test]
fn should_sample_out_when_draw_is_at_or_above_half_rate() {
    let clock = Arc::new(ManualClock::new(1_000));
    let rng = Arc::new(ManualRng::sequence(vec![0.5]));
    let mut sdk = sdk_with_clock_and_rng(0.5, clock.clone(), rng);
    let err = sdk
        .enqueue_candidate(click_at(clock.now_ms()))
        .expect_err("sampled out");
    assert!(matches!(err, CaptureError::SampledOut));
    assert!(matches!(sdk.build_batch(), Err(BatchError::EmptyQueue)));
}

#[test]
fn should_not_redraw_sample_decision_mid_session() {
    let clock = Arc::new(ManualClock::new(1_000));
    let rng = Arc::new(ManualRng::sequence(vec![0.49, 0.99]));
    let mut sdk = sdk_with_clock_and_rng(0.5, clock.clone(), Arc::clone(&rng));
    sdk.enqueue_candidate(click_at(clock.now_ms()))
        .expect("first click sampled in");
    sdk.enqueue_candidate(click_at(clock.now_ms() + 1))
        .expect("second click still sampled in");
    let batch = sdk.build_batch().expect("batch");
    assert_eq!(batch.events.len(), 3);
    assert_eq!(rng.draw_count(), 1);
}

#[test]
fn should_increment_sampled_out_diagnostic_when_enqueue_rejected() {
    let clock = Arc::new(ManualClock::new(1_000));
    let rng = Arc::new(ManualRng::sequence(vec![0.99]));
    let mut sdk = sdk_with_clock_and_rng(0.0, clock.clone(), rng);
    assert_eq!(sdk.diagnostics().sampled_out, 0);
    let _ = sdk.enqueue_candidate(click_at(clock.now_ms()));
    assert_eq!(sdk.diagnostics().sampled_out, 1);
    let _ = sdk.enqueue_candidate(click_at(clock.now_ms() + 1));
    assert_eq!(sdk.diagnostics().sampled_out, 2);
}

#[test]
fn should_redraw_sample_decision_after_inactivity_timeout() {
    let clock = Arc::new(ManualClock::new(1_000));
    let rng = Arc::new(ManualRng::sequence(vec![0.99, 0.1]));
    let mut sdk = sdk_with_clock_and_rng(0.5, clock.clone(), Arc::clone(&rng));
    let err = sdk
        .enqueue_candidate(click_at(clock.now_ms()))
        .expect_err("sampled out");
    assert!(matches!(err, CaptureError::SampledOut));
    clock.advance_ms(60_000 + 1);
    sdk.enqueue_candidate(click_at(clock.now_ms()))
        .expect("sampled in after rotate");
    assert_eq!(rng.draw_count(), 2);
    assert_eq!(sdk.build_batch().expect("batch").events.len(), 2);
}

#[test]
fn should_persist_sample_in_decision_when_session_starts() {
    let clock = Arc::new(ManualClock::new(1_000));
    let rng = Arc::new(ManualRng::sequence(vec![0.99]));
    let store: Arc<dyn IdentityStore> = Arc::new(InMemoryIdentityStore::new());
    let mut sdk = HeatmapSdk::init_with_transport_store_clock_and_rng(
        config_with_sample_rate(0.0),
        StubTransport,
        Arc::clone(&store),
        clock.clone(),
        rng,
    )
    .expect("init");
    let _ = sdk.enqueue_candidate(click_at(clock.now_ms()));
    assert_eq!(store.read(STORE_KEY_SAMPLE_IN).as_deref(), Some("0"));
}
