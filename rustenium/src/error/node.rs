use thiserror::Error;
use rustenium_core::error::{CommandResultError, CdpCommandResultError};

/// Error from DOM-level node actions: scroll, visibility check, delete, etc.
#[derive(Debug, Error)]
pub enum NodeActionError {
    #[error("BiDi command failed")]
    Bidi(CommandResultError),
    #[error("CDP command failed")]
    Cdp(CdpCommandResultError),
    #[error("CDP command build failed: {0}")]
    CdpBuild(String),
    #[error("Node does not have a shared ID")]
    NoSharedId,
    #[error("{0}")]
    Other(String),
}

/// Error from mouse input operations: move, click.
#[derive(Debug, Error)]
pub enum NodeMouseError {
    #[error(transparent)]
    Action(#[from] NodeActionError),
    #[error("BiDi mouse input failed")]
    Bidi(CommandResultError),
    #[error("CDP mouse command failed")]
    Cdp(CdpCommandResultError),
    #[error("CDP mouse command build failed: {0}")]
    CdpBuild(String),
    #[error("Mouse button '{0}' is already pressed")]
    ButtonAlreadyPressed(String),
    #[error("Mouse button '{0}' is not pressed")]
    ButtonNotPressed(String),
    #[error("Node does not have a valid position")]
    InvalidPosition,
}

/// Error from keyboard / text input operations.
#[derive(Debug, Error)]
pub enum NodeInputError {
    #[error("Unknown key: \"{0}\"")]
    UnknownKey(String),
    #[error("BiDi input command failed")]
    Bidi(CommandResultError),
    #[error("CDP input command failed")]
    Cdp(CdpCommandResultError),
    #[error("CDP input command build failed: {0}")]
    CdpBuild(String),
    #[error("{0}")]
    Other(String),
}

/// Error from node screenshot operations.
#[derive(Debug, Error)]
pub enum NodeScreenshotError {
    #[error("BiDi screenshot command failed")]
    Bidi(CommandResultError),
    #[error("Node does not have a shared ID")]
    NoSharedId,
    #[error("Node does not have a context")]
    NoContext,
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Failed to decode base64 data: {0}")]
    Base64DecodeError(String),
    #[error("Failed to write file: {0}")]
    FileWriteError(String),
    #[error("{0}")]
    Other(String),
}
