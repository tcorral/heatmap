use std::collections::HashMap;
use std::ffi::{c_char, c_void, CStr};
use std::ptr;
use std::sync::{Arc, Condvar, Mutex, OnceLock};

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

struct HandleGate {
    in_flight: usize,
    closed: bool,
}

struct HandleInner {
    sdk: Mutex<PublicHeatmapSdk>,
    gate: Mutex<HandleGate>,
    wait: Condvar,
}

fn read_cstr(ptr: *const c_char) -> Result<String, i32> {
    if ptr.is_null() {
        return Err(HEATMAP_ERR_NULL);
    }
    let cstr = unsafe { CStr::from_ptr(ptr) };
    cstr.to_str()
        .map(|s| s.to_owned())
        .map_err(|_| HEATMAP_ERR_UTF8)
}

fn lock_mutex<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn handle_table() -> &'static Mutex<HashMap<usize, Arc<HandleInner>>> {
    static HANDLES: OnceLock<Mutex<HashMap<usize, Arc<HandleInner>>>> = OnceLock::new();
    HANDLES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn begin_sdk_use(inner: &HandleInner) -> Result<(), i32> {
    let mut gate = lock_mutex(&inner.gate);
    if gate.closed {
        return Err(HEATMAP_ERR_NULL);
    }
    gate.in_flight += 1;
    Ok(())
}

fn end_sdk_use(inner: &HandleInner) {
    let mut gate = lock_mutex(&inner.gate);
    gate.in_flight -= 1;
    if gate.in_flight == 0 {
        inner.wait.notify_all();
    }
}

fn acquire_handle(handle: HeatmapSdkHandle) -> Result<Arc<HandleInner>, i32> {
    if handle.is_null() {
        return Err(HEATMAP_ERR_NULL);
    }
    lock_mutex(handle_table())
        .get(&(handle as usize))
        .cloned()
        .ok_or(HEATMAP_ERR_NULL)
}

fn with_sdk<F>(handle: HeatmapSdkHandle, action: F) -> i32
where
    F: FnOnce(&mut PublicHeatmapSdk) -> i32,
{
    let inner = match acquire_handle(handle) {
        Ok(inner) => inner,
        Err(code) => return code,
    };
    if begin_sdk_use(&inner).is_err() {
        return HEATMAP_ERR_NULL;
    }
    let result = {
        let mut sdk = lock_mutex(&inner.sdk);
        action(&mut sdk)
    };
    end_sdk_use(&inner);
    result
}

fn wait_until_idle_then_close(inner: &HandleInner) {
    let mut gate = lock_mutex(&inner.gate);
    gate.closed = true;
    while gate.in_flight > 0 {
        gate = inner
            .wait
            .wait(gate)
            .unwrap_or_else(|poisoned| poisoned.into_inner());
    }
}

fn into_handle(sdk: PublicHeatmapSdk) -> HeatmapSdkHandle {
    let inner = Arc::new(HandleInner {
        sdk: Mutex::new(sdk),
        gate: Mutex::new(HandleGate {
            in_flight: 0,
            closed: false,
        }),
        wait: Condvar::new(),
    });
    let key = Arc::as_ptr(&inner) as usize;
    lock_mutex(handle_table()).insert(key, inner);
    key as HeatmapSdkHandle
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
            unsafe {
                *out_handle = into_handle(sdk);
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
    let inner = match lock_mutex(handle_table()).remove(&(handle as usize)) {
        Some(inner) => inner,
        None => return,
    };
    wait_until_idle_then_close(&inner);
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
pub extern "C" fn heatmap_sdk_poll_flush(handle: HeatmapSdkHandle) -> i32 {
    with_sdk(handle, |sdk| match sdk.poll_flush() {
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
    window_id: *const c_char,
    os_version: *const c_char,
    app_version: *const c_char,
) -> i32 {
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
        match sdk.record_windows_screen_view(viewport_width, viewport_height, window, os, app) {
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
    window_id: *const c_char,
    os_version: *const c_char,
    app_version: *const c_char,
) -> i32 {
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
        match sdk.record_windows_scroll(viewport_width, viewport_height, window, os, app) {
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
        std::ffi::CString::new(json)
            .map_err(|_| HEATMAP_ERR_UTF8)?
            .as_ptr(),
        &mut handle,
    );
    if code == HEATMAP_OK {
        Ok(handle)
    } else {
        Err(code)
    }
}
