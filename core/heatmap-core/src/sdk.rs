use std::sync::Arc;

use uuid::Uuid;

use crate::batch::{BatchError, HeatmapBatch};
use crate::capture::{candidate_to_event, CaptureCandidate, CaptureError};
use crate::config::{validate_config, HeatmapConfig, InitError};
use crate::event::HeatmapEvent;
use crate::transport::{BatchTransport, FlushError, HttpBatchTransport};

pub struct HeatmapSdk {
    config: HeatmapConfig,
    transport: Arc<dyn BatchTransport>,
    tracking_enabled: bool,
    session_id: String,
    anonymous_id: String,
    sample_in: bool,
    queue: Vec<HeatmapEvent>,
}

impl HeatmapSdk {
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

    pub fn enqueue_candidate(&mut self, candidate: CaptureCandidate) -> Result<(), CaptureError> {
        if !self.tracking_enabled {
            return Err(CaptureError::TrackingDisabled);
        }
        if !self.sample_in {
            return Err(CaptureError::SampledOut);
        }
        let event = candidate_to_event(candidate, &self.session_id, &self.anonymous_id)?;
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
