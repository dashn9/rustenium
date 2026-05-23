use rustenium_core::error::CdpCommandResultError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NavigateError {
    #[error("An error occured executing command")]
    CommandResultError(CdpCommandResultError),
}

#[derive(Debug, Error)]
pub enum CreateTargetError {
    #[error("An error occured executing command")]
    CommandResultError(CdpCommandResultError),
}

#[derive(Debug, Error)]
pub enum CreateTabError {
    #[error("An error occured creating target")]
    CreateTargetError(CreateTargetError),
}

#[derive(Debug, Error)]
pub enum EmulateDeviceMetricsError {
    #[error("An error occured executing command")]
    CommandResultError(CdpCommandResultError),
}

#[derive(Debug, Error)]
pub enum NodesFetchError {
    #[error("An error occured executing command")]
    CommandResultError(CdpCommandResultError),
    #[error("Failed to parse accessibility tree result")]
    ParseError(String),
}

#[derive(Debug, Error)]
pub enum MouseInputError {
    #[error("An error occured executing CDP command")]
    CommandError(CdpCommandResultError),
    #[error("Mouse button '{0}' is already pressed")]
    ButtonAlreadyPressed(String),
    #[error("Mouse button '{0}' is not pressed")]
    ButtonNotPressed(String),
}

#[derive(Debug, Error)]
pub enum InputError {
    #[error("Unknown key: \"{0}\"")]
    UnknownKey(String),
    #[error("An error occured executing CDP command")]
    CommandError(CdpCommandResultError),
}

#[derive(Debug, Error)]
pub enum LocateError {
    #[error("An error occured executing command")]
    CommandResultError(CdpCommandResultError),
    #[error("Failed to parse result: {0}")]
    ParseError(String),
    #[error("Timed out waiting for selector: {0}")]
    Timeout(String),
}

#[derive(Debug, Error)]
pub enum EvaluateScriptError {
    #[error("An error occured executing command")]
    CommandResultError(CdpCommandResultError),
    #[error("Failed to parse evaluate result: {0}")]
    ParseError(String),
}

#[derive(Debug, Error)]
pub enum PreloadScriptError {
    #[error("An error occured executing command")]
    CommandResultError(CdpCommandResultError),
    #[error("Failed to parse preload script result: {0}")]
    ParseError(String),
}

#[derive(Debug, Error)]
pub enum ScreenshotError {
    #[error("An error occured executing command")]
    CommandResultError(CdpCommandResultError),
    #[error("Failed to parse screenshot result: {0}")]
    ParseError(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Failed to decode base64 data: {0}")]
    Base64DecodeError(String),
    #[error("Failed to write file: {0}")]
    FileWriteError(String),
}
