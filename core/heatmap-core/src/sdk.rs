use std::collections::VecDeque;
use std::sync::Arc;

use uuid::Uuid;

use crate::batch::{BatchError, HeatmapBatch};
use crate::capture::{
    capture_is_excluded, candidate_to_event, CaptureCandidate, CaptureContext, CaptureError,
};
use crate::clock::{Clock, SystemClock};
use crate::config::{validate_config, HeatmapConfig, InitError};
use crate::diagnostics::HeatmapDiagnostics;
use crate::event::{HeatmapEvent, InteractionEvent, InteractionType, ScreenInfo, SdkInfo, Viewport};
use crate::event::PlatformInfo;
use crate::identity_store::{
    IdentityStore, InMemoryIdentityStore, STORE_KEY_ANONYMOUS_ID, STORE_KEY_LAST_EVENT_AT,
    STORE_KEY_SAMPLE_IN, STORE_KEY_SESSION_ID, STORE_KEY_USER_ID,
};
use crate::flush_policy::{
    flush_error_is_permanent, flush_error_is_transient, flush_event_limit_reached,
    flush_interval_elapsed, queue_flush_byte_limit_reached, INGEST_MAX_BYTES, INGEST_MAX_EVENTS,
};
use crate::persistent_queue::{
    event_json_bytes, load_persisted_queue, persist_queue, push_with_overflow, queue_json_bytes,
    QueueLimits,
};
use crate::rng::{session_sampled_in, RandomSource, SystemRng};
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
    clock: Arc<dyn Clock>,
    tracking_enabled: bool,
    session_id: Option<String>,
    last_event_at: Option<i64>,
    anonymous_id: String,
    user_id: Option<String>,
    current_screen: Option<ScreenInfo>,
    sample_in: bool,
    rng: Arc<dyn RandomSource>,
    diagnostics: HeatmapDiagnostics,
    queue: VecDeque<HeatmapEvent>,
    queue_events_bytes: usize,
    queue_limits: QueueLimits,
    queue_pending_since_ms: Option<i64>,
    retry_batch_id: Option<String>,
    retry_not_before_ms: i64,
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
        Self::init_with_transport_store_and_clock(
            config,
            transport,
            identity_store,
            Arc::new(SystemClock),
        )
    }

    pub fn init_with_transport_store_and_clock(
        config: HeatmapConfig,
        transport: impl BatchTransport + 'static,
        identity_store: Arc<dyn IdentityStore>,
        clock: Arc<dyn Clock>,
    ) -> Result<Self, InitError> {
        Self::init_with_transport_store_clock_and_rng(
            config,
            transport,
            identity_store,
            clock,
            Arc::new(SystemRng),
        )
    }

    pub fn init_with_transport_store_clock_and_rng(
        config: HeatmapConfig,
        transport: impl BatchTransport + 'static,
        identity_store: Arc<dyn IdentityStore>,
        clock: Arc<dyn Clock>,
        rng: Arc<dyn RandomSource>,
    ) -> Result<Self, InitError> {
        validate_config(&config)?;
        let now = clock.now_ms();
        let (session_id, last_event_at) =
            load_persisted_session(identity_store.as_ref(), now, config.session_timeout_ms);
        let anonymous_id = load_or_create_anonymous_id(identity_store.as_ref());
        let user_id = load_persisted_user_id(identity_store.as_ref());
        let sample_in = load_persisted_sample_in(
            identity_store.as_ref(),
            session_id.as_ref().is_some_and(|id| !id.trim().is_empty()),
        );
        let queue = load_persisted_queue(identity_store.as_ref());
        let queue_events_bytes = queue_json_bytes(&queue);
        Ok(Self {
            config,
            transport: Arc::new(transport),
            identity_store,
            clock,
            tracking_enabled: true,
            session_id,
            last_event_at,
            anonymous_id,
            user_id,
            current_screen: None,
            sample_in,
            rng,
            diagnostics: HeatmapDiagnostics::default(),
            queue,
            queue_events_bytes,
            queue_limits: QueueLimits::production(),
            queue_pending_since_ms: None,
            retry_batch_id: None,
            retry_not_before_ms: 0,
        })
    }

    pub fn diagnostics(&self) -> HeatmapDiagnostics {
        self.diagnostics
    }

    #[doc(hidden)]
    pub fn set_queue_limits_for_testing(&mut self, limits: QueueLimits) {
        self.queue_limits = limits;
    }

    #[doc(hidden)]
    pub fn queued_event_count_for_testing(&self) -> usize {
        self.queue.len()
    }

    #[doc(hidden)]
    pub fn flush_byte_limit_reached_for_testing(&self) -> bool {
        queue_flush_byte_limit_reached(self.queue_events_bytes, self.queue.len())
    }

    pub fn config(&self) -> &HeatmapConfig {
        &self.config
    }

    pub fn set_tracking_enabled(&mut self, enabled: bool) {
        self.tracking_enabled = enabled;
    }

    pub fn on_background(&mut self) {
        self.persist_session_state();
        let _ = self.try_flush();
        self.sync_persisted_queue();
    }

    pub fn on_foreground(&mut self) {}

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
            timestamp: self.clock.now_ms(),
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
        let now = self.clock.now_ms();
        let new_session = self.ensure_active_session(now);
        if !self.sample_in {
            self.diagnostics.record_sampled_out();
            self.record_session_activity(now);
            return Err(CaptureError::SampledOut);
        }
        if capture_is_excluded(
            &candidate,
            self.current_screen.as_ref(),
            &self.config.exclude_screens,
            &self.config.exclude_elements,
        ) {
            return Err(CaptureError::Excluded);
        }
        let is_session_start = interaction_is_session_start(&candidate.interaction);
        if new_session && !is_session_start {
            self.push_capture_event(session_start_from(&candidate)?)?;
        }
        self.push_capture_event(candidate)?;
        self.record_session_activity(now);
        Ok(())
    }

    pub fn build_batch(&mut self) -> Result<HeatmapBatch, BatchError> {
        if self.queue.is_empty() {
            return Err(BatchError::EmptyQueue);
        }
        let batch_id = new_id("batch");
        let events = self.take_queued_events();
        self.sync_persisted_queue();
        Ok(HeatmapBatch { batch_id, events })
    }

    pub fn flush(&mut self) -> Result<(), FlushError> {
        self.try_flush()
    }

    pub fn poll_flush(&mut self) -> Result<(), FlushError> {
        if !self.should_auto_flush(self.clock.now_ms()) {
            return Ok(());
        }
        self.try_flush()
    }

    fn try_flush(&mut self) -> Result<(), FlushError> {
        if self.clock.now_ms() < self.retry_not_before_ms {
            return Ok(());
        }
        if self.queue.is_empty() {
            return Ok(());
        }
        let batch = self
            .take_batch_for_upload()
            .map_err(|err| FlushError::Transport(err.to_string()))?;
        match self.transport.send_batch(
            &self.config.endpoint,
            &self.config.project_key,
            &batch,
        ) {
            Ok(()) => {
                self.finish_successful_upload();
                Ok(())
            }
            Err(err) => self.handle_upload_failure(batch, err),
        }
    }

    fn take_batch_for_upload(&mut self) -> Result<HeatmapBatch, BatchError> {
        if self.queue.is_empty() {
            return Err(BatchError::EmptyQueue);
        }
        let batch_id = self
            .retry_batch_id
            .take()
            .unwrap_or_else(|| new_id("batch"));
        let events = self.take_queued_events_for_ingest();
        self.sync_persisted_queue();
        Ok(HeatmapBatch { batch_id, events })
    }

    fn take_queued_events(&mut self) -> Vec<HeatmapEvent> {
        self.queue_events_bytes = 0;
        self.queue.drain(..).collect()
    }

    fn take_queued_events_for_ingest(&mut self) -> Vec<HeatmapEvent> {
        let mut events = Vec::new();
        let mut bytes = 0usize;
        while let Some(front) = self.queue.front() {
            if events.len() >= INGEST_MAX_EVENTS {
                break;
            }
            let event_bytes = event_json_bytes(front);
            if !events.is_empty() && bytes.saturating_add(event_bytes) > INGEST_MAX_BYTES {
                break;
            }
            let Some(event) = self.queue.pop_front() else {
                break;
            };
            self.queue_events_bytes = self.queue_events_bytes.saturating_sub(event_bytes);
            bytes = bytes.saturating_add(event_bytes);
            events.push(event);
        }
        events
    }

    fn finish_successful_upload(&mut self) {
        self.retry_batch_id = None;
        self.retry_not_before_ms = 0;
        if self.queue.is_empty() {
            self.queue_pending_since_ms = None;
        }
    }

    fn handle_upload_failure(
        &mut self,
        batch: HeatmapBatch,
        err: FlushError,
    ) -> Result<(), FlushError> {
        if flush_error_is_permanent(&err) || !flush_error_is_transient(&err) {
            self.retry_batch_id = None;
            self.retry_not_before_ms = 0;
            self.diagnostics.record_permanent_drop(1);
            if self.queue.is_empty() {
                self.queue_pending_since_ms = None;
            }
            return Err(err);
        }
        self.retry_batch_id = Some(batch.batch_id);
        for event in batch.events {
            self.queue_events_bytes = self
                .queue_events_bytes
                .saturating_add(event_json_bytes(&event));
            self.queue.push_back(event);
        }
        self.sync_persisted_queue();
        self.retry_not_before_ms = retry_not_before_ms(&err, self.clock.now_ms());
        Err(err)
    }

    fn maybe_auto_flush(&mut self, now: i64) {
        if !self.should_auto_flush(now) {
            return;
        }
        let _ = self.try_flush();
    }

    fn should_auto_flush(&self, now: i64) -> bool {
        if self.queue.is_empty() {
            return false;
        }
        if flush_event_limit_reached(self.queue.len()) {
            return true;
        }
        if queue_flush_byte_limit_reached(self.queue_events_bytes, self.queue.len()) {
            return true;
        }
        self.queue_pending_since_ms
            .is_some_and(|since| flush_interval_elapsed(since, now))
    }

    fn note_queue_activity(&mut self, now: i64) {
        if self.queue_pending_since_ms.is_none() {
            self.queue_pending_since_ms = Some(now);
        }
    }

    fn ensure_active_session(&mut self, now: i64) -> bool {
        if session_still_active(
            self.session_id.as_deref(),
            self.last_event_at,
            now,
            self.config.session_timeout_ms,
        ) {
            return false;
        }
        self.start_new_session();
        true
    }

    fn start_new_session(&mut self) {
        let session_id = new_id("sess");
        self.session_id = Some(session_id.clone());
        self.last_event_at = None;
        self.sample_in = decide_session_sample_in(self.config.sample_rate, self.rng.as_ref());
        self.identity_store
            .write(STORE_KEY_SESSION_ID, Some(session_id.as_str()));
        self.identity_store.write(STORE_KEY_LAST_EVENT_AT, None);
        persist_sample_in(self.identity_store.as_ref(), self.sample_in);
    }

    fn record_session_activity(&mut self, now: i64) {
        self.last_event_at = Some(now);
        self.persist_session_state();
    }

    fn persist_session_state(&mut self) {
        let Some(session_id) = self.session_id.as_deref() else {
            return;
        };
        self.identity_store
            .write(STORE_KEY_SESSION_ID, Some(session_id));
        let last = self
            .last_event_at
            .map(|value| value.to_string())
            .unwrap_or_default();
        if self.last_event_at.is_some() {
            self.identity_store
                .write(STORE_KEY_LAST_EVENT_AT, Some(last.as_str()));
        }
    }

    fn push_capture_event(&mut self, candidate: CaptureCandidate) -> Result<(), CaptureError> {
        let session_id = self
            .session_id
            .as_deref()
            .expect("active session required before capture");
        let event = candidate_to_event(
            candidate,
            CaptureContext {
                session_id,
                anonymous_id: &self.anonymous_id,
                user_id: self.user_id.as_deref(),
                inherited_screen: self.current_screen.as_ref(),
                mask_elements: &self.config.mask_elements,
            },
        )?;
        let dropped = push_with_overflow(
            &mut self.queue,
            &mut self.queue_events_bytes,
            event,
            &self.queue_limits,
        );
        if dropped > 0 {
            self.diagnostics.record_queue_dropped(dropped);
        }
        self.sync_persisted_queue();
        self.note_queue_activity(self.clock.now_ms());
        self.maybe_auto_flush(self.clock.now_ms());
        Ok(())
    }

    fn sync_persisted_queue(&self) {
        persist_queue(self.identity_store.as_ref(), &self.queue);
    }
}

