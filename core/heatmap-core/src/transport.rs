use std::fmt;

use crate::batch::HeatmapBatch;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlushError {
    Transport(String),
    HttpStatus(u16),
}

impl fmt::Display for FlushError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(message) => write!(f, "batch upload failed: {message}"),
            Self::HttpStatus(code) => write!(f, "batch upload failed with HTTP status {code}"),
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

pub struct HttpBatchTransport;

impl Default for HttpBatchTransport {
    fn default() -> Self {
        Self
    }
}

impl HttpBatchTransport {
    pub fn new() -> Self {
        Self
    }
}

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
            Err(FlushError::HttpStatus(status))
        }
    }
}
