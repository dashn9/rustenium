// Generated events for module

use serde::{Serialize, Deserialize};
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkEvent {
    AuthRequired(AuthRequired),
    BeforeRequestSent(BeforeRequestSent),
    FetchError(FetchError),
    ResponseCompleted(ResponseCompleted),
    ResponseStarted(ResponseStarted),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkAuthRequiredMethod {
    #[serde(rename = "network.AuthRequired")]
    NetworkAuthRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkBeforeRequestSentMethod {
    #[serde(rename = "network.BeforeRequestSent")]
    NetworkBeforeRequestSent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkFetchErrorMethod {
    #[serde(rename = "network.FetchError")]
    NetworkFetchError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkResponseCompletedMethod {
    #[serde(rename = "network.ResponseCompleted")]
    NetworkResponseCompleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkResponseStartedMethod {
    #[serde(rename = "network.ResponseStarted")]
    NetworkResponseStarted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequired {
    #[serde(rename = "method")]
    pub method: NetworkAuthRequiredMethod,
    #[serde(rename = "params")]
    pub params: AuthRequiredParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeforeRequestSent {
    #[serde(rename = "method")]
    pub method: NetworkBeforeRequestSentMethod,
    #[serde(rename = "params")]
    pub params: BeforeRequestSentParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchError {
    #[serde(rename = "method")]
    pub method: NetworkFetchErrorMethod,
    #[serde(rename = "params")]
    pub params: FetchErrorParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseCompleted {
    #[serde(rename = "method")]
    pub method: NetworkResponseCompletedMethod,
    #[serde(rename = "params")]
    pub params: ResponseCompletedParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStarted {
    #[serde(rename = "method")]
    pub method: NetworkResponseStartedMethod,
    #[serde(rename = "params")]
    pub params: ResponseStartedParameters,
}

