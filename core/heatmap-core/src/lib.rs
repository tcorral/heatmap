//! Canonical heatmap event envelope (V1).

mod batch;
mod capture;
mod clock;
mod config;
mod diagnostics;
mod event;
mod flush_policy;
mod identity_store;
mod persistent_queue;
mod rng;
mod sdk;
mod transport;

pub use batch::{BatchError, HeatmapBatch};
pub use capture::{CaptureCandidate, CaptureError};
pub use clock::{Clock, ManualClock, SystemClock};
pub use config::{HeatmapConfig, InitError};
pub use diagnostics::HeatmapDiagnostics;
pub use event::{
    HeatmapEvent, InteractionEvent, InteractionType, InvalidEventField, InvalidInteractionType,
    InvalidPosition, PlatformInfo, PlatformType, Position, ScreenInfo, SdkInfo, Viewport,
    SCHEMA_VERSION,
};
pub use identity_store::{
    FileIdentityStore, IdentityStore, InMemoryIdentityStore, STORE_KEY_ANONYMOUS_ID,
    STORE_KEY_LAST_EVENT_AT, STORE_KEY_QUEUE, STORE_KEY_SAMPLE_IN, STORE_KEY_SESSION_ID,
    STORE_KEY_USER_ID,
};
pub use flush_policy::{FLUSH_INTERVAL_MS, FLUSH_MAX_BYTES, FLUSH_MAX_EVENTS};
pub use persistent_queue::{QueueLimits, QUEUE_MAX_BYTES, QUEUE_MAX_EVENTS};
pub use rng::{ManualRng, RandomSource, SystemRng};
pub use sdk::{HeatmapSdk, IdentityError, ScreenContextError};
pub use transport::{batch_ingest_url, BatchTransport, FlushError};
#[cfg(not(target_arch = "wasm32"))]
pub use transport::HttpBatchTransport;
