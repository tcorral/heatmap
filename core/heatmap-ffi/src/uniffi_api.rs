use std::sync::Mutex;

use crate::errors::{CaptureError, FlushError, IdentityError, InitError, ScreenContextError};
use crate::public_api::PublicHeatmapSdk;

#[derive(uniffi::Record)]
pub struct InitConfig {
    pub project_key: String,
    pub endpoint: String,
    pub session_timeout_ms: u64,
    pub sample_rate: f64,
    pub exclude_elements: Vec<String>,
    pub exclude_screens: Vec<String>,
    pub mask_elements: Vec<String>,
}

impl From<InitConfig> for heatmap_core::HeatmapConfig {
    fn from(value: InitConfig) -> Self {
        Self {
            project_key: value.project_key,
            endpoint: value.endpoint,
            session_timeout_ms: value.session_timeout_ms,
            sample_rate: value.sample_rate,
            exclude_elements: value.exclude_elements,
            exclude_screens: value.exclude_screens,
            mask_elements: value.mask_elements,
        }
    }
}

#[derive(uniffi::Object)]
pub struct HeatmapSdk {
    state: Mutex<PublicHeatmapSdk>,
}

#[uniffi::export]
impl HeatmapSdk {
    #[uniffi::constructor]
    pub fn init_from_json(config_json: String) -> Result<Self, InitError> {
        let inner = PublicHeatmapSdk::init_from_json(&config_json).map_err(InitError::from)?;
        Ok(Self {
            state: Mutex::new(inner),
        })
    }

    #[uniffi::constructor(name = "init")]
    pub fn init_with_config(config: InitConfig) -> Result<Self, InitError> {
        let core_config: heatmap_core::HeatmapConfig = config.into();
        let inner = PublicHeatmapSdk::init(core_config).map_err(InitError::from)?;
        Ok(Self {
            state: Mutex::new(inner),
        })
    }

    pub fn identify(&self, user_id: String) -> Result<(), IdentityError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard.identify(user_id).map_err(IdentityError::from)
    }

    pub fn sign_out(&self) {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard.sign_out();
    }

    pub fn screen(
        &self,
        screen_id: String,
        layout_version: Option<String>,
    ) -> Result<(), ScreenContextError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .screen(screen_id, layout_version)
            .map_err(ScreenContextError::from)
    }

    pub fn set_tracking_enabled(&self, enabled: bool) {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard.set_tracking_enabled(enabled);
    }

    pub fn flush(&self) -> Result<(), FlushError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard.flush().map_err(FlushError::from)
    }

    /// iOS adapter-only lifecycle hook.
    pub fn on_background(&self) {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard.on_background();
    }

    /// iOS adapter-only lifecycle hook.
    pub fn on_foreground(&self) {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard.on_foreground();
    }

    /// iOS adapter-only click enqueue; not exposed on `HeatmapClient`.
    pub fn record_activation_click(
        &self,
        x: f64,
        y: f64,
        element_id: Option<String>,
        viewport_width: u32,
        viewport_height: u32,
        timestamp_ms: i64,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .record_activation_click(
                x,
                y,
                element_id,
                viewport_width,
                viewport_height,
                timestamp_ms,
                os_version,
                app_version,
            )
            .map_err(CaptureError::from)
    }

    pub fn record_ios_screen_view(
        &self,
        viewport_width: u32,
        viewport_height: u32,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .record_ios_screen_view(viewport_width, viewport_height, os_version, app_version)
            .map_err(CaptureError::from)
    }

    pub fn record_ios_scroll(
        &self,
        viewport_width: u32,
        viewport_height: u32,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .record_ios_scroll(viewport_width, viewport_height, os_version, app_version)
            .map_err(CaptureError::from)
    }

    /// macOS adapter-only click enqueue; not exposed on `HeatmapClient`.
    pub fn record_macos_activation_click(
        &self,
        x: f64,
        y: f64,
        element_id: Option<String>,
        viewport_width: u32,
        viewport_height: u32,
        timestamp_ms: i64,
        window_id: String,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .record_macos_activation_click(
                x,
                y,
                element_id,
                viewport_width,
                viewport_height,
                timestamp_ms,
                window_id,
                os_version,
                app_version,
            )
            .map_err(CaptureError::from)
    }

    pub fn record_macos_screen_view(
        &self,
        viewport_width: u32,
        viewport_height: u32,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .record_macos_screen_view(viewport_width, viewport_height, os_version, app_version)
            .map_err(CaptureError::from)
    }

    pub fn record_macos_scroll(
        &self,
        viewport_width: u32,
        viewport_height: u32,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .record_macos_scroll(viewport_width, viewport_height, os_version, app_version)
            .map_err(CaptureError::from)
    }

    /// Returns window ids for macOS click events from a built batch (adapter tests).
    pub fn adapter_macos_click_window_ids_from_batch(&self) -> Result<Vec<String>, CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .adapter_macos_click_window_ids_from_batch()
            .map_err(CaptureError::from)
    }

    /// Returns session ids for click events from a built batch (adapter tests).
    pub fn adapter_click_session_ids_from_batch(&self) -> Result<Vec<String>, CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .adapter_click_session_ids_from_batch()
            .map_err(CaptureError::from)
    }
}
