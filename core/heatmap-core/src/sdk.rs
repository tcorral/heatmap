use std::sync::Arc;

use uuid::Uuid;

use crate::batch::{BatchError, HeatmapBatch};
use crate::capture::{candidate_to_event, CaptureCandidate, CaptureContext, CaptureError};
use crate::config::{validate_config, HeatmapConfig, InitError};
use crate::event::{HeatmapEvent, InteractionEvent, ScreenInfo, SdkInfo, Viewport};
use crate::event::PlatformInfo;
use crate::identity_store::{
    IdentityStore, InMemoryIdentityStore, STORE_KEY_ANONYMOUS_ID, STORE_KEY_USER_ID,
};
use crate::transport::{BatchTransport, FlushError};
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::HttpBatchTransport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityError {
    EmptyUserId,
}

impl std::fmt::Display for IdentityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyUserId => write!(f, "user_id is required"),
        }
    }
}

impl std::error::Error for IdentityError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScreenContextError {
    EmptyScreenId,
}

impl std::fmt::Display for ScreenContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyScreenId => write!(f, "screen_id is required"),
        }
    }
}

impl std::error::Error for ScreenContextError {}

pub struct HeatmapSdk {
    config: HeatmapConfig,
    transport: Arc<dyn BatchTransport>,
    identity_store: Arc<dyn IdentityStore>,
    tracking_enabled: bool,
    session_id: String,
    anonymous_id: String,
    user_id: Option<String>,
    current_screen: Option<ScreenInfo>,
    sample_in: bool,
    queue: Vec<HeatmapEvent>,
}

impl HeatmapSdk {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn init(config: HeatmapConfig) -> Result<Self, InitError> {
        Self::init_with_transport(config, HttpBatchTransport::new())
    }

    pub fn init_with_transport(
        config: HeatmapConfig,
        transport: impl BatchTransport + 'static,
    ) -> Result<Self, InitError> {
        Self::init_with_transport_and_store(
            config,
            transport,
            Arc::new(InMemoryIdentityStore::new()),
        )
    }

    pub fn init_with_transport_and_store(
        config: HeatmapConfig,
        transport: impl BatchTransport + 'static,
        identity_store: Arc<dyn IdentityStore>,
    ) -> Result<Self, InitError> {
        validate_config(&config)?;
        let session_id = new_id("sess");
        let anonymous_id = load_or_create_anonymous_id(identity_store.as_ref());
        let user_id = load_persisted_user_id(identity_store.as_ref());
        let sample_in = config.sample_rate >= DEFAULT_SAMPLE_IN_THRESHOLD;
        Ok(Self {
            config,
            transport: Arc::new(transport),
            identity_store,
            tracking_enabled: true,
            session_id,
            anonymous_id,
            user_id,
            current_screen: None,
            sample_in,
            queue: Vec::new(),
        })
    }

    pub fn config(&self) -> &HeatmapConfig {
        &self.config
    }

    pub fn set_tracking_enabled(&mut self, enabled: bool) {
        self.tracking_enabled = enabled;
    }

    pub fn identify(&mut self, user_id: String) -> Result<(), IdentityError> {
        if user_id.trim().is_empty() {
            return Err(IdentityError::EmptyUserId);
        }
        self.user_id = Some(user_id.clone());
        self.identity_store
            .write(STORE_KEY_USER_ID, Some(user_id.as_str()));
        Ok(())
    }

    pub fn sign_out(&mut self) {
        self.user_id = None;
        self.identity_store.write(STORE_KEY_USER_ID, None);
    }

    pub fn screen(
        &mut self,
        screen_id: String,
        layout_version: Option<String>,
    ) -> Result<(), ScreenContextError> {
        if screen_id.trim().is_empty() {
            return Err(ScreenContextError::EmptyScreenId);
        }
        let layout = layout_version.filter(|version| !version.trim().is_empty());
        self.current_screen = Some(ScreenInfo {
            id: screen_id,
            layout_version: layout,
        });
        Ok(())
    }

