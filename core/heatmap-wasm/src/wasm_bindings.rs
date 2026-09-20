use wasm_bindgen::prelude::*;

use heatmap_core::{CaptureError, FlushError, IdentityError, InitError, ScreenContextError};

use crate::public_api::PublicHeatmapSdk;

#[wasm_bindgen]
pub struct HeatmapSdkJs {
    sdk: PublicHeatmapSdk,
}

#[wasm_bindgen]
impl HeatmapSdkJs {
    #[wasm_bindgen(js_name = init)]
    pub fn init(config_json: &str) -> Result<HeatmapSdkJs, JsValue> {
        let mut sdk = PublicHeatmapSdk::init_from_json(config_json).map_err(init_error)?;
        sdk.bootstrap_web_session().map_err(capture_error)?;
        Ok(Self { sdk })
    }

    #[wasm_bindgen(js_name = identify)]
    pub fn identify(&mut self, user_id: String) -> Result<(), JsValue> {
        self.sdk.identify(user_id).map_err(identity_error)
    }

    #[wasm_bindgen(js_name = signOut)]
    pub fn sign_out(&mut self) {
        self.sdk.sign_out();
    }

    #[wasm_bindgen(js_name = screen)]
    pub fn screen(
        &mut self,
        screen_id: String,
        layout_version: Option<String>,
    ) -> Result<(), JsValue> {
        self.sdk
            .screen(screen_id.clone(), layout_version.clone())
            .map_err(screen_error)?;
        self.sdk.record_screen_view().map_err(capture_error)
    }

    #[wasm_bindgen(js_name = recordScroll)]
    pub fn record_scroll(&mut self) -> Result<(), JsValue> {
        self.sdk.record_scroll().map_err(capture_error)
    }

    #[wasm_bindgen(js_name = setTrackingEnabled)]
    pub fn set_tracking_enabled(&mut self, enabled: bool) {
        self.sdk.set_tracking_enabled(enabled);
    }

    #[wasm_bindgen(js_name = flush)]
    pub fn flush(&mut self) -> Result<(), JsValue> {
        self.sdk.flush().map_err(flush_error)
    }

    #[wasm_bindgen(js_name = pollFlush)]
    pub fn poll_flush(&mut self) -> Result<(), JsValue> {
        self.sdk.poll_flush().map_err(flush_error)
    }

    #[wasm_bindgen(js_name = drainBatchJson)]
    pub fn drain_batch_json(&mut self) -> Option<String> {
        self.sdk.drain_batch_json()
    }

    #[wasm_bindgen(js_name = beginFlushBatchJson)]
    pub fn begin_flush_batch_json(&mut self) -> Option<String> {
        self.sdk.begin_flush_batch_json()
    }

    #[wasm_bindgen(js_name = completeFlush)]
    pub fn complete_flush(&mut self, accepted: bool) {
        self.sdk.complete_flush(accepted);
    }

    #[wasm_bindgen(js_name = completeFlushHttp)]
    pub fn complete_flush_http(&mut self, status: u16, retry_after_secs: Option<u64>) {
        self.sdk.complete_flush_http(status, retry_after_secs);
    }

    #[wasm_bindgen(js_name = recordActivationClick)]
    pub fn record_activation_click(
        &mut self,
        x: f64,
        y: f64,
        element_id: Option<String>,
        viewport_width: u32,
        viewport_height: u32,
        timestamp_ms: f64,
    ) -> Result<(), JsValue> {
        self.sdk
            .record_activation_click(
                x,
                y,
                element_id,
                viewport_width,
                viewport_height,
                timestamp_ms as i64,
            )
            .map_err(capture_error)
    }

    #[wasm_bindgen(js_name = setQueueLimitsForTesting)]
    pub fn set_queue_limits_for_testing(&mut self, max_events: usize, max_bytes: usize) {
        self.sdk.set_queue_limits_for_testing(max_events, max_bytes);
    }

    #[wasm_bindgen(js_name = queuedEventCountForTesting)]
    pub fn queued_event_count_for_testing(&self) -> usize {
        self.sdk.queued_event_count_for_testing()
    }
}

fn init_error(err: InitError) -> JsValue {
    JsValue::from_str(&err.to_string())
}

fn identity_error(err: IdentityError) -> JsValue {
    JsValue::from_str(&err.to_string())
}

fn screen_error(err: ScreenContextError) -> JsValue {
    JsValue::from_str(&err.to_string())
}

fn flush_error(err: FlushError) -> JsValue {
    JsValue::from_str(&err.to_string())
}

fn capture_error(err: CaptureError) -> JsValue {
    JsValue::from_str(&err.to_string())
}
