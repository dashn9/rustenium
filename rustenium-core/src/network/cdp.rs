use crate::error::{CdpCommandResultError, CdpSessionSendError, PostDataError};
use crate::session::CdpSession;
use crate::transport::ConnectionTransport;
use base64::{Engine as _, engine::general_purpose};
use form_urlencoded;
use rustenium_cdp_definitions::Command;
use rustenium_cdp_definitions::base::EventResponse;
use rustenium_cdp_definitions::browser_protocol::fetch::command_builders::{
    ContinueRequestBuilder, ContinueResponseBuilder, FulfillRequestBuilder,
};
use rustenium_cdp_definitions::browser_protocol::fetch::commands::{
    ContinueRequest, ContinueWithAuth, FailRequest, GetResponseBody,
};
use rustenium_cdp_definitions::browser_protocol::fetch::events::{
    AuthRequired, AuthRequiredParams, RequestPaused, RequestPausedParams,
};
use rustenium_cdp_definitions::browser_protocol::fetch::results::GetResponseBodyResult;
use rustenium_cdp_definitions::browser_protocol::fetch::types::{
    AuthChallenge, AuthChallengeResponse, AuthChallengeResponseResponse, HeaderEntry, RequestId,
};
use rustenium_cdp_definitions::browser_protocol::network::types::{
    ErrorReason, Headers, Request, RequestId as NetworkRequestId, ResourceType as CdpResourceType,
};
use rustenium_cdp_definitions::browser_protocol::page::types::FrameId;
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::request::{
    ContinueRequestOverrides, ErrorCode, HttpRequest, Interception, InterceptionError,
    ResourceType, ResponseForRequest, handle_protocol_error,
};

fn error_reason(error: ErrorCode) -> ErrorReason {
    match error {
        ErrorCode::Aborted => ErrorReason::Aborted,
        ErrorCode::AccessDenied => ErrorReason::AccessDenied,
        ErrorCode::AddressUnreachable => ErrorReason::AddressUnreachable,
        ErrorCode::BlockedByClient => ErrorReason::BlockedByClient,
        ErrorCode::BlockedByResponse => ErrorReason::BlockedByResponse,
        ErrorCode::ConnectionAborted => ErrorReason::ConnectionAborted,
        ErrorCode::ConnectionClosed => ErrorReason::ConnectionClosed,
        ErrorCode::ConnectionFailed => ErrorReason::ConnectionFailed,
        ErrorCode::ConnectionRefused => ErrorReason::ConnectionRefused,
        ErrorCode::ConnectionReset => ErrorReason::ConnectionReset,
        ErrorCode::InternetDisconnected => ErrorReason::InternetDisconnected,
        ErrorCode::NameNotResolved => ErrorReason::NameNotResolved,
        ErrorCode::TimedOut => ErrorReason::TimedOut,
        ErrorCode::Failed => ErrorReason::Failed,
    }
}

fn resource_type(resource_type: &CdpResourceType) -> ResourceType {
    match resource_type {
        CdpResourceType::Document => ResourceType::Document,
        CdpResourceType::Stylesheet => ResourceType::Stylesheet,
        CdpResourceType::Image => ResourceType::Image,
        CdpResourceType::Media => ResourceType::Media,
        CdpResourceType::Font => ResourceType::Font,
        CdpResourceType::Script => ResourceType::Script,
        CdpResourceType::TextTrack => ResourceType::TextTrack,
        CdpResourceType::Xhr => ResourceType::Xhr,
        CdpResourceType::Fetch => ResourceType::Fetch,
        CdpResourceType::Prefetch => ResourceType::Prefetch,
        CdpResourceType::EventSource => ResourceType::EventSource,
        CdpResourceType::WebSocket => ResourceType::WebSocket,
        CdpResourceType::Manifest => ResourceType::Manifest,
        CdpResourceType::SignedExchange => ResourceType::SignedExchange,
        CdpResourceType::Ping => ResourceType::Ping,
        CdpResourceType::CspViolationReport => ResourceType::CspViolationReport,
        CdpResourceType::Preflight => ResourceType::Preflight,
        CdpResourceType::FedCm => ResourceType::FedCm,
        CdpResourceType::Other => ResourceType::Other,
    }
}

