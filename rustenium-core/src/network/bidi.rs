use crate::error::{PostDataError, SessionSendError};
use crate::transport::ConnectionTransport;
use crate::{BidiSession, CommandResponseState};
use form_urlencoded;
use rustenium_bidi_definitions::Command;
use rustenium_bidi_definitions::network::command_builders::{
    ContinueRequestBuilder, ContinueWithAuthBuilder, FailRequestBuilder, ProvideResponseBuilder,
};
use rustenium_bidi_definitions::network::events::{AuthRequiredParams, BeforeRequestSentParams};
use rustenium_bidi_definitions::network::type_builders::{
    ContinueWithAuthCredentialsBuilder, ContinueWithAuthNoCredentialsBuilder,
};
use rustenium_bidi_definitions::network::types::{
    AuthCredentials, Base64Value, Base64ValueType, BaseParameters, BytesValue,
    ContinueWithAuthCredentialsAction,
    ContinueWithAuthCredentialsContinueWithAuthNoCredentialsUnion,
    ContinueWithAuthNoCredentialsAction, Header, Request, StringValue, StringValueType,
};
use tokio::sync::oneshot;

use base64::{Engine as _, engine::general_purpose};

use super::NetworkRequestHandledState;
use super::request::{
    ContinueRequestOverrides, ErrorCode, HttpRequest, Interception, InterceptionError,
    ResourceType, ResponseForRequest, handle_protocol_error,
};
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Represents a network request that can be intercepted
pub struct NetworkRequest<T: ConnectionTransport> {
    pub base: BaseParameters,
    session: Arc<Mutex<BidiSession<T>>>,
    interception: Interception,
}

impl<T: ConnectionTransport> std::fmt::Debug for NetworkRequest<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NetworkRequest")
            .field("base", &self.base)
            .field("session", &"<Arc<Mutex<Session>>>")
            .finish()
    }
}

impl<T: ConnectionTransport> NetworkRequest<T> {
    pub fn new(params: BeforeRequestSentParams, session: Arc<Mutex<BidiSession<T>>>) -> Self {
        NetworkRequest {
            base: params.base_parameters,
            session,
            interception: Interception::default(),
        }
    }

    pub fn from_auth_required(
        params: AuthRequiredParams,
        session: Arc<Mutex<BidiSession<T>>>,
    ) -> Self {
        NetworkRequest {
            base: params.base_parameters,
            session,
            interception: Interception::default(),
        }
    }

    /// Get the request ID
    pub fn request_id(&self) -> &Request {
        &self.base.request.request
    }

    /// Get the request URL
    pub fn url(&self) -> &str {
        &self.base.request.url
    }

    /// Get the request headers
    pub fn headers(&self) -> &Vec<Header> {
        &self.base.request.headers
    }

    /// Get the request method
    pub fn method(&self) -> &str {
        &self.base.request.method
    }

