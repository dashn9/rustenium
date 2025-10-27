use thiserror::Error;
use rustenium_bidi_commands::ErrorResponse;
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
}