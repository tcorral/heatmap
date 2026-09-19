use std::fmt;

use crate::batch::HeatmapBatch;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlushError {
    Transport(String),
    HttpStatus {
        code: u16,
        retry_after_secs: Option<u64>,
    },
}

impl FlushError {
    pub fn http_status(code: u16, retry_after_secs: Option<u64>) -> Self {
        Self::HttpStatus {
            code,
            retry_after_secs,
        }
    }
}

impl fmt::Display for FlushError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(message) => write!(f, "batch upload failed: {message}"),
            Self::HttpStatus { code, .. } => {
                write!(f, "batch upload failed with HTTP status {code}")
            }
        }
    }
}

impl std::error::Error for FlushError {}

pub fn batch_ingest_url(endpoint: &str) -> String {
    let base = endpoint.trim_end_matches('/');
    format!("{base}/v1/batches")
}

pub trait BatchTransport: Send + Sync {
    fn send_batch(
        &self,
        endpoint: &str,
        project_key: &str,
        batch: &HeatmapBatch,
    ) -> Result<(), FlushError>;
}

#[cfg(not(target_arch = "wasm32"))]
pub struct HttpBatchTransport;

#[cfg(not(target_arch = "wasm32"))]
impl Default for HttpBatchTransport {
    fn default() -> Self {
        Self
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl HttpBatchTransport {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl BatchTransport for HttpBatchTransport {
    fn send_batch(
        &self,
        endpoint: &str,
        project_key: &str,
        batch: &HeatmapBatch,
    ) -> Result<(), FlushError> {
        let url = batch_ingest_url(endpoint);
        let response = ureq::post(&url)
            .set("X-Project-Key", project_key)
            .send_json(batch)
            .map_err(|err| FlushError::Transport(err.to_string()))?;
        let status = response.status();
        if (200..300).contains(&status) {
            Ok(())
        } else {
            let retry_after_secs = parse_retry_after_header(&response);
            Err(FlushError::http_status(status, retry_after_secs))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_retry_after_header(response: &ureq::Response) -> Option<u64> {
    response
        .header("Retry-After")
        .and_then(|value| value.parse::<u64>().ok())
}
