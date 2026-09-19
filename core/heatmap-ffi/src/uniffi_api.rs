use std::sync::Mutex;

use crate::errors::{FlushError, IdentityError, InitError, ScreenContextError};
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
}
