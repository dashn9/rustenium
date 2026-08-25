pub mod bidi;
pub mod cdp;
pub mod request;

pub use bidi::NetworkRequest;
pub use cdp::CdpHttpRequest;
pub use request::{
    ContinueRequestOverrides, ErrorCode, HttpRequest, Interception, InterceptionError,
    ResourceType, ResponseForRequest,
};

/// How a network request was handled
#[derive(Debug, Clone)]
pub enum NetworkRequestHandledState {
    /// Request was continued (with or without modifications)
    Continued,
    /// Request was aborted/failed
    Aborted,
    /// Request was responded with a custom response
    Responded,
}