fn new_id(prefix: &str) -> String {
    format!("{prefix}_{}", Uuid::new_v4().simple())
}

fn retry_not_before_ms(err: &FlushError, now_ms: i64) -> i64 {
    let secs = match err {
        FlushError::Transport(_) => return 0,
        FlushError::HttpStatus {
            retry_after_secs, ..
        } => retry_after_secs,
    };
    let Some(secs) = secs else {
        return 0;
    };
    now_ms.saturating_add((*secs as i64).saturating_mul(1_000))
}

fn interaction_is_session_start(interaction: &InteractionEvent) -> bool {
    interaction.interaction_type == InteractionType::SessionStart
}

fn session_start_from(candidate: &CaptureCandidate) -> Result<CaptureCandidate, CaptureError> {
    Ok(CaptureCandidate {
        timestamp: candidate.timestamp,
        sdk: candidate.sdk.clone(),
        platform: candidate.platform.clone(),
        screen: candidate.screen.clone(),
        interaction: InteractionEvent::session_start()
            .map_err(CaptureError::InvalidEventField)?,
        position: None,
        viewport: candidate.viewport.clone(),
        window_id: candidate.window_id.clone(),
    })
}

fn session_still_active(
    session_id: Option<&str>,
    last_event_at: Option<i64>,
    now: i64,
    timeout_ms: u64,
) -> bool {
    if session_id.is_none_or(|id| id.trim().is_empty()) {
        return false;
    }
    match last_event_at {
        Some(last) => !session_inactive(last, now, timeout_ms),
        None => true,
    }
}

