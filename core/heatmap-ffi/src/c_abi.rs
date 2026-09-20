use std::ffi::{c_char, c_void, CStr};
use std::ptr;

use crate::public_api::PublicHeatmapSdk;

pub type HeatmapSdkHandle = *mut c_void;

pub const HEATMAP_OK: i32 = 0;
pub const HEATMAP_ERR_NULL: i32 = 1;
pub const HEATMAP_ERR_UTF8: i32 = 2;
pub const HEATMAP_ERR_INIT: i32 = 3;
pub const HEATMAP_ERR_IDENTITY: i32 = 4;
pub const HEATMAP_ERR_SCREEN: i32 = 5;
pub const HEATMAP_ERR_FLUSH: i32 = 6;
pub const HEATMAP_ERR_CAPTURE: i32 = 7;

struct SdkBox(PublicHeatmapSdk);

fn read_cstr(ptr: *const c_char) -> Result<String, i32> {
    if ptr.is_null() {
        return Err(HEATMAP_ERR_NULL);
    }
    let cstr = unsafe { CStr::from_ptr(ptr) };
    cstr.to_str()
        .map(|s| s.to_owned())
        .map_err(|_| HEATMAP_ERR_UTF8)
}

fn with_sdk<F>(handle: HeatmapSdkHandle, action: F) -> i32
where
    F: FnOnce(&mut PublicHeatmapSdk) -> i32,
{
    if handle.is_null() {
        return HEATMAP_ERR_NULL;
    }
    let sdk = unsafe { &mut (*(handle as *mut SdkBox)).0 };
    action(sdk)
}

#[no_mangle]
pub extern "C" fn heatmap_sdk_init(
    config_json: *const c_char,
    out_handle: *mut HeatmapSdkHandle,
) -> i32 {
    if out_handle.is_null() {
        return HEATMAP_ERR_NULL;
    }
    let json = match read_cstr(config_json) {
        Ok(value) => value,
        Err(code) => return code,
    };
    match PublicHeatmapSdk::init_from_json(&json) {
        Ok(sdk) => {
            let boxed = Box::new(SdkBox(sdk));
            unsafe {
                *out_handle = Box::into_raw(boxed) as HeatmapSdkHandle;
            }
            HEATMAP_OK
        }
        Err(_) => HEATMAP_ERR_INIT,
    }
}

#[no_mangle]
pub extern "C" fn heatmap_sdk_free(handle: HeatmapSdkHandle) {
    if handle.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(handle as *mut SdkBox));
    }
}

#[no_mangle]
pub extern "C" fn heatmap_sdk_identify(handle: HeatmapSdkHandle, user_id: *const c_char) -> i32 {
    let id = match read_cstr(user_id) {
        Ok(value) => value,
        Err(code) => return code,
    };
    with_sdk(handle, |sdk| match sdk.identify(id) {
        Ok(()) => HEATMAP_OK,
        Err(_) => HEATMAP_ERR_IDENTITY,
    })
}

#[no_mangle]
pub extern "C" fn heatmap_sdk_sign_out(handle: HeatmapSdkHandle) -> i32 {
    with_sdk(handle, |sdk| {
        sdk.sign_out();
        HEATMAP_OK
    })
}

#[no_mangle]
pub extern "C" fn heatmap_sdk_screen(
    handle: HeatmapSdkHandle,
    screen_id: *const c_char,
    layout_version: *const c_char,
) -> i32 {
    let screen = match read_cstr(screen_id) {
        Ok(value) => value,
        Err(code) => return code,
    };
    let layout = nullable_cstr(layout_version);
    with_sdk(handle, |sdk| match sdk.screen(screen, layout) {
        Ok(()) => HEATMAP_OK,
        Err(_) => HEATMAP_ERR_SCREEN,
    })
}

#[no_mangle]
pub extern "C" fn heatmap_sdk_set_tracking_enabled(handle: HeatmapSdkHandle, enabled: u8) -> i32 {
    with_sdk(handle, |sdk| {
        sdk.set_tracking_enabled(enabled != 0);
        HEATMAP_OK
    })
}

