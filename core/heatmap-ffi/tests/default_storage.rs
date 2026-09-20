use std::fs;

use heatmap_core::HeatmapConfig;
use heatmap_ffi::public_api::{default_native_storage_directory, PublicHeatmapSdk};

fn valid_config() -> HeatmapConfig {
    HeatmapConfig {
        project_key: "pk_test".into(),
        endpoint: "https://ingest.example.com".into(),
        session_timeout_ms: HeatmapConfig::DEFAULT_SESSION_TIMEOUT_MS,
        sample_rate: HeatmapConfig::DEFAULT_SAMPLE_RATE,
        exclude_elements: vec![],
        exclude_screens: vec![],
        mask_elements: vec![],
        app_version: HeatmapConfig::DEFAULT_APP_VERSION.into(),
    }
}

#[test]
fn should_persist_identity_when_storage_directory_omitted() {
    let dir = default_native_storage_directory();
    let user_file = dir.join("heatmap.user_id.dat");
    let _ = fs::remove_file(&user_file);
    {
        let mut sdk = PublicHeatmapSdk::init(valid_config()).expect("init");
        sdk.identify("user-stable".into()).expect("identify");
    }
    let stored = fs::read_to_string(&user_file).expect("persisted user id");
    assert_eq!(stored, "user-stable");
}