fn session_inactive(last_event_at: i64, now: i64, timeout_ms: u64) -> bool {
    now.saturating_sub(last_event_at) as u64 > timeout_ms
}

fn load_persisted_session(
    store: &dyn IdentityStore,
    now: i64,
    timeout_ms: u64,
) -> (Option<String>, Option<i64>) {
    let session_id = store
        .read(STORE_KEY_SESSION_ID)
        .filter(|id| !id.trim().is_empty());
    let last_event_at = store
        .read(STORE_KEY_LAST_EVENT_AT)
        .and_then(|value| value.parse::<i64>().ok());
    let Some(session_id) = session_id else {
        return (None, None);
    };
    let Some(last) = last_event_at else {
        return (Some(session_id), None);
    };
    if session_inactive(last, now, timeout_ms) {
        clear_persisted_session(store);
        return (None, None);
    }
    (Some(session_id), Some(last))
}

fn clear_persisted_session(store: &dyn IdentityStore) {
    store.write(STORE_KEY_SESSION_ID, None);
    store.write(STORE_KEY_LAST_EVENT_AT, None);
    store.write(STORE_KEY_SAMPLE_IN, None);
}

fn load_persisted_sample_in(store: &dyn IdentityStore, session_active: bool) -> bool {
    if !session_active {
        return false;
    }
    match store.read(STORE_KEY_SAMPLE_IN).as_deref() {
        Some("1") => true,
        Some("0") => false,
        _ => true,
    }
}

fn persist_sample_in(store: &dyn IdentityStore, sample_in: bool) {
    let flag = if sample_in { "1" } else { "0" };
    store.write(STORE_KEY_SAMPLE_IN, Some(flag));
}

fn decide_session_sample_in(sample_rate: f64, rng: &dyn RandomSource) -> bool {
    if sample_rate >= 1.0 {
        return true;
    }
    if sample_rate <= 0.0 {
        return false;
    }
    session_sampled_in(sample_rate, rng.next_unit_f64())
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
