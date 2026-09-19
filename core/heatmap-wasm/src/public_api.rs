use heatmap_core::{
    CaptureCandidate, CaptureError, FlushError, HeatmapConfig, HeatmapSdk as CoreHeatmapSdk,
    IdentityError, InitError, InteractionEvent, PlatformInfo, PlatformType, Position,
    ScreenContextError, SdkInfo, Viewport,
};

use crate::config::InitConfigJson;
use crate::stub_transport::StubBatchTransport;

pub struct PublicHeatmapSdk {
    inner: CoreHeatmapSdk,
}

impl PublicHeatmapSdk {
    pub fn init_from_json(raw: &str) -> Result<Self, InitError> {
        let parsed = InitConfigJson::parse(raw)?;
        Self::init(parsed.into_core_config())
    }

    pub fn init(config: HeatmapConfig) -> Result<Self, InitError> {
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
            sdk: SdkInfo {
                name: WEB_SDK_NAME.into(),
                version: WEB_SDK_VERSION.into(),
            },
            platform: PlatformInfo {
                platform_type: PlatformType::Web,
                os_version: "unknown".into(),
                app_version: "0.0.0".into(),
            },
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

const WEB_SDK_NAME: &str = "heatmap-web";
const WEB_SDK_VERSION: &str = "0.1.0";

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
