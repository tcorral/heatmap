use std::fs;
use std::path::PathBuf;

use heatmap_ffi::public_api::{
    c_export_names, export_names, forbidden_export_names,
};

fn read_src(name: &str) -> String {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(manifest.join("src").join(name)).expect("read source")
}

fn ffi_source() -> String {
    [
        read_src("uniffi_api.rs"),
        read_src("c_abi.rs"),
        read_src("public_api.rs"),
        read_src("lib.rs"),
    ]
    .join("\n")
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
fn should_not_expose_adapter_only_apis_on_ffi_boundary() {
    let source = ffi_source();
    for forbidden in [
        forbidden_export_names::TRACK,
        forbidden_export_names::ENQUEUE_CANDIDATE,
        forbidden_export_names::RECORD_ACTIVATION_CLICK,
        forbidden_export_names::CAPTURE,
        forbidden_export_names::DRAIN_BATCH_JSON,
    ] {
        assert!(
            !source.contains(&format!("pub fn {forbidden}")),
            "unexpected public FFI export: {forbidden}"
        );
        assert!(
            !source.contains(&format!("fn heatmap_sdk_{forbidden}")),
            "unexpected C ABI export: {forbidden}"
        );
    }
}
