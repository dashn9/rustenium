use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddDataCollectorParams {
    #[serde(rename = "dataTypes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data_types: Vec<super::types::DataType>,
    #[serde(rename = "maxEncodedDataSize")]
    pub max_encoded_data_size: u64,
    #[serde(rename = "collectorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub collector_type: Option<super::types::CollectorType>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl AddDataCollectorParams {
    pub fn new(
        data_types: Vec<super::types::DataType>,
        max_encoded_data_size: impl Into<u64>,
    ) -> Self {
        Self {
            data_types,
            max_encoded_data_size: max_encoded_data_size.into(),
            collector_type: None,
            contexts: None,
            user_contexts: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddDataCollectorMethod {
    #[serde(rename = "network.addDataCollector")]
    AddDataCollector,
}
impl AddDataCollectorMethod {
    pub const IDENTIFIER: &'static str = "network.addDataCollector";
}
#[derive(Debug, Clone, PartialEq)]
pub struct AddDataCollector {
    pub method: AddDataCollectorMethod,
    pub params: AddDataCollectorParams,
}
impl crate::CommandResult for AddDataCollector {
    type Result = super::results::AddDataCollectorResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddInterceptParams {
    #[serde(rename = "phases")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub phases: Vec<super::types::InterceptPhase>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    #[serde(rename = "urlPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url_patterns: Option<Vec<super::types::UrlPattern>>,
}
impl AddInterceptParams {
    pub fn new(phases: Vec<super::types::InterceptPhase>) -> Self {
        Self {
            phases,
            contexts: None,
            url_patterns: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddInterceptMethod {
    #[serde(rename = "network.addIntercept")]
    AddIntercept,
}
impl AddInterceptMethod {
    pub const IDENTIFIER: &'static str = "network.addIntercept";
}
#[derive(Debug, Clone, PartialEq)]
pub struct AddIntercept {
    pub method: AddInterceptMethod,
    pub params: AddInterceptParams,
}
impl crate::CommandResult for AddIntercept {
    type Result = super::results::AddInterceptResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueRequestParams {
    #[serde(rename = "request")]
    pub request: super::types::Request,
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub body: Option<super::types::BytesValue>,
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cookies: Option<Vec<super::types::CookieHeader>>,
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub headers: Option<Vec<super::types::Header>>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub method: Option<String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
}
impl ContinueRequestParams {
    pub fn new(request: impl Into<super::types::Request>) -> Self {
        Self {
            request: request.into(),
            body: None,
            cookies: None,
            headers: None,
            method: None,
            url: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContinueRequestMethod {
    #[serde(rename = "network.continueRequest")]
    ContinueRequest,
}
impl ContinueRequestMethod {
    pub const IDENTIFIER: &'static str = "network.continueRequest";
}
#[derive(Debug, Clone, PartialEq)]
pub struct ContinueRequest {
    pub method: ContinueRequestMethod,
    pub params: ContinueRequestParams,
}
impl crate::CommandResult for ContinueRequest {
    type Result = super::results::ContinueRequestResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueResponseParams {
    #[serde(rename = "request")]
    pub request: super::types::Request,
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cookies: Option<Vec<super::types::SetCookieHeader>>,
    #[serde(rename = "credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub credentials: Option<super::types::AuthCredentials>,
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub headers: Option<Vec<super::types::Header>>,
    #[serde(rename = "reasonPhrase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reason_phrase: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_code: Option<u64>,
}
impl ContinueResponseParams {
    pub fn new(request: impl Into<super::types::Request>) -> Self {
        Self {
            request: request.into(),
            cookies: None,
            credentials: None,
            headers: None,
            reason_phrase: None,
            status_code: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContinueResponseMethod {
    #[serde(rename = "network.continueResponse")]
    ContinueResponse,
}
impl ContinueResponseMethod {
    pub const IDENTIFIER: &'static str = "network.continueResponse";
}
#[derive(Debug, Clone, PartialEq)]
pub struct ContinueResponse {
    pub method: ContinueResponseMethod,
    pub params: ContinueResponseParams,
}
impl crate::CommandResult for ContinueResponse {
    type Result = super::results::ContinueResponseResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueWithAuthParams {
    #[serde(rename = "request")]
    pub request: super::types::Request,
}
impl ContinueWithAuthParams {
    pub fn new(request: impl Into<super::types::Request>) -> Self {
        Self {
            request: request.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContinueWithAuthMethod {
    #[serde(rename = "network.continueWithAuth")]
    ContinueWithAuth,
}
impl ContinueWithAuthMethod {
    pub const IDENTIFIER: &'static str = "network.continueWithAuth";
}
#[derive(Debug, Clone, PartialEq)]
pub struct ContinueWithAuth {
    pub method: ContinueWithAuthMethod,
    pub params: ContinueWithAuthParams,
}
impl crate::CommandResult for ContinueWithAuth {
    type Result = super::results::ContinueWithAuthResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisownDataParams {
    #[serde(rename = "dataType")]
    pub data_type: super::types::DataType,
    #[serde(rename = "collector")]
    pub collector: super::types::Collector,
    #[serde(rename = "request")]
    pub request: super::types::Request,
}
impl DisownDataParams {
    pub fn new(
        data_type: impl Into<super::types::DataType>,
        collector: impl Into<super::types::Collector>,
        request: impl Into<super::types::Request>,
    ) -> Self {
        Self {
            data_type: data_type.into(),
            collector: collector.into(),
            request: request.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisownDataMethod {
    #[serde(rename = "network.disownData")]
    DisownData,
}
impl DisownDataMethod {
    pub const IDENTIFIER: &'static str = "network.disownData";
}
#[derive(Debug, Clone, PartialEq)]
pub struct DisownData {
    pub method: DisownDataMethod,
    pub params: DisownDataParams,
}
impl crate::CommandResult for DisownData {
    type Result = super::results::DisownDataResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FailRequestParams {
    #[serde(rename = "request")]
    pub request: super::types::Request,
}
impl FailRequestParams {
    pub fn new(request: impl Into<super::types::Request>) -> Self {
        Self {
            request: request.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FailRequestMethod {
    #[serde(rename = "network.failRequest")]
    FailRequest,
}
impl FailRequestMethod {
    pub const IDENTIFIER: &'static str = "network.failRequest";
}
#[derive(Debug, Clone, PartialEq)]
pub struct FailRequest {
    pub method: FailRequestMethod,
    pub params: FailRequestParams,
}
impl crate::CommandResult for FailRequest {
    type Result = super::results::FailRequestResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDataParams {
    #[serde(rename = "dataType")]
    pub data_type: super::types::DataType,
    #[serde(rename = "collector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub collector: Option<super::types::Collector>,
    #[serde(rename = "disown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_get_data_params_disown")]
    pub disown: Option<bool>,
    #[serde(rename = "request")]
    pub request: super::types::Request,
}
fn default_get_data_params_disown() -> Option<bool> {
    Some(false)
}
impl GetDataParams {
    pub fn new(
        data_type: impl Into<super::types::DataType>,
        request: impl Into<super::types::Request>,
    ) -> Self {
        Self {
            data_type: data_type.into(),
            request: request.into(),
            collector: None,
            disown: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetDataMethod {
    #[serde(rename = "network.getData")]
    GetData,
}
impl GetDataMethod {
    pub const IDENTIFIER: &'static str = "network.getData";
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetData {
    pub method: GetDataMethod,
    pub params: GetDataParams,
}
impl crate::CommandResult for GetData {
    type Result = super::results::GetDataResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProvideResponseParams {
    #[serde(rename = "request")]
    pub request: super::types::Request,
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub body: Option<super::types::BytesValue>,
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cookies: Option<Vec<super::types::SetCookieHeader>>,
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub headers: Option<Vec<super::types::Header>>,
    #[serde(rename = "reasonPhrase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reason_phrase: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_code: Option<u64>,
}
impl ProvideResponseParams {
    pub fn new(request: impl Into<super::types::Request>) -> Self {
        Self {
            request: request.into(),
            body: None,
            cookies: None,
            headers: None,
            reason_phrase: None,
            status_code: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProvideResponseMethod {
    #[serde(rename = "network.provideResponse")]
    ProvideResponse,
}
impl ProvideResponseMethod {
    pub const IDENTIFIER: &'static str = "network.provideResponse";
}
#[derive(Debug, Clone, PartialEq)]
pub struct ProvideResponse {
    pub method: ProvideResponseMethod,
    pub params: ProvideResponseParams,
}
impl crate::CommandResult for ProvideResponse {
    type Result = super::results::ProvideResponseResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveDataCollectorParams {
    #[serde(rename = "collector")]
    pub collector: super::types::Collector,
}
impl RemoveDataCollectorParams {
    pub fn new(collector: impl Into<super::types::Collector>) -> Self {
        Self {
            collector: collector.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveDataCollectorMethod {
    #[serde(rename = "network.removeDataCollector")]
    RemoveDataCollector,
}
impl RemoveDataCollectorMethod {
    pub const IDENTIFIER: &'static str = "network.removeDataCollector";
}
#[derive(Debug, Clone, PartialEq)]
pub struct RemoveDataCollector {
    pub method: RemoveDataCollectorMethod,
    pub params: RemoveDataCollectorParams,
}
impl crate::CommandResult for RemoveDataCollector {
    type Result = super::results::RemoveDataCollectorResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveInterceptParams {
    #[serde(rename = "intercept")]
    pub intercept: super::types::Intercept,
}
impl RemoveInterceptParams {
    pub fn new(intercept: impl Into<super::types::Intercept>) -> Self {
        Self {
            intercept: intercept.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveInterceptMethod {
    #[serde(rename = "network.removeIntercept")]
    RemoveIntercept,
}
impl RemoveInterceptMethod {
    pub const IDENTIFIER: &'static str = "network.removeIntercept";
}
#[derive(Debug, Clone, PartialEq)]
pub struct RemoveIntercept {
    pub method: RemoveInterceptMethod,
    pub params: RemoveInterceptParams,
}
impl crate::CommandResult for RemoveIntercept {
    type Result = super::results::RemoveInterceptResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCacheBehaviorParams {
    #[serde(rename = "cacheBehavior")]
    pub cache_behavior: SetCacheBehaviorCacheBehavior,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetCacheBehaviorCacheBehavior {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "bypass")]
    Bypass,
}
impl SetCacheBehaviorParams {
    pub fn new(cache_behavior: impl Into<SetCacheBehaviorCacheBehavior>) -> Self {
        Self {
            cache_behavior: cache_behavior.into(),
            contexts: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetCacheBehaviorMethod {
    #[serde(rename = "network.setCacheBehavior")]
    SetCacheBehavior,
}
impl SetCacheBehaviorMethod {
    pub const IDENTIFIER: &'static str = "network.setCacheBehavior";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetCacheBehavior {
    pub method: SetCacheBehaviorMethod,
    pub params: SetCacheBehaviorParams,
}
impl crate::CommandResult for SetCacheBehavior {
    type Result = super::results::SetCacheBehaviorResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetExtraHeadersParams {
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<super::types::Header>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl SetExtraHeadersParams {
    pub fn new(headers: Vec<super::types::Header>) -> Self {
        Self {
            headers,
            contexts: None,
            user_contexts: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetExtraHeadersMethod {
    #[serde(rename = "network.setExtraHeaders")]
    SetExtraHeaders,
}
impl SetExtraHeadersMethod {
    pub const IDENTIFIER: &'static str = "network.setExtraHeaders";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetExtraHeaders {
    pub method: SetExtraHeadersMethod,
    pub params: SetExtraHeadersParams,
}
impl crate::CommandResult for SetExtraHeaders {
    type Result = super::results::SetExtraHeadersResult;
}
group_enum ! (NetworkCommands { AddDataCollector (AddDataCollector) , AddIntercept (AddIntercept) , ContinueRequest (ContinueRequest) , ContinueResponse (ContinueResponse) , ContinueWithAuth (ContinueWithAuth) , DisownData (DisownData) , FailRequest (FailRequest) , GetData (GetData) , ProvideResponse (ProvideResponse) , RemoveDataCollector (RemoveDataCollector) , RemoveIntercept (RemoveIntercept) , SetCacheBehavior (SetCacheBehavior) , SetExtraHeaders (SetExtraHeaders) });
