use heatmap_core::{
    FlushError, HeatmapConfig, HeatmapSdk as CoreHeatmapSdk, IdentityError, InitError,
    ScreenContextError,
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
