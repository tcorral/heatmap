use heatmap_core::{
    CaptureCandidate, CaptureError, HeatmapConfig, HeatmapSdk, InteractionEvent, PlatformInfo,
    PlatformType, Position, ScreenInfo, SdkInfo, Viewport,
};
use serde_json::Value;

fn base_config() -> HeatmapConfig {
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

fn web_sdk() -> SdkInfo {
    SdkInfo {
        name: "heatmap-web".into(),
        version: "0.1.0".into(),
    }
}

fn web_platform() -> PlatformInfo {
    PlatformInfo {
        platform_type: PlatformType::Web,
        os_version: "macOS 14.6".into(),
        app_version: "1.0.0".into(),
    }
}

fn click_candidate(element_id: &str) -> CaptureCandidate {
    CaptureCandidate {
        timestamp: 1,
        sdk: web_sdk(),
        platform: web_platform(),
        screen: Some(ScreenInfo {
            id: "Checkout".into(),
            layout_version: None,
        }),
        interaction: InteractionEvent::click(Some(element_id.into())).expect("click"),
        position: Some(Position::new(0.4, 0.6).expect("position")),
        viewport: Some(Viewport {
            width: 1280,
            height: 720,
        }),
        window_id: None,
    }
}

#[test]
fn should_drop_click_when_element_id_is_excluded() {
    let mut config = base_config();
    config.exclude_elements = vec!["checkout.pay".into()];
    let mut sdk = HeatmapSdk::init(config).expect("init");
    let err = sdk
        .enqueue_candidate(click_candidate("checkout.pay"))
        .expect_err("excluded element");
    assert!(matches!(err, CaptureError::Excluded));
    assert!(sdk.build_batch().is_err());
}

#[test]
fn should_drop_capture_when_candidate_screen_is_excluded() {
    let mut config = base_config();
    config.exclude_screens = vec!["Checkout".into()];
    let mut sdk = HeatmapSdk::init(config).expect("init");
    let err = sdk
        .enqueue_candidate(click_candidate("nav.home"))
        .expect_err("excluded screen");
    assert!(matches!(err, CaptureError::Excluded));
    assert!(sdk.build_batch().is_err());
}

#[test]
fn should_drop_capture_when_inherited_screen_is_excluded() {
    let mut config = base_config();
    config.exclude_screens = vec!["Settings".into()];
    let mut sdk = HeatmapSdk::init(config).expect("init");
    sdk.screen("Settings".into(), None).expect("screen");
    let candidate = CaptureCandidate {
        timestamp: 1,
        sdk: web_sdk(),
        platform: web_platform(),
        screen: None,
        interaction: InteractionEvent::scroll().expect("scroll"),
        position: None,
        viewport: Some(Viewport {
            width: 1280,
            height: 720,
        }),
        window_id: None,
    };
    let err = sdk
        .enqueue_candidate(candidate)
        .expect_err("excluded inherited screen");
    assert!(matches!(err, CaptureError::Excluded));
}

#[test]
fn should_not_inject_session_start_when_first_capture_is_excluded() {
    let mut config = base_config();
    config.exclude_elements = vec!["checkout.pay".into()];
    let mut sdk = HeatmapSdk::init(config).expect("init");
    let _ = sdk.enqueue_candidate(click_candidate("checkout.pay"));
    assert!(sdk.build_batch().is_err());
}

#[test]
fn should_enqueue_masked_click_without_element_id_but_with_position() {
    let mut config = base_config();
    config.mask_elements = vec!["checkout.card".into()];
    let mut sdk = HeatmapSdk::init(config).expect("init");
    sdk.enqueue_candidate(click_candidate("checkout.card"))
        .expect("masked click enqueues");
    let batch = sdk.build_batch().expect("batch");
    let click = batch
        .events
        .iter()
        .find(|event| event.event.interaction_type == heatmap_core::InteractionType::Click)
        .expect("click event");
    assert!(click.event.element_id.is_none());
    assert_eq!(
        click.position,
        Some(Position::new(0.4, 0.6).expect("position"))
    );
}

#[test]
fn should_still_enqueue_non_excluded_click_when_mask_list_only_matches_other_elements() {
    let mut config = base_config();
    config.mask_elements = vec!["other.control".into()];
    let mut sdk = HeatmapSdk::init(config).expect("init");
    sdk.enqueue_candidate(click_candidate("checkout.pay"))
        .expect("unmasked click");
    let batch = sdk.build_batch().expect("batch");
    let click = batch
        .events
        .iter()
        .find(|event| event.event.interaction_type == heatmap_core::InteractionType::Click)
        .expect("click");
    assert_eq!(click.event.element_id.as_deref(), Some("checkout.pay"));
}

fn collect_object_keys(value: &Value, keys: &mut Vec<String>) {
    if let Some(object) = value.as_object() {
        for (key, nested) in object {
            keys.push(key.clone());
            collect_object_keys(nested, keys);
        }
    } else if let Some(array) = value.as_array() {
        for item in array {
            collect_object_keys(item, keys);
        }
    }
}

const FORBIDDEN_PAYLOAD_KEYS: &[&str] = &["text", "label", "value", "title"];

#[test]
fn should_omit_forbidden_sensitive_keys_from_serialized_batch_events() {
    let mut config = base_config();
    config.mask_elements = vec!["checkout.card".into()];
    let mut sdk = HeatmapSdk::init(config).expect("init");
    sdk.enqueue_candidate(click_candidate("checkout.card"))
        .expect("enqueue");
    let batch = sdk.build_batch().expect("batch");
    let json = serde_json::to_value(&batch).expect("serialize batch");
    let mut keys = Vec::new();
    collect_object_keys(&json, &mut keys);
    for forbidden in FORBIDDEN_PAYLOAD_KEYS {
        assert!(
            !keys.iter().any(|key| key == forbidden),
            "found forbidden key {forbidden} in {json}"
        );
    }
}

#[test]
fn should_omit_forbidden_sensitive_keys_from_diagnostics_snapshot() {
    let sdk = HeatmapSdk::init(base_config()).expect("init");
    let json = serde_json::to_value(sdk.diagnostics()).expect("serialize diagnostics");
    let mut keys = Vec::new();
    collect_object_keys(&json, &mut keys);
    for forbidden in FORBIDDEN_PAYLOAD_KEYS {
        assert!(
            !keys.iter().any(|key| key == forbidden),
            "found forbidden key {forbidden} in {json}"
        );
    }
}
