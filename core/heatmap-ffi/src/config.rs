use heatmap_core::{HeatmapConfig, InitError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InitConfigJson {
    pub project_key: String,
    pub endpoint: String,
    #[serde(default = "default_session_timeout_ms")]
    pub session_timeout_ms: u64,
    #[serde(default = "default_sample_rate")]
    pub sample_rate: f64,
    #[serde(default)]
    pub exclude_elements: Vec<String>,
    #[serde(default)]
    pub exclude_screens: Vec<String>,
    #[serde(default)]
    pub mask_elements: Vec<String>,
    #[serde(default)]
    pub storage_directory: Option<String>,
    #[serde(default = "default_app_version")]
    pub app_version: String,
}

fn default_app_version() -> String {
    HeatmapConfig::DEFAULT_APP_VERSION.to_string()
}

fn default_session_timeout_ms() -> u64 {
    HeatmapConfig::DEFAULT_SESSION_TIMEOUT_MS
}

fn default_sample_rate() -> f64 {
    HeatmapConfig::DEFAULT_SAMPLE_RATE
}

impl InitConfigJson {
    pub fn parse(raw: &str) -> Result<Self, InitError> {
        serde_json::from_str(raw).map_err(|_| InitError::InvalidConfig)
    }

    pub fn into_core_config(self) -> HeatmapConfig {
        HeatmapConfig {
            project_key: self.project_key,
            endpoint: self.endpoint,
            session_timeout_ms: self.session_timeout_ms,
            sample_rate: self.sample_rate,
            exclude_elements: self.exclude_elements,
            exclude_screens: self.exclude_screens,
            mask_elements: self.mask_elements,
            app_version: self.app_version,
        }
    }
}
