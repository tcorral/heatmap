use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct HeatmapConfig {
    pub project_key: String,
    pub endpoint: String,
    pub session_timeout_ms: u64,
    pub sample_rate: f64,
    pub exclude_elements: Vec<String>,
    pub exclude_screens: Vec<String>,
    pub mask_elements: Vec<String>,
}

impl HeatmapConfig {
    pub const DEFAULT_SESSION_TIMEOUT_MS: u64 = 1_800_000;
    pub const DEFAULT_SAMPLE_RATE: f64 = 1.0;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitError {
    MissingProjectKey,
    MissingEndpoint,
    InsecureEndpoint,
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingProjectKey => write!(f, "project_key is required"),
            Self::MissingEndpoint => write!(f, "endpoint is required"),
            Self::InsecureEndpoint => write!(f, "endpoint must use HTTPS"),
        }
    }
}

impl std::error::Error for InitError {}

pub(crate) fn validate_config(config: &HeatmapConfig) -> Result<(), InitError> {
    if config.project_key.trim().is_empty() {
        return Err(InitError::MissingProjectKey);
    }
    if config.endpoint.trim().is_empty() {
        return Err(InitError::MissingEndpoint);
    }
    if !config.endpoint.starts_with("https://") {
        return Err(InitError::InsecureEndpoint);
    }
    Ok(())
}
