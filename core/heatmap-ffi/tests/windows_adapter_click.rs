use heatmap_ffi::PublicHeatmapSdk;

#[test]
fn should_attach_window_id_and_windows_platform_to_click_batch() {
    let config = r#"{
        "project_key": "pk_test",
        "endpoint": "https://ingest.example.com",
        "session_timeout_ms": 10000
    }"#;
    let mut sdk = PublicHeatmapSdk::init_from_json(config).expect("init");
    sdk.record_windows_activation_click(
        0.15,
        0.2,
        Some("checkout.pay".into()),
        800,
        600,
        1_726_771_205_000,
        "win_checkout_primary".into(),
        "Windows 11 23H2".into(),
        "1.0.0".into(),
    )
    .expect("click");

    let window_ids = sdk
        .adapter_windows_click_window_ids_from_batch()
        .expect("batch");
    assert_eq!(window_ids, vec!["win_checkout_primary".to_string()]);
}
