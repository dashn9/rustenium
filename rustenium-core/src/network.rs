use crate::error::{PostDataError, SessionSendError};
use crate::transport::ConnectionTransport;
use crate::{CommandResponseState, Session};
use form_urlencoded;
use tokio::sync::oneshot;
use rustenium_bidi_commands::network::commands::{
    ContinueRequest, ContinueRequestParameters, FailRequest, NetworkFailRequestMethod,
    FailRequestParameters, NetworkContinueRequestMethod, ProvideResponse, NetworkProvideResponseMethod,
    ProvideResponseParameters,
};
use rustenium_bidi_commands::network::types::BeforeRequestSentParameters;
use rustenium_bidi_commands::network::types::{BytesValue, CookieHeader, SetCookieHeader, Header};
use rustenium_bidi_commands::{CommandData, NetworkCommand};
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Represents a network request that can be intercepted
pub struct  NetworkRequest<T: ConnectionTransport> {
    pub params: BeforeRequestSentParameters,
    session: Arc<Mutex<Session<T>>>,
}

impl<T: ConnectionTransport> std::fmt::Debug for NetworkRequest<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NetworkRequest")
            .field("params", &self.params)
            .field("session", &"<Arc<Mutex<Session>>>")
            .finish()
    }
}

impl<T: ConnectionTransport> NetworkRequest<T> {
    pub fn new(params: BeforeRequestSentParameters, session: Arc<Mutex<Session<T>>>) -> Self {
        NetworkRequest { params, session }
    }

    /// Get the request ID
    pub fn request_id(&self) -> &str {
        &self.params.base_parameters.request.request
    }

    /// Get the request URL
    pub fn url(&self) -> &str {
        &self.params.base_parameters.request.url
    }

    /// Get the request headers
    pub fn headers(&self) -> &Vec<Header> {
        &self.params.base_parameters.request.headers
    }

    /// Get the request method
    pub fn method(&self) -> &str {
        &self.params.base_parameters.request.method
    }

    /// Check if the request has POST data (Chrome-specific)
    pub fn has_post_data(&self) -> bool {
        self.params
            .base_parameters
            .request
            .extensible
            .get("goog:hasPostData")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Get the POST data as a raw string if available (Chrome-specific)
    pub fn post_data(&self) -> Option<&str> {
        self.params
            .base_parameters
            .request
            .extensible
            .get("goog:postData")
            .and_then(|v| v.as_str())
    }

    /// Parse the POST data as JSON (Chrome-specific)
    /// Returns an error if POST data is missing, not valid JSON, or not a JSON object
    pub fn post_data_json(&self) -> Result<serde_json::Map<String, serde_json::Value>, PostDataError> {
        let data = self.post_data()
            .ok_or(PostDataError::NoPostData)?;

        let value: serde_json::Value = serde_json::from_str(data)?;

        value.as_object()
            .cloned()
            .ok_or(PostDataError::NotJsonObject)
    }

    /// Parse the POST data as URL-encoded form data (Chrome-specific)
    /// Returns an error if POST data is missing
    pub fn post_data_form(&self) -> Result<HashMap<String, String>, PostDataError> {
        let data = self.post_data()
            .ok_or(PostDataError::NoPostData)?;

        let parsed: HashMap<String, String> = form_urlencoded::parse(data.as_bytes())
            .into_owned()
            .collect();

        Ok(parsed)
    }

    /// Continue the request without modifications
    pub async fn continue_(&self) -> oneshot::Receiver<CommandResponseState> {
        let command =
            CommandData::NetworkCommand(NetworkCommand::ContinueRequest(ContinueRequest {
                method: NetworkContinueRequestMethod::NetworkContinueRequest,
                params: ContinueRequestParameters {
                    request: self.params.base_parameters.request.request.clone(),
                    body: None,
                    cookies: None,
                    headers: None,
                    method: None,
                    url: None,
                },
            }));

        let mut session = self.session.lock().await;
        session.send_and_get_receiver(command).await
    }

    /// Continue the request with modifications
    pub async fn continue_with(
        &self,
        headers: Option<Vec<Header>>,
        cookies: Option<Vec<CookieHeader>>,
        url: Option<String>,
        method: Option<String>,
        body: Option<BytesValue>,
    ) -> oneshot::Receiver<CommandResponseState> {
        let request = self.params.base_parameters.request.request.clone();
        let command =
            CommandData::NetworkCommand(NetworkCommand::ContinueRequest(ContinueRequest {
                method: NetworkContinueRequestMethod::NetworkContinueRequest,
                params: ContinueRequestParameters {
                    request,
                    body,
                    cookies,
                    headers,
                    method,
                    url,
                },
            }));

        let mut session = self.session.lock().await;
        session.send_and_get_receiver(command).await
    }

    /// Abort/fail the request
    pub async fn abort(&self) -> oneshot::Receiver<CommandResponseState> {
        let command = CommandData::NetworkCommand(NetworkCommand::FailRequest(FailRequest {
            method: NetworkFailRequestMethod::NetworkFailRequest,
            params: FailRequestParameters {
                request: self.params.base_parameters.request.request.clone(),
            },
        }));

        let mut session = self.session.lock().await;
        session.send_and_get_receiver(command).await
    }

    /// Provide a custom response
    pub async fn respond(
        &self,
        status_code: Option<u64>,
        reason_phrase: Option<String>,
        headers: Option<Vec<Header>>,
        cookies: Option<Vec<SetCookieHeader>>,
        body: Option<BytesValue>,
    ) -> oneshot::Receiver<CommandResponseState> {
        let command =
            CommandData::NetworkCommand(NetworkCommand::ProvideResponse(ProvideResponse {
                method: NetworkProvideResponseMethod::NetworkProvideResponse,
                params: ProvideResponseParameters {
                    request: self.params.base_parameters.request.request.clone(),
                    body,
                    cookies,
                    headers,
                    reason_phrase,
                    status_code,
                },
            }));

        let mut session = self.session.lock().await;
        session.send_and_get_receiver(command).await
    }
}
