use rustenium_bidi_definitions::script::types::EvaluateResultException;
use rustenium_core::error::{CdpCommandResultError, CommandResultError};
use thiserror::Error;

use crate::error::bidi::{
    ContextIndexError, EvaluateResultError, InputError as BidiInputError,
    MouseInputError as BidiMouseInputError, ScreenshotError as BidiScreenshotError,
};

#[derive(Debug, Error)]
pub enum BidiNodeActionError {
    #[error(transparent)]
    Command(#[from] CommandResultError),
    #[error("BiDi script evaluation failed with exception")]
    Exception(EvaluateResultException),
    #[error(transparent)]
    ContextIndex(#[from] ContextIndexError),
    #[error("Node does not have a shared ID")]
    NoSharedId,
}

#[derive(Debug, Error)]
pub enum CdpNodeActionError {
    #[error(transparent)]
    Command(#[from] CdpCommandResultError),
    #[error("CDP response decode failed: {0}")]
    Decode(String),
    #[error("CDP resolveNode did not return an object id")]
    MissingObjectId,
}

/// Error from DOM-level node actions: scroll, visibility check, delete, etc.
#[derive(Debug, Error)]
pub enum NodeActionError {
    #[error(transparent)]
    Bidi(#[from] BidiNodeActionError),
    #[error(transparent)]
    Cdp(#[from] CdpNodeActionError),
}

#[derive(Debug, Error)]
pub enum BidiNodeMouseError {
    #[error(transparent)]
    Command(#[from] CommandResultError),
    #[error(transparent)]
    ContextIndex(#[from] ContextIndexError),
    #[error("Touch has already started")]
    TouchAlreadyStarted,
}

#[derive(Debug, Error)]
pub enum CdpNodeMouseError {
    #[error(transparent)]
    Command(#[from] CdpCommandResultError),
}

/// Error from mouse input operations: move, click.
#[derive(Debug, Error)]
pub enum NodeMouseError {
    #[error(transparent)]
    Action(#[from] NodeActionError),
    #[error(transparent)]
    Bidi(#[from] BidiNodeMouseError),
    #[error(transparent)]
    Cdp(#[from] CdpNodeMouseError),
    #[error("Mouse driver failed: {0}")]
    Driver(String),
    #[error("Mouse button '{0}' is already pressed")]
    ButtonAlreadyPressed(String),
    #[error("Mouse button '{0}' is not pressed")]
    ButtonNotPressed(String),
    #[error("Node does not have a valid position")]
    InvalidPosition,
}

#[derive(Debug, Error)]
pub enum BidiNodeInputError {
    #[error(transparent)]
    Command(#[from] CommandResultError),
}

#[derive(Debug, Error)]
pub enum CdpNodeInputError {
    #[error(transparent)]
    Command(#[from] CdpCommandResultError),
}

/// Error from keyboard / text input operations.
#[derive(Debug, Error)]
pub enum NodeInputError {
    #[error(transparent)]
    Action(#[from] NodeActionError),
    #[error("Unknown key: \"{0}\"")]
    UnknownKey(String),
    #[error(transparent)]
    Bidi(#[from] BidiNodeInputError),
    #[error(transparent)]
    Cdp(#[from] CdpNodeInputError),
    #[error("Touch has already started")]
    TouchAlreadyStarted,
}

#[derive(Debug, Error)]
pub enum BidiNodeScreenshotError {
    #[error(transparent)]
    Command(#[from] CommandResultError),
    #[error(transparent)]
    ContextIndex(#[from] ContextIndexError),
    #[error("Node does not have a shared ID")]
    NoSharedId,
    #[error("Node does not have a context")]
    NoContext,
}

#[derive(Debug, Error)]
pub enum CdpNodeScreenshotError {
    #[error(transparent)]
    Command(#[from] CdpCommandResultError),
    #[error("CDP response decode failed: {0}")]
    Decode(String),
    #[error("Could not determine node position")]
    NoPosition,
}

/// Error from node screenshot operations.
#[derive(Debug, Error)]
pub enum NodeScreenshotError {
    #[error(transparent)]
    Bidi(#[from] BidiNodeScreenshotError),
    #[error(transparent)]
    Cdp(#[from] CdpNodeScreenshotError),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Failed to decode base64 data: {0}")]
    Base64DecodeError(String),
    #[error("Failed to write file: {0}")]
    FileWriteError(String),
}

impl From<CommandResultError> for NodeActionError {
    fn from(error: CommandResultError) -> Self {
        Self::Bidi(error.into())
    }
}

impl From<CdpCommandResultError> for NodeActionError {
    fn from(error: CdpCommandResultError) -> Self {
        Self::Cdp(error.into())
    }
}

impl From<CommandResultError> for NodeMouseError {
    fn from(error: CommandResultError) -> Self {
        Self::Bidi(error.into())
    }
}

impl From<CdpCommandResultError> for NodeMouseError {
    fn from(error: CdpCommandResultError) -> Self {
        Self::Cdp(error.into())
    }
}

impl From<CommandResultError> for NodeInputError {
    fn from(error: CommandResultError) -> Self {
        Self::Bidi(error.into())
    }
}

impl From<CdpCommandResultError> for NodeInputError {
    fn from(error: CdpCommandResultError) -> Self {
        Self::Cdp(error.into())
    }
}

impl From<CommandResultError> for NodeScreenshotError {
    fn from(error: CommandResultError) -> Self {
        Self::Bidi(error.into())
    }
}

impl From<CdpCommandResultError> for NodeScreenshotError {
    fn from(error: CdpCommandResultError) -> Self {
        Self::Cdp(error.into())
    }
}

impl From<EvaluateResultError> for BidiNodeActionError {
    fn from(error: EvaluateResultError) -> Self {
        match error {
            EvaluateResultError::ContextIndexError(error) => Self::ContextIndex(error),
            EvaluateResultError::ExceptionError(error) => Self::Exception(error),
            EvaluateResultError::CommandResultError(error) => Self::Command(error),
            EvaluateResultError::NoSharedId => Self::NoSharedId,
        }
    }
}

impl From<EvaluateResultError> for NodeActionError {
    fn from(error: EvaluateResultError) -> Self {
        Self::Bidi(error.into())
    }
}

impl From<BidiInputError> for NodeInputError {
    fn from(error: BidiInputError) -> Self {
        match error {
            BidiInputError::UnknownKey(key) => Self::UnknownKey(key),
            BidiInputError::TouchAlreadyStarted => Self::TouchAlreadyStarted,
            BidiInputError::CommandResultError(error) => Self::Bidi(error.into()),
        }
    }
}

impl From<BidiMouseInputError> for NodeMouseError {
    fn from(error: BidiMouseInputError) -> Self {
        match error {
            BidiMouseInputError::ContextIndexError(error) => Self::Bidi(error.into()),
            BidiMouseInputError::EvaluateResultError(error) => Self::Action(error.into()),
            BidiMouseInputError::InputError(BidiInputError::CommandResultError(error)) => {
                Self::Bidi(error.into())
            }
            BidiMouseInputError::InputError(BidiInputError::UnknownKey(_)) => {
                unreachable!("mouse input should not produce unknown key errors")
            }
            BidiMouseInputError::InputError(BidiInputError::TouchAlreadyStarted) => {
                Self::Bidi(BidiNodeMouseError::TouchAlreadyStarted)
            }
            BidiMouseInputError::InvalidPositionError(_) => Self::InvalidPosition,
        }
    }
}

impl From<BidiScreenshotError> for NodeScreenshotError {
    fn from(error: BidiScreenshotError) -> Self {
        match error {
            BidiScreenshotError::ContextIndexError(error) => Self::Bidi(error.into()),
            BidiScreenshotError::CommandResultError(error) => Self::Bidi(error.into()),
            BidiScreenshotError::InvalidPath(path) => Self::InvalidPath(path),
            BidiScreenshotError::Base64DecodeError(error) => Self::Base64DecodeError(error),
            BidiScreenshotError::FileWriteError(error) => Self::FileWriteError(error),
            BidiScreenshotError::NoSharedId => Self::Bidi(BidiNodeScreenshotError::NoSharedId),
            BidiScreenshotError::NoContext => Self::Bidi(BidiNodeScreenshotError::NoContext),
        }
    }
}