fn header_entries(headers: &HashMap<String, String>) -> Vec<HeaderEntry> {
    headers
        .iter()
        .map(|(name, value)| HeaderEntry::new(name.clone(), value.clone()))
        .collect()
}

/// A network request paused by the CDP `Fetch` domain.
///
/// Built from a single `Fetch.requestPaused` or `Fetch.authRequired` event and
/// nothing else. Puppeteer instead correlates the pause with the matching
/// `Network.requestWillBeSent` so it can expose an initiator and a redirect
/// chain, and pays for it with an event queue, duplicate-pause handling, and
/// requests that hang forever when the network event never arrives. The Fetch
/// event already carries URL, method, headers, frame and resource type, so
/// dropping the correlation costs only initiator and timing data — and removes
/// that entire class of stall.
pub struct CdpHttpRequest<T: ConnectionTransport> {
    pub request_id: RequestId,
    pub request: Request,
    pub frame_id: FrameId,
    pub cdp_resource_type: CdpResourceType,
    /// Set only when the request paused at the response stage.
    pub response_status_code: Option<i64>,
    pub response_status_text: Option<String>,
    pub response_headers: Option<Vec<HeaderEntry>>,
    pub response_error_reason: Option<ErrorReason>,
    /// Matches the `requestId` of this request's `Network.requestWillBeSent`,
    /// when one was fired.
    pub network_id: Option<NetworkRequestId>,
    /// The request this one was redirected from, on a redirect hop.
    pub redirected_request_id: Option<RequestId>,
    /// Set only for `Fetch.authRequired`.
    pub auth_challenge: Option<AuthChallenge>,
    interception: Interception,
    raw: serde_json::Value,
    session_id: Option<String>,
    session: Arc<Mutex<CdpSession<T>>>,
}

impl<T: ConnectionTransport> std::fmt::Debug for CdpHttpRequest<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CdpHttpRequest")
            .field("request_id", &self.request_id)
            .field("url", &self.request.url)
            .field("method", &self.request.method)
            .field("resource_type", &self.cdp_resource_type)
            .field("response_status_code", &self.response_status_code)
            .field("session_id", &self.session_id)
            .field("handled", &self.interception.is_handled())
            .finish_non_exhaustive()
    }
}

impl<T: ConnectionTransport + Send + Sync> CdpHttpRequest<T> {
    /// Returns `None` for any event that is neither `Fetch.requestPaused` nor
    /// `Fetch.authRequired`, or whose params fail to parse.
    pub fn from_event(event: EventResponse, session: Arc<Mutex<CdpSession<T>>>) -> Option<Self> {
        let session_id = event.session_id.clone();
        let raw = event.params.clone();

        if event.method == RequestPaused::IDENTIFIER {
            let params: RequestPausedParams = serde_json::from_value(event.params).ok()?;
            Some(Self {
                request_id: params.request_id,
                request: params.request,
                frame_id: params.frame_id,
                cdp_resource_type: params.resource_type,
                response_status_code: params.response_status_code,
                response_status_text: params.response_status_text,
                response_headers: params.response_headers,
                response_error_reason: params.response_error_reason,
                network_id: params.network_id,
                redirected_request_id: params.redirected_request_id,
                auth_challenge: None,
                interception: Interception::default(),
                raw,
                session_id,
                session,
            })
        } else if event.method == AuthRequired::IDENTIFIER {
            let params: AuthRequiredParams = serde_json::from_value(event.params).ok()?;
            Some(Self {
                request_id: params.request_id,
                request: params.request,
                frame_id: params.frame_id,
                cdp_resource_type: params.resource_type,
                response_status_code: None,
                response_status_text: None,
                response_headers: None,
                response_error_reason: None,
                network_id: None,
                redirected_request_id: None,
                auth_challenge: Some(params.auth_challenge),
                interception: Interception::default(),
                raw,
                session_id,
                session,
            })
        } else {
            None
        }
    }

    pub fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    /// The headers exactly as CDP models them, before the lowercasing that
    /// [`HttpRequest::headers`] applies.
    pub fn cdp_headers(&self) -> &Headers {
        &self.request.headers
    }

    pub fn frame_id(&self) -> &FrameId {
        &self.frame_id
    }

    pub fn auth_challenge(&self) -> Option<&AuthChallenge> {
        self.auth_challenge.as_ref()
    }

