use std::fmt;

use crate::event::{
    HeatmapEvent, InteractionEvent, InvalidEventField, InvalidPosition, PlatformInfo, Position,
    ScreenInfo, SdkInfo, Viewport, SCHEMA_VERSION,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CaptureCandidate {
    pub timestamp: i64,
    pub sdk: SdkInfo,
    pub platform: PlatformInfo,
    pub screen: Option<ScreenInfo>,
    pub interaction: InteractionEvent,
    pub position: Option<Position>,
    pub viewport: Option<Viewport>,
    pub window_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CaptureError {
    TrackingDisabled,
    SampledOut,
    InvalidEventField(InvalidEventField),
    InvalidPosition(InvalidPosition),
}

impl fmt::Display for CaptureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TrackingDisabled => write!(f, "tracking is disabled"),
            Self::SampledOut => write!(f, "session is sampled out"),
            Self::InvalidEventField(err) => write!(f, "{err}"),
            Self::InvalidPosition(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for CaptureError {}

pub(crate) fn validate_candidate(candidate: &CaptureCandidate) -> Result<(), CaptureError> {
    if let Some(position) = candidate.position {
        Position::new(position.x, position.y).map_err(CaptureError::InvalidPosition)?;
    }
    Ok(())
}

pub(crate) fn candidate_to_event(
    candidate: CaptureCandidate,
    session_id: &str,
    anonymous_id: &str,
) -> Result<HeatmapEvent, CaptureError> {
    validate_candidate(&candidate)?;
    Ok(HeatmapEvent {
        schema_version: SCHEMA_VERSION,
        session_id: session_id.to_owned(),
        anonymous_id: anonymous_id.to_owned(),
        user_id: None,
        timestamp: candidate.timestamp,
        window_id: candidate.window_id,
        sdk: candidate.sdk,
        platform: candidate.platform,
        screen: candidate.screen,
        event: candidate.interaction,
        position: candidate.position,
        viewport: candidate.viewport,
    })
}
