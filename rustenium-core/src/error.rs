use thiserror::Error;
use rustenium_bidi_commands::{ErrorResponse, ResultData};

#[derive(Debug, Error)]
pub enum SessionSendError {
    #[error("Remote End Returned an Error Response")]
    ErrorResponse(ErrorResponse),
    #[error("Could not receive response for command in time")]
    ResponseReceiveTimeoutError(ResponseReceiveTimeoutError)
}
#[derive(Debug, Error)]
pub enum CommandResultError {
    #[error("Invalid Result gotten For Command")]
    InvalidResultTypeError(ResultData),
    #[error("Error Occurred with Command")]
    SessionSendError(SessionSendError)
}

#[derive(Debug, Error)]
pub struct  ResponseReceiveTimeoutError;

impl std::fmt::Display for ResponseReceiveTimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Could not receive response for command in time")
    }
}
