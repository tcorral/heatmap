use std::ffi::CString;

use heatmap_ffi::c_abi::{
    heatmap_sdk_flush, heatmap_sdk_free, heatmap_sdk_identify, heatmap_sdk_init,
    heatmap_sdk_poll_flush, heatmap_sdk_screen, heatmap_sdk_set_tracking_enabled,
    heatmap_sdk_sign_out, HeatmapSdkHandle, HEATMAP_OK,
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

fn init_test_handle() -> HeatmapSdkHandle {
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
    handle
}

#[test]
fn should_serialize_concurrent_calls_on_one_c_abi_handle() {
    let handle = init_test_handle();
    let handle_bits = handle as usize;
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(move || {
                let handle = handle_bits as HeatmapSdkHandle;
                for _ in 0..40 {
                    assert_eq!(heatmap_sdk_sign_out(handle), HEATMAP_OK);
                    assert_eq!(heatmap_sdk_set_tracking_enabled(handle, 1), HEATMAP_OK);
                    assert_eq!(heatmap_sdk_poll_flush(handle), HEATMAP_OK);
                }
            });
        }
    });
    heatmap_sdk_free(handle);
}

#[test]
fn should_return_null_error_for_null_handle() {
    assert_eq!(
        heatmap_sdk_sign_out(std::ptr::null_mut()),
        heatmap_ffi::c_abi::HEATMAP_ERR_NULL
    );
}

#[test]
fn should_reject_or_complete_calls_overlapping_free() {
    let handle = init_test_handle();
    let handle_bits = handle as usize;
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(move || {
                let handle = handle_bits as HeatmapSdkHandle;
                for _ in 0..40 {
                    let _ = heatmap_sdk_sign_out(handle);
                    let _ = heatmap_sdk_poll_flush(handle);
                }
            });
        }
        scope.spawn(move || {
            heatmap_sdk_free(handle_bits as HeatmapSdkHandle);
        });
    });
    assert_eq!(
        heatmap_sdk_sign_out(handle_bits as HeatmapSdkHandle),
        heatmap_ffi::c_abi::HEATMAP_ERR_NULL
    );
}
