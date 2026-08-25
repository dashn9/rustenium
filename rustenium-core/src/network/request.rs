//! The protocol-neutral request surface shared by BiDi and CDP interception.
//!
//! Modelled on Puppeteer's abstract `HTTPRequest`: neutral parameter types plus
//! a trait whose provided methods hold the shared behaviour, leaving each
//! protocol to implement only [`HttpRequest::continue_impl`],
//! [`HttpRequest::respond_impl`] and [`HttpRequest::abort_impl`].
//!
//! Puppeteer's priority/`finalizeInterceptions` resolution machinery is
//! deliberately absent. It exists there because `page.on('request')` is a
//! synchronous emitter that cannot await its listeners, so priorities are how
//! several listeners negotiate a single outcome. Handlers here are awaited in
//! registration order, which gives the same power through plain
//! first-resolver-wins semantics — later handlers observe
//! [`HttpRequest::is_handled`].

use std::collections::HashMap;
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};

use thiserror::Error;

/// How a resource is going to be used, as CDP reports it.
///
/// WebDriver BiDi has no equivalent field, which is why
/// [`HttpRequest::resource_type`] is optional.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Document,
    Stylesheet,
    Image,
    Media,
    Font,
    Script,
    TextTrack,
    Xhr,
    Fetch,
    Prefetch,
    EventSource,
    WebSocket,
    Manifest,
    SignedExchange,
    Ping,
    CspViolationReport,
    Preflight,
    FedCm,
    Other,
}

/// Why a request was aborted.
///
/// Carried through to `Fetch.failRequest` on CDP. WebDriver BiDi's
/// `network.failRequest` takes no reason, so it is ignored there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ErrorCode {
    Aborted,
    AccessDenied,
    AddressUnreachable,
    BlockedByClient,
    BlockedByResponse,
    ConnectionAborted,
    ConnectionClosed,
    ConnectionFailed,
    ConnectionRefused,
    ConnectionReset,
    InternetDisconnected,
    NameNotResolved,
    TimedOut,
    #[default]
    Failed,
}

/// Modifications applied to a request as it is let through.
#[derive(Debug, Clone, Default)]
pub struct ContinueRequestOverrides {
    pub url: Option<String>,
    pub method: Option<String>,
    pub post_data: Option<Vec<u8>>,
    pub headers: Option<HashMap<String, String>>,
}

impl ContinueRequestOverrides {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    pub fn post_data(mut self, post_data: impl Into<Vec<u8>>) -> Self {
        self.post_data = Some(post_data.into());
        self
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers
            .get_or_insert_with(HashMap::new)
            .insert(name.into(), value.into());
        self
    }

    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }
}

/// A response to fulfil a request with, instead of letting it reach the network.
#[derive(Debug, Clone)]
pub struct ResponseForRequest {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub content_type: Option<String>,
    pub body: Vec<u8>,
}

impl Default for ResponseForRequest {
    fn default() -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            content_type: None,
            body: Vec::new(),
        }
    }
}

impl ResponseForRequest {
    pub fn new(status: u16) -> Self {
        Self {
            status,
            ..Default::default()
        }
    }

    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self
    }

    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Header names lowercased, with `content-type` and `content-length` filled
    /// in from the body when the caller did not set them.
    pub fn normalized_headers(&self) -> HashMap<String, String> {
        let mut headers: HashMap<String, String> = self
            .headers
            .iter()
            .map(|(name, value)| (name.to_lowercase(), value.clone()))
            .collect();

        if let Some(content_type) = &self.content_type {
            headers.insert("content-type".to_string(), content_type.clone());
        }
        if !self.body.is_empty() {
            headers
                .entry("content-length".to_string())
                .or_insert_with(|| self.body.len().to_string());
        }
        headers
    }
}

#[derive(Debug, Error)]
pub enum InterceptionError {
    #[error("Request is already handled")]
    AlreadyHandled,
    #[error("{0}")]
    Protocol(String),
}

/// Whether a request has been resolved yet.
///
/// One of these is shared by every handler that sees a given request, so the
/// first to continue, abort or respond wins and the rest are told so.
#[derive(Debug, Default)]
pub struct Interception {
    handled: AtomicBool,
}

impl Interception {
    pub fn is_handled(&self) -> bool {
        self.handled.load(Ordering::Acquire)
    }

