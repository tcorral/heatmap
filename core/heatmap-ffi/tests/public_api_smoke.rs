use heatmap_ffi::PublicHeatmapSdk;

#[test]
fn should_init_identify_screen_and_flush_through_public_wrapper() {
    let config = r#"{
        "project_key": "pk_test",
        "endpoint": "https://ingest.example.com"
    }"#;
    let mut sdk = PublicHeatmapSdk::init_from_json(config).expect("init");
    sdk.identify("user-42".into()).expect("identify");
    sdk.screen("Checkout".into(), Some("layout-a".into()))
        .expect("screen");
    sdk.set_tracking_enabled(true);
    assert!(sdk.flush().is_ok(), "empty flush should succeed");
    sdk.sign_out();
}