    pub fn bootstrap_web_session(
        &mut self,
        sdk: SdkInfo,
        platform: PlatformInfo,
        viewport: Viewport,
    ) -> Result<(), CaptureError> {
        self.enqueue_lifecycle(
            sdk,
            platform,
            InteractionEvent::session_start().map_err(CaptureError::InvalidEventField)?,
            viewport,
        )
    }

    pub fn record_screen_view(
        &mut self,
        sdk: SdkInfo,
        platform: PlatformInfo,
        viewport: Viewport,
    ) -> Result<(), CaptureError> {
        self.enqueue_lifecycle(
            sdk,
            platform,
            InteractionEvent::screen_view().map_err(CaptureError::InvalidEventField)?,
            viewport,
        )
    }

    pub fn record_scroll(
        &mut self,
        sdk: SdkInfo,
        platform: PlatformInfo,
        viewport: Viewport,
    ) -> Result<(), CaptureError> {
        self.enqueue_lifecycle(
            sdk,
            platform,
            InteractionEvent::scroll().map_err(CaptureError::InvalidEventField)?,
            viewport,
        )
    }

    fn enqueue_lifecycle(
        &mut self,
        sdk: SdkInfo,
        platform: PlatformInfo,
        interaction: InteractionEvent,
        viewport: Viewport,
    ) -> Result<(), CaptureError> {
        let candidate = CaptureCandidate {
            timestamp: current_timestamp_ms(),
            sdk,
            platform,
            screen: None,
            interaction,
            position: None,
            viewport: Some(viewport),
            window_id: None,
        };
        self.enqueue_candidate(candidate)
    }

    pub fn enqueue_candidate(&mut self, candidate: CaptureCandidate) -> Result<(), CaptureError> {
        if !self.tracking_enabled {
            return Err(CaptureError::TrackingDisabled);
        }
        if !self.sample_in {
            return Err(CaptureError::SampledOut);
        }
        let event = candidate_to_event(
            candidate,
            CaptureContext {
                session_id: &self.session_id,
                anonymous_id: &self.anonymous_id,
                user_id: self.user_id.as_deref(),
                inherited_screen: self.current_screen.as_ref(),
            },
        )?;
        self.queue.push(event);
        Ok(())
    }

    pub fn build_batch(&mut self) -> Result<HeatmapBatch, BatchError> {
        if self.queue.is_empty() {
            return Err(BatchError::EmptyQueue);
        }
        let batch_id = new_id("batch");
        let events = std::mem::take(&mut self.queue);
        Ok(HeatmapBatch { batch_id, events })
    }

    pub fn flush(&mut self) -> Result<(), FlushError> {
        if self.queue.is_empty() {
            return Ok(());
        }
        let batch = self
            .build_batch()
            .map_err(|err| FlushError::Transport(err.to_string()))?;
        match self.transport.send_batch(
            &self.config.endpoint,
            &self.config.project_key,
            &batch,
        ) {
            Ok(()) => Ok(()),
            Err(err) => {
                self.queue.extend(batch.events);
                Err(err)
            }
        }
    }
}

const DEFAULT_SAMPLE_IN_THRESHOLD: f64 = 1.0;

fn new_id(prefix: &str) -> String {
    format!("{prefix}_{}", Uuid::new_v4().simple())
}

fn load_or_create_anonymous_id(store: &dyn IdentityStore) -> String {
    if let Some(id) = store.read(STORE_KEY_ANONYMOUS_ID).filter(|id| !id.trim().is_empty()) {
        return id;
    }
    let id = new_id("anon");
    store.write(STORE_KEY_ANONYMOUS_ID, Some(id.as_str()));
    id
}

fn load_persisted_user_id(store: &dyn IdentityStore) -> Option<String> {
    store
        .read(STORE_KEY_USER_ID)
        .filter(|id| !id.trim().is_empty())
}

fn current_timestamp_ms() -> i64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now() as i64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis() as i64)
            .unwrap_or(0)
    }
}
