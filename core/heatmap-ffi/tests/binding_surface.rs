use std::fs;
use std::path::PathBuf;

use heatmap_ffi::public_api::{
    c_export_names, export_names, forbidden_export_names,
};

fn read_src(name: &str) -> String {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(manifest.join("src").join(name)).expect("read source")
}

#[test]
fn should_expose_public_methods_on_uniffi_layer() {
    let source = read_src("uniffi_api.rs");
    for name in [
        export_names::INIT_FROM_JSON,
        export_names::IDENTIFY,
        export_names::SIGN_OUT,
        export_names::SCREEN,
        export_names::SET_TRACKING_ENABLED,
        export_names::FLUSH,
    ] {
        assert!(
            source.contains(&format!("fn {name}")),
            "missing uniffi method: {name}"
        );
    }
    assert!(
        source.contains("fn init_with_config"),
        "structured init constructor"
    );
}

#[test]
fn should_expose_stable_c_abi_symbols() {
    let source = read_src("c_abi.rs");
    for symbol in [
        c_export_names::INIT,
        c_export_names::FREE,
        c_export_names::IDENTIFY,
        c_export_names::SIGN_OUT,
        c_export_names::SCREEN,
        c_export_names::SET_TRACKING_ENABLED,
        c_export_names::FLUSH,
    ] {
        assert!(
            source.contains(&format!("fn {symbol}")),
            "missing C ABI export: {symbol}"
        );
    }
}

#[test]
fn should_not_expose_adapter_only_apis_on_integrator_surface() {
    let public_api = read_src("public_api.rs");
    let c_abi = read_src("c_abi.rs");
    let ios_adapter = read_src("ios_adapter.rs");
    let macos_adapter = read_src("macos_adapter.rs");
    let android_adapter = read_src("android_adapter.rs");
    assert!(
        ios_adapter.contains("record_activation_click"),
        "ios adapter should own activation enqueue"
    );
    assert!(
        macos_adapter.contains("record_macos_activation_click"),
        "macos adapter should own activation enqueue with window_id"
    );
    assert!(
        android_adapter.contains("record_android_activation_click"),
        "android adapter should own view activation enqueue"
    );
    for forbidden in [
        forbidden_export_names::TRACK,
        forbidden_export_names::ENQUEUE_CANDIDATE,
        forbidden_export_names::RECORD_ACTIVATION_CLICK,
        forbidden_export_names::CAPTURE,
        forbidden_export_names::DRAIN_BATCH_JSON,
    ] {
        for source in [&public_api, &c_abi] {
            assert!(
                !source.contains(&format!("pub fn {forbidden}")),
                "unexpected integrator export: {forbidden}"
            );
        }
        assert!(
            !c_abi.contains(&format!("fn heatmap_sdk_{forbidden}")),
            "unexpected C ABI export: {forbidden}"
        );
    }
}
