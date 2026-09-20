use std::fs;
use std::path::PathBuf;

use heatmap_ffi::public_api::{c_export_names, export_names, forbidden_export_names};

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
        export_names::POLL_FLUSH,
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
        c_export_names::POLL_FLUSH,
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
    let windows_adapter = read_src("windows_adapter.rs");
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
    assert!(
        windows_adapter.contains("record_windows_activation_click"),
        "windows adapter should own activation enqueue with window_id"
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

#[test]
fn should_keep_generated_init_config_in_sync_with_uniffi_record() {
    let uniffi = read_src("uniffi_api.rs");
    assert!(
        uniffi.contains("pub app_version: String"),
        "uniffi InitConfig must include app_version"
    );
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let swift =
        fs::read_to_string(root.join("sdk/apple/shared/Sources/HeatmapFFI/heatmap_ffi.swift"))
            .expect("swift bindings");
    assert!(
        swift.contains("self.appVersion = appVersion"),
        "Swift InitConfig missing appVersion"
    );
    let kotlin = fs::read_to_string(
        root.join("sdk/android/heatmap-android/src/main/kotlin/com/heatmap/sdk/heatmap_ffi.kt"),
    )
    .expect("kotlin bindings");
    assert!(
        kotlin.contains("var `appVersion`"),
        "Kotlin InitConfig missing appVersion"
    );
}