    /// Check if the request has POST data (Chrome-specific)
    pub fn has_post_data(&self) -> bool {
        self.base
            .request
            .extensible
            .get("goog:hasPostData")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Get the POST data as a raw string if available (Chrome-specific)
    pub fn post_data(&self) -> Option<&str> {
        self.base
            .request
            .extensible
            .get("goog:postData")
            .and_then(|v| v.as_str())
    }

    /// Parse the POST data as JSON (Chrome-specific)
    /// Returns an error if POST data is missing, not valid JSON, or not a JSON object
    pub fn post_data_json(
        &self,
    ) -> Result<serde_json::Map<String, serde_json::Value>, PostDataError> {
        let data = self.post_data().ok_or(PostDataError::NoPostData)?;

        let value: serde_json::Value = serde_json::from_str(data)?;

        value
            .as_object()
            .cloned()
            .ok_or(PostDataError::NotJsonObject)
    }

    /// Parse the POST data as URL-encoded form data (Chrome-specific)
    /// Returns an error if POST data is missing
    pub fn post_data_form(&self) -> Result<HashMap<String, String>, PostDataError> {
        let data = self.post_data().ok_or(PostDataError::NoPostData)?;

        let parsed: HashMap<String, String> = form_urlencoded::parse(data.as_bytes())
            .into_owned()
            .collect();

        Ok(parsed)
    }

    /// Check if this request has already been handled
    pub async fn is_handled(&self) -> bool {
        self.session
            .lock()
            .await
            .handled_network_requests
            .lock()
            .unwrap()
            .contains_key(self.request_id().as_ref())
    }

    /// Get the handled state if the request was already handled
    pub async fn get_handled_state(&self) -> Option<NetworkRequestHandledState> {
        self.session
            .lock()
            .await
            .handled_network_requests
            .lock()
            .unwrap()
            .get(self.request_id().as_ref())
            .cloned()
    }

    /// Mark this request as handled with the given state
    async fn mark_handled(&self, state: NetworkRequestHandledState) {
        let session = self.session.lock().await;
        session
            .handled_network_requests
            .lock()
            .unwrap()
            .insert(self.base.request.request.clone().into(), state);
    }

    /// Continue the request without modifications
    pub async fn continue_(&self) -> oneshot::Receiver<CommandResponseState> {
        let command: Command = ContinueRequestBuilder::default()
            .request(self.base.request.request.clone())
            .build()
            .unwrap()
            .into(); // This should never result in an error

        let rx = self
            .session
            .lock()
            .await
            .send_and_get_receiver(command)
            .await;
        self.mark_handled(NetworkRequestHandledState::Continued)
            .await;
        rx
    }

    /// Continue the request with modifications
    pub async fn continue_with(
        &self,
        continue_request: ContinueRequestBuilder,
    ) -> oneshot::Receiver<CommandResponseState> {
        let continue_request = continue_request
            .request(self.base.request.request.clone())
            .build()
            .unwrap();
        let rx = self
            .session
            .lock()
            .await
            .send_and_get_receiver(continue_request)
            .await;
        self.mark_handled(NetworkRequestHandledState::Continued)
            .await;
        rx
    }

    /// Abort/fail the request
    pub async fn abort(&self) -> oneshot::Receiver<CommandResponseState> {
        let command = FailRequestBuilder::default()
            .request(self.base.request.request.clone())
            .build()
            .unwrap();

        let rx = self
            .session
            .lock()
            .await
            .send_and_get_receiver(command)
            .await;
        self.mark_handled(NetworkRequestHandledState::Aborted).await;
        rx
    }

    /// Provide a custom response
    pub async fn respond(
        &self,
        provide_response_builder: ProvideResponseBuilder,
    ) -> oneshot::Receiver<CommandResponseState> {
        let command = provide_response_builder
            .request(self.request_id().clone())
            .build()
            .unwrap();

        let rx = self
            .session
            .lock()
            .await
            .send_and_get_receiver(command)
            .await;
        self.mark_handled(NetworkRequestHandledState::Responded)
            .await;
        rx
    }

    /// Continue with HTTP authentication
    pub async fn continue_with_auth(
        &self,
        credentials: AuthCredentials,
    ) -> Result<(), SessionSendError> {
        let command =
                ContinueWithAuthBuilder::default().continue_with_auth_credentials_continue_with_auth_no_credentials_union(ContinueWithAuthCredentialsContinueWithAuthNoCredentialsUnion::ContinueWithAuthCredentials(
                    ContinueWithAuthCredentialsBuilder::default().action(ContinueWithAuthCredentialsAction::ProvideCredentials).credentials(
                        credentials).build().unwrap())).request(self.request_id().clone()).build().unwrap();

        self.session.lock().await.send(command).await.map(|_| ())
    }
    pub async fn continue_with_no_auth(
        &self,
        action: ContinueWithAuthNoCredentialsAction,
    ) -> Result<(), SessionSendError> {
        let command = ContinueWithAuthBuilder::default().continue_with_auth_credentials_continue_with_auth_no_credentials_union(ContinueWithAuthCredentialsContinueWithAuthNoCredentialsUnion::ContinueWithAuthNoCredentials(
                    ContinueWithAuthNoCredentialsBuilder::default().action(action).build().unwrap()
                )).request(self.request_id().clone()).build().unwrap();

        self.session.lock().await.send(command).await.map(|_| ())
    }
}

/// Maps the Fetch destination WebDriver BiDi reports to the CDP-shaped resource
/// type the neutral surface uses.
///
/// Puppeteer gives up here and throws `UnsupportedOperation` unless CDP is also
/// connected. The destination is not the same taxonomy, but it answers the same
/// question closely enough to be worth translating.
fn resource_type(destination: &str) -> ResourceType {
    match destination {
        "document" | "frame" | "iframe" => ResourceType::Document,
        "style" => ResourceType::Stylesheet,
        "image" => ResourceType::Image,
        "audio" | "video" => ResourceType::Media,
        "font" => ResourceType::Font,
        "script" | "serviceworker" | "sharedworker" | "worker" | "audioworklet"
        | "paintworklet" | "xslt" => ResourceType::Script,
        "track" => ResourceType::TextTrack,
        "manifest" => ResourceType::Manifest,
        "report" => ResourceType::CspViolationReport,
        "" => ResourceType::Fetch,
        _ => ResourceType::Other,
    }
}

fn string_header(name: &str, value: &str) -> Header {
    Header::new(name, StringValue::new(StringValueType::String, value))
}

impl<T: ConnectionTransport> NetworkRequest<T> {
    async fn send_intercept(&self, command: impl Into<Command>) -> Result<(), InterceptionError> {
        match self.session.lock().await.send(command).await {
            Ok(_) => Ok(()),
            Err(SessionSendError::ErrorResponse(response)) => {
                handle_protocol_error(response.message)
            }
            Err(error) => Err(InterceptionError::Protocol(error.to_string())),
        }
    }
}

/// The protocol-neutral surface.
///
/// The inherent methods above shadow these on `NetworkRequest` itself, so
/// existing call sites keep the BiDi-shaped return types; generic code written
/// against [`HttpRequest`] sees the neutral ones.
///
/// One caveat: [`HttpRequest::is_handled`] reports whether *this* handle
/// resolved the request, because BiDi builds a fresh `NetworkRequest` per
/// handler. The cross-handler view stays on the inherent async
/// [`NetworkRequest::is_handled`], which consults the session.
impl<T: ConnectionTransport + Send + Sync> HttpRequest for NetworkRequest<T> {
    fn url(&self) -> &str {
        &self.base.request.url
    }

