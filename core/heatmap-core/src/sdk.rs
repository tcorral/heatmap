use std::sync::Arc;

use uuid::Uuid;

use crate::batch::{BatchError, HeatmapBatch};
use crate::capture::{candidate_to_event, CaptureCandidate, CaptureContext, CaptureError};
use crate::config::{validate_config, HeatmapConfig, InitError};
use crate::event::HeatmapEvent;
use crate::event::ScreenInfo;
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
        validate_config(&config)?;
        let session_id = new_id("sess");
        let anonymous_id = new_id("anon");
        let sample_in = config.sample_rate >= DEFAULT_SAMPLE_IN_THRESHOLD;
        Ok(Self {
            config,
            transport: Arc::new(transport),
            tracking_enabled: true,
            session_id,
            anonymous_id,
            user_id: None,
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
        self.user_id = Some(user_id);
        Ok(())
    }

    pub fn sign_out(&mut self) {
        self.user_id = None;
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
