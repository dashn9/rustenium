// Generated commands for module

use serde::{Serialize, Deserialize};
use crate::browser::types::UserContext;
use crate::browsing_context::types::BrowsingContext;
use crate::EmptyResult;
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
pub enum NetworkAddDataCollectorMethod {
    #[serde(rename = "network.addDataCollector")]
    NetworkAddDataCollector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkAddInterceptMethod {
    #[serde(rename = "network.addIntercept")]
    NetworkAddIntercept,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkContinueRequestMethod {
    #[serde(rename = "network.continueRequest")]
    NetworkContinueRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkContinueResponseMethod {
    #[serde(rename = "network.continueResponse")]
    NetworkContinueResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkContinueWithAuthMethod {
    #[serde(rename = "network.continueWithAuth")]
    NetworkContinueWithAuth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkDisownDataMethod {
    #[serde(rename = "network.disownData")]
    NetworkDisownData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkFailRequestMethod {
    #[serde(rename = "network.failRequest")]
    NetworkFailRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkGetDataMethod {
    #[serde(rename = "network.getData")]
    NetworkGetData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkProvideResponseMethod {
    #[serde(rename = "network.provideResponse")]
    NetworkProvideResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkRemoveDataCollectorMethod {
    #[serde(rename = "network.removeDataCollector")]
    NetworkRemoveDataCollector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkRemoveInterceptMethod {
    #[serde(rename = "network.removeIntercept")]
    NetworkRemoveIntercept,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkSetCacheBehaviorMethod {
    #[serde(rename = "network.setCacheBehavior")]
    NetworkSetCacheBehavior,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkSetExtraHeadersMethod {
    #[serde(rename = "network.setExtraHeaders")]
    NetworkSetExtraHeaders,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDataCollectorParameters {
    #[serde(rename = "dataTypes")]
    pub data_types: Vec<DataType>,
    #[serde(rename = "maxEncodedDataSize")]
    pub max_encoded_data_size: u64,
    #[serde(rename = "collectorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_type: Option<CollectorType>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddInterceptParameters {
    #[serde(rename = "phases")]
    pub phases: Vec<InterceptPhase>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "urlPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_patterns: Option<Vec<UrlPattern>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueRequestParameters {
    #[serde(rename = "request")]
    pub request: Request,
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<BytesValue>,
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Vec<CookieHeader>>,
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<Header>>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueResponseParameters {
    #[serde(rename = "request")]
    pub request: Request,
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Vec<SetCookieHeader>>,
    #[serde(rename = "credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<AuthCredentials>,
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<Header>>,
    #[serde(rename = "reasonPhrase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_phrase: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
pub struct DisownDataParameters {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDataParameters {
    #[serde(rename = "dataType")]
    pub data_type: DataType,
    #[serde(rename = "collector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector: Option<Collector>,
    #[serde(rename = "disown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disown: Option<bool>,
    #[serde(rename = "request")]
    pub request: Request,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvideResponseParameters {
    #[serde(rename = "request")]
    pub request: Request,
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<BytesValue>,
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Vec<SetCookieHeader>>,
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<Header>>,
    #[serde(rename = "reasonPhrase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_phrase: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetExtraHeadersParameters {
    #[serde(rename = "headers")]
    pub headers: Vec<Header>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub params: DisownDataParameters,
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
    AddDataCollectorResult(AddDataCollectorResult),
    AddInterceptResult(AddInterceptResult),
    ContinueRequestResult(ContinueRequestResult),
    ContinueResponseResult(ContinueResponseResult),
    ContinueWithAuthResult(ContinueWithAuthResult),
    DisownDataResult(DisownDataResult),
    FailRequestResult(FailRequestResult),
    GetDataResult(GetDataResult),
    ProvideResponseResult(ProvideResponseResult),
    RemoveDataCollectorResult(RemoveDataCollectorResult),
    RemoveInterceptResult(RemoveInterceptResult),
    SetCacheBehaviorResult(SetCacheBehaviorResult),
    SetExtraHeadersResult(SetExtraHeadersResult),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDataCollectorResult {
    #[serde(rename = "collector")]
    pub collector: Collector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddInterceptResult {
    #[serde(rename = "intercept")]
    pub intercept: Intercept,
}

pub type ContinueRequestResult = EmptyResult;


pub type ContinueResponseResult = EmptyResult;


pub type ContinueWithAuthResult = EmptyResult;


pub type DisownDataResult = EmptyResult;


pub type FailRequestResult = EmptyResult;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDataResult {
    #[serde(rename = "bytes")]
    pub bytes: BytesValue,
}

pub type ProvideResponseResult = EmptyResult;


pub type RemoveDataCollectorResult = EmptyResult;


pub type RemoveInterceptResult = EmptyResult;


pub type SetCacheBehaviorResult = EmptyResult;


pub type SetExtraHeadersResult = EmptyResult;