    /// The session the pause arrived on. Every reply is addressed back at it, so
    /// a request paused in a worker or out-of-process iframe is answered there
    /// rather than at the page.
    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    /// The event params exactly as the browser sent them, including fields the
    /// generated bindings drop.
    pub fn raw(&self) -> &serde_json::Value {
        &self.raw
    }

    /// Whether the request paused after the response headers arrived rather than
    /// before the request was sent. Only a request paused here can have its body
    /// read or its response continued.
    pub fn is_response_stage(&self) -> bool {
        self.response_status_code.is_some() || self.response_error_reason.is_some()
    }

    pub fn status_code(&self) -> Option<i64> {
        self.response_status_code
    }

    pub fn post_data_json(
        &self,
    ) -> Result<serde_json::Map<String, serde_json::Value>, PostDataError> {
        let data = self.post_data().ok_or(PostDataError::NoPostData)?;
        let value: serde_json::Value = serde_json::from_str(&data)?;
        value
            .as_object()
            .cloned()
            .ok_or(PostDataError::NotJsonObject)
    }

    pub fn post_data_form(&self) -> Result<HashMap<String, String>, PostDataError> {
        let data = self.post_data().ok_or(PostDataError::NoPostData)?;
        Ok(form_urlencoded::parse(data.as_bytes())
            .into_owned()
            .collect())
    }

    /// Queues the reply, then waits for it with the session lock released — a
    /// paused request is answered while another command is often mid-flight, and
    /// holding the lock across the wait deadlocks both.
    async fn send(&self, command: impl Into<Command>) -> Result<(), InterceptionError> {
        let rx = {
            let mut session = self.session.lock().await;
            session
                .send_and_get_receiver_for(command, self.session_id.clone())
                .await
        };
        match CdpSession::<T>::await_response(rx).await {
            Ok(_) => Ok(()),
            Err(CdpSessionSendError::ErrorResponse(response)) => {
                handle_protocol_error(response.error.message)
            }
            Err(error) => Err(InterceptionError::Protocol(error.to_string())),
        }
    }

    /// Escape hatch for the parts of `Fetch.continueRequest` the neutral
    /// overrides do not cover, such as `interceptResponse`.
    pub async fn continue_with_builder(
        &self,
        continue_request: ContinueRequestBuilder,
    ) -> Result<(), InterceptionError> {
        self.interception.claim()?;
        let command = continue_request
            .request_id(self.request_id.clone())
            .build()
            .unwrap();
        self.send(command)
            .await
            .inspect_err(|_| self.interception.release())
    }

    /// Only valid for a request paused at the response stage.
    pub async fn continue_response(
        &self,
        continue_response: ContinueResponseBuilder,
    ) -> Result<(), InterceptionError> {
        self.interception.claim()?;
        let command = continue_response
            .request_id(self.request_id.clone())
            .build()
            .unwrap();
        self.send(command)
            .await
            .inspect_err(|_| self.interception.release())
    }

    /// Escape hatch for the parts of `Fetch.fulfillRequest` the neutral response
    /// does not cover, such as `binaryResponseHeaders`.
    ///
    /// Errors when the builder is missing the response code that
    /// `Fetch.fulfillRequest` requires.
    pub async fn respond_with_builder(
        &self,
        fulfill_request: FulfillRequestBuilder,
    ) -> Result<(), InterceptionError> {
        let command = fulfill_request
            .request_id(self.request_id.clone())
            .build()
            .map_err(InterceptionError::Protocol)?;
        self.interception.claim()?;
        self.send(command)
            .await
            .inspect_err(|_| self.interception.release())
    }

