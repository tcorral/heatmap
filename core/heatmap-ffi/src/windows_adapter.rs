use heatmap_core::{
    BatchError, CaptureCandidate, CaptureError, InteractionEvent, InteractionType, PlatformInfo,
    PlatformType, Position, SdkInfo, Viewport,
};

use crate::public_api::PublicHeatmapSdk;

impl PublicHeatmapSdk {
    pub fn record_windows_activation_click(
        &mut self,
        x: f64,
        y: f64,
        element_id: Option<String>,
        viewport_width: u32,
        viewport_height: u32,
        timestamp_ms: i64,
        window_id: String,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let position = Position::new(x, y).map_err(CaptureError::InvalidPosition)?;
        let element_id = element_id.filter(|id| !id.trim().is_empty());
        let window_id = window_id.trim().to_string();
        let candidate = CaptureCandidate {
            timestamp: timestamp_ms,
            sdk: windows_sdk_info(),
            platform: windows_platform_info(os_version, app_version),
            screen: None,
            interaction: InteractionEvent::click(element_id)
                .map_err(CaptureError::InvalidEventField)?,
            position: Some(position),
            viewport: Some(Viewport {
                width: viewport_width,
                height: viewport_height,
            }),
            window_id: Some(window_id),
        };
        self.inner.enqueue_candidate(candidate)
    }

    pub fn record_windows_screen_view(
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
            windows_sdk_info(),
            windows_platform_info(os_version, app_version),
            viewport,
        )
    }

    pub fn record_windows_scroll(
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
        self.inner.record_scroll(
            windows_sdk_info(),
            windows_platform_info(os_version, app_version),
            viewport,
        )
    }

    pub fn adapter_windows_click_window_ids_from_batch(&mut self) -> Result<Vec<String>, BatchError> {
        let batch = self.inner.build_batch()?;
        Ok(batch
            .events
            .iter()
            .filter(|event| event.event.interaction_type == InteractionType::Click)
            .filter_map(|event| event.window_id.clone())
            .collect())
    }
}

fn windows_sdk_info() -> SdkInfo {
    SdkInfo {
        name: "heatmap-windows".into(),
        version: "0.1.0".into(),
    }
}

fn windows_platform_info(os_version: String, app_version: String) -> PlatformInfo {
    PlatformInfo {
        platform_type: PlatformType::Windows,
        os_version,
        app_version,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use heatmap_core::InteractionType;

    #[test]
    fn should_tag_windows_sdk_and_platform_on_click() {
        let config = heatmap_core::HeatmapConfig {
            project_key: "pk_test".into(),
            endpoint: "https://ingest.example.com".into(),
            session_timeout_ms: 10_000,
            sample_rate: 1.0,
            exclude_elements: vec![],
            exclude_screens: vec![],
            mask_elements: vec![],
        };
        let mut sdk = PublicHeatmapSdk::init(config).expect("init");
        sdk.record_windows_activation_click(
            0.5,
            0.5,
            Some("checkout.pay".into()),
            800,
            600,
            100,
            "win_a".into(),
            "Windows 11".into(),
            "1.0.0".into(),
        )
        .expect("click");
        let batch = sdk.inner.build_batch().expect("batch");
        let click = batch
            .events
            .iter()
            .find(|event| event.event.interaction_type == InteractionType::Click)
            .expect("click");
        assert_eq!(click.sdk.name, "heatmap-windows");
        assert_eq!(click.platform.platform_type, PlatformType::Windows);
    }
}
