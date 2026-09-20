use serde::{Deserialize, Serialize};
use std::fmt;

pub const SCHEMA_VERSION: i32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformType {
    Web,
    Ios,
    Android,
    Macos,
    Windows,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformInfo {
    #[serde(rename = "type")]
    pub platform_type: PlatformType,
    pub os_version: String,
    pub app_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenInfo {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_version: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionType {
    SessionStart,
    SessionEnd,
    ScreenView,
    Click,
    Scroll,
}

impl InteractionType {
    pub fn parse(label: &str) -> Result<Self, InvalidInteractionType> {
        match label {
            "session_start" => Ok(Self::SessionStart),
            "session_end" => Ok(Self::SessionEnd),
            "screen_view" => Ok(Self::ScreenView),
            "click" => Ok(Self::Click),
            "scroll" => Ok(Self::Scroll),
            _ => Err(InvalidInteractionType {
                label: label.to_owned(),
            }),
        }
    }
}

fn interaction_type_label(interaction_type: InteractionType) -> &'static str {
    match interaction_type {
        InteractionType::SessionStart => "session_start",
        InteractionType::SessionEnd => "session_end",
        InteractionType::ScreenView => "screen_view",
        InteractionType::Click => "click",
        InteractionType::Scroll => "scroll",
    }
}

impl Serialize for InteractionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(interaction_type_label(*self))
    }
}

impl<'de> Deserialize<'de> for InteractionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let label = String::deserialize(deserializer)?;
        InteractionType::parse(&label).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidInteractionType {
    pub label: String,
}

impl fmt::Display for InvalidInteractionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unsupported interaction type: {}", self.label)
    }
}

impl std::error::Error for InvalidInteractionType {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidEventField {
    ElementIdNotAllowed { interaction_type: InteractionType },
}

impl fmt::Display for InvalidEventField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ElementIdNotAllowed { interaction_type } => {
                write!(
                    f,
                    "element_id is not allowed for interaction type {:?}",
                    interaction_type
                )
            }
        }
    }
}

impl std::error::Error for InvalidEventField {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractionEvent {
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
}

impl InteractionEvent {
    pub fn new(
        interaction_type: InteractionType,
        element_id: Option<String>,
    ) -> Result<Self, InvalidEventField> {
        if element_id.is_some() && interaction_type != InteractionType::Click {
            return Err(InvalidEventField::ElementIdNotAllowed { interaction_type });
        }
        Ok(Self {
            interaction_type,
            element_id,
        })
    }

    pub fn session_start() -> Result<Self, InvalidEventField> {
        Self::new(InteractionType::SessionStart, None)
    }

    pub fn session_end() -> Result<Self, InvalidEventField> {
        Self::new(InteractionType::SessionEnd, None)
    }

    pub fn screen_view() -> Result<Self, InvalidEventField> {
        Self::new(InteractionType::ScreenView, None)
    }

    pub fn scroll() -> Result<Self, InvalidEventField> {
        Self::new(InteractionType::Scroll, None)
    }

    pub fn click(element_id: Option<String>) -> Result<Self, InvalidEventField> {
        Self::new(InteractionType::Click, element_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvalidPosition {
    OutOfRange { x: f64, y: f64 },
}

impl fmt::Display for InvalidPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange { x, y } => {
                write!(
                    f,
                    "position ({x}, {y}) must be within 0.0..=1.0 window fractions"
                )
            }
        }
    }
}

impl std::error::Error for InvalidPosition {}

impl Position {
    pub fn new(x: f64, y: f64) -> Result<Self, InvalidPosition> {
        if !(0.0..=1.0).contains(&x) || !(0.0..=1.0).contains(&y) {
            return Err(InvalidPosition::OutOfRange { x, y });
        }
        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeatmapEvent {
    pub schema_version: i32,
    pub session_id: String,
    pub anonymous_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub timestamp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    pub sdk: SdkInfo,
    pub platform: PlatformInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen: Option<ScreenInfo>,
    pub event: InteractionEvent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<Viewport>,
}
