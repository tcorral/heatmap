use heatmap_core::{PlatformInfo, PlatformType, SdkInfo, Viewport};

pub const WEB_SDK_NAME: &str = "heatmap-web";
pub const WEB_SDK_VERSION: &str = "0.1.0";

pub fn web_sdk_info() -> SdkInfo {
    SdkInfo {
        name: WEB_SDK_NAME.into(),
        version: WEB_SDK_VERSION.into(),
    }
}

pub fn web_platform_info(app_version: &str) -> PlatformInfo {
    PlatformInfo {
        platform_type: PlatformType::Web,
        os_version: web_os_version(),
        app_version: app_version.to_string(),
    }
}

pub fn viewport_from_window() -> Viewport {
    let window = web_sys::window().unwrap_or_else(|| unreachable!("browser window"));
    Viewport {
        width: window
            .inner_width()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0) as u32,
        height: window
            .inner_height()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0) as u32,
    }
}

fn web_os_version() -> String {
    web_sys::window()
        .map(|window| window.navigator())
        .and_then(|navigator| navigator.user_agent().ok())
        .unwrap_or_else(|| "browser".into())
}
