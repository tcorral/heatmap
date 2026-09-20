use heatmap_core::InteractionType;
use heatmap_ffi::PublicHeatmapSdk;

fn init_sdk() -> PublicHeatmapSdk {
    let config = r#"{
        "project_key": "pk_test",
        "endpoint": "https://ingest.example.com",
        "session_timeout_ms": 10000
    }"#;
    PublicHeatmapSdk::init_from_json(config).expect("init")
}

#[test]
fn should_attach_window_id_to_macos_screen_view() {
    let mut sdk = init_sdk();
    sdk.screen("Checkout".into(), None).expect("screen");
    sdk.record_macos_screen_view(
        800,
        600,
        "win_checkout_primary".into(),
        "macOS 14.6".into(),
        "1.0.0".into(),
    )
    .expect("screen_view");

    let window_ids = sdk
        .adapter_macos_lifecycle_window_ids_from_batch(InteractionType::ScreenView)
        .expect("batch");
    assert_eq!(window_ids, vec!["win_checkout_primary".to_string()]);
}

#[test]
fn should_attach_window_id_to_macos_scroll() {
    let mut sdk = init_sdk();
    sdk.screen("Checkout".into(), None).expect("screen");
    sdk.record_macos_scroll(
        800,
        600,
        "win_checkout_primary".into(),
        "macOS 14.6".into(),
        "1.0.0".into(),
    )
    .expect("scroll");

    let window_ids = sdk
        .adapter_macos_lifecycle_window_ids_from_batch(InteractionType::Scroll)
        .expect("batch");
    assert_eq!(window_ids, vec!["win_checkout_primary".to_string()]);
}
