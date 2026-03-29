use thiserror::Error;
use rustenium_core::error::CdpCommandResultError;

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
