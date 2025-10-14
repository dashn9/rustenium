// Generated types for module

use serde::{Serialize, Deserialize};
use crate::browsing_context::types::BrowsingContext;
use crate::browsing_context::types::Navigation;
use crate::script::types::StackTrace;
use crate::Extensible;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataType {
    #[serde(rename = "response")]
    Response,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CollectorType {
    #[serde(rename = "blob")]
    Blob,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InterceptPhase {
    #[serde(rename = "beforeRequestSent")]
    BeforeRequestSent,
    #[serde(rename = "responseStarted")]
    ResponseStarted,
    #[serde(rename = "authRequired")]
    AuthRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatternEnum {
    #[serde(rename = "pattern")]
    Pattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlPatternPattern {
    #[serde(rename = "type")]
    pub r#type: PatternEnum,
    #[serde(rename = "protocol")]
    pub protocol: Option<String>,
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    #[serde(rename = "port")]
    pub port: Option<String>,
    #[serde(rename = "pathname")]
    pub pathname: Option<String>,
    #[serde(rename = "search")]
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringEnum {
    #[serde(rename = "string")]
    String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlPatternString {
    #[serde(rename = "type")]
    pub r#type: StringEnum,
    #[serde(rename = "pattern")]
    pub pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UrlPattern {
    UrlPatternPattern(UrlPatternPattern),
    UrlPatternString(UrlPatternString),
}

pub type Request = String;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringValue {
    #[serde(rename = "type")]
    pub r#type: StringEnum,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Base64Enum {
    #[serde(rename = "base64")]
    Base64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base64Value {
    #[serde(rename = "type")]
    pub r#type: Base64Enum,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BytesValue {
    StringValue(StringValue),
    Base64Value(Base64Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieHeader {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: BytesValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: BytesValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SameSite {
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "lax")]
    Lax,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "default")]
    Default,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCookieHeader {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: BytesValue,
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    #[serde(rename = "httpOnly")]
    pub http_only: Option<bool>,
    #[serde(rename = "expiry")]
    pub expiry: Option<String>,
    #[serde(rename = "maxAge")]
    pub max_age: Option<i64>,
    #[serde(rename = "path")]
    pub path: Option<String>,
    #[serde(rename = "sameSite")]
    pub same_site: Option<SameSite>,
    #[serde(rename = "secure")]
    pub secure: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PasswordEnum {
    #[serde(rename = "password")]
    Password,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCredentials {
    #[serde(rename = "type")]
    pub r#type: PasswordEnum,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "password")]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProvideCredentialsEnum {
    #[serde(rename = "provideCredentials")]
    ProvideCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueWithAuthCredentials {
    #[serde(rename = "action")]
    pub action: ProvideCredentialsEnum,
    #[serde(rename = "credentials")]
    pub credentials: AuthCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActionUnion {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "cancel")]
    Cancel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueWithAuthNoCredentials {
    #[serde(rename = "action")]
    pub action: ActionUnion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContinueWithAuthCredentialsContinueWithAuthNoCredentialsUnion {
    ContinueWithAuthCredentials(ContinueWithAuthCredentials),
    ContinueWithAuthNoCredentials(ContinueWithAuthNoCredentials),
}

pub type Collector = String;


pub type Intercept = String;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CacheBehaviorUnion {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "bypass")]
    Bypass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: BytesValue,
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "size")]
    pub size: u64,
    #[serde(rename = "httpOnly")]
    pub http_only: bool,
    #[serde(rename = "secure")]
    pub secure: bool,
    #[serde(rename = "sameSite")]
    pub same_site: SameSite,
    #[serde(rename = "expiry")]
    pub expiry: Option<u64>,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchTimingInfo {
    #[serde(rename = "timeOrigin")]
    pub time_origin: f64,
    #[serde(rename = "requestTime")]
    pub request_time: f64,
    #[serde(rename = "redirectStart")]
    pub redirect_start: f64,
    #[serde(rename = "redirectEnd")]
    pub redirect_end: f64,
    #[serde(rename = "fetchStart")]
    pub fetch_start: f64,
    #[serde(rename = "dnsStart")]
    pub dns_start: f64,
    #[serde(rename = "dnsEnd")]
    pub dns_end: f64,
    #[serde(rename = "connectStart")]
    pub connect_start: f64,
    #[serde(rename = "connectEnd")]
    pub connect_end: f64,
    #[serde(rename = "tlsStart")]
    pub tls_start: f64,
    #[serde(rename = "requestStart")]
    pub request_start: f64,
    #[serde(rename = "responseStart")]
    pub response_start: f64,
    #[serde(rename = "responseEnd")]
    pub response_end: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestData {
    #[serde(rename = "request")]
    pub request: Request,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "headers")]
    pub headers: Vec<Header>,
    #[serde(rename = "cookies")]
    pub cookies: Vec<Cookie>,
    #[serde(rename = "headersSize")]
    pub headers_size: u64,
    #[serde(rename = "bodySize")]
    pub body_size: Option<u64>,
    #[serde(rename = "destination")]
    pub destination: String,
    #[serde(rename = "initiatorType")]
    pub initiator_type: Option<String>,
    #[serde(rename = "timings")]
    pub timings: FetchTimingInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseParameters {
    #[serde(rename = "context")]
    pub context: Option<BrowsingContext>,
    #[serde(rename = "isBlocked")]
    pub is_blocked: bool,
    #[serde(rename = "navigation")]
    pub navigation: Option<Navigation>,
    #[serde(rename = "redirectCount")]
    pub redirect_count: u64,
    #[serde(rename = "request")]
    pub request: RequestData,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "intercepts")]
    pub intercepts: Option<Vec<Intercept>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseContent {
    #[serde(rename = "size")]
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthChallenge {
    #[serde(rename = "scheme")]
    pub scheme: String,
    #[serde(rename = "realm")]
    pub realm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseData {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(rename = "status")]
    pub status: u64,
    #[serde(rename = "statusText")]
    pub status_text: String,
    #[serde(rename = "fromCache")]
    pub from_cache: bool,
    #[serde(rename = "headers")]
    pub headers: Vec<Header>,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "bytesReceived")]
    pub bytes_received: u64,
    #[serde(rename = "headersSize")]
    pub headers_size: Option<u64>,
    #[serde(rename = "bodySize")]
    pub body_size: Option<u64>,
    #[serde(rename = "content")]
    pub content: ResponseContent,
    #[serde(rename = "authChallenges")]
    pub auth_challenges: Option<Vec<AuthChallenge>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequiredParameters {
    #[serde(flatten)]
    pub base_parameters: BaseParameters,
    #[serde(rename = "response")]
    pub response: ResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypeUnion {
    #[serde(rename = "parser")]
    Parser,
    #[serde(rename = "script")]
    Script,
    #[serde(rename = "preflight")]
    Preflight,
    #[serde(rename = "other")]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Initiator {
    #[serde(rename = "columnNumber")]
    pub column_number: Option<u64>,
    #[serde(rename = "lineNumber")]
    pub line_number: Option<u64>,
    #[serde(rename = "request")]
    pub request: Option<Request>,
    #[serde(rename = "stackTrace")]
    pub stack_trace: Option<StackTrace>,
    #[serde(rename = "type")]
    pub r#type: Option<TypeUnion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeforeRequestSentParameters {
    #[serde(flatten)]
    pub base_parameters: BaseParameters,
    #[serde(rename = "initiator")]
    pub initiator: Option<Initiator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchErrorParameters {
    #[serde(flatten)]
    pub base_parameters: BaseParameters,
    #[serde(rename = "errorText")]
    pub error_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseCompletedParameters {
    #[serde(flatten)]
    pub base_parameters: BaseParameters,
    #[serde(rename = "response")]
    pub response: ResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStartedParameters {
    #[serde(flatten)]
    pub base_parameters: BaseParameters,
    #[serde(rename = "response")]
    pub response: ResponseData,
}

