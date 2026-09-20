use heatmap_core::{
    BatchError, CaptureCandidate, CaptureError, InteractionEvent, InteractionType, PlatformInfo,
    PlatformType, Position, SdkInfo, Viewport,
};

use crate::public_api::PublicHeatmapSdk;

impl PublicHeatmapSdk {
    pub fn on_background(&mut self) {
        self.inner.on_background();
    }

    pub fn on_foreground(&mut self) {
        self.inner.on_foreground();
    }

    pub fn record_activation_click(
        &mut self,
        x: f64,
        y: f64,
        element_id: Option<String>,
        viewport_width: u32,
        viewport_height: u32,
        timestamp_ms: i64,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let position = Position::new(x, y).map_err(CaptureError::InvalidPosition)?;
        let element_id = element_id.filter(|id| !id.trim().is_empty());
        let candidate = CaptureCandidate {
            timestamp: timestamp_ms,
            sdk: ios_sdk_info(),
            platform: ios_platform_info(os_version, app_version),
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

    pub fn record_ios_screen_view(
        &mut self,
        viewport_width: u32,
        viewport_height: u32,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let viewport = Viewport {
            width: viewport_width,
            height: viewport_height,
        };
        self.inner.record_screen_view(
            ios_sdk_info(),
            ios_platform_info(os_version, app_version),
            viewport,
        )
    }

    pub fn record_ios_scroll(
        &mut self,
        viewport_width: u32,
        viewport_height: u32,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let viewport = Viewport {
            width: viewport_width,
            height: viewport_height,
        };
        self.inner
            .record_scroll(ios_sdk_info(), ios_platform_info(os_version, app_version), viewport)
    }

    pub fn adapter_click_session_ids_from_batch(&mut self) -> Result<Vec<String>, BatchError> {
        let batch = self.inner.build_batch()?;
        Ok(batch
            .events
            .iter()
            .filter(|event| event.event.interaction_type == InteractionType::Click)
            .map(|event| event.session_id.clone())
            .collect())
    }
}

fn ios_sdk_info() -> SdkInfo {
    SdkInfo {
        name: "heatmap-ios".into(),
        version: "0.1.0".into(),
    }
}

fn ios_platform_info(os_version: String, app_version: String) -> PlatformInfo {
    PlatformInfo {
        platform_type: PlatformType::Ios,
        os_version,
        app_version,
    }
}
