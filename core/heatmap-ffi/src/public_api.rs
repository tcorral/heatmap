use heatmap_core::{
    FlushError, HeatmapConfig, HeatmapSdk as CoreHeatmapSdk, IdentityError, InitError,
    ScreenContextError,
};

use crate::config::InitConfigJson;

pub struct PublicHeatmapSdk {
    pub(crate) inner: CoreHeatmapSdk,
}

impl PublicHeatmapSdk {
    pub fn init_from_json(raw: &str) -> Result<Self, InitError> {
        let parsed = InitConfigJson::parse(raw)?;
        Self::init(parsed.into_core_config())
    }

    pub fn init(config: HeatmapConfig) -> Result<Self, InitError> {
        let inner = CoreHeatmapSdk::init(config)?;
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

pub mod export_names {
    pub const INIT_FROM_JSON: &str = "init_from_json";
    pub const INIT: &str = "init";
    pub const IDENTIFY: &str = "identify";
    pub const SIGN_OUT: &str = "sign_out";
    pub const SCREEN: &str = "screen";
    pub const SET_TRACKING_ENABLED: &str = "set_tracking_enabled";
    pub const FLUSH: &str = "flush";
}

pub mod c_export_names {
    pub const INIT: &str = "heatmap_sdk_init";
    pub const FREE: &str = "heatmap_sdk_free";
    pub const IDENTIFY: &str = "heatmap_sdk_identify";
    pub const SIGN_OUT: &str = "heatmap_sdk_sign_out";
    pub const SCREEN: &str = "heatmap_sdk_screen";
    pub const SET_TRACKING_ENABLED: &str = "heatmap_sdk_set_tracking_enabled";
    pub const FLUSH: &str = "heatmap_sdk_flush";
}

pub mod forbidden_export_names {
    pub const TRACK: &str = "track";
    pub const ENQUEUE_CANDIDATE: &str = "enqueue_candidate";
    pub const RECORD_ACTIVATION_CLICK: &str = "record_activation_click";
    pub const CAPTURE: &str = "capture";
    pub const DRAIN_BATCH_JSON: &str = "drain_batch_json";
}
