#[cfg(target_arch = "wasm32")]
use std::sync::Arc;

use heatmap_core::{
    CaptureCandidate, CaptureError, FlushError, HeatmapConfig, HeatmapSdk as CoreHeatmapSdk,
    IdentityError, InitError, InteractionEvent, PlatformInfo, Position, QueueLimits,
    ScreenContextError, SdkInfo, Viewport,
};

use crate::config::InitConfigJson;

#[cfg(target_arch = "wasm32")]
use crate::fetch_transport::FetchBatchTransport;
#[cfg(not(target_arch = "wasm32"))]
use crate::stub_transport::StubBatchTransport;
#[cfg(target_arch = "wasm32")]
use crate::web_context::{web_platform_info, web_sdk_info};

pub struct PublicHeatmapSdk {
    inner: CoreHeatmapSdk,
}

impl PublicHeatmapSdk {
    pub fn init_from_json(raw: &str) -> Result<Self, InitError> {
        let parsed = InitConfigJson::parse(raw)?;
        Self::init(parsed.into_core_config())
    }

    pub fn init(config: HeatmapConfig) -> Result<Self, InitError> {
        #[cfg(target_arch = "wasm32")]
        let inner = CoreHeatmapSdk::init_with_transport_and_store(
            config,
            FetchBatchTransport::new(),
            Arc::new(crate::local_storage_identity::LocalStorageIdentityStore::new()),
        )?;
        #[cfg(not(target_arch = "wasm32"))]
        let inner = CoreHeatmapSdk::init_with_transport(config, StubBatchTransport::new())?;
        Ok(Self { inner })
    }

    pub fn identify(&mut self, user_id: String) -> Result<(), IdentityError> {
        self.inner.identify(user_id)
    }

    pub fn sign_out(&mut self) {
        self.inner.sign_out();
    }

    pub fn screen(
        &mut self,
        screen_id: String,
        layout_version: Option<String>,
    ) -> Result<(), ScreenContextError> {
        self.inner.screen(screen_id, layout_version)
    }

    pub fn set_tracking_enabled(&mut self, enabled: bool) {
        self.inner.set_tracking_enabled(enabled);
    }

    pub fn flush(&mut self) -> Result<(), FlushError> {
        self.inner.flush()
    }

    pub fn poll_flush(&mut self) -> Result<(), FlushError> {
        self.inner.poll_flush()
    }

    pub fn drain_batch_json(&mut self) -> Option<String> {
        let batch = self.inner.build_batch().ok()?;
        serde_json::to_string(&batch).ok()
    }

    #[doc(hidden)]
    pub fn set_queue_limits_for_testing(&mut self, max_events: usize, max_bytes: usize) {
        self.inner.set_queue_limits_for_testing(QueueLimits {
            max_events,
            max_bytes,
        });
    }

    #[doc(hidden)]
    pub fn queued_event_count_for_testing(&self) -> usize {
        self.inner.queued_event_count_for_testing()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn bootstrap_web_session(&mut self) -> Result<(), CaptureError> {
        let viewport = crate::web_context::viewport_from_window();
        self.inner.bootstrap_web_session(
            web_sdk_info(),
            web_platform_info(self.inner.config().app_version.as_str()),
            viewport,
        )
    }

    #[cfg(target_arch = "wasm32")]
    pub fn record_screen_view(&mut self) -> Result<(), CaptureError> {
        let viewport = crate::web_context::viewport_from_window();
        self.inner.record_screen_view(
            web_sdk_info(),
            web_platform_info(self.inner.config().app_version.as_str()),
            viewport,
        )
    }

    #[cfg(target_arch = "wasm32")]
    pub fn record_scroll(&mut self) -> Result<(), CaptureError> {
        let viewport = crate::web_context::viewport_from_window();
        self.inner.record_scroll(
            web_sdk_info(),
            web_platform_info(self.inner.config().app_version.as_str()),
            viewport,
        )
    }

    /// Adapter-only web click enqueue; not part of the public integrator API.
    pub fn record_activation_click(
        &mut self,
        x: f64,
        y: f64,
        element_id: Option<String>,
        viewport_width: u32,
        viewport_height: u32,
        timestamp_ms: i64,
    ) -> Result<(), CaptureError> {
        let position = Position::new(x, y).map_err(CaptureError::InvalidPosition)?;
        let element_id = element_id.filter(|id| !id.trim().is_empty());
        let candidate = CaptureCandidate {
            timestamp: timestamp_ms,
            sdk: web_capture_sdk(),
            platform: web_capture_platform(self.inner.config().app_version.as_str()),
            screen: None,
            interaction: InteractionEvent::click(element_id)
                .map_err(CaptureError::InvalidEventField)?,
            position: Some(position),
            viewport: Some(Viewport {
                width: viewport_width,
                height: viewport_height,
            }),
            window_id: None,
        };
        self.inner.enqueue_candidate(candidate)
    }
}

fn web_capture_sdk() -> SdkInfo {
    #[cfg(target_arch = "wasm32")]
    {
        web_sdk_info()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        SdkInfo {
            name: "heatmap-web".into(),
            version: "0.1.0".into(),
        }
    }
}

fn web_capture_platform(app_version: &str) -> PlatformInfo {
    #[cfg(target_arch = "wasm32")]
    {
        web_platform_info(app_version)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use heatmap_core::PlatformType;
        PlatformInfo {
            platform_type: PlatformType::Web,
            os_version: "unknown".into(),
            app_version: app_version.to_string(),
        }
    }
}

/// JS export names for binding-surface tests (must stay in sync with wasm_bindgen attrs).
pub mod export_names {
    pub const INIT: &str = "init";
    pub const IDENTIFY: &str = "identify";
    pub const SIGN_OUT: &str = "signOut";
    pub const SCREEN: &str = "screen";
    pub const SET_TRACKING_ENABLED: &str = "setTrackingEnabled";
    pub const FLUSH: &str = "flush";
}

pub mod forbidden_export_names {
    pub const TRACK: &str = "track";
    pub const ENQUEUE_CANDIDATE: &str = "enqueueCandidate";
    pub const CAPTURE: &str = "capture";
}
