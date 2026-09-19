use heatmap_wasm::PublicHeatmapSdk;

#[test]
fn should_enqueue_marked_web_click_through_adapter_only_api() {
    let config = r#"{
        "project_key": "pk_test",
        "endpoint": "https://ingest.example.com"
    }"#;
    let mut sdk = PublicHeatmapSdk::init_from_json(config).expect("init");
    sdk.record_activation_click(
        0.5,
        0.5,
        Some("checkout.pay".into()),
        1280,
        720,
        1_726_771_205_000,
    )
    .expect("record");

    let err = sdk.flush().expect_err("stub transport should reject pending batch");
    assert!(err.to_string().contains("not wired"));
}
