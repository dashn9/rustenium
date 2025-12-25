use thiserror::Error;
use rustenium_bidi_commands::script::types::EvaluateResultException;
use rustenium_core::error::{CommandResultError};

#[derive(Debug, Error)]
pub enum ContextCreationListenError {
    #[error(transparent)]
    ZeroBrowsingContextAtStart(#[from] ZeroBrowsingContextAtStartError),
    #[error("An error occured executing command")]
    CommandResultError(CommandResultError),
}

#[derive(Debug, Error)]
#[error("No Browsing Context Detected On Browser Startup Within Specified Limit")]
pub struct ZeroBrowsingContextAtStartError;

#[derive(Debug, Error)]
#[error("Context does not exist for this ")]
pub struct ContextIndexError{}

#[derive(Debug, Error)]
pub enum OpenUrlError {
    #[error(transparent)]
    ContextIndexError(#[from] ContextIndexError),
    #[error("An error occured executing command")]
    CommandResultError(CommandResultError),
}

#[derive(Debug, Error)]
pub enum FindNodesError {
    #[error(transparent)]
    ContextIndexError(#[from] ContextIndexError),
    #[error("An error occured executing command")]
    CommandResultError(CommandResultError),
}

#[derive(Debug, Error)]
pub enum EvaluateResultError {
    #[error(transparent)]
    ContextIndexError(#[from] ContextIndexError),
    #[error("Script Evaluation failed with Exception")]
    ExceptionError(EvaluateResultException),
    #[error("An error occured executing command")]
    CommandResultError(CommandResultError),
    #[error("Node does not have a shared ID")]
    NoSharedId,
}

#[derive(Debug, Error)]
pub enum InterceptNetworkError {
    #[error(transparent)]
    ContextIndexError(#[from] ContextIndexError),
    #[error("An error occured executing command")]
    CommandResultError(CommandResultError),
}

#[derive(Debug, Error)]
pub enum InputError {
    #[error("Unknown key: \"{0}\"")]
    UnknownKey(String),
    #[error("Touch has already started")]
    TouchAlreadyStarted,
    #[error("An error occured executing command")]
    CommandResultError(CommandResultError),
}

#[derive(Debug, Error)]
#[error("Node does not have a valid position")]
pub struct InvalidPositionError;

#[derive(Debug, Error)]
pub enum MouseInputError {
    #[error(transparent)]
    ContextIndexError(#[from] ContextIndexError),
    #[error(transparent)]
    EvaluateResultError(#[from] EvaluateResultError),
    #[error(transparent)]
    InputError(#[from] InputError),
    #[error(transparent)]
    InvalidPositionError(#[from] InvalidPositionError),
}

#[derive(Debug, Error)]
pub enum ScreenshotError {
    #[error(transparent)]
    ContextIndexError(#[from] ContextIndexError),
    #[error("An error occured executing command")]
    CommandResultError(CommandResultError),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Failed to decode base64 data: {0}")]
    Base64DecodeError(String),
    #[error("Failed to write file: {0}")]
    FileWriteError(String),
    #[error("Node does not have a shared ID")]
    NoSharedId,
    #[error("Node does not have a context")]
    NoContext,
}

#[derive(Debug, Error)]
pub enum EmulationError {
    #[error(transparent)]
    ContextIndexError(#[from] ContextIndexError),
    #[error("An error occured executing command")]
    CommandResultError(CommandResultError),
}