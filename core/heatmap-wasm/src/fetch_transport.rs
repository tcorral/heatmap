use heatmap_core::{batch_ingest_url, BatchTransport, FlushError, HeatmapBatch};
use wasm_bindgen::JsCast;
use web_sys::{XmlHttpRequest, XmlHttpRequestResponseType};

/// Browser batch upload uses synchronous XHR so the shared Rust `BatchTransport` trait
/// can stay synchronous. WASM init sets `defer_network_flush`, so auto-flush and
/// `poll_flush` do not hit this transport; sync XHR runs only when something calls
/// explicit `flush()`. Recommended uploads use async JS `fetch` with
/// `beginFlushBatchJson` / `completeFlush`.
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
        let body =
            serde_json::to_string(batch).map_err(|err| FlushError::Transport(err.to_string()))?;
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
            let retry_after_secs = parse_retry_after_header(&xhr);
            Err(FlushError::http_status(status as u16, retry_after_secs))
        }
    }
}

fn parse_retry_after_header(xhr: &XmlHttpRequest) -> Option<u64> {
    xhr.get_response_header("Retry-After")
        .ok()
        .flatten()
        .and_then(|value| value.parse::<u64>().ok())
}

fn transport_err(err: wasm_bindgen::JsValue) -> FlushError {
    FlushError::Transport(
        err.dyn_into::<js_sys::Error>()
            .map(|value| value.message().into())
            .unwrap_or_else(|value| format!("{value:?}")),
    )
}
