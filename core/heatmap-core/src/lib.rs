//! Canonical heatmap event envelope (V1).

mod batch;
mod capture;
mod config;
mod event;
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
pub use sdk::HeatmapSdk;
pub use transport::{
    batch_ingest_url, BatchTransport, FlushError, HttpBatchTransport,
};
