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
    pub app_version: String,
}

impl HeatmapConfig {
    pub const DEFAULT_SESSION_TIMEOUT_MS: u64 = 1_800_000;
    pub const DEFAULT_SAMPLE_RATE: f64 = 1.0;
    pub const DEFAULT_APP_VERSION: &str = "1.0.0";
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitError {
    MissingProjectKey,
    MissingEndpoint,
    InsecureEndpoint,
    InvalidConfig,
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingProjectKey => write!(f, "project_key is required"),
            Self::MissingEndpoint => write!(f, "endpoint is required"),
            Self::InsecureEndpoint => write!(f, "endpoint must use HTTPS"),
            Self::InvalidConfig => write!(f, "config is not valid JSON"),
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
    if !endpoint_is_allowed(&config.endpoint) {
        return Err(InitError::InsecureEndpoint);
    }
    Ok(())
}

fn endpoint_is_allowed(endpoint: &str) -> bool {
    let trimmed = endpoint.trim();
    if trimmed.starts_with("https://") {
        return true;
    }
    is_loopback_http(trimmed)
}

fn is_loopback_http(endpoint: &str) -> bool {
    let Some(rest) = endpoint.strip_prefix("http://") else {
        return false;
    };
    let hostport = rest.split(['/', '?']).next().unwrap_or(rest);
    let host = hostport
        .rsplit_once('@')
        .map(|(_, host)| host)
        .unwrap_or(hostport);
    let hostname = host.rsplit_once(':').map(|(name, _)| name).unwrap_or(host);
    hostname == "127.0.0.1" || hostname == "localhost"
}
