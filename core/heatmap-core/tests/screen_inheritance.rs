use heatmap_core::{
    CaptureCandidate, HeatmapConfig, HeatmapSdk, InteractionEvent, InteractionType, PlatformInfo,
    PlatformType, SdkInfo, Viewport,
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

fn web_viewport() -> Viewport {
    Viewport {
        width: 1280,
        height: 720,
    }
}

fn web_platform() -> PlatformInfo {
    PlatformInfo {
        platform_type: PlatformType::Web,
        os_version: "macOS 14.6".into(),
        app_version: "1.0.0".into(),
    }
}

fn web_sdk() -> SdkInfo {
    SdkInfo {
        name: "heatmap-web".into(),
        version: "0.1.0".into(),
    }
}

fn screen_id(event: &heatmap_core::HeatmapEvent) -> Option<&str> {
    event.screen.as_ref().map(|screen| screen.id.as_str())
}

fn layout_version(event: &heatmap_core::HeatmapEvent) -> Option<&str> {
    event
        .screen
        .as_ref()
        .and_then(|screen| screen.layout_version.as_deref())
}

#[test]
fn should_inherit_checkout_screen_and_layout_on_lifecycle_capture() {
    let mut sdk = HeatmapSdk::init(valid_config()).expect("init");
    sdk.screen("Checkout".into(), Some("layout-a".into()))
        .expect("screen");
    sdk.record_scroll(web_sdk(), web_platform(), web_viewport(), None)
        .expect("scroll");
    let batch = sdk.build_batch().expect("batch");
    let scroll = &batch.events[1];
    assert_eq!(scroll.event.interaction_type, InteractionType::Scroll);
    assert_eq!(screen_id(scroll), Some("Checkout"));
    assert_eq!(layout_version(scroll), Some("layout-a"));
}

#[test]
fn should_not_carry_checkout_after_confirmation_screen_change() {
    let mut sdk = HeatmapSdk::init(valid_config()).expect("init");
    sdk.screen("Checkout".into(), Some("checkout-layout".into()))
        .expect("checkout screen");
    sdk.record_scroll(web_sdk(), web_platform(), web_viewport(), None)
        .expect("checkout scroll");
    sdk.screen("Confirmation".into(), None)
        .expect("confirmation screen");
    sdk.record_scroll(web_sdk(), web_platform(), web_viewport(), None)
        .expect("confirmation scroll");
    let batch = sdk.build_batch().expect("batch");
    assert_eq!(batch.events.len(), 3);
    let checkout_scroll = &batch.events[1];
    let confirmation_scroll = &batch.events[2];
    assert_eq!(screen_id(checkout_scroll), Some("Checkout"));
    assert_eq!(layout_version(checkout_scroll), Some("checkout-layout"));
    assert_eq!(screen_id(confirmation_scroll), Some("Confirmation"));
    assert_eq!(layout_version(confirmation_scroll), None);
}

#[test]
fn should_omit_screen_on_events_when_screen_never_declared() {
    let mut sdk = HeatmapSdk::init(valid_config()).expect("init");
    let candidate = CaptureCandidate {
        timestamp: 1,
        sdk: web_sdk(),
        platform: web_platform(),
        screen: None,
        interaction: InteractionEvent::click(Some("nav.home".into())).expect("click"),
        position: None,
        viewport: Some(web_viewport()),
        window_id: None,
    };
    sdk.enqueue_candidate(candidate).expect("enqueue");
    let batch = sdk.build_batch().expect("batch");
    for event in &batch.events {
        assert!(event.screen.is_none(), "expected no inherited screen");
    }
}