#[no_mangle]
pub extern "C" fn heatmap_sdk_flush(handle: HeatmapSdkHandle) -> i32 {
    with_sdk(handle, |sdk| match sdk.flush() {
        Ok(()) => HEATMAP_OK,
        Err(_) => HEATMAP_ERR_FLUSH,
    })
}

#[no_mangle]
pub extern "C" fn heatmap_sdk_adapter_on_background(handle: HeatmapSdkHandle) -> i32 {
    with_sdk(handle, |sdk| {
        sdk.on_background();
        HEATMAP_OK
    })
}

#[no_mangle]
pub extern "C" fn heatmap_sdk_adapter_on_foreground(handle: HeatmapSdkHandle) -> i32 {
    with_sdk(handle, |sdk| {
        sdk.on_foreground();
        HEATMAP_OK
    })
}

#[no_mangle]
pub extern "C" fn heatmap_sdk_adapter_record_windows_activation_click(
    handle: HeatmapSdkHandle,
    x: f64,
    y: f64,
    element_id: *const c_char,
    viewport_width: u32,
    viewport_height: u32,
    timestamp_ms: i64,
    window_id: *const c_char,
    os_version: *const c_char,
    app_version: *const c_char,
) -> i32 {
    let element = nullable_cstr(element_id);
    let window = match read_cstr(window_id) {
        Ok(value) => value,
        Err(code) => return code,
    };
    let os = match read_cstr(os_version) {
        Ok(value) => value,
        Err(code) => return code,
    };
    let app = match read_cstr(app_version) {
        Ok(value) => value,
        Err(code) => return code,
    };
    with_sdk(handle, |sdk| {
        match sdk.record_windows_activation_click(
            x,
            y,
            element,
            viewport_width,
            viewport_height,
            timestamp_ms,
            window,
            os,
            app,
        ) {
            Ok(()) => HEATMAP_OK,
            Err(_) => HEATMAP_ERR_CAPTURE,
        }
    })
}

#[no_mangle]
pub extern "C" fn heatmap_sdk_adapter_record_windows_screen_view(
    handle: HeatmapSdkHandle,
    viewport_width: u32,
    viewport_height: u32,
    os_version: *const c_char,
    app_version: *const c_char,
) -> i32 {
    let os = match read_cstr(os_version) {
        Ok(value) => value,
        Err(code) => return code,
    };
    let app = match read_cstr(app_version) {
        Ok(value) => value,
        Err(code) => return code,
    };
    with_sdk(handle, |sdk| {
        match sdk.record_windows_screen_view(viewport_width, viewport_height, os, app) {
            Ok(()) => HEATMAP_OK,
            Err(_) => HEATMAP_ERR_CAPTURE,
        }
    })
}

#[no_mangle]
pub extern "C" fn heatmap_sdk_adapter_record_windows_scroll(
    handle: HeatmapSdkHandle,
    viewport_width: u32,
    viewport_height: u32,
    os_version: *const c_char,
    app_version: *const c_char,
) -> i32 {
    let os = match read_cstr(os_version) {
        Ok(value) => value,
        Err(code) => return code,
    };
    let app = match read_cstr(app_version) {
        Ok(value) => value,
        Err(code) => return code,
    };
    with_sdk(handle, |sdk| {
        match sdk.record_windows_scroll(viewport_width, viewport_height, os, app) {
            Ok(()) => HEATMAP_OK,
            Err(_) => HEATMAP_ERR_CAPTURE,
        }
    })
}

fn nullable_cstr(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    read_cstr(ptr).ok()
}

pub fn init_handle_from_json(json: &str) -> Result<HeatmapSdkHandle, i32> {
    let mut handle = ptr::null_mut();
    let code = heatmap_sdk_init(
        std::ffi::CString::new(json).map_err(|_| HEATMAP_ERR_UTF8)?.as_ptr(),
        &mut handle,
    );
    if code == HEATMAP_OK {
        Ok(handle)
    } else {
        Err(code)
    }
}
