use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRequiredParams {
    #[serde(flatten)]
    #[serde(default)]
    pub base_parameters: super::types::BaseParameters,
    #[serde(rename = "response")]
    pub response: super::types::ResponseData,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthRequiredMethod {
    #[serde(rename = "network.authRequired")]
    AuthRequired,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRequired {
    pub method: AuthRequiredMethod,
    pub params: AuthRequiredParams,
}
impl AuthRequired {
    pub const IDENTIFIER: &'static str = "network.authRequired";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeforeRequestSentParams {
    #[serde(flatten)]
    #[serde(default)]
    pub base_parameters: super::types::BaseParameters,
    #[serde(rename = "initiator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub initiator: Option<super::types::Initiator>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BeforeRequestSentMethod {
    #[serde(rename = "network.beforeRequestSent")]
    BeforeRequestSent,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeforeRequestSent {
    pub method: BeforeRequestSentMethod,
    pub params: BeforeRequestSentParams,
}
impl BeforeRequestSent {
    pub const IDENTIFIER: &'static str = "network.beforeRequestSent";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FetchErrorParams {
    #[serde(flatten)]
    #[serde(default)]
    pub base_parameters: super::types::BaseParameters,
    #[serde(rename = "errorText")]
    pub error_text: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FetchErrorMethod {
    #[serde(rename = "network.fetchError")]
    FetchError,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FetchError {
    pub method: FetchErrorMethod,
    pub params: FetchErrorParams,
}
impl FetchError {
    pub const IDENTIFIER: &'static str = "network.fetchError";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseCompletedParams {
    #[serde(flatten)]
    #[serde(default)]
    pub base_parameters: super::types::BaseParameters,
    #[serde(rename = "response")]
    pub response: super::types::ResponseData,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResponseCompletedMethod {
    #[serde(rename = "network.responseCompleted")]
    ResponseCompleted,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseCompleted {
    pub method: ResponseCompletedMethod,
    pub params: ResponseCompletedParams,
}
impl ResponseCompleted {
    pub const IDENTIFIER: &'static str = "network.responseCompleted";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseStartedParams {
    #[serde(flatten)]
    #[serde(default)]
    pub base_parameters: super::types::BaseParameters,
    #[serde(rename = "response")]
    pub response: super::types::ResponseData,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResponseStartedMethod {
    #[serde(rename = "network.responseStarted")]
    ResponseStarted,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseStarted {
    pub method: ResponseStartedMethod,
    pub params: ResponseStartedParams,
}
impl ResponseStarted {
    pub const IDENTIFIER: &'static str = "network.responseStarted";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (NetworkEvent { AuthRequired (AuthRequired) , BeforeRequestSent (BeforeRequestSent) , FetchError (FetchError) , ResponseCompleted (ResponseCompleted) , ResponseStarted (ResponseStarted) });
