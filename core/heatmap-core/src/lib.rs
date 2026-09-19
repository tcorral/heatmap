//! Canonical heatmap event envelope (V1).

mod event;

pub use event::{
    HeatmapEvent, InteractionEvent, InteractionType, InvalidEventField, InvalidInteractionType,
    InvalidPosition, PlatformInfo, PlatformType, Position, ScreenInfo, SdkInfo, Viewport,
    SCHEMA_VERSION,
};
