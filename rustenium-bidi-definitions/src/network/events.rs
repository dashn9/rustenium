use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRequiredParams {
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub context: Option<crate::browsing_context::types::BrowsingContext>,
    #[serde(rename = "isBlocked")]
    pub is_blocked: bool,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<crate::browsing_context::types::Navigation>,
    #[serde(rename = "redirectCount")]
    pub redirect_count: u64,
    #[serde(rename = "request")]
    pub request: super::types::RequestData,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "intercepts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub intercepts: Option<Vec<super::types::Intercept>>,
    #[serde(rename = "response")]
    pub response: super::types::ResponseData,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthRequiredMethod {
    #[serde(rename = "network.authRequired")]
    AuthRequired,
}
#[derive(Debug, Clone, PartialEq)]
pub struct AuthRequired {
    pub method: AuthRequiredMethod,
    pub params: AuthRequiredParams,
}
impl AuthRequired {
    pub const IDENTIFIER: &'static str = "network.authRequired";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRequiredParametersParams {
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub context: Option<crate::browsing_context::types::BrowsingContext>,
    #[serde(rename = "isBlocked")]
    pub is_blocked: bool,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<crate::browsing_context::types::Navigation>,
    #[serde(rename = "redirectCount")]
    pub redirect_count: u64,
    #[serde(rename = "request")]
    pub request: super::types::RequestData,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "intercepts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub intercepts: Option<Vec<super::types::Intercept>>,
    #[serde(rename = "initiator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub initiator: Option<super::types::Initiator>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthRequiredParametersMethod {
    #[serde(rename = "network.beforeRequestSent")]
    AuthRequiredParameters,
}
#[derive(Debug, Clone, PartialEq)]
pub struct AuthRequiredParameters {
    pub method: AuthRequiredParametersMethod,
    pub params: AuthRequiredParametersParams,
}
impl AuthRequiredParameters {
    pub const IDENTIFIER: &'static str = "network.beforeRequestSent";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeforeRequestSentParametersParams {
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub context: Option<crate::browsing_context::types::BrowsingContext>,
    #[serde(rename = "isBlocked")]
    pub is_blocked: bool,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<crate::browsing_context::types::Navigation>,
    #[serde(rename = "redirectCount")]
    pub redirect_count: u64,
    #[serde(rename = "request")]
    pub request: super::types::RequestData,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "intercepts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub intercepts: Option<Vec<super::types::Intercept>>,
    #[serde(rename = "errorText")]
    pub error_text: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BeforeRequestSentParametersMethod {
    #[serde(rename = "network.fetchError")]
    BeforeRequestSentParameters,
}
#[derive(Debug, Clone, PartialEq)]
pub struct BeforeRequestSentParameters {
    pub method: BeforeRequestSentParametersMethod,
    pub params: BeforeRequestSentParametersParams,
}
impl BeforeRequestSentParameters {
    pub const IDENTIFIER: &'static str = "network.fetchError";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FetchErrorParametersParams {
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub context: Option<crate::browsing_context::types::BrowsingContext>,
    #[serde(rename = "isBlocked")]
    pub is_blocked: bool,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<crate::browsing_context::types::Navigation>,
    #[serde(rename = "redirectCount")]
    pub redirect_count: u64,
    #[serde(rename = "request")]
    pub request: super::types::RequestData,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "intercepts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub intercepts: Option<Vec<super::types::Intercept>>,
    #[serde(rename = "response")]
    pub response: super::types::ResponseData,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FetchErrorParametersMethod {
    #[serde(rename = "network.responseCompleted")]
    FetchErrorParameters,
}
#[derive(Debug, Clone, PartialEq)]
pub struct FetchErrorParameters {
    pub method: FetchErrorParametersMethod,
    pub params: FetchErrorParametersParams,
}
impl FetchErrorParameters {
    pub const IDENTIFIER: &'static str = "network.responseCompleted";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseCompletedParametersParams {
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub context: Option<crate::browsing_context::types::BrowsingContext>,
    #[serde(rename = "isBlocked")]
    pub is_blocked: bool,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<crate::browsing_context::types::Navigation>,
    #[serde(rename = "redirectCount")]
    pub redirect_count: u64,
    #[serde(rename = "request")]
    pub request: super::types::RequestData,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "intercepts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub intercepts: Option<Vec<super::types::Intercept>>,
    #[serde(rename = "response")]
    pub response: super::types::ResponseData,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResponseCompletedParametersMethod {
    #[serde(rename = "network.responseStarted")]
    ResponseCompletedParameters,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ResponseCompletedParameters {
    pub method: ResponseCompletedParametersMethod,
    pub params: ResponseCompletedParametersParams,
}
impl ResponseCompletedParameters {
    pub const IDENTIFIER: &'static str = "network.responseStarted";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
group_enum ! (NetworkEvents { AuthRequired (AuthRequired) , AuthRequiredParameters (AuthRequiredParameters) , BeforeRequestSentParameters (BeforeRequestSentParameters) , FetchErrorParameters (FetchErrorParameters) , ResponseCompletedParameters (ResponseCompletedParameters) });
