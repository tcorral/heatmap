use std::ffi::CString;

use heatmap_ffi::c_abi::{
    heatmap_sdk_flush, heatmap_sdk_free, heatmap_sdk_identify, heatmap_sdk_init,
    heatmap_sdk_screen, heatmap_sdk_set_tracking_enabled, heatmap_sdk_sign_out, HeatmapSdkHandle,
    HEATMAP_OK,
};

#[test]
fn should_drive_c_abi_init_identify_screen_and_flush() {
    let config = CString::new(
        r#"{
        "project_key": "pk_test",
        "endpoint": "https://ingest.example.com"
    }"#,
    )
    .expect("config cstring");

    let mut handle: HeatmapSdkHandle = std::ptr::null_mut();
    assert_eq!(
        heatmap_sdk_init(config.as_ptr(), &mut handle),
        HEATMAP_OK,
        "init"
    );
    assert!(!handle.is_null());

    let user = CString::new("user-99").expect("user id");
    assert_eq!(
        heatmap_sdk_identify(handle, user.as_ptr()),
        HEATMAP_OK,
        "identify"
    );

    let screen = CString::new("Home").expect("screen");
    assert_eq!(
        heatmap_sdk_screen(handle, screen.as_ptr(), std::ptr::null()),
        HEATMAP_OK,
        "screen"
    );

    assert_eq!(
        heatmap_sdk_set_tracking_enabled(handle, 1),
        HEATMAP_OK,
        "tracking"
    );
    assert_eq!(heatmap_sdk_flush(handle), HEATMAP_OK, "flush");
    assert_eq!(heatmap_sdk_sign_out(handle), HEATMAP_OK, "sign_out");

    heatmap_sdk_free(handle);
}
