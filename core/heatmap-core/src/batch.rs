use serde::{Deserialize, Serialize};

use crate::event::HeatmapEvent;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeatmapBatch {
    pub batch_id: String,
    pub events: Vec<HeatmapEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchError {
    EmptyQueue,
}

impl std::fmt::Display for BatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyQueue => write!(f, "cannot build batch from an empty queue"),
        }
    }
}

impl std::error::Error for BatchError {}
