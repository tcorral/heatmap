//! Canonical heatmap event envelope (V1).

mod batch;
mod capture;
mod config;
mod event;
mod identity_store;
mod sdk;
mod transport;

pub use batch::{BatchError, HeatmapBatch};
pub use capture::{CaptureCandidate, CaptureError};
pub use config::{HeatmapConfig, InitError};
pub use event::{
    HeatmapEvent, InteractionEvent, InteractionType, InvalidEventField, InvalidInteractionType,
    InvalidPosition, PlatformInfo, PlatformType, Position, ScreenInfo, SdkInfo, Viewport,
    SCHEMA_VERSION,
};
pub use identity_store::{
    IdentityStore, InMemoryIdentityStore, STORE_KEY_ANONYMOUS_ID, STORE_KEY_USER_ID,
};
pub use sdk::{HeatmapSdk, IdentityError, ScreenContextError};
pub use transport::{batch_ingest_url, BatchTransport, FlushError};
#[cfg(not(target_arch = "wasm32"))]
pub use transport::HttpBatchTransport;
