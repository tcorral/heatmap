use heatmap_core::{
    CaptureCandidate, CaptureError, InteractionEvent, PlatformInfo, PlatformType, Position,
    SdkInfo, Viewport,
};

use crate::public_api::PublicHeatmapSdk;

impl PublicHeatmapSdk {
    /// Android View adapter-only click enqueue; not exposed on `HeatmapClient`.
    pub fn record_android_activation_click(
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
            sdk: android_sdk_info(),
            platform: android_platform_info(os_version, app_version),
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

    pub fn record_android_screen_view(
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
            android_sdk_info(),
            android_platform_info(os_version, app_version),
            viewport,
            None,
        )
    }

    pub fn record_android_scroll(
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
            android_sdk_info(),
            android_platform_info(os_version, app_version),
            viewport,
            None,
        )
    }
}

fn android_sdk_info() -> SdkInfo {
    SdkInfo {
        name: "heatmap-android".into(),
        version: "0.1.0".into(),
    }
}

fn android_platform_info(os_version: String, app_version: String) -> PlatformInfo {
    PlatformInfo {
        platform_type: PlatformType::Android,
        os_version,
        app_version,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use heatmap_core::InteractionType;

    #[test]
    fn should_tag_android_sdk_and_platform_on_click() {
        let config = heatmap_core::HeatmapConfig {
            project_key: "pk_test".into(),
            endpoint: "https://ingest.example.com".into(),
            session_timeout_ms: 10_000,
            sample_rate: 1.0,
            exclude_elements: vec![],
            exclude_screens: vec![],
            mask_elements: vec![],
            app_version: heatmap_core::HeatmapConfig::DEFAULT_APP_VERSION.into(),
        };
        let mut sdk =
            PublicHeatmapSdk::init_with_transport(config, heatmap_core::HttpBatchTransport::new())
                .expect("init");
        sdk.record_android_activation_click(
            0.5,
            0.5,
            Some("checkout.pay".into()),
            800,
            600,
            100,
            "Android 14".into(),
            "1.0.0".into(),
        )
        .expect("click");
        let batch = sdk.inner.build_batch().expect("batch");
        let click = batch
            .events
            .iter()
            .find(|event| event.event.interaction_type == InteractionType::Click)
            .expect("click");
        assert_eq!(click.sdk.name, "heatmap-android");
        assert_eq!(click.platform.platform_type, PlatformType::Android);
    }

    #[test]
    fn should_enqueue_screen_view_when_android_screen_is_set() {
        let config = heatmap_core::HeatmapConfig {
            project_key: "pk_test".into(),
            endpoint: "https://ingest.example.com".into(),
            session_timeout_ms: 10_000,
            sample_rate: 1.0,
            exclude_elements: vec![],
            exclude_screens: vec![],
            mask_elements: vec![],
            app_version: heatmap_core::HeatmapConfig::DEFAULT_APP_VERSION.into(),
        };
        let mut sdk =
            PublicHeatmapSdk::init_with_transport(config, heatmap_core::HttpBatchTransport::new())
                .expect("init");
        sdk.screen("Home".into(), None).expect("screen");
        sdk.record_android_screen_view(800, 600, "Android 14".into(), "1.0.0".into())
            .expect("screen_view");
        let batch = sdk.inner.build_batch().expect("batch");
        let screen_view = batch
            .events
            .iter()
            .find(|event| event.event.interaction_type == InteractionType::ScreenView)
            .expect("screen_view");
        assert_eq!(
            screen_view.screen.as_ref().map(|screen| screen.id.as_str()),
            Some("Home")
        );
        assert_eq!(screen_view.sdk.name, "heatmap-android");
        assert_eq!(screen_view.platform.platform_type, PlatformType::Android);
    }

    #[test]
    fn should_enqueue_scroll_when_android_screen_is_set() {
        let config = heatmap_core::HeatmapConfig {
            project_key: "pk_test".into(),
            endpoint: "https://ingest.example.com".into(),
            session_timeout_ms: 10_000,
            sample_rate: 1.0,
            exclude_elements: vec![],
            exclude_screens: vec![],
            mask_elements: vec![],
            app_version: heatmap_core::HeatmapConfig::DEFAULT_APP_VERSION.into(),
        };
        let mut sdk =
            PublicHeatmapSdk::init_with_transport(config, heatmap_core::HttpBatchTransport::new())
                .expect("init");
        sdk.screen("Checkout".into(), Some("2024-09".into()))
            .expect("screen");
        sdk.record_android_scroll(800, 600, "Android 14".into(), "1.0.0".into())
            .expect("scroll");
        let batch = sdk.inner.build_batch().expect("batch");
        let scroll = batch
            .events
            .iter()
            .find(|event| event.event.interaction_type == InteractionType::Scroll)
            .expect("scroll");
        assert_eq!(
            scroll.screen.as_ref().map(|screen| screen.id.as_str()),
            Some("Checkout")
        );
        assert_eq!(scroll.sdk.name, "heatmap-android");
        assert_eq!(scroll.platform.platform_type, PlatformType::Android);
    }
}
