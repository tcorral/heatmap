use heatmap_core::{batch_ingest_url, BatchTransport, FlushError, HeatmapBatch};
use wasm_bindgen::JsCast;
use web_sys::{XmlHttpRequest, XmlHttpRequestResponseType};

pub struct FetchBatchTransport;

impl FetchBatchTransport {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FetchBatchTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl BatchTransport for FetchBatchTransport {
    fn send_batch(
        &self,
        endpoint: &str,
        project_key: &str,
        batch: &HeatmapBatch,
    ) -> Result<(), FlushError> {
        let url = batch_ingest_url(endpoint);
        let body = serde_json::to_string(batch)
            .map_err(|err| FlushError::Transport(err.to_string()))?;
        let xhr = XmlHttpRequest::new().map_err(transport_err)?;
        xhr.open_with_async("POST", &url, false)
            .map_err(transport_err)?;
        xhr.set_request_header("Content-Type", "application/json")
            .map_err(transport_err)?;
        xhr.set_request_header("X-Project-Key", project_key)
            .map_err(transport_err)?;
        xhr.set_response_type(XmlHttpRequestResponseType::Text);
        xhr.send_with_opt_str(Some(&body)).map_err(transport_err)?;
        let status = xhr.status().map_err(transport_err)?;
        if (200..300).contains(&status) {
            Ok(())
        } else {
            Err(FlushError::http_status(status as u16, None))
        }
    }
}

fn transport_err(err: wasm_bindgen::JsValue) -> FlushError {
    FlushError::Transport(
        err.dyn_into::<js_sys::Error>()
            .map(|value| value.message().into())
            .unwrap_or_else(|value| format!("{value:?}")),
    )
}
