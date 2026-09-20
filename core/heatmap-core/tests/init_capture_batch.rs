use heatmap_core::{
    CaptureCandidate, HeatmapBatch, HeatmapConfig, HeatmapSdk, InitError, PlatformInfo,
    PlatformType, Position, ScreenInfo, SdkInfo, Viewport,
};
use heatmap_core::{CaptureError, InteractionEvent};
use serde_json::Value;

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
        window_id: Some("win_browser_main".into()),
    }
}

fn assert_batch_matches_batch_v1_shape(batch: &HeatmapBatch) {
    let value = serde_json::to_value(batch).expect("serialize batch");
    let object = value.as_object().expect("batch object");
    assert_eq!(object.len(), 2, "batch must only have batch_id and events");
    let batch_id = object.get("batch_id").expect("batch_id");
    assert!(
        batch_id.as_str().is_some_and(|id| !id.is_empty()),
        "batch_id must be non-empty string"
    );
    let events = object
        .get("events")
        .and_then(Value::as_array)
        .expect("events array");
    assert!(!events.is_empty(), "events must have at least one item");
    for event in events {
        assert_eq!(event.get("schema_version"), Some(&Value::from(1)));
        assert!(event.get("session_id").and_then(Value::as_str).is_some());
        assert!(event.get("anonymous_id").and_then(Value::as_str).is_some());
        assert!(event.get("event").and_then(Value::as_object).is_some());
    }
}

#[test]
fn should_enqueue_click_after_init_without_separate_enable_call() {
    let mut sdk = HeatmapSdk::init(valid_config()).expect("init");
    sdk.enqueue_candidate(checkout_pay_click_candidate())
        .expect("enqueue after init");
    let batch = sdk.build_batch().expect("batch");
    assert_eq!(batch.events.len(), 2);
    assert_eq!(
        batch.events[1].event.interaction_type,
        heatmap_core::InteractionType::Click
    );
}

#[test]
fn should_reject_candidate_when_tracking_disabled() {
    let mut sdk = HeatmapSdk::init(valid_config()).expect("init");
    sdk.set_tracking_enabled(false);
    let err = sdk
        .enqueue_candidate(checkout_pay_click_candidate())
        .expect_err("disabled");
    assert!(matches!(err, CaptureError::TrackingDisabled));
}

#[test]
fn should_resume_enqueue_after_tracking_re_enabled() {
    let mut sdk = HeatmapSdk::init(valid_config()).expect("init");
    sdk.set_tracking_enabled(false);
    assert!(sdk
        .enqueue_candidate(checkout_pay_click_candidate())
        .is_err());
    sdk.set_tracking_enabled(true);
    sdk.enqueue_candidate(checkout_pay_click_candidate())
        .expect("enqueue after re-enable");
    let batch = sdk.build_batch().expect("batch");
    assert_eq!(batch.events.len(), 2);
}

#[test]
fn should_enqueue_click_when_default_sample_rate_is_sampled_in() {
    let config = valid_config();
    assert!((config.sample_rate - 1.0).abs() < f64::EPSILON);
    let mut sdk = HeatmapSdk::init(config).expect("init");
    sdk.enqueue_candidate(checkout_pay_click_candidate())
        .expect("sampled-in enqueue");
    let batch = sdk.build_batch().expect("batch");
    assert_eq!(batch.events.len(), 2);
    assert!(!batch.events[1].session_id.is_empty());
    assert!(!batch.events[1].anonymous_id.is_empty());
}

#[test]
fn should_assign_new_batch_id_on_each_build_batch() {
    let mut sdk = HeatmapSdk::init(valid_config()).expect("init");
    sdk.enqueue_candidate(checkout_pay_click_candidate())
        .expect("enqueue");
    let first = sdk.build_batch().expect("first batch");
    sdk.enqueue_candidate(checkout_pay_click_candidate())
        .expect("enqueue again");
    let second = sdk.build_batch().expect("second batch");
    assert_ne!(first.batch_id, second.batch_id);
    assert_batch_matches_batch_v1_shape(&first);
    assert_batch_matches_batch_v1_shape(&second);
}

#[test]
fn should_reject_init_when_project_key_is_empty() {
    let mut config = valid_config();
    config.project_key.clear();
    match HeatmapSdk::init(config) {
        Err(err) => assert!(matches!(err, InitError::MissingProjectKey)),
        Ok(_) => panic!("expected empty project key to fail init"),
    }
}

#[test]
fn should_reject_init_when_endpoint_is_not_https() {
    let mut config = valid_config();
    config.endpoint = "http://ingest.example.com".into();
    match HeatmapSdk::init(config) {
        Err(err) => assert!(matches!(err, InitError::InsecureEndpoint)),
        Ok(_) => panic!("expected non-HTTPS endpoint to fail init"),
    }
}

#[test]
fn should_reject_init_when_endpoint_is_empty() {
    let mut config = valid_config();
    config.endpoint.clear();
    match HeatmapSdk::init(config) {
        Err(err) => assert!(matches!(err, InitError::MissingEndpoint)),
        Ok(_) => panic!("expected empty endpoint to fail init"),
    }
}

#[test]
fn should_reject_candidate_with_invalid_position() {
    let mut sdk = HeatmapSdk::init(valid_config()).expect("init");
    let mut candidate = checkout_pay_click_candidate();
    candidate.position = Some(Position { x: 1.5, y: 0.5 });
    let err = sdk.enqueue_candidate(candidate).expect_err("bad position");
    assert!(matches!(err, CaptureError::InvalidPosition(_)));
}
