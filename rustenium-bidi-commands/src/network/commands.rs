// Generated commands for module

use serde::{Serialize, Deserialize};
use crate::browser::types::UserContext;
use crate::browsing_context::types::BrowsingContext;
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkCommand {
    AddDataCollector(AddDataCollector),
    AddIntercept(AddIntercept),
    ContinueRequest(ContinueRequest),
    ContinueResponse(ContinueResponse),
    ContinueWithAuth(ContinueWithAuth),
    DisownData(DisownData),
    FailRequest(FailRequest),
    GetData(GetData),
    ProvideResponse(ProvideResponse),
    RemoveDataCollector(RemoveDataCollector),
    RemoveIntercept(RemoveIntercept),
    SetCacheBehavior(SetCacheBehavior),
    SetExtraHeaders(SetExtraHeaders),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkAddDataCollectorMethod {
    #[serde(rename = "network.AddDataCollector")]
    NetworkAddDataCollector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkAddInterceptMethod {
    #[serde(rename = "network.AddIntercept")]
    NetworkAddIntercept,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkContinueRequestMethod {
    #[serde(rename = "network.ContinueRequest")]
    NetworkContinueRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkContinueResponseMethod {
    #[serde(rename = "network.ContinueResponse")]
    NetworkContinueResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkContinueWithAuthMethod {
    #[serde(rename = "network.ContinueWithAuth")]
    NetworkContinueWithAuth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkDisownDataMethod {
    #[serde(rename = "network.DisownData")]
    NetworkDisownData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkFailRequestMethod {
    #[serde(rename = "network.FailRequest")]
    NetworkFailRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkGetDataMethod {
    #[serde(rename = "network.GetData")]
    NetworkGetData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkProvideResponseMethod {
    #[serde(rename = "network.ProvideResponse")]
    NetworkProvideResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkRemoveDataCollectorMethod {
    #[serde(rename = "network.RemoveDataCollector")]
    NetworkRemoveDataCollector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkRemoveInterceptMethod {
    #[serde(rename = "network.RemoveIntercept")]
    NetworkRemoveIntercept,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkSetCacheBehaviorMethod {
    #[serde(rename = "network.SetCacheBehavior")]
    NetworkSetCacheBehavior,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkSetExtraHeadersMethod {
    #[serde(rename = "network.SetExtraHeaders")]
    NetworkSetExtraHeaders,
}

fn add_data_collector_parameters_default_collector_type() -> CollectorType {
    CollectorType::Blob
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDataCollectorParameters {
    #[serde(rename = "dataTypes")]
    pub data_types: Vec<DataType>,
    #[serde(rename = "maxEncodedDataSize")]
    pub max_encoded_data_size: u64,
    #[serde(rename = "collectorType")]
    #[serde(default = "add_data_collector_parameters_default_collector_type")]
    pub collector_type: CollectorType,
    #[serde(rename = "contexts")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddInterceptParameters {
    #[serde(rename = "phases")]
    pub phases: Vec<InterceptPhase>,
    #[serde(rename = "contexts")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "urlPatterns")]
    pub url_patterns: Option<Vec<UrlPattern>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueRequestParameters {
    #[serde(rename = "request")]
    pub request: Request,
    #[serde(rename = "body")]
    pub body: Option<BytesValue>,
    #[serde(rename = "cookies")]
    pub cookies: Option<Vec<CookieHeader>>,
    #[serde(rename = "headers")]
    pub headers: Option<Vec<Header>>,
    #[serde(rename = "method")]
    pub method: Option<String>,
    #[serde(rename = "url")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueResponseParameters {
    #[serde(rename = "request")]
    pub request: Request,
    #[serde(rename = "cookies")]
    pub cookies: Option<Vec<SetCookieHeader>>,
    #[serde(rename = "credentials")]
    pub credentials: Option<AuthCredentials>,
    #[serde(rename = "headers")]
    pub headers: Option<Vec<Header>>,
    #[serde(rename = "reasonPhrase")]
    pub reason_phrase: Option<String>,
    #[serde(rename = "statusCode")]
    pub status_code: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueWithAuthParameters {
    #[serde(rename = "request")]
    pub request: Request,
    #[serde(flatten)]
    pub continue_with_auth_credentials_continue_with_auth_no_credentials_union: ContinueWithAuthCredentialsContinueWithAuthNoCredentialsUnion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct disownDataParameters {
    #[serde(rename = "dataType")]
    pub data_type: DataType,
    #[serde(rename = "collector")]
    pub collector: Collector,
    #[serde(rename = "request")]
    pub request: Request,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailRequestParameters {
    #[serde(rename = "request")]
    pub request: Request,
}

fn get_data_parameters_default_disown() -> bool {
    false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDataParameters {
    #[serde(rename = "dataType")]
    pub data_type: DataType,
    #[serde(rename = "collector")]
    pub collector: Option<Collector>,
    #[serde(rename = "disown")]
    #[serde(default = "get_data_parameters_default_disown")]
    pub disown: bool,
    #[serde(rename = "request")]
    pub request: Request,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvideResponseParameters {
    #[serde(rename = "request")]
    pub request: Request,
    #[serde(rename = "body")]
    pub body: Option<BytesValue>,
    #[serde(rename = "cookies")]
    pub cookies: Option<Vec<SetCookieHeader>>,
    #[serde(rename = "headers")]
    pub headers: Option<Vec<Header>>,
    #[serde(rename = "reasonPhrase")]
    pub reason_phrase: Option<String>,
    #[serde(rename = "statusCode")]
    pub status_code: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveDataCollectorParameters {
    #[serde(rename = "collector")]
    pub collector: Collector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveInterceptParameters {
    #[serde(rename = "intercept")]
    pub intercept: Intercept,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheBehaviorParameters {
    #[serde(rename = "cacheBehavior")]
    pub cache_behavior: CacheBehaviorUnion,
    #[serde(rename = "contexts")]
    pub contexts: Option<Vec<BrowsingContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetExtraHeadersParameters {
    #[serde(rename = "headers")]
    pub headers: Vec<Header>,
    #[serde(rename = "contexts")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDataCollector {
    #[serde(rename = "method")]
    pub method: NetworkAddDataCollectorMethod,
    #[serde(rename = "params")]
    pub params: AddDataCollectorParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddIntercept {
    #[serde(rename = "method")]
    pub method: NetworkAddInterceptMethod,
    #[serde(rename = "params")]
    pub params: AddInterceptParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueRequest {
    #[serde(rename = "method")]
    pub method: NetworkContinueRequestMethod,
    #[serde(rename = "params")]
    pub params: ContinueRequestParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueResponse {
    #[serde(rename = "method")]
    pub method: NetworkContinueResponseMethod,
    #[serde(rename = "params")]
    pub params: ContinueResponseParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueWithAuth {
    #[serde(rename = "method")]
    pub method: NetworkContinueWithAuthMethod,
    #[serde(rename = "params")]
    pub params: ContinueWithAuthParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisownData {
    #[serde(rename = "method")]
    pub method: NetworkDisownDataMethod,
    #[serde(rename = "params")]
    pub params: disownDataParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailRequest {
    #[serde(rename = "method")]
    pub method: NetworkFailRequestMethod,
    #[serde(rename = "params")]
    pub params: FailRequestParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetData {
    #[serde(rename = "method")]
    pub method: NetworkGetDataMethod,
    #[serde(rename = "params")]
    pub params: GetDataParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvideResponse {
    #[serde(rename = "method")]
    pub method: NetworkProvideResponseMethod,
    #[serde(rename = "params")]
    pub params: ProvideResponseParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveDataCollector {
    #[serde(rename = "method")]
    pub method: NetworkRemoveDataCollectorMethod,
    #[serde(rename = "params")]
    pub params: RemoveDataCollectorParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveIntercept {
    #[serde(rename = "method")]
    pub method: NetworkRemoveInterceptMethod,
    #[serde(rename = "params")]
    pub params: RemoveInterceptParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheBehavior {
    #[serde(rename = "method")]
    pub method: NetworkSetCacheBehaviorMethod,
    #[serde(rename = "params")]
    pub params: SetCacheBehaviorParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetExtraHeaders {
    #[serde(rename = "method")]
    pub method: NetworkSetExtraHeadersMethod,
    #[serde(rename = "params")]
    pub params: SetExtraHeadersParameters,
}

// Generated results

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkResult {
    AddInterceptResult(AddInterceptResult),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddInterceptResult {
    #[serde(rename = "intercept")]
    pub intercept: Intercept,
}

