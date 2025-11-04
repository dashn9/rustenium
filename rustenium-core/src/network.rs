use crate::error::SessionSendError;
use crate::transport::ConnectionTransport;
use crate::Session;
use rustenium_bidi_commands::network::commands::{
    ContinueRequest, ContinueRequestParameters, FailRequest, NetworkFailRequestMethod,
    FailRequestParameters, NetworkContinueRequestMethod, ProvideResponse, NetworkProvideResponseMethod,
    ProvideResponseParameters,
};
use rustenium_bidi_commands::network::types::BeforeRequestSentParameters;
use rustenium_bidi_commands::network::types::{BytesValue, CookieHeader, SetCookieHeader, Header};
use rustenium_bidi_commands::{CommandData, NetworkCommand};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Represents a network request that can be intercepted
pub struct NetworkRequest<T: ConnectionTransport> {
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

    /// Continue the request without modifications
    pub async fn continue_(&self) -> Result<(), SessionSendError> {
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

        self.session.lock().await.send(command).await?;
        Ok(())
    }

    /// Continue the request with modifications
    pub async fn continue_with(
        &self,
        headers: Option<Vec<Header>>,
        cookies: Option<Vec<CookieHeader>>,
        url: Option<String>,
        method: Option<String>,
        body: Option<BytesValue>,
    ) -> Result<(), SessionSendError> {
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

        self.session.lock().await.send(command).await?;
        Ok(())
    }

    /// Abort/fail the request
    pub async fn abort(&self) -> Result<(), SessionSendError> {
        let command = CommandData::NetworkCommand(NetworkCommand::FailRequest(FailRequest {
            method: NetworkFailRequestMethod::NetworkFailRequest,
            params: FailRequestParameters {
                request: self.params.base_parameters.request.request.clone(),
            },
        }));

        self.session.lock().await.send(command).await?;
        Ok(())
    }

    /// Provide a custom response
    pub async fn respond(
        &self,
        status_code: Option<u64>,
        reason_phrase: Option<String>,
        headers: Option<Vec<Header>>,
        cookies: Option<Vec<SetCookieHeader>>,
        body: Option<BytesValue>,
    ) -> Result<(), SessionSendError> {
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

        self.session.lock().await.send(command).await?;
        Ok(())
    }
}
