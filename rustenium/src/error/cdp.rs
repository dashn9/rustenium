use thiserror::Error;
use rustenium_core::error::CdpCommandResultError;

#[derive(Debug, Error)]
pub enum NavigateError {
    #[error("An error occured executing command")]
    CommandResultError(CdpCommandResultError),
}
