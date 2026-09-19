use std::fmt;

use heatmap_core::{
    FlushError as CoreFlushError, IdentityError as CoreIdentityError,
    InitError as CoreInitError, ScreenContextError as CoreScreenContextError,
};

#[derive(Debug, uniffi::Error)]
pub enum InitError {
    MissingProjectKey,
    MissingEndpoint,
    InsecureEndpoint,
    InvalidConfig,
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingProjectKey => write!(f, "project_key is required"),
            Self::MissingEndpoint => write!(f, "endpoint is required"),
            Self::InsecureEndpoint => write!(f, "endpoint must use HTTPS"),
            Self::InvalidConfig => write!(f, "config is not valid JSON"),
        }
    }
}

impl From<CoreInitError> for InitError {
    fn from(value: CoreInitError) -> Self {
        match value {
            CoreInitError::MissingProjectKey => Self::MissingProjectKey,
            CoreInitError::MissingEndpoint => Self::MissingEndpoint,
            CoreInitError::InsecureEndpoint => Self::InsecureEndpoint,
            CoreInitError::InvalidConfig => Self::InvalidConfig,
        }
    }
}

#[derive(Debug, uniffi::Error)]
pub enum IdentityError {
    EmptyUserId,
}

impl fmt::Display for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyUserId => write!(f, "user_id is required"),
        }
    }
}

impl From<CoreIdentityError> for IdentityError {
    fn from(value: CoreIdentityError) -> Self {
        match value {
            CoreIdentityError::EmptyUserId => Self::EmptyUserId,
        }
    }
}

#[derive(Debug, uniffi::Error)]
pub enum ScreenContextError {
    EmptyScreenId,
}

impl fmt::Display for ScreenContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyScreenId => write!(f, "screen_id is required"),
        }
    }
}

impl From<CoreScreenContextError> for ScreenContextError {
    fn from(value: CoreScreenContextError) -> Self {
        match value {
            CoreScreenContextError::EmptyScreenId => Self::EmptyScreenId,
        }
    }
}

#[derive(Debug, uniffi::Error)]
pub enum FlushError {
    Transport { message: String },
    HttpStatus { code: u16 },
}

impl fmt::Display for FlushError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport { message } => write!(f, "batch upload failed: {message}"),
            Self::HttpStatus { code } => {
                write!(f, "batch upload failed with HTTP status {code}")
            }
        }
    }
}

impl From<CoreFlushError> for FlushError {
    fn from(value: CoreFlushError) -> Self {
        match value {
            CoreFlushError::Transport(message) => Self::Transport { message },
            CoreFlushError::HttpStatus { code, .. } => Self::HttpStatus { code },
        }
    }
}
