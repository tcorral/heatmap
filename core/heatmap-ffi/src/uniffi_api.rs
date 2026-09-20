use std::sync::Mutex;

use crate::errors::{CaptureError, FlushError, IdentityError, InitError, ScreenContextError};
use crate::public_api::PublicHeatmapSdk;

#[derive(uniffi::Record)]
pub struct InitConfig {
    pub project_key: String,
    pub endpoint: String,
    pub session_timeout_ms: u64,
    pub sample_rate: f64,
    pub exclude_elements: Vec<String>,
    pub exclude_screens: Vec<String>,
    pub mask_elements: Vec<String>,
    pub storage_directory: Option<String>,
    pub app_version: String,
}

impl From<InitConfig> for heatmap_core::HeatmapConfig {
    fn from(value: InitConfig) -> Self {
        Self {
            project_key: value.project_key,
            endpoint: value.endpoint,
            session_timeout_ms: value.session_timeout_ms,
            sample_rate: value.sample_rate,
            exclude_elements: value.exclude_elements,
            exclude_screens: value.exclude_screens,
            mask_elements: value.mask_elements,
            app_version: if value.app_version.trim().is_empty() {
                heatmap_core::HeatmapConfig::DEFAULT_APP_VERSION.to_string()
            } else {
                value.app_version
            },
        }
    }
}

#[derive(uniffi::Object)]
pub struct HeatmapSdk {
    state: Mutex<PublicHeatmapSdk>,
}

#[uniffi::export]
impl HeatmapSdk {
    #[uniffi::constructor]
    pub fn init_from_json(config_json: String) -> Result<Self, InitError> {
        let inner = PublicHeatmapSdk::init_from_json(&config_json).map_err(InitError::from)?;
        Ok(Self {
            state: Mutex::new(inner),
        })
    }

    #[uniffi::constructor(name = "init")]
    pub fn init_with_config(config: InitConfig) -> Result<Self, InitError> {
        let storage = config.storage_directory.clone();
        let core_config: heatmap_core::HeatmapConfig = config.into();
        let inner =
            PublicHeatmapSdk::init_with_storage(core_config, storage).map_err(InitError::from)?;
        Ok(Self {
            state: Mutex::new(inner),
        })
    }

    pub fn identify(&self, user_id: String) -> Result<(), IdentityError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard.identify(user_id).map_err(IdentityError::from)
    }

    pub fn sign_out(&self) {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard.sign_out();
    }

    pub fn screen(
        &self,
        screen_id: String,
        layout_version: Option<String>,
    ) -> Result<(), ScreenContextError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .screen(screen_id, layout_version)
            .map_err(ScreenContextError::from)
    }

    pub fn set_tracking_enabled(&self, enabled: bool) {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard.set_tracking_enabled(enabled);
    }

    pub fn flush(&self) -> Result<(), FlushError> {
        crate::flush_lock::run_prepared_flush(&self.state, false).map_err(FlushError::from)
    }

    pub fn poll_flush(&self) -> Result<(), FlushError> {
        crate::flush_lock::run_prepared_flush(&self.state, true).map_err(FlushError::from)
    }

    /// Adapter-only lifecycle hook (iOS, Android).
    pub fn on_background(&self) {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard.on_background();
    }

    /// Adapter-only lifecycle hook (iOS, Android).
    pub fn on_foreground(&self) {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard.on_foreground();
    }

    /// iOS adapter-only click enqueue; not exposed on `HeatmapClient`.
    pub fn record_activation_click(
        &self,
        x: f64,
        y: f64,
        element_id: Option<String>,
        viewport_width: u32,
        viewport_height: u32,
        timestamp_ms: i64,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .record_activation_click(
                x,
                y,
                element_id,
                viewport_width,
                viewport_height,
                timestamp_ms,
                os_version,
                app_version,
            )
            .map_err(CaptureError::from)
    }

    pub fn record_ios_screen_view(
        &self,
        viewport_width: u32,
        viewport_height: u32,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .record_ios_screen_view(viewport_width, viewport_height, os_version, app_version)
            .map_err(CaptureError::from)
    }

    pub fn record_ios_scroll(
        &self,
        viewport_width: u32,
        viewport_height: u32,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .record_ios_scroll(viewport_width, viewport_height, os_version, app_version)
            .map_err(CaptureError::from)
    }

    /// macOS adapter-only click enqueue; not exposed on `HeatmapClient`.
    pub fn record_macos_activation_click(
        &self,
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
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .record_macos_activation_click(
                x,
                y,
                element_id,
                viewport_width,
                viewport_height,
                timestamp_ms,
                window_id,
                os_version,
                app_version,
            )
            .map_err(CaptureError::from)
    }

    pub fn record_macos_screen_view(
        &self,
        viewport_width: u32,
        viewport_height: u32,
        window_id: String,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .record_macos_screen_view(
                viewport_width,
                viewport_height,
                window_id,
                os_version,
                app_version,
            )
            .map_err(CaptureError::from)
    }

    pub fn record_macos_scroll(
        &self,
        viewport_width: u32,
        viewport_height: u32,
        window_id: String,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .record_macos_scroll(
                viewport_width,
                viewport_height,
                window_id,
                os_version,
                app_version,
            )
            .map_err(CaptureError::from)
    }

    /// Android View adapter-only click enqueue; not exposed on `HeatmapClient`.
    pub fn record_android_activation_click(
        &self,
        x: f64,
        y: f64,
        element_id: Option<String>,
        viewport_width: u32,
        viewport_height: u32,
        timestamp_ms: i64,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .record_android_activation_click(
                x,
                y,
                element_id,
                viewport_width,
                viewport_height,
                timestamp_ms,
                os_version,
                app_version,
            )
            .map_err(CaptureError::from)
    }

    pub fn record_android_screen_view(
        &self,
        viewport_width: u32,
        viewport_height: u32,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .record_android_screen_view(viewport_width, viewport_height, os_version, app_version)
            .map_err(CaptureError::from)
    }

    pub fn record_android_scroll(
        &self,
        viewport_width: u32,
        viewport_height: u32,
        os_version: String,
        app_version: String,
    ) -> Result<(), CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .record_android_scroll(viewport_width, viewport_height, os_version, app_version)
            .map_err(CaptureError::from)
    }

    /// Returns window ids for macOS click events from a built batch (adapter tests).
    pub fn adapter_macos_click_window_ids_from_batch(&self) -> Result<Vec<String>, CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .adapter_macos_click_window_ids_from_batch()
            .map_err(CaptureError::from)
    }

    /// Returns session ids for click events from a built batch (adapter tests).
    pub fn adapter_click_session_ids_from_batch(&self) -> Result<Vec<String>, CaptureError> {
        let mut guard = self.state.lock().expect("heatmap sdk lock");
        guard
            .adapter_click_session_ids_from_batch()
            .map_err(CaptureError::from)
    }
}

