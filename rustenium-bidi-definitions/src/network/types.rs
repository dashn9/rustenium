use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCredentials {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "password")]
    pub password: String,
}
impl AuthCredentials {
    pub fn new(
        r#type: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            username: username.into(),
            password: password.into(),
        }
    }
}
impl AuthCredentials {
    pub const IDENTIFIER: &'static str = "network.AuthCredentials";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BytesValue(serde_json::Value);
impl BytesValue {
    pub fn new(val: impl Into<serde_json::Value>) -> Self {
        BytesValue(val.into())
    }
    pub fn inner(&self) -> &serde_json::Value {
        &self.0
    }
}
impl BytesValue {
    pub const IDENTIFIER: &'static str = "network.BytesValue";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringValue {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl StringValue {
    pub fn new(r#type: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl StringValue {
    pub const IDENTIFIER: &'static str = "network.StringValue";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Base64Value {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl Base64Value {
    pub fn new(r#type: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl Base64Value {
    pub const IDENTIFIER: &'static str = "network.Base64Value";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct Collector(String);
impl Collector {
    pub fn new(val: impl Into<String>) -> Self {
        Collector(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for Collector {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<Collector> for String {
    fn from(el: Collector) -> String {
        el.0
    }
}
impl From<String> for Collector {
    fn from(expr: String) -> Self {
        Collector(expr)
    }
}
impl Collector {
    pub const IDENTIFIER: &'static str = "network.Collector";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct CollectorType(String);
impl CollectorType {
    pub fn new(val: impl Into<String>) -> Self {
        CollectorType(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for CollectorType {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<CollectorType> for String {
    fn from(el: CollectorType) -> String {
        el.0
    }
}
impl From<String> for CollectorType {
    fn from(expr: String) -> Self {
        CollectorType(expr)
    }
}
impl CollectorType {
    pub const IDENTIFIER: &'static str = "network.CollectorType";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub expiry: Option<u64>,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl Cookie {
    pub const IDENTIFIER: &'static str = "network.Cookie";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookieHeader {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: BytesValue,
}
impl CookieHeader {
    pub fn new(name: impl Into<String>, value: impl Into<BytesValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
impl CookieHeader {
    pub const IDENTIFIER: &'static str = "network.CookieHeader";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataType {
    #[serde(rename = "request")]
    Request,
    #[serde(rename = "response")]
    Response,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Header {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: BytesValue,
}
impl Header {
    pub fn new(name: impl Into<String>, value: impl Into<BytesValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
impl Header {
    pub const IDENTIFIER: &'static str = "network.Header";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct Intercept(String);
impl Intercept {
    pub fn new(val: impl Into<String>) -> Self {
        Intercept(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for Intercept {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<Intercept> for String {
    fn from(el: Intercept) -> String {
        el.0
    }
}
impl From<String> for Intercept {
    fn from(expr: String) -> Self {
        Intercept(expr)
    }
}
impl Intercept {
    pub const IDENTIFIER: &'static str = "network.Intercept";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct Request(String);
impl Request {
    pub fn new(val: impl Into<String>) -> Self {
        Request(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for Request {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<Request> for String {
    fn from(el: Request) -> String {
        el.0
    }
}
impl From<String> for Request {
    fn from(expr: String) -> Self {
        Request(expr)
    }
}
impl Request {
    pub const IDENTIFIER: &'static str = "network.Request";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCookieHeader {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: BytesValue,
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(rename = "httpOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub http_only: Option<bool>,
    #[serde(rename = "expiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub expiry: Option<String>,
    #[serde(rename = "maxAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_age: Option<i64>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub path: Option<String>,
    #[serde(rename = "sameSite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub same_site: Option<SameSite>,
    #[serde(rename = "secure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub secure: Option<bool>,
}
impl SetCookieHeader {
    pub fn new(name: impl Into<String>, value: impl Into<BytesValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            domain: None,
            http_only: None,
            expiry: None,
            max_age: None,
            path: None,
            same_site: None,
            secure: None,
        }
    }
}
impl SetCookieHeader {
    pub const IDENTIFIER: &'static str = "network.SetCookieHeader";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UrlPattern {
    UrlPatternPattern(UrlPatternPattern),
    UrlPatternString(UrlPatternString),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrlPatternPattern {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub hostname: Option<String>,
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub port: Option<String>,
    #[serde(rename = "pathname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pathname: Option<String>,
    #[serde(rename = "search")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub search: Option<String>,
}
impl UrlPatternPattern {
    pub fn new(r#type: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            protocol: None,
            hostname: None,
            port: None,
            pathname: None,
            search: None,
        }
    }
}
impl<T: Into<String>> From<T> for UrlPatternPattern {
    fn from(url: T) -> Self {
        UrlPatternPattern::new(url)
    }
}
impl UrlPatternPattern {
    pub const IDENTIFIER: &'static str = "network.UrlPatternPattern";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrlPatternString {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "pattern")]
    pub pattern: String,
}
impl UrlPatternString {
    pub fn new(r#type: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            pattern: pattern.into(),
        }
    }
}
impl UrlPatternString {
    pub const IDENTIFIER: &'static str = "network.UrlPatternString";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InterceptPhase {
    #[serde(rename = "beforeRequestSent")]
    BeforeRequestSent,
    #[serde(rename = "responseStarted")]
    ResponseStarted,
    #[serde(rename = "authRequired")]
    AuthRequired,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueWithAuthCredentials {
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "credentials")]
    pub credentials: AuthCredentials,
}
impl ContinueWithAuthCredentials {
    pub fn new(action: impl Into<String>, credentials: impl Into<AuthCredentials>) -> Self {
        Self {
            action: action.into(),
            credentials: credentials.into(),
        }
    }
}
impl ContinueWithAuthCredentials {
    pub const IDENTIFIER: &'static str = "network.ContinueWithAuthCredentials";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueWithAuthNoCredentials {
    #[serde(rename = "action")]
    pub action: ContinueWithAuthNoCredentialsAction,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContinueWithAuthNoCredentialsAction {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "cancel")]
    Cancel,
}
impl ContinueWithAuthNoCredentials {
    pub fn new(action: impl Into<ContinueWithAuthNoCredentialsAction>) -> Self {
        Self {
            action: action.into(),
        }
    }
}
impl ContinueWithAuthNoCredentials {
    pub const IDENTIFIER: &'static str = "network.ContinueWithAuthNoCredentials";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthChallenge {
    #[serde(rename = "scheme")]
    pub scheme: String,
    #[serde(rename = "realm")]
    pub realm: String,
}
impl AuthChallenge {
    pub fn new(scheme: impl Into<String>, realm: impl Into<String>) -> Self {
        Self {
            scheme: scheme.into(),
            realm: realm.into(),
        }
    }
}
impl AuthChallenge {
    pub const IDENTIFIER: &'static str = "network.AuthChallenge";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseParameters {
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
    pub request: RequestData,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "intercepts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub intercepts: Option<Vec<Intercept>>,
}
impl BaseParameters {
    pub fn new(
        is_blocked: impl Into<bool>,
        redirect_count: impl Into<u64>,
        request: impl Into<RequestData>,
        timestamp: impl Into<u64>,
    ) -> Self {
        Self {
            is_blocked: is_blocked.into(),
            redirect_count: redirect_count.into(),
            request: request.into(),
            timestamp: timestamp.into(),
            context: None,
            navigation: None,
            intercepts: None,
        }
    }
}
impl BaseParameters {
    pub const IDENTIFIER: &'static str = "network.BaseParameters";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
impl FetchTimingInfo {
    pub const IDENTIFIER: &'static str = "network.FetchTimingInfo";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Initiator {
    #[serde(rename = "columnNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub column_number: Option<u64>,
    #[serde(rename = "lineNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub line_number: Option<u64>,
    #[serde(rename = "request")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request: Option<Request>,
    #[serde(rename = "stackTrace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stack_trace: Option<crate::script::types::StackTrace>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub r#type: Option<InitiatorType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InitiatorType {
    #[serde(rename = "parser")]
    Parser,
    #[serde(rename = "script")]
    Script,
    #[serde(rename = "preflight")]
    Preflight,
    #[serde(rename = "other")]
    Other,
}
impl Initiator {
    pub const IDENTIFIER: &'static str = "network.Initiator";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestData {
    #[serde(rename = "request")]
    pub request: Request,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<Header>,
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookies: Vec<Cookie>,
    #[serde(rename = "headersSize")]
    pub headers_size: u64,
    #[serde(rename = "bodySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub body_size: Option<u64>,
    #[serde(rename = "destination")]
    pub destination: String,
    #[serde(rename = "initiatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub initiator_type: Option<String>,
    #[serde(rename = "timings")]
    pub timings: FetchTimingInfo,
}
impl RequestData {
    pub const IDENTIFIER: &'static str = "network.RequestData";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseContent {
    #[serde(rename = "size")]
    pub size: u64,
}
impl ResponseContent {
    pub fn new(size: impl Into<u64>) -> Self {
        Self { size: size.into() }
    }
}
impl ResponseContent {
    pub const IDENTIFIER: &'static str = "network.ResponseContent";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<Header>,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "bytesReceived")]
    pub bytes_received: u64,
    #[serde(rename = "headersSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub headers_size: Option<u64>,
    #[serde(rename = "bodySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub body_size: Option<u64>,
    #[serde(rename = "content")]
    pub content: ResponseContent,
    #[serde(rename = "authChallenges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub auth_challenges: Option<Vec<AuthChallenge>>,
}
impl ResponseData {
    pub const IDENTIFIER: &'static str = "network.ResponseData";
}
group_enum ! (NetworkTypes { AuthCredentials (AuthCredentials) , BytesValue (BytesValue) , StringValue (StringValue) , Base64Value (Base64Value) , Collector (Collector) , CollectorType (CollectorType) , SameSite (SameSite) , Cookie (Cookie) , CookieHeader (CookieHeader) , DataType (DataType) , Header (Header) , Intercept (Intercept) , Request (Request) , SetCookieHeader (SetCookieHeader) , UrlPattern (UrlPattern) , UrlPatternPattern (UrlPatternPattern) , UrlPatternString (UrlPatternString) , InterceptPhase (InterceptPhase) , ContinueWithAuthCredentials (ContinueWithAuthCredentials) , ContinueWithAuthNoCredentials (ContinueWithAuthNoCredentials) , AuthChallenge (AuthChallenge) , BaseParameters (BaseParameters) , FetchTimingInfo (FetchTimingInfo) , Initiator (Initiator) , RequestData (RequestData) , ResponseContent (ResponseContent) , ResponseData (ResponseData) });
