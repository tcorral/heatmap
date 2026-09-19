use heatmap_core::{
    HeatmapConfig, HeatmapSdk, InteractionType, PlatformInfo, PlatformType, SdkInfo, Viewport,
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

#[test]
fn should_enqueue_session_start_when_bootstrap_called_after_init() {
    let mut sdk = HeatmapSdk::init(valid_config()).expect("init");
    sdk.bootstrap_web_session(web_sdk(), web_platform(), web_viewport())
        .expect("bootstrap");
    let batch = sdk.build_batch().expect("batch");
    assert_eq!(batch.events.len(), 1);
    assert_eq!(
        batch.events[0].event.interaction_type,
        InteractionType::SessionStart
    );
}

#[test]
fn should_enqueue_screen_view_when_screen_is_set_with_context() {
    let mut sdk = HeatmapSdk::init(valid_config()).expect("init");
    sdk.screen("Home".into(), None).expect("screen");
    sdk.record_screen_view(web_sdk(), web_platform(), web_viewport())
        .expect("screen_view");
    let batch = sdk.build_batch().expect("batch");
    assert_eq!(batch.events.len(), 1);
    assert_eq!(
        batch.events[0].screen.as_ref().map(|screen| screen.id.as_str()),
        Some("Home")
    );
}

#[test]
fn should_enqueue_scroll_with_inherited_checkout_screen() {
    let mut sdk = HeatmapSdk::init(valid_config()).expect("init");
    sdk.screen("Checkout".into(), Some("2024-09".into()))
        .expect("screen");
    sdk.record_scroll(web_sdk(), web_platform(), web_viewport())
        .expect("scroll");
    let batch = sdk.build_batch().expect("batch");
    assert_eq!(
        batch.events[0].event.interaction_type,
        InteractionType::Scroll
    );
    assert_eq!(
        batch.events[0].screen.as_ref().map(|screen| screen.id.as_str()),
        Some("Checkout")
    );
}