impl HeatmapSdk {
    #[cfg(test)]
    pub(crate) fn from_public(inner: PublicHeatmapSdk) -> Self {
        Self {
            state: Mutex::new(inner),
        }
    }
}

#[cfg(test)]
mod flush_lock_tests {
    use super::*;
    use heatmap_core::{BatchTransport, FlushError, HeatmapBatch, HeatmapConfig};
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;
    use std::time::Duration;

    struct BlockingTransport {
        started: Arc<(Mutex<bool>, Condvar)>,
        release: Arc<(Mutex<bool>, Condvar)>,
    }

    impl BatchTransport for BlockingTransport {
        fn send_batch(
            &self,
            _endpoint: &str,
            _project_key: &str,
            _batch: &HeatmapBatch,
        ) -> Result<(), FlushError> {
            let (started_lock, started_wait) = &*self.started;
            *started_lock.lock().expect("started") = true;
            started_wait.notify_all();
            let (release_lock, release_wait) = &*self.release;
            let mut released = release_lock.lock().expect("release");
            while !*released {
                released = release_wait.wait(released).expect("wait");
            }
            Ok(())
        }
    }

    fn test_config() -> HeatmapConfig {
        HeatmapConfig {
            project_key: "pk_test".into(),
            endpoint: "https://ingest.example.com".into(),
            session_timeout_ms: HeatmapConfig::DEFAULT_SESSION_TIMEOUT_MS,
            sample_rate: HeatmapConfig::DEFAULT_SAMPLE_RATE,
            exclude_elements: vec![],
            exclude_screens: vec![],
            mask_elements: vec![],
            app_version: HeatmapConfig::DEFAULT_APP_VERSION.into(),
        }
    }

    fn record_click(sdk: &HeatmapSdk, timestamp_ms: i64) -> Result<(), CaptureError> {
        sdk.record_activation_click(
            0.5,
            0.5,
            Some("checkout.pay".into()),
            800,
            600,
            timestamp_ms,
            "iOS 18.0".into(),
            "1.0.0".into(),
        )
    }

    #[test]
    fn should_return_capture_while_flush_http_is_blocked() {
        let started = Arc::new((Mutex::new(false), Condvar::new()));
        let release = Arc::new((Mutex::new(false), Condvar::new()));
        let transport = BlockingTransport {
            started: Arc::clone(&started),
            release: Arc::clone(&release),
        };
        let sdk = Arc::new(HeatmapSdk::from_public(
            PublicHeatmapSdk::init_with_transport(test_config(), transport).expect("init"),
        ));
        record_click(&sdk, 1_000).expect("seed queue");

        let flush_sdk = Arc::clone(&sdk);
        let flush = thread::spawn(move || flush_sdk.flush());

        let (started_lock, started_wait) = &*started;
        let mut is_started = started_lock.lock().expect("started");
        let timeout = Duration::from_secs(2);
        while !*is_started {
            let (guard, timed_out) = started_wait
                .wait_timeout(is_started, timeout)
                .expect("wait");
            is_started = guard;
            assert!(!timed_out.timed_out(), "flush never reached transport");
        }
        drop(is_started);

        let capture_sdk = Arc::clone(&sdk);
        let (done_tx, done_rx) = std::sync::mpsc::channel();
        thread::spawn(move || {
            let result = record_click(&capture_sdk, 2_000);
            let _ = done_tx.send(result);
        });
        let capture = done_rx
            .recv_timeout(Duration::from_millis(300))
            .expect("capture blocked behind flush HTTP");
        capture.expect("capture");

        let (release_lock, release_wait) = &*release;
        *release_lock.lock().expect("release") = true;
        release_wait.notify_all();
        flush.join().expect("flush thread").expect("flush");
    }
}
