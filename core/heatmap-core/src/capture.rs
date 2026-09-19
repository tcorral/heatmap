use std::fmt;

use crate::event::{
    HeatmapEvent, InteractionEvent, InteractionType, InvalidEventField, InvalidPosition,
    PlatformInfo, Position, ScreenInfo, SdkInfo, Viewport, SCHEMA_VERSION,
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
    Excluded,
    InvalidEventField(InvalidEventField),
    InvalidPosition(InvalidPosition),
}

impl fmt::Display for CaptureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TrackingDisabled => write!(f, "tracking is disabled"),
            Self::SampledOut => write!(f, "session is sampled out"),
            Self::Excluded => write!(f, "capture excluded by config"),
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

pub(crate) struct CaptureContext<'a> {
    pub session_id: &'a str,
    pub anonymous_id: &'a str,
    pub user_id: Option<&'a str>,
    pub inherited_screen: Option<&'a ScreenInfo>,
    pub mask_elements: &'a [String],
}

pub(crate) fn capture_is_excluded(
    candidate: &CaptureCandidate,
    inherited_screen: Option<&ScreenInfo>,
    exclude_screens: &[String],
    exclude_elements: &[String],
) -> bool {
    if screen_id_is_excluded(candidate, inherited_screen, exclude_screens) {
        return true;
    }
    click_element_is_excluded(&candidate.interaction, exclude_elements)
}

fn effective_screen_id<'a>(
    candidate: &'a CaptureCandidate,
    inherited_screen: Option<&'a ScreenInfo>,
) -> Option<&'a str> {
    candidate
        .screen
        .as_ref()
        .map(|screen| screen.id.as_str())
        .or_else(|| inherited_screen.map(|screen| screen.id.as_str()))
}

fn screen_id_is_excluded(
    candidate: &CaptureCandidate,
    inherited_screen: Option<&ScreenInfo>,
    exclude_screens: &[String],
) -> bool {
    let Some(screen_id) = effective_screen_id(candidate, inherited_screen) else {
        return false;
    };
    exclude_screens.iter().any(|id| id == screen_id)
}

fn click_element_is_excluded(
    interaction: &InteractionEvent,
    exclude_elements: &[String],
) -> bool {
    if interaction.interaction_type != InteractionType::Click {
        return false;
    }
    let Some(element_id) = interaction.element_id.as_deref() else {
        return false;
    };
    exclude_elements.iter().any(|id| id == element_id)
}

fn mask_click_element_id(
    interaction: InteractionEvent,
    mask_elements: &[String],
) -> InteractionEvent {
    if interaction.interaction_type != InteractionType::Click {
        return interaction;
    }
    let Some(element_id) = interaction.element_id.as_deref() else {
        return interaction;
    };
    if mask_elements.iter().any(|id| id == element_id) {
        InteractionEvent {
            interaction_type: interaction.interaction_type,
            element_id: None,
        }
    } else {
        interaction
    }
}

pub(crate) fn candidate_to_event(
    candidate: CaptureCandidate,
    context: CaptureContext<'_>,
) -> Result<HeatmapEvent, CaptureError> {
    validate_candidate(&candidate)?;
    let screen = candidate
        .screen
        .or_else(|| context.inherited_screen.cloned());
    let event = mask_click_element_id(candidate.interaction, context.mask_elements);
    Ok(HeatmapEvent {
        schema_version: SCHEMA_VERSION,
        session_id: context.session_id.to_owned(),
        anonymous_id: context.anonymous_id.to_owned(),
        user_id: context.user_id.map(str::to_owned),
        timestamp: candidate.timestamp,
        window_id: candidate.window_id,
        sdk: candidate.sdk,
        platform: candidate.platform,
        screen,
        event,
        position: candidate.position,
        viewport: candidate.viewport,
    })
}