    /// Takes ownership of resolving this request, or reports that someone else
    /// already has.
    pub fn claim(&self) -> Result<(), InterceptionError> {
        self.handled
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map(|_| ())
            .map_err(|_| InterceptionError::AlreadyHandled)
    }

    /// Hands the claim back so a failed resolution can be retried.
    pub fn release(&self) {
        self.handled.store(false, Ordering::Release);
    }
}

/// Errors that mean the request is already gone rather than that the call was
/// wrong.
///
/// A paused request dies with its target, and the pause is released by whoever
/// gets there first, so both races are routine during teardown and neither is
/// worth surfacing. Everything else — a malformed header above all — is a real
/// mistake and is returned.
pub fn handle_protocol_error(message: String) -> Result<(), InterceptionError> {
    const BENIGN: &[&str] = &[
        "Invalid InterceptionId",
        "Session closed",
        "Target closed",
        "Session with given id not found",
        "No resource with given identifier",
    ];

    if BENIGN.iter().any(|benign| message.contains(benign)) {
        tracing::debug!("[Interception] ignoring stale request: {}", message);
        return Ok(());
    }
    Err(InterceptionError::Protocol(message))
}

/// A network request paused mid-flight, whatever protocol paused it.
pub trait HttpRequest: Send + Sync {
    fn url(&self) -> &str;

    fn method(&self) -> &str;

    /// Header names lowercased, as HTTP treats them case-insensitively.
    fn headers(&self) -> HashMap<String, String>;

    fn post_data(&self) -> Option<String>;

    fn has_post_data(&self) -> bool;

    /// `None` where the protocol does not report one.
    fn resource_type(&self) -> Option<ResourceType>;

    fn interception(&self) -> &Interception;

    fn continue_impl(
        &self,
        overrides: ContinueRequestOverrides,
    ) -> impl Future<Output = Result<(), InterceptionError>> + Send;

    fn respond_impl(
        &self,
        response: ResponseForRequest,
    ) -> impl Future<Output = Result<(), InterceptionError>> + Send;

    fn abort_impl(
        &self,
        error: ErrorCode,
    ) -> impl Future<Output = Result<(), InterceptionError>> + Send;

    fn is_handled(&self) -> bool {
        self.interception().is_handled()
    }

    fn header(&self, name: &str) -> Option<String> {
        self.headers().remove(&name.to_lowercase())
    }

    fn continue_(&self) -> impl Future<Output = Result<(), InterceptionError>> + Send
    where
        Self: Sized,
    {
        self.continue_with(ContinueRequestOverrides::default())
    }

    fn continue_with(
        &self,
        overrides: ContinueRequestOverrides,
    ) -> impl Future<Output = Result<(), InterceptionError>> + Send
    where
        Self: Sized,
    {
        async move { self.resolve(self.continue_impl(overrides)).await }
    }

    fn respond(
        &self,
        response: ResponseForRequest,
    ) -> impl Future<Output = Result<(), InterceptionError>> + Send
    where
        Self: Sized,
    {
        async move { self.resolve(self.respond_impl(response)).await }
    }

    fn abort(&self) -> impl Future<Output = Result<(), InterceptionError>> + Send
    where
        Self: Sized,
    {
        self.abort_with(ErrorCode::default())
    }

    fn abort_with(
        &self,
        error: ErrorCode,
    ) -> impl Future<Output = Result<(), InterceptionError>> + Send
    where
        Self: Sized,
    {
        async move { self.resolve(self.abort_impl(error)).await }
    }

    /// Claims the request, runs the protocol call, and hands the claim back if
    /// it failed so a handler can try something else.
    ///
    /// `data:` URLs are answered as if resolved: the network service never
    /// routes them through interception, so nothing is waiting on a reply and
    /// sending one is an error.
    fn resolve<F>(&self, action: F) -> impl Future<Output = Result<(), InterceptionError>> + Send
    where
        Self: Sized,
        F: Future<Output = Result<(), InterceptionError>> + Send,
    {
        async move {
            if self.url().starts_with("data:") {
                return Ok(());
            }
            self.interception().claim()?;
            match action.await {
                Ok(()) => Ok(()),
                Err(error) => {
                    self.interception().release();
                    Err(error)
                }
            }
        }
    }
}
