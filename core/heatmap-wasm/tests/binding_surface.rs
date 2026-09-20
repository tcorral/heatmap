use std::fs;
use std::path::PathBuf;

use heatmap_wasm::public_api::{export_names, forbidden_export_names, PublicHeatmapSdk};

fn lib_rs_source() -> String {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(manifest.join("src/wasm_bindings.rs")).expect("read wasm_bindings.rs")
}

#[test]
fn should_expose_public_sdk_methods_on_wasm_bindings() {
    let source = lib_rs_source();
    assert!(source.contains(&format!("js_name = {}", export_names::INIT)));
    assert!(source.contains(&format!("js_name = {}", export_names::IDENTIFY)));
    assert!(source.contains(&format!("js_name = {}", export_names::SIGN_OUT)));
    assert!(source.contains(&format!("js_name = {}", export_names::SCREEN)));
    assert!(source.contains(&format!("js_name = {}", export_names::SET_TRACKING_ENABLED)));
    assert!(source.contains(&format!("js_name = {}", export_names::FLUSH)));
}

#[test]
fn should_not_expose_track_or_internal_capture_on_wasm_bindings() {
    let source = lib_rs_source();
    for forbidden in [
        forbidden_export_names::TRACK,
        forbidden_export_names::ENQUEUE_CANDIDATE,
        forbidden_export_names::CAPTURE,
    ] {
        assert!(
            !source.contains(&format!("js_name = {forbidden}")),
            "unexpected public export: {forbidden}"
        );
    }
}

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
