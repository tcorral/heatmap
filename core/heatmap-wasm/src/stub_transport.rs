use heatmap_core::{BatchTransport, FlushError, HeatmapBatch};

/// Placeholder transport until web fetch upload is wired (later steps).
pub struct StubBatchTransport;

impl StubBatchTransport {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StubBatchTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl BatchTransport for StubBatchTransport {
    fn send_batch(
        &self,
        _endpoint: &str,
        _project_key: &str,
        _batch: &HeatmapBatch,
    ) -> Result<(), FlushError> {
        Err(FlushError::Transport(
            "wasm batch upload is not wired yet".into(),
        ))
    }
}
