use heatmap_ffi::PublicHeatmapSdk;

#[test]
fn should_keep_session_after_background_then_foreground_before_timeout() {
    let config = r#"{
        "project_key": "pk_test",
        "endpoint": "https://ingest.example.com",
        "session_timeout_ms": 10000
    }"#;
    let mut sdk = PublicHeatmapSdk::init_from_json(config).expect("init");
    sdk.record_windows_activation_click(
        0.5,
        0.5,
        Some("checkout.pay".into()),
        800,
        600,
        100,
        "win_a".into(),
        "Windows 11".into(),
        "1.0.0".into(),
    )
    .expect("first click");
    let first_ids = sdk
        .adapter_click_session_ids_from_batch()
        .expect("first batch");
    assert_eq!(first_ids.len(), 1);
    let session_id = first_ids[0].clone();

    sdk.on_background();
    sdk.on_foreground();
    sdk.record_windows_activation_click(
        0.25,
        0.5,
        None,
        800,
        600,
        200,
        "win_a".into(),
        "Windows 11".into(),
        "1.0.0".into(),
    )
    .expect("second click");
    let second_ids = sdk
        .adapter_click_session_ids_from_batch()
        .expect("second batch");
    assert_eq!(second_ids.len(), 1);
    assert_eq!(second_ids[0], session_id);
}