    fn method(&self) -> &str {
        &self.base.request.method
    }

    fn headers(&self) -> HashMap<String, String> {
        self.base
            .request
            .headers
            .iter()
            .map(|header| {
                let value = match &header.value {
                    BytesValue::StringValue(value) => value.value.clone(),
                    BytesValue::Base64Value(value) => value.value.clone(),
                };
                (header.name.to_lowercase(), value)
            })
            .collect()
    }

    fn post_data(&self) -> Option<String> {
        NetworkRequest::post_data(self).map(str::to_string)
    }

    fn has_post_data(&self) -> bool {
        NetworkRequest::has_post_data(self)
    }

    fn header(&self, name: &str) -> Option<String> {
        self.base
            .request
            .headers
            .iter()
            .find(|header| header.name.eq_ignore_ascii_case(name))
            .map(|header| match &header.value {
                BytesValue::StringValue(value) => value.value.clone(),
                BytesValue::Base64Value(value) => value.value.clone(),
            })
    }

    fn resource_type(&self) -> Option<ResourceType> {
        Some(resource_type(&self.base.request.destination))
    }

    fn interception(&self) -> &Interception {
        &self.interception
    }

    async fn continue_impl(
        &self,
        overrides: ContinueRequestOverrides,
    ) -> Result<(), InterceptionError> {
        let mut builder =
            ContinueRequestBuilder::default().request(self.base.request.request.clone());
        if let Some(url) = overrides.url {
            builder = builder.url(url);
        }
        if let Some(method) = overrides.method {
            builder = builder.method(method);
        }
        if let Some(post_data) = overrides.post_data {
            builder = builder.body(Base64Value::new(
                Base64ValueType::Base64,
                general_purpose::STANDARD.encode(post_data),
            ));
        }
        if let Some(headers) = overrides.headers {
            builder = builder.headers(
                headers
                    .iter()
                    .map(|(name, value)| string_header(name, value))
                    .collect::<Vec<_>>(),
            );
        }
        self.send_intercept(builder.build().unwrap()).await
    }

    async fn respond_impl(&self, response: ResponseForRequest) -> Result<(), InterceptionError> {
        let command = ProvideResponseBuilder::default()
            .request(self.base.request.request.clone())
            .status_code(response.status as u64)
            .headers(
                response
                    .normalized_headers()
                    .iter()
                    .map(|(name, value)| string_header(name, value))
                    .collect::<Vec<_>>(),
            )
            .body(Base64Value::new(
                Base64ValueType::Base64,
                general_purpose::STANDARD.encode(&response.body),
            ))
            .build()
            .unwrap();
        self.send_intercept(command).await
    }

    /// WebDriver BiDi's `network.failRequest` carries no reason, so `error` is
    /// only advisory here.
    async fn abort_impl(&self, _error: ErrorCode) -> Result<(), InterceptionError> {
        let command = FailRequestBuilder::default()
            .request(self.base.request.request.clone())
            .build()
            .unwrap();
        self.send_intercept(command).await
    }
}
