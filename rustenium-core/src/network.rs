use crate::error::{PostDataError, SessionSendError};
use crate::transport::ConnectionTransport;
use crate::{BidiSession, CommandResponseState};
use form_urlencoded;
use rustenium_bidi_definitions::Command;
use rustenium_bidi_definitions::network::command_builders::{
    ContinueRequestBuilder, ContinueWithAuthBuilder, FailRequestBuilder, ProvideResponseBuilder
};
use rustenium_bidi_definitions::network::events::{AuthRequiredParams, BeforeRequestSentParams
};
use rustenium_bidi_definitions::network::type_builders::{ContinueWithAuthCredentialsBuilder, ContinueWithAuthNoCredentialsBuilder};
use rustenium_bidi_definitions::network::types::{AuthCredentials, BaseParameters, ContinueWithAuthCredentialsAction, ContinueWithAuthCredentialsContinueWithAuthNoCredentialsUnion, ContinueWithAuthNoCredentialsAction, Header, Request};
use tokio::sync::oneshot;

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
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Represents a network request that can be intercepted
pub struct NetworkRequest<T: ConnectionTransport> {
    pub base: BaseParameters,
    session: Arc<Mutex<BidiSession<T>>>,
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
        }
    }

    pub fn from_auth_required(params: AuthRequiredParams, session: Arc<Mutex<BidiSession<T>>>) -> Self {
        NetworkRequest {
            base: params.base_parameters,
            session,
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
        let continue_request = continue_request.request(self.base.request.request.clone()).build().unwrap();
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
            .build().unwrap();

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
        provide_response_builder: ProvideResponseBuilder
    ) -> oneshot::Receiver<CommandResponseState> {
        let command = provide_response_builder.request(self.request_id().clone()).build().unwrap();

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
    pub async fn continue_with_no_auth(&self, action: ContinueWithAuthNoCredentialsAction) -> Result<(), SessionSendError> {
        let command = ContinueWithAuthBuilder::default().continue_with_auth_credentials_continue_with_auth_no_credentials_union(ContinueWithAuthCredentialsContinueWithAuthNoCredentialsUnion::ContinueWithAuthNoCredentials(
                    ContinueWithAuthNoCredentialsBuilder::default().action(action).build().unwrap()
                )).request(self.request_id().clone()).build().unwrap();

                
        self.session.lock().await.send(command).await.map(|_| ())
    }
}
