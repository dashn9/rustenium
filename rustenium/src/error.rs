use thiserror::Error;
use rustenium_bidi_commands::ErrorResponse;
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