    pub async fn continue_with_auth(
        &self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<(), InterceptionError> {
        let challenge_response = AuthChallengeResponse::builder()
            .response(AuthChallengeResponseResponse::ProvideCredentials)
            .username(username)
            .password(password)
            .build()
            .unwrap();
        self.send_auth(challenge_response).await
    }

    pub async fn continue_with_no_auth(
        &self,
        response: AuthChallengeResponseResponse,
    ) -> Result<(), InterceptionError> {
        let challenge_response = AuthChallengeResponse::builder()
            .response(response)
            .build()
            .unwrap();
        self.send_auth(challenge_response).await
    }

    async fn send_auth(
        &self,
        challenge_response: AuthChallengeResponse,
    ) -> Result<(), InterceptionError> {
        let command = ContinueWithAuth::builder()
            .request_id(self.request_id.clone())
            .auth_challenge_response(challenge_response)
            .build()
            .unwrap();
        self.send(command).await
    }

    /// Only valid for a request paused at the response stage — Chrome answers
    /// anything else with "Can only get response body on HeadersReceived pattern
    /// matched requests".
    pub async fn response_body(&self) -> Result<GetResponseBodyResult, CdpCommandResultError> {
        let command = GetResponseBody::builder()
            .request_id(self.request_id.clone())
            .build()
            .unwrap();
        let rx = {
            let mut session = self.session.lock().await;
            session
                .send_and_get_receiver_for(command, self.session_id.clone())
                .await
        };
        let result_value = CdpSession::<T>::await_response(rx)
            .await
            .map_err(CdpCommandResultError::SessionSendError)?
            .result;

        GetResponseBodyResult::try_from(result_value.clone())
            .map_err(|_| CdpCommandResultError::InvalidResultTypeError(result_value))
    }
}

impl<T: ConnectionTransport + Send + Sync> HttpRequest for CdpHttpRequest<T> {
    fn url(&self) -> &str {
        &self.request.url
    }

    fn method(&self) -> &str {
        &self.request.method
    }

    fn headers(&self) -> HashMap<String, String> {
        self.request
            .headers
            .inner()
            .as_object()
            .map(|headers| {
                headers
                    .iter()
                    .filter_map(|(name, value)| {
                        Some((name.to_lowercase(), value.as_str()?.to_string()))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Chrome still sends the deprecated `postData` field that the generated
    /// bindings drop, so read that first and fall back to decoding the
    /// `postDataEntries` that replaced it.
    fn post_data(&self) -> Option<String> {
        if let Some(data) = self
            .raw
            .get("request")
            .and_then(|request| request.get("postData"))
            .and_then(|data| data.as_str())
        {
            return Some(data.to_string());
        }

        let entries = self.request.post_data_entries.as_ref()?;
        let mut data = String::new();
        for entry in entries {
            let bytes = entry.bytes.as_ref()?;
            let decoded = general_purpose::STANDARD.decode(bytes).ok()?;
            data.push_str(std::str::from_utf8(&decoded).ok()?);
        }
        Some(data)
    }

    fn has_post_data(&self) -> bool {
        self.request.has_post_data.unwrap_or(false)
    }

    fn header(&self, name: &str) -> Option<String> {
        self.request
            .headers
            .inner()
            .as_object()?
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .and_then(|(_, value)| value.as_str())
            .map(str::to_string)
    }

    fn resource_type(&self) -> Option<ResourceType> {
        Some(resource_type(&self.cdp_resource_type))
    }

    fn interception(&self) -> &Interception {
        &self.interception
    }

    async fn continue_impl(
        &self,
        overrides: ContinueRequestOverrides,
    ) -> Result<(), InterceptionError> {
        let mut builder = ContinueRequest::builder().request_id(self.request_id.clone());
        if let Some(url) = overrides.url {
            builder = builder.url(url);
        }
        if let Some(method) = overrides.method {
            builder = builder.method(method);
        }
        if let Some(post_data) = overrides.post_data {
            builder = builder.post_data(general_purpose::STANDARD.encode(post_data));
        }
        if let Some(headers) = overrides.headers {
            builder = builder.headers(header_entries(&headers));
        }
        self.send(builder.build().unwrap()).await
    }

    async fn respond_impl(&self, response: ResponseForRequest) -> Result<(), InterceptionError> {
        let builder = FulfillRequestBuilder::default()
            .request_id(self.request_id.clone())
            .response_code(response.status as i64)
            .response_headers(header_entries(&response.normalized_headers()))
            .body(general_purpose::STANDARD.encode(&response.body));
        self.send(builder.build().unwrap()).await
    }

    async fn abort_impl(&self, error: ErrorCode) -> Result<(), InterceptionError> {
        let command = FailRequest::builder()
            .request_id(self.request_id.clone())
            .error_reason(error_reason(error))
            .build()
            .unwrap();
        self.send(command).await
    }
}
