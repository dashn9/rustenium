use serde::{Deserialize, Serialize};
#[doc = "Resource type as it was perceived by the rendering engine."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    #[serde(rename = "Document")]
    Document,
    #[serde(rename = "Stylesheet")]
    Stylesheet,
    #[serde(rename = "Image")]
    Image,
    #[serde(rename = "Media")]
    Media,
    #[serde(rename = "Font")]
    Font,
    #[serde(rename = "Script")]
    Script,
    #[serde(rename = "TextTrack")]
    TextTrack,
    #[serde(rename = "XHR")]
    Xhr,
    #[serde(rename = "Fetch")]
    Fetch,
    #[serde(rename = "Prefetch")]
    Prefetch,
    #[serde(rename = "EventSource")]
    EventSource,
    #[serde(rename = "WebSocket")]
    WebSocket,
    #[serde(rename = "Manifest")]
    Manifest,
    #[serde(rename = "SignedExchange")]
    SignedExchange,
    #[serde(rename = "Ping")]
    Ping,
    #[serde(rename = "CSPViolationReport")]
    CspViolationReport,
    #[serde(rename = "Preflight")]
    Preflight,
    #[serde(rename = "FedCM")]
    FedCm,
    #[serde(rename = "Other")]
    Other,
}
#[doc = "Unique loader identifier.\n[LoaderId](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-LoaderId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct LoaderId(String);
impl LoaderId {
    pub fn new(val: impl Into<String>) -> Self {
        LoaderId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for LoaderId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<LoaderId> for String {
    fn from(el: LoaderId) -> String {
        el.0
    }
}
impl From<String> for LoaderId {
    fn from(expr: String) -> Self {
        LoaderId(expr)
    }
}
impl std::borrow::Borrow<str> for LoaderId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl LoaderId {
    pub const IDENTIFIER: &'static str = "Network.LoaderId";
}
#[doc = "Unique network request identifier.\nNote that this does not identify individual HTTP requests that are part of\na network request.\n[RequestId](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-RequestId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct RequestId(String);
impl RequestId {
    pub fn new(val: impl Into<String>) -> Self {
        RequestId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for RequestId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<RequestId> for String {
    fn from(el: RequestId) -> String {
        el.0
    }
}
impl From<String> for RequestId {
    fn from(expr: String) -> Self {
        RequestId(expr)
    }
}
impl std::borrow::Borrow<str> for RequestId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl RequestId {
    pub const IDENTIFIER: &'static str = "Network.RequestId";
}
#[doc = "Unique intercepted request identifier.\n[InterceptionId](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-InterceptionId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct InterceptionId(String);
impl InterceptionId {
    pub fn new(val: impl Into<String>) -> Self {
        InterceptionId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for InterceptionId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<InterceptionId> for String {
    fn from(el: InterceptionId) -> String {
        el.0
    }
}
impl From<String> for InterceptionId {
    fn from(expr: String) -> Self {
        InterceptionId(expr)
    }
}
impl std::borrow::Borrow<str> for InterceptionId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl InterceptionId {
    pub const IDENTIFIER: &'static str = "Network.InterceptionId";
}
#[doc = "Network level fetch failure reason."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorReason {
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Aborted")]
    Aborted,
    #[serde(rename = "TimedOut")]
    TimedOut,
    #[serde(rename = "AccessDenied")]
    AccessDenied,
    #[serde(rename = "ConnectionClosed")]
    ConnectionClosed,
    #[serde(rename = "ConnectionReset")]
    ConnectionReset,
    #[serde(rename = "ConnectionRefused")]
    ConnectionRefused,
    #[serde(rename = "ConnectionAborted")]
    ConnectionAborted,
    #[serde(rename = "ConnectionFailed")]
    ConnectionFailed,
    #[serde(rename = "NameNotResolved")]
    NameNotResolved,
    #[serde(rename = "InternetDisconnected")]
    InternetDisconnected,
    #[serde(rename = "AddressUnreachable")]
    AddressUnreachable,
    #[serde(rename = "BlockedByClient")]
    BlockedByClient,
    #[serde(rename = "BlockedByResponse")]
    BlockedByResponse,
}
#[doc = "UTC time in seconds, counted from January 1, 1970.\n[TimeSinceEpoch](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-TimeSinceEpoch)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TimeSinceEpoch(f64);
impl TimeSinceEpoch {
    pub fn new(val: impl Into<f64>) -> Self {
        TimeSinceEpoch(val.into())
    }
    pub fn inner(&self) -> &f64 {
        &self.0
    }
}
impl TimeSinceEpoch {
    pub const IDENTIFIER: &'static str = "Network.TimeSinceEpoch";
}
#[doc = "Monotonically increasing time in seconds since an arbitrary point in the past.\n[MonotonicTime](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-MonotonicTime)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MonotonicTime(f64);
impl MonotonicTime {
    pub fn new(val: impl Into<f64>) -> Self {
        MonotonicTime(val.into())
    }
    pub fn inner(&self) -> &f64 {
        &self.0
    }
}
impl MonotonicTime {
    pub const IDENTIFIER: &'static str = "Network.MonotonicTime";
}
#[doc = "Request / response headers as keys / values of JSON object.\n[Headers](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-Headers)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Headers(serde_json::Value);
impl Headers {
    pub fn new(val: impl Into<serde_json::Value>) -> Self {
        Headers(val.into())
    }
    pub fn inner(&self) -> &serde_json::Value {
        &self.0
    }
}
impl Headers {
    pub const IDENTIFIER: &'static str = "Network.Headers";
}
#[doc = "The underlying connection technology that the browser is supposedly using."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConnectionType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "cellular2g")]
    Cellular2g,
    #[serde(rename = "cellular3g")]
    Cellular3g,
    #[serde(rename = "cellular4g")]
    Cellular4g,
    #[serde(rename = "bluetooth")]
    Bluetooth,
    #[serde(rename = "ethernet")]
    Ethernet,
    #[serde(rename = "wifi")]
    Wifi,
    #[serde(rename = "wimax")]
    Wimax,
    #[serde(rename = "other")]
    Other,
}
#[doc = "Represents the cookie's 'SameSite' status:\nhttps://tools.ietf.org/html/draft-west-first-party-cookies"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CookieSameSite {
    #[serde(rename = "Strict")]
    Strict,
    #[serde(rename = "Lax")]
    Lax,
    #[serde(rename = "None")]
    None,
}
#[doc = "Represents the cookie's 'Priority' status:\nhttps://tools.ietf.org/html/draft-west-cookie-priority-00"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CookiePriority {
    #[serde(rename = "Low")]
    Low,
    #[serde(rename = "Medium")]
    Medium,
    #[serde(rename = "High")]
    High,
}
#[doc = "Represents the source scheme of the origin that originally set the cookie.\nA value of \"Unset\" allows protocol clients to emulate legacy cookie scope for the scheme.\nThis is a temporary ability and it will be removed in the future."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CookieSourceScheme {
    #[serde(rename = "Unset")]
    Unset,
    #[serde(rename = "NonSecure")]
    NonSecure,
    #[serde(rename = "Secure")]
    Secure,
}
#[doc = "Timing information for the request.\n[ResourceTiming](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-ResourceTiming)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceTiming {
    #[doc = "Timing's requestTime is a baseline in seconds, while the other numbers are ticks in\nmilliseconds relatively to this requestTime."]
    #[serde(rename = "requestTime")]
    pub request_time: f64,
    #[doc = "Started resolving proxy."]
    #[serde(rename = "proxyStart")]
    pub proxy_start: f64,
    #[doc = "Finished resolving proxy."]
    #[serde(rename = "proxyEnd")]
    pub proxy_end: f64,
    #[doc = "Started DNS address resolve."]
    #[serde(rename = "dnsStart")]
    pub dns_start: f64,
    #[doc = "Finished DNS address resolve."]
    #[serde(rename = "dnsEnd")]
    pub dns_end: f64,
    #[doc = "Started connecting to the remote host."]
    #[serde(rename = "connectStart")]
    pub connect_start: f64,
    #[doc = "Connected to the remote host."]
    #[serde(rename = "connectEnd")]
    pub connect_end: f64,
    #[doc = "Started SSL handshake."]
    #[serde(rename = "sslStart")]
    pub ssl_start: f64,
    #[doc = "Finished SSL handshake."]
    #[serde(rename = "sslEnd")]
    pub ssl_end: f64,
    #[doc = "Started running ServiceWorker."]
    #[serde(rename = "workerStart")]
    pub worker_start: f64,
    #[doc = "Finished Starting ServiceWorker."]
    #[serde(rename = "workerReady")]
    pub worker_ready: f64,
    #[doc = "Started fetch event."]
    #[serde(rename = "workerFetchStart")]
    pub worker_fetch_start: f64,
    #[doc = "Settled fetch event respondWith promise."]
    #[serde(rename = "workerRespondWithSettled")]
    pub worker_respond_with_settled: f64,
    #[doc = "Started ServiceWorker static routing source evaluation."]
    #[serde(rename = "workerRouterEvaluationStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub worker_router_evaluation_start: Option<f64>,
    #[doc = "Started cache lookup when the source was evaluated to `cache`."]
    #[serde(rename = "workerCacheLookupStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub worker_cache_lookup_start: Option<f64>,
    #[doc = "Started sending request."]
    #[serde(rename = "sendStart")]
    pub send_start: f64,
    #[doc = "Finished sending request."]
    #[serde(rename = "sendEnd")]
    pub send_end: f64,
    #[doc = "Time the server started pushing request."]
    #[serde(rename = "pushStart")]
    pub push_start: f64,
    #[doc = "Time the server finished pushing request."]
    #[serde(rename = "pushEnd")]
    pub push_end: f64,
    #[doc = "Started receiving response headers."]
    #[serde(rename = "receiveHeadersStart")]
    pub receive_headers_start: f64,
    #[doc = "Finished receiving response headers."]
    #[serde(rename = "receiveHeadersEnd")]
    pub receive_headers_end: f64,
}
impl ResourceTiming {
    pub const IDENTIFIER: &'static str = "Network.ResourceTiming";
}
#[doc = "Loading priority of a resource request."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourcePriority {
    #[serde(rename = "VeryLow")]
    VeryLow,
    #[serde(rename = "Low")]
    Low,
    #[serde(rename = "Medium")]
    Medium,
    #[serde(rename = "High")]
    High,
    #[serde(rename = "VeryHigh")]
    VeryHigh,
}
#[doc = "The render blocking behavior of a resource request."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RenderBlockingBehavior {
    #[serde(rename = "Blocking")]
    Blocking,
    #[serde(rename = "InBodyParserBlocking")]
    InBodyParserBlocking,
    #[serde(rename = "NonBlocking")]
    NonBlocking,
    #[serde(rename = "NonBlockingDynamic")]
    NonBlockingDynamic,
    #[serde(rename = "PotentiallyBlocking")]
    PotentiallyBlocking,
}
#[doc = "Post data entry for HTTP request\n[PostDataEntry](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-PostDataEntry)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PostDataEntry {
    #[serde(rename = "bytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bytes: Option<super::super::super::Binary>,
}
impl PostDataEntry {
    pub const IDENTIFIER: &'static str = "Network.PostDataEntry";
}
#[doc = "HTTP request data.\n[Request](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-Request)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    #[doc = "Request URL (without fragment)."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Fragment of the requested URL starting with hash, if present."]
    #[serde(rename = "urlFragment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url_fragment: Option<String>,
    #[doc = "HTTP request method."]
    #[serde(rename = "method")]
    pub method: String,
    #[doc = "HTTP request headers."]
    #[serde(rename = "headers")]
    pub headers: Headers,
    #[doc = "True when the request has POST data. Note that postData might still be omitted when this flag is true when the data is too long."]
    #[serde(rename = "hasPostData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub has_post_data: Option<bool>,
    #[doc = "Request body elements (post data broken into individual entries)."]
    #[serde(rename = "postDataEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub post_data_entries: Option<Vec<PostDataEntry>>,
    #[doc = "The mixed content type of the request."]
    #[serde(rename = "mixedContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mixed_content_type: Option<super::super::security::types::MixedContentType>,
    #[doc = "Priority of the resource request at the time request is sent."]
    #[serde(rename = "initialPriority")]
    pub initial_priority: ResourcePriority,
    #[doc = "The referrer policy of the request, as defined in https://www.w3.org/TR/referrer-policy/"]
    #[serde(rename = "referrerPolicy")]
    pub referrer_policy: RequestReferrerPolicy,
    #[doc = "Whether is loaded via link preload."]
    #[serde(rename = "isLinkPreload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_link_preload: Option<bool>,
    #[doc = "Set for requests when the TrustToken API is used. Contains the parameters\npassed by the developer (e.g. via \"fetch\") as understood by the backend."]
    #[serde(rename = "trustTokenParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub trust_token_params: Option<TrustTokenParams>,
    #[doc = "True if this resource request is considered to be the 'same site' as the\nrequest corresponding to the main frame."]
    #[serde(rename = "isSameSite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_same_site: Option<bool>,
    #[doc = "True when the resource request is ad-related."]
    #[serde(rename = "isAdRelated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_ad_related: Option<bool>,
}
#[doc = "The referrer policy of the request, as defined in https://www.w3.org/TR/referrer-policy/"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RequestReferrerPolicy {
    #[serde(rename = "unsafe-url")]
    UnsafeUrl,
    #[serde(rename = "no-referrer-when-downgrade")]
    NoReferrerWhenDowngrade,
    #[serde(rename = "no-referrer")]
    NoReferrer,
    #[serde(rename = "origin")]
    Origin,
    #[serde(rename = "origin-when-cross-origin")]
    OriginWhenCrossOrigin,
    #[serde(rename = "same-origin")]
    SameOrigin,
    #[serde(rename = "strict-origin")]
    StrictOrigin,
    #[serde(rename = "strict-origin-when-cross-origin")]
    StrictOriginWhenCrossOrigin,
}
impl Request {
    pub const IDENTIFIER: &'static str = "Network.Request";
}
#[doc = "Details of a signed certificate timestamp (SCT).\n[SignedCertificateTimestamp](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-SignedCertificateTimestamp)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignedCertificateTimestamp {
    #[doc = "Validation status."]
    #[serde(rename = "status")]
    pub status: String,
    #[doc = "Origin."]
    #[serde(rename = "origin")]
    pub origin: String,
    #[doc = "Log name / description."]
    #[serde(rename = "logDescription")]
    pub log_description: String,
    #[doc = "Log ID."]
    #[serde(rename = "logId")]
    pub log_id: String,
    #[doc = "Issuance date. Unlike TimeSinceEpoch, this contains the number of\nmilliseconds since January 1, 1970, UTC, not the number of seconds."]
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    #[doc = "Hash algorithm."]
    #[serde(rename = "hashAlgorithm")]
    pub hash_algorithm: String,
    #[doc = "Signature algorithm."]
    #[serde(rename = "signatureAlgorithm")]
    pub signature_algorithm: String,
    #[doc = "Signature data."]
    #[serde(rename = "signatureData")]
    pub signature_data: String,
}
impl SignedCertificateTimestamp {
    pub const IDENTIFIER: &'static str = "Network.SignedCertificateTimestamp";
}
#[doc = "Security details about a request.\n[SecurityDetails](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-SecurityDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityDetails {
    #[doc = "Protocol name (e.g. \"TLS 1.2\" or \"QUIC\")."]
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[doc = "Key Exchange used by the connection, or the empty string if not applicable."]
    #[serde(rename = "keyExchange")]
    pub key_exchange: String,
    #[doc = "(EC)DH group used by the connection, if applicable."]
    #[serde(rename = "keyExchangeGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub key_exchange_group: Option<String>,
    #[doc = "Cipher name."]
    #[serde(rename = "cipher")]
    pub cipher: String,
    #[doc = "TLS MAC. Note that AEAD ciphers do not have separate MACs."]
    #[serde(rename = "mac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mac: Option<String>,
    #[doc = "Certificate ID value."]
    #[serde(rename = "certificateId")]
    pub certificate_id: super::super::security::types::CertificateId,
    #[doc = "Certificate subject name."]
    #[serde(rename = "subjectName")]
    pub subject_name: String,
    #[doc = "Subject Alternative Name (SAN) DNS names and IP addresses."]
    #[serde(rename = "sanList")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub san_list: Vec<String>,
    #[doc = "Name of the issuing CA."]
    #[serde(rename = "issuer")]
    pub issuer: String,
    #[doc = "Certificate valid from date."]
    #[serde(rename = "validFrom")]
    pub valid_from: TimeSinceEpoch,
    #[doc = "Certificate valid to (expiration) date"]
    #[serde(rename = "validTo")]
    pub valid_to: TimeSinceEpoch,
    #[doc = "List of signed certificate timestamps (SCTs)."]
    #[serde(rename = "signedCertificateTimestampList")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub signed_certificate_timestamp_list: Vec<SignedCertificateTimestamp>,
    #[doc = "Whether the request complied with Certificate Transparency policy"]
    #[serde(rename = "certificateTransparencyCompliance")]
    pub certificate_transparency_compliance: CertificateTransparencyCompliance,
    #[doc = "The signature algorithm used by the server in the TLS server signature,\nrepresented as a TLS SignatureScheme code point. Omitted if not\napplicable or not known."]
    #[serde(rename = "serverSignatureAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub server_signature_algorithm: Option<i64>,
    #[doc = "Whether the connection used Encrypted ClientHello"]
    #[serde(rename = "encryptedClientHello")]
    pub encrypted_client_hello: bool,
}
impl SecurityDetails {
    pub const IDENTIFIER: &'static str = "Network.SecurityDetails";
}
#[doc = "Whether the request complied with Certificate Transparency policy."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CertificateTransparencyCompliance {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "not-compliant")]
    NotCompliant,
    #[serde(rename = "compliant")]
    Compliant,
}
#[doc = "The reason why request was blocked."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlockedReason {
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "csp")]
    Csp,
    #[serde(rename = "mixed-content")]
    MixedContent,
    #[serde(rename = "origin")]
    Origin,
    #[serde(rename = "inspector")]
    Inspector,
    #[serde(rename = "integrity")]
    Integrity,
    #[serde(rename = "subresource-filter")]
    SubresourceFilter,
    #[serde(rename = "content-type")]
    ContentType,
    #[serde(rename = "coep-frame-resource-needs-coep-header")]
    CoepFrameResourceNeedsCoepHeader,
    #[serde(rename = "coop-sandboxed-iframe-cannot-navigate-to-coop-page")]
    CoopSandboxedIframeCannotNavigateToCoopPage,
    #[serde(rename = "corp-not-same-origin")]
    CorpNotSameOrigin,
    #[serde(rename = "corp-not-same-origin-after-defaulted-to-same-origin-by-coep")]
    CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
    #[serde(rename = "corp-not-same-origin-after-defaulted-to-same-origin-by-dip")]
    CorpNotSameOriginAfterDefaultedToSameOriginByDip,
    #[serde(rename = "corp-not-same-origin-after-defaulted-to-same-origin-by-coep-and-dip")]
    CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip,
    #[serde(rename = "corp-not-same-site")]
    CorpNotSameSite,
    #[serde(rename = "sri-message-signature-mismatch")]
    SriMessageSignatureMismatch,
}
#[doc = "The reason why request was blocked."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CorsError {
    #[serde(rename = "DisallowedByMode")]
    DisallowedByMode,
    #[serde(rename = "InvalidResponse")]
    InvalidResponse,
    #[serde(rename = "WildcardOriginNotAllowed")]
    WildcardOriginNotAllowed,
    #[serde(rename = "MissingAllowOriginHeader")]
    MissingAllowOriginHeader,
    #[serde(rename = "MultipleAllowOriginValues")]
    MultipleAllowOriginValues,
    #[serde(rename = "InvalidAllowOriginValue")]
    InvalidAllowOriginValue,
    #[serde(rename = "AllowOriginMismatch")]
    AllowOriginMismatch,
    #[serde(rename = "InvalidAllowCredentials")]
    InvalidAllowCredentials,
    #[serde(rename = "CorsDisabledScheme")]
    CorsDisabledScheme,
    #[serde(rename = "PreflightInvalidStatus")]
    PreflightInvalidStatus,
    #[serde(rename = "PreflightDisallowedRedirect")]
    PreflightDisallowedRedirect,
    #[serde(rename = "PreflightWildcardOriginNotAllowed")]
    PreflightWildcardOriginNotAllowed,
    #[serde(rename = "PreflightMissingAllowOriginHeader")]
    PreflightMissingAllowOriginHeader,
    #[serde(rename = "PreflightMultipleAllowOriginValues")]
    PreflightMultipleAllowOriginValues,
    #[serde(rename = "PreflightInvalidAllowOriginValue")]
    PreflightInvalidAllowOriginValue,
    #[serde(rename = "PreflightAllowOriginMismatch")]
    PreflightAllowOriginMismatch,
    #[serde(rename = "PreflightInvalidAllowCredentials")]
    PreflightInvalidAllowCredentials,
    #[doc = "TODO(https://crbug.com/1263483): Remove this once frontend code does\nnot reference it anymore."]
    #[serde(rename = "PreflightMissingAllowExternal")]
    PreflightMissingAllowExternal,
    #[doc = "TODO(https://crbug.com/1263483): Remove this once frontend code does\nnot reference it anymore."]
    #[serde(rename = "PreflightInvalidAllowExternal")]
    PreflightInvalidAllowExternal,
    #[serde(rename = "InvalidAllowMethodsPreflightResponse")]
    InvalidAllowMethodsPreflightResponse,
    #[serde(rename = "InvalidAllowHeadersPreflightResponse")]
    InvalidAllowHeadersPreflightResponse,
    #[serde(rename = "MethodDisallowedByPreflightResponse")]
    MethodDisallowedByPreflightResponse,
    #[serde(rename = "HeaderDisallowedByPreflightResponse")]
    HeaderDisallowedByPreflightResponse,
    #[serde(rename = "RedirectContainsCredentials")]
    RedirectContainsCredentials,
    #[doc = "Request was a private network request initiated by a non-secure context."]
    #[serde(rename = "InsecureLocalNetwork")]
    InsecureLocalNetwork,
    #[doc = "Request carried a target IP address space property that did not match\nthe target resource's address space."]
    #[serde(rename = "InvalidLocalNetworkAccess")]
    InvalidLocalNetworkAccess,
    #[serde(rename = "NoCorsRedirectModeNotFollow")]
    NoCorsRedirectModeNotFollow,
    #[doc = "Request was a local network request and is denied by user permission.\nhttps://wicg.github.io/local-network-access/"]
    #[serde(rename = "LocalNetworkAccessPermissionDenied")]
    LocalNetworkAccessPermissionDenied,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorsErrorStatus {
    #[serde(rename = "corsError")]
    pub cors_error: CorsError,
    #[serde(rename = "failedParameter")]
    pub failed_parameter: String,
}
impl CorsErrorStatus {
    pub fn new(cors_error: impl Into<CorsError>, failed_parameter: impl Into<String>) -> Self {
        Self {
            cors_error: cors_error.into(),
            failed_parameter: failed_parameter.into(),
        }
    }
}
impl CorsErrorStatus {
    pub const IDENTIFIER: &'static str = "Network.CorsErrorStatus";
}
#[doc = "Source of serviceworker response."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceWorkerResponseSource {
    #[serde(rename = "cache-storage")]
    CacheStorage,
    #[serde(rename = "http-cache")]
    HttpCache,
    #[serde(rename = "fallback-code")]
    FallbackCode,
    #[serde(rename = "network")]
    Network,
}
#[doc = "Determines what type of Trust Token operation is executed and\ndepending on the type, some additional parameters. The values\nare specified in third_party/blink/renderer/core/fetch/trust_token.idl.\n[TrustTokenParams](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-TrustTokenParams)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrustTokenParams {
    #[serde(rename = "operation")]
    pub operation: TrustTokenOperationType,
    #[doc = "Only set for \"token-redemption\" operation and determine whether\nto request a fresh SRR or use a still valid cached SRR."]
    #[serde(rename = "refreshPolicy")]
    pub refresh_policy: TrustTokenParamsRefreshPolicy,
    #[doc = "Origins of issuers from whom to request tokens or redemption\nrecords."]
    #[serde(rename = "issuers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub issuers: Option<Vec<String>>,
}
#[doc = "Only set for \"token-redemption\" operation and determine whether\nto request a fresh SRR or use a still valid cached SRR."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrustTokenParamsRefreshPolicy {
    #[serde(rename = "UseCached")]
    UseCached,
    #[serde(rename = "Refresh")]
    Refresh,
}
impl TrustTokenParams {
    pub fn new(
        operation: impl Into<TrustTokenOperationType>,
        refresh_policy: impl Into<TrustTokenParamsRefreshPolicy>,
    ) -> Self {
        Self {
            operation: operation.into(),
            refresh_policy: refresh_policy.into(),
            issuers: None,
        }
    }
}
impl TrustTokenParams {
    pub const IDENTIFIER: &'static str = "Network.TrustTokenParams";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrustTokenOperationType {
    #[doc = "Type \"token-request\" in the Trust Token API."]
    #[serde(rename = "Issuance")]
    Issuance,
    #[doc = "Type \"token-redemption\" in the Trust Token API."]
    #[serde(rename = "Redemption")]
    Redemption,
    #[doc = "Type \"send-redemption-record\" in the Trust Token API."]
    #[serde(rename = "Signing")]
    Signing,
}
#[doc = "The reason why Chrome uses a specific transport protocol for HTTP semantics."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlternateProtocolUsage {
    #[doc = "Alternate Protocol was used without racing a normal connection."]
    #[serde(rename = "alternativeJobWonWithoutRace")]
    AlternativeJobWonWithoutRace,
    #[doc = "Alternate Protocol was used by winning a race with a normal connection."]
    #[serde(rename = "alternativeJobWonRace")]
    AlternativeJobWonRace,
    #[doc = "Alternate Protocol was not used by losing a race with a normal connection."]
    #[serde(rename = "mainJobWonRace")]
    MainJobWonRace,
    #[doc = "Alternate Protocol was not used because no Alternate-Protocol information\nwas available when the request was issued, but an Alternate-Protocol header\nwas present in the response."]
    #[serde(rename = "mappingMissing")]
    MappingMissing,
    #[doc = "Alternate Protocol was not used because it was marked broken."]
    #[serde(rename = "broken")]
    Broken,
    #[doc = "HTTPS DNS protocol upgrade job was used without racing with a normal\nconnection and an Alternate Protocol job."]
    #[serde(rename = "dnsAlpnH3JobWonWithoutRace")]
    DnsAlpnH3JobWonWithoutRace,
    #[doc = "HTTPS DNS protocol upgrade job won a race with a normal connection and\nan Alternate Protocol job."]
    #[serde(rename = "dnsAlpnH3JobWonRace")]
    DnsAlpnH3JobWonRace,
    #[doc = "This value is used when the reason is unknown."]
    #[serde(rename = "unspecifiedReason")]
    UnspecifiedReason,
}
#[doc = "Source of service worker router."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceWorkerRouterSource {
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "cache")]
    Cache,
    #[serde(rename = "fetch-event")]
    FetchEvent,
    #[serde(rename = "race-network-and-fetch-handler")]
    RaceNetworkAndFetchHandler,
    #[serde(rename = "race-network-and-cache")]
    RaceNetworkAndCache,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceWorkerRouterInfo {
    #[doc = "ID of the rule matched. If there is a matched rule, this field will\nbe set, otherwiser no value will be set."]
    #[serde(rename = "ruleIdMatched")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub rule_id_matched: Option<i64>,
    #[doc = "The router source of the matched rule. If there is a matched rule, this\nfield will be set, otherwise no value will be set."]
    #[serde(rename = "matchedSourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub matched_source_type: Option<ServiceWorkerRouterSource>,
    #[doc = "The actual router source used."]
    #[serde(rename = "actualSourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub actual_source_type: Option<ServiceWorkerRouterSource>,
}
impl ServiceWorkerRouterInfo {
    pub const IDENTIFIER: &'static str = "Network.ServiceWorkerRouterInfo";
}
#[doc = "HTTP response data.\n[Response](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-Response)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    #[doc = "Response URL. This URL can be different from CachedResource.url in case of redirect."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "HTTP response status code."]
    #[serde(rename = "status")]
    pub status: i64,
    #[doc = "HTTP response status text."]
    #[serde(rename = "statusText")]
    pub status_text: String,
    #[doc = "HTTP response headers."]
    #[serde(rename = "headers")]
    pub headers: Headers,
    #[doc = "Resource mimeType as determined by the browser."]
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[doc = "Resource charset as determined by the browser (if applicable)."]
    #[serde(rename = "charset")]
    pub charset: String,
    #[doc = "Refined HTTP request headers that were actually transmitted over the network."]
    #[serde(rename = "requestHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request_headers: Option<Headers>,
    #[doc = "Specifies whether physical connection was actually reused for this request."]
    #[serde(rename = "connectionReused")]
    pub connection_reused: bool,
    #[doc = "Physical connection id that was actually used for this request."]
    #[serde(rename = "connectionId")]
    pub connection_id: f64,
    #[doc = "Remote IP address."]
    #[serde(rename = "remoteIPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub remote_ip_address: Option<String>,
    #[doc = "Remote port."]
    #[serde(rename = "remotePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub remote_port: Option<i64>,
    #[doc = "Specifies that the request was served from the disk cache."]
    #[serde(rename = "fromDiskCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub from_disk_cache: Option<bool>,
    #[doc = "Specifies that the request was served from the ServiceWorker."]
    #[serde(rename = "fromServiceWorker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub from_service_worker: Option<bool>,
    #[doc = "Specifies that the request was served from the prefetch cache."]
    #[serde(rename = "fromPrefetchCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub from_prefetch_cache: Option<bool>,
    #[doc = "Specifies that the request was served from the prefetch cache."]
    #[serde(rename = "fromEarlyHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub from_early_hints: Option<bool>,
    #[doc = "Information about how ServiceWorker Static Router API was used. If this\nfield is set with `matchedSourceType` field, a matching rule is found.\nIf this field is set without `matchedSource`, no matching rule is found.\nOtherwise, the API is not used."]
    #[serde(rename = "serviceWorkerRouterInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub service_worker_router_info: Option<ServiceWorkerRouterInfo>,
    #[doc = "Total number of bytes received for this request so far."]
    #[serde(rename = "encodedDataLength")]
    pub encoded_data_length: f64,
    #[doc = "Timing information for the given request."]
    #[serde(rename = "timing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub timing: Option<ResourceTiming>,
    #[doc = "Response source of response from ServiceWorker."]
    #[serde(rename = "serviceWorkerResponseSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub service_worker_response_source: Option<ServiceWorkerResponseSource>,
    #[doc = "The time at which the returned response was generated."]
    #[serde(rename = "responseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub response_time: Option<TimeSinceEpoch>,
    #[doc = "Cache Storage Cache Name."]
    #[serde(rename = "cacheStorageCacheName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cache_storage_cache_name: Option<String>,
    #[doc = "Protocol used to fetch this request."]
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub protocol: Option<String>,
    #[doc = "The reason why Chrome uses a specific transport protocol for HTTP semantics."]
    #[serde(rename = "alternateProtocolUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub alternate_protocol_usage: Option<AlternateProtocolUsage>,
    #[doc = "Security state of the request resource."]
    #[serde(rename = "securityState")]
    pub security_state: super::super::security::types::SecurityState,
    #[doc = "Security details for the request."]
    #[serde(rename = "securityDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub security_details: Option<SecurityDetails>,
}
impl Response {
    pub const IDENTIFIER: &'static str = "Network.Response";
}
#[doc = "WebSocket request data.\n[WebSocketRequest](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-WebSocketRequest)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebSocketRequest {
    #[doc = "HTTP request headers."]
    #[serde(rename = "headers")]
    pub headers: Headers,
}
impl WebSocketRequest {
    pub fn new(headers: impl Into<Headers>) -> Self {
        Self {
            headers: headers.into(),
        }
    }
}
impl WebSocketRequest {
    pub const IDENTIFIER: &'static str = "Network.WebSocketRequest";
}
#[doc = "WebSocket response data.\n[WebSocketResponse](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-WebSocketResponse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebSocketResponse {
    #[doc = "HTTP response status code."]
    #[serde(rename = "status")]
    pub status: i64,
    #[doc = "HTTP response status text."]
    #[serde(rename = "statusText")]
    pub status_text: String,
    #[doc = "HTTP response headers."]
    #[serde(rename = "headers")]
    pub headers: Headers,
    #[doc = "HTTP response headers text."]
    #[serde(rename = "headersText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub headers_text: Option<String>,
    #[doc = "HTTP request headers."]
    #[serde(rename = "requestHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request_headers: Option<Headers>,
    #[doc = "HTTP request headers text."]
    #[serde(rename = "requestHeadersText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request_headers_text: Option<String>,
}
impl WebSocketResponse {
    pub fn new(
        status: impl Into<i64>,
        status_text: impl Into<String>,
        headers: impl Into<Headers>,
    ) -> Self {
        Self {
            status: status.into(),
            status_text: status_text.into(),
            headers: headers.into(),
            headers_text: None,
            request_headers: None,
            request_headers_text: None,
        }
    }
}
impl WebSocketResponse {
    pub const IDENTIFIER: &'static str = "Network.WebSocketResponse";
}
#[doc = "WebSocket message data. This represents an entire WebSocket message, not just a fragmented frame as the name suggests.\n[WebSocketFrame](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-WebSocketFrame)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebSocketFrame {
    #[doc = "WebSocket message opcode."]
    #[serde(rename = "opcode")]
    pub opcode: f64,
    #[doc = "WebSocket message mask."]
    #[serde(rename = "mask")]
    pub mask: bool,
    #[doc = "WebSocket message payload data.\nIf the opcode is 1, this is a text message and payloadData is a UTF-8 string.\nIf the opcode isn't 1, then payloadData is a base64 encoded string representing binary data."]
    #[serde(rename = "payloadData")]
    pub payload_data: String,
}
impl WebSocketFrame {
    pub fn new(
        opcode: impl Into<f64>,
        mask: impl Into<bool>,
        payload_data: impl Into<String>,
    ) -> Self {
        Self {
            opcode: opcode.into(),
            mask: mask.into(),
            payload_data: payload_data.into(),
        }
    }
}
impl WebSocketFrame {
    pub const IDENTIFIER: &'static str = "Network.WebSocketFrame";
}
#[doc = "Information about the cached resource.\n[CachedResource](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-CachedResource)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CachedResource {
    #[doc = "Resource URL. This is the url of the original network request."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Type of this resource."]
    #[serde(rename = "type")]
    pub r#type: ResourceType,
    #[doc = "Cached response data."]
    #[serde(rename = "response")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub response: Option<Response>,
    #[doc = "Cached response body size."]
    #[serde(rename = "bodySize")]
    pub body_size: f64,
}
impl CachedResource {
    pub fn new(
        url: impl Into<String>,
        r#type: impl Into<ResourceType>,
        body_size: impl Into<f64>,
    ) -> Self {
        Self {
            url: url.into(),
            r#type: r#type.into(),
            body_size: body_size.into(),
            response: None,
        }
    }
}
impl CachedResource {
    pub const IDENTIFIER: &'static str = "Network.CachedResource";
}
#[doc = "Information about the request initiator.\n[Initiator](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-Initiator)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Initiator {
    #[doc = "Type of this initiator."]
    #[serde(rename = "type")]
    pub r#type: InitiatorType,
    #[doc = "Initiator JavaScript stack trace, set for Script only.\nRequires the Debugger domain to be enabled."]
    #[serde(rename = "stack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stack: Option<super::super::super::js_protocol::runtime::types::StackTrace>,
    #[doc = "Initiator URL, set for Parser type or for Script type (when script is importing module) or for SignedExchange type."]
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
    #[doc = "Initiator line number, set for Parser type or for Script type (when script is importing\nmodule) (0-based)."]
    #[serde(rename = "lineNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub line_number: Option<f64>,
    #[doc = "Initiator column number, set for Parser type or for Script type (when script is importing\nmodule) (0-based)."]
    #[serde(rename = "columnNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub column_number: Option<f64>,
    #[doc = "Set if another request triggered this request (e.g. preflight)."]
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request_id: Option<RequestId>,
}
#[doc = "Type of this initiator."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InitiatorType {
    #[serde(rename = "parser")]
    Parser,
    #[serde(rename = "script")]
    Script,
    #[serde(rename = "preload")]
    Preload,
    #[serde(rename = "SignedExchange")]
    SignedExchange,
    #[serde(rename = "preflight")]
    Preflight,
    #[serde(rename = "FedCM")]
    FedCm,
    #[serde(rename = "other")]
    Other,
}
impl Initiator {
    pub fn new(r#type: impl Into<InitiatorType>) -> Self {
        Self {
            r#type: r#type.into(),
            stack: None,
            url: None,
            line_number: None,
            column_number: None,
            request_id: None,
        }
    }
}
impl Initiator {
    pub const IDENTIFIER: &'static str = "Network.Initiator";
}
#[doc = "cookiePartitionKey object\nThe representation of the components of the key that are created by the cookiePartitionKey class contained in net/cookies/cookie_partition_key.h.\n[CookiePartitionKey](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-CookiePartitionKey)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookiePartitionKey {
    #[doc = "The site of the top-level URL the browser was visiting at the start\nof the request to the endpoint that set the cookie."]
    #[serde(rename = "topLevelSite")]
    pub top_level_site: String,
    #[doc = "Indicates if the cookie has any ancestors that are cross-site to the topLevelSite."]
    #[serde(rename = "hasCrossSiteAncestor")]
    pub has_cross_site_ancestor: bool,
}
impl CookiePartitionKey {
    pub fn new(
        top_level_site: impl Into<String>,
        has_cross_site_ancestor: impl Into<bool>,
    ) -> Self {
        Self {
            top_level_site: top_level_site.into(),
            has_cross_site_ancestor: has_cross_site_ancestor.into(),
        }
    }
}
impl CookiePartitionKey {
    pub const IDENTIFIER: &'static str = "Network.CookiePartitionKey";
}
#[doc = "Cookie object\n[Cookie](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-Cookie)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cookie {
    #[doc = "Cookie name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Cookie value."]
    #[serde(rename = "value")]
    pub value: String,
    #[doc = "Cookie domain."]
    #[serde(rename = "domain")]
    pub domain: String,
    #[doc = "Cookie path."]
    #[serde(rename = "path")]
    pub path: String,
    #[doc = "Cookie expiration date as the number of seconds since the UNIX epoch.\nThe value is set to -1 if the expiry date is not set.\nThe value can be null for values that cannot be represented in\nJSON (±Inf)."]
    #[serde(rename = "expires")]
    pub expires: f64,
    #[doc = "Cookie size."]
    #[serde(rename = "size")]
    pub size: i64,
    #[doc = "True if cookie is http-only."]
    #[serde(rename = "httpOnly")]
    pub http_only: bool,
    #[doc = "True if cookie is secure."]
    #[serde(rename = "secure")]
    pub secure: bool,
    #[doc = "True in case of session cookie."]
    #[serde(rename = "session")]
    pub session: bool,
    #[doc = "Cookie SameSite type."]
    #[serde(rename = "sameSite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub same_site: Option<CookieSameSite>,
    #[doc = "Cookie Priority"]
    #[serde(rename = "priority")]
    pub priority: CookiePriority,
    #[doc = "Cookie source scheme type."]
    #[serde(rename = "sourceScheme")]
    pub source_scheme: CookieSourceScheme,
    #[doc = "Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.\nAn unspecified port value allows protocol clients to emulate legacy cookie scope for the port.\nThis is a temporary ability and it will be removed in the future."]
    #[serde(rename = "sourcePort")]
    pub source_port: i64,
    #[doc = "Cookie partition key."]
    #[serde(rename = "partitionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub partition_key: Option<CookiePartitionKey>,
    #[doc = "True if cookie partition key is opaque."]
    #[serde(rename = "partitionKeyOpaque")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub partition_key_opaque: Option<bool>,
}
impl Cookie {
    pub const IDENTIFIER: &'static str = "Network.Cookie";
}
#[doc = "Types of reasons why a cookie may not be stored from a response."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetCookieBlockedReason {
    #[doc = "The cookie had the \"Secure\" attribute but was not received over a secure connection."]
    #[serde(rename = "SecureOnly")]
    SecureOnly,
    #[doc = "The cookie had the \"SameSite=Strict\" attribute but came from a cross-origin response.\nThis includes navigation requests initiated by other origins."]
    #[serde(rename = "SameSiteStrict")]
    SameSiteStrict,
    #[doc = "The cookie had the \"SameSite=Lax\" attribute but came from a cross-origin response."]
    #[serde(rename = "SameSiteLax")]
    SameSiteLax,
    #[doc = "The cookie didn't specify a \"SameSite\" attribute and was defaulted to \"SameSite=Lax\" and\nbroke the same rules specified in the SameSiteLax value."]
    #[serde(rename = "SameSiteUnspecifiedTreatedAsLax")]
    SameSiteUnspecifiedTreatedAsLax,
    #[doc = "The cookie had the \"SameSite=None\" attribute but did not specify the \"Secure\" attribute,\nwhich is required in order to use \"SameSite=None\"."]
    #[serde(rename = "SameSiteNoneInsecure")]
    SameSiteNoneInsecure,
    #[doc = "The cookie was not stored due to user preferences."]
    #[serde(rename = "UserPreferences")]
    UserPreferences,
    #[doc = "The cookie was blocked due to third-party cookie phaseout."]
    #[serde(rename = "ThirdPartyPhaseout")]
    ThirdPartyPhaseout,
    #[doc = "The cookie was blocked by third-party cookie blocking between sites in\nthe same First-Party Set."]
    #[serde(rename = "ThirdPartyBlockedInFirstPartySet")]
    ThirdPartyBlockedInFirstPartySet,
    #[doc = "The syntax of the Set-Cookie header of the response was invalid."]
    #[serde(rename = "SyntaxError")]
    SyntaxError,
    #[doc = "The scheme of the connection is not allowed to store cookies."]
    #[serde(rename = "SchemeNotSupported")]
    SchemeNotSupported,
    #[doc = "The cookie was not sent over a secure connection and would have overwritten a cookie with\nthe Secure attribute."]
    #[serde(rename = "OverwriteSecure")]
    OverwriteSecure,
    #[doc = "The cookie's domain attribute was invalid with regards to the current host url."]
    #[serde(rename = "InvalidDomain")]
    InvalidDomain,
    #[doc = "The cookie used the \"__Secure-\" or \"__Host-\" prefix in its name and broke the additional\nrules applied to cookies with these prefixes as defined in\nhttps://tools.ietf.org/html/draft-west-cookie-prefixes-05"]
    #[serde(rename = "InvalidPrefix")]
    InvalidPrefix,
    #[doc = "An unknown error was encountered when trying to store this cookie."]
    #[serde(rename = "UnknownError")]
    UnknownError,
    #[doc = "The cookie had the \"SameSite=Strict\" attribute but came from a response\nwith the same registrable domain but a different scheme.\nThis includes navigation requests initiated by other origins.\nThis is the \"Schemeful Same-Site\" version of the blocked reason."]
    #[serde(rename = "SchemefulSameSiteStrict")]
    SchemefulSameSiteStrict,
    #[doc = "The cookie had the \"SameSite=Lax\" attribute but came from a response\nwith the same registrable domain but a different scheme.\nThis is the \"Schemeful Same-Site\" version of the blocked reason."]
    #[serde(rename = "SchemefulSameSiteLax")]
    SchemefulSameSiteLax,
    #[doc = "The cookie didn't specify a \"SameSite\" attribute and was defaulted to\n\"SameSite=Lax\" and broke the same rules specified in the SchemefulSameSiteLax\nvalue.\nThis is the \"Schemeful Same-Site\" version of the blocked reason."]
    #[serde(rename = "SchemefulSameSiteUnspecifiedTreatedAsLax")]
    SchemefulSameSiteUnspecifiedTreatedAsLax,
    #[doc = "The cookie's name/value pair size exceeded the size limit defined in\nRFC6265bis."]
    #[serde(rename = "NameValuePairExceedsMaxSize")]
    NameValuePairExceedsMaxSize,
    #[doc = "The cookie contained a forbidden ASCII control character, or the tab\ncharacter if it appears in the middle of the cookie name, value, an\nattribute name, or an attribute value."]
    #[serde(rename = "DisallowedCharacter")]
    DisallowedCharacter,
    #[doc = "Cookie contains no content or only whitespace."]
    #[serde(rename = "NoCookieContent")]
    NoCookieContent,
}
#[doc = "Types of reasons why a cookie may not be sent with a request."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CookieBlockedReason {
    #[doc = "The cookie had the \"Secure\" attribute and the connection was not secure."]
    #[serde(rename = "SecureOnly")]
    SecureOnly,
    #[doc = "The cookie's path was not within the request url's path."]
    #[serde(rename = "NotOnPath")]
    NotOnPath,
    #[doc = "The cookie's domain is not configured to match the request url's domain, even though they\nshare a common TLD+1 (TLD+1 of foo.bar.example.com is example.com)."]
    #[serde(rename = "DomainMismatch")]
    DomainMismatch,
    #[doc = "The cookie had the \"SameSite=Strict\" attribute and the request was made on on a different\nsite. This includes navigation requests initiated by other sites."]
    #[serde(rename = "SameSiteStrict")]
    SameSiteStrict,
    #[doc = "The cookie had the \"SameSite=Lax\" attribute and the request was made on a different site.\nThis does not include navigation requests initiated by other sites."]
    #[serde(rename = "SameSiteLax")]
    SameSiteLax,
    #[doc = "The cookie didn't specify a SameSite attribute when it was stored and was defaulted to\n\"SameSite=Lax\" and broke the same rules specified in the SameSiteLax value. The cookie had\nto have been set with \"SameSite=None\" to enable third-party usage."]
    #[serde(rename = "SameSiteUnspecifiedTreatedAsLax")]
    SameSiteUnspecifiedTreatedAsLax,
    #[doc = "The cookie had the \"SameSite=None\" attribute and the connection was not secure. Cookies\nwithout SameSite restrictions must be sent over a secure connection."]
    #[serde(rename = "SameSiteNoneInsecure")]
    SameSiteNoneInsecure,
    #[doc = "The cookie was not sent due to user preferences."]
    #[serde(rename = "UserPreferences")]
    UserPreferences,
    #[doc = "The cookie was blocked due to third-party cookie phaseout."]
    #[serde(rename = "ThirdPartyPhaseout")]
    ThirdPartyPhaseout,
    #[doc = "The cookie was blocked by third-party cookie blocking between sites in\nthe same First-Party Set."]
    #[serde(rename = "ThirdPartyBlockedInFirstPartySet")]
    ThirdPartyBlockedInFirstPartySet,
    #[doc = "An unknown error was encountered when trying to send this cookie."]
    #[serde(rename = "UnknownError")]
    UnknownError,
    #[doc = "The cookie had the \"SameSite=Strict\" attribute but came from a response\nwith the same registrable domain but a different scheme.\nThis includes navigation requests initiated by other origins.\nThis is the \"Schemeful Same-Site\" version of the blocked reason."]
    #[serde(rename = "SchemefulSameSiteStrict")]
    SchemefulSameSiteStrict,
    #[doc = "The cookie had the \"SameSite=Lax\" attribute but came from a response\nwith the same registrable domain but a different scheme.\nThis is the \"Schemeful Same-Site\" version of the blocked reason."]
    #[serde(rename = "SchemefulSameSiteLax")]
    SchemefulSameSiteLax,
    #[doc = "The cookie didn't specify a \"SameSite\" attribute and was defaulted to\n\"SameSite=Lax\" and broke the same rules specified in the SchemefulSameSiteLax\nvalue.\nThis is the \"Schemeful Same-Site\" version of the blocked reason."]
    #[serde(rename = "SchemefulSameSiteUnspecifiedTreatedAsLax")]
    SchemefulSameSiteUnspecifiedTreatedAsLax,
    #[doc = "The cookie's name/value pair size exceeded the size limit defined in\nRFC6265bis."]
    #[serde(rename = "NameValuePairExceedsMaxSize")]
    NameValuePairExceedsMaxSize,
    #[doc = "The cookie's source port value does not match the request origin's port."]
    #[serde(rename = "PortMismatch")]
    PortMismatch,
    #[doc = "The cookie's source scheme value does not match the request origin's scheme."]
    #[serde(rename = "SchemeMismatch")]
    SchemeMismatch,
    #[doc = "Unpartitioned cookie access from an anonymous context was blocked."]
    #[serde(rename = "AnonymousContext")]
    AnonymousContext,
}
#[doc = "Types of reasons why a cookie should have been blocked by 3PCD but is exempted for the request."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CookieExemptionReason {
    #[doc = "The default value. Cookie with this reason could either be blocked or included."]
    #[serde(rename = "None")]
    None,
    #[doc = "The cookie should have been blocked by 3PCD but is exempted by explicit user setting."]
    #[serde(rename = "UserSetting")]
    UserSetting,
    #[doc = "The cookie should have been blocked by 3PCD but is exempted by metadata mitigation."]
    #[serde(rename = "TPCDMetadata")]
    TpcdMetadata,
    #[doc = "The cookie should have been blocked by 3PCD but is exempted by Deprecation Trial mitigation."]
    #[serde(rename = "TPCDDeprecationTrial")]
    TpcdDeprecationTrial,
    #[doc = "The cookie should have been blocked by 3PCD but is exempted by Top-level Deprecation Trial mitigation."]
    #[serde(rename = "TopLevelTPCDDeprecationTrial")]
    TopLevelTpcdDeprecationTrial,
    #[doc = "The cookie should have been blocked by 3PCD but is exempted by heuristics mitigation."]
    #[serde(rename = "TPCDHeuristics")]
    TpcdHeuristics,
    #[doc = "The cookie should have been blocked by 3PCD but is exempted by Enterprise Policy."]
    #[serde(rename = "EnterprisePolicy")]
    EnterprisePolicy,
    #[doc = "The cookie should have been blocked by 3PCD but is exempted by Storage Access API."]
    #[serde(rename = "StorageAccess")]
    StorageAccess,
    #[doc = "The cookie should have been blocked by 3PCD but is exempted by Top-level Storage Access API."]
    #[serde(rename = "TopLevelStorageAccess")]
    TopLevelStorageAccess,
    #[doc = "The cookie should have been blocked by 3PCD but is exempted by the first-party URL scheme."]
    #[serde(rename = "Scheme")]
    Scheme,
    #[doc = "The cookie was included due to the 'allow-same-site-none-cookies' value being set in the sandboxing policy."]
    #[serde(rename = "SameSiteNoneCookiesInSandbox")]
    SameSiteNoneCookiesInSandbox,
}
#[doc = "A cookie which was not stored from a response with the corresponding reason.\n[BlockedSetCookieWithReason](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-BlockedSetCookieWithReason)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockedSetCookieWithReason {
    #[doc = "The reason(s) this cookie was blocked."]
    #[serde(rename = "blockedReasons")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub blocked_reasons: Vec<SetCookieBlockedReason>,
    #[doc = "The string representing this individual cookie as it would appear in the header.\nThis is not the entire \"cookie\" or \"set-cookie\" header which could have multiple cookies."]
    #[serde(rename = "cookieLine")]
    pub cookie_line: String,
    #[doc = "The cookie object which represents the cookie which was not stored. It is optional because\nsometimes complete cookie information is not available, such as in the case of parsing\nerrors."]
    #[serde(rename = "cookie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cookie: Option<Cookie>,
}
impl BlockedSetCookieWithReason {
    pub fn new(
        blocked_reasons: Vec<SetCookieBlockedReason>,
        cookie_line: impl Into<String>,
    ) -> Self {
        Self {
            blocked_reasons,
            cookie_line: cookie_line.into(),
            cookie: None,
        }
    }
}
impl BlockedSetCookieWithReason {
    pub const IDENTIFIER: &'static str = "Network.BlockedSetCookieWithReason";
}
#[doc = "A cookie should have been blocked by 3PCD but is exempted and stored from a response with the\ncorresponding reason. A cookie could only have at most one exemption reason.\n[ExemptedSetCookieWithReason](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-ExemptedSetCookieWithReason)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExemptedSetCookieWithReason {
    #[doc = "The reason the cookie was exempted."]
    #[serde(rename = "exemptionReason")]
    pub exemption_reason: CookieExemptionReason,
    #[doc = "The string representing this individual cookie as it would appear in the header."]
    #[serde(rename = "cookieLine")]
    pub cookie_line: String,
    #[doc = "The cookie object representing the cookie."]
    #[serde(rename = "cookie")]
    pub cookie: Cookie,
}
impl ExemptedSetCookieWithReason {
    pub fn new(
        exemption_reason: impl Into<CookieExemptionReason>,
        cookie_line: impl Into<String>,
        cookie: impl Into<Cookie>,
    ) -> Self {
        Self {
            exemption_reason: exemption_reason.into(),
            cookie_line: cookie_line.into(),
            cookie: cookie.into(),
        }
    }
}
impl ExemptedSetCookieWithReason {
    pub const IDENTIFIER: &'static str = "Network.ExemptedSetCookieWithReason";
}
#[doc = "A cookie associated with the request which may or may not be sent with it.\nIncludes the cookies itself and reasons for blocking or exemption.\n[AssociatedCookie](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-AssociatedCookie)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssociatedCookie {
    #[doc = "The cookie object representing the cookie which was not sent."]
    #[serde(rename = "cookie")]
    pub cookie: Cookie,
    #[doc = "The reason(s) the cookie was blocked. If empty means the cookie is included."]
    #[serde(rename = "blockedReasons")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub blocked_reasons: Vec<CookieBlockedReason>,
    #[doc = "The reason the cookie should have been blocked by 3PCD but is exempted. A cookie could\nonly have at most one exemption reason."]
    #[serde(rename = "exemptionReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exemption_reason: Option<CookieExemptionReason>,
}
impl AssociatedCookie {
    pub fn new(cookie: impl Into<Cookie>, blocked_reasons: Vec<CookieBlockedReason>) -> Self {
        Self {
            cookie: cookie.into(),
            blocked_reasons,
            exemption_reason: None,
        }
    }
}
impl AssociatedCookie {
    pub const IDENTIFIER: &'static str = "Network.AssociatedCookie";
}
#[doc = "Cookie parameter object\n[CookieParam](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-CookieParam)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookieParam {
    #[doc = "Cookie name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Cookie value."]
    #[serde(rename = "value")]
    pub value: String,
    #[doc = "The request-URI to associate with the setting of the cookie. This value can affect the\ndefault domain, path, source port, and source scheme values of the created cookie."]
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
    #[doc = "Cookie domain."]
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub domain: Option<String>,
    #[doc = "Cookie path."]
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub path: Option<String>,
    #[doc = "True if cookie is secure."]
    #[serde(rename = "secure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub secure: Option<bool>,
    #[doc = "True if cookie is http-only."]
    #[serde(rename = "httpOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub http_only: Option<bool>,
    #[doc = "Cookie SameSite type."]
    #[serde(rename = "sameSite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub same_site: Option<CookieSameSite>,
    #[doc = "Cookie expiration date, session cookie if not set"]
    #[serde(rename = "expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub expires: Option<TimeSinceEpoch>,
    #[doc = "Cookie Priority."]
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub priority: Option<CookiePriority>,
    #[doc = "Cookie source scheme type."]
    #[serde(rename = "sourceScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_scheme: Option<CookieSourceScheme>,
    #[doc = "Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.\nAn unspecified port value allows protocol clients to emulate legacy cookie scope for the port.\nThis is a temporary ability and it will be removed in the future."]
    #[serde(rename = "sourcePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_port: Option<i64>,
    #[doc = "Cookie partition key. If not set, the cookie will be set as not partitioned."]
    #[serde(rename = "partitionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub partition_key: Option<CookiePartitionKey>,
}
impl CookieParam {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            url: None,
            domain: None,
            path: None,
            secure: None,
            http_only: None,
            same_site: None,
            expires: None,
            priority: None,
            source_scheme: None,
            source_port: None,
            partition_key: None,
        }
    }
}
impl CookieParam {
    pub const IDENTIFIER: &'static str = "Network.CookieParam";
}
#[doc = "Authorization challenge for HTTP status code 401 or 407.\n[AuthChallenge](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-AuthChallenge)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthChallenge {
    #[doc = "Source of the authentication challenge."]
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source: Option<AuthChallengeSource>,
    #[doc = "Origin of the challenger."]
    #[serde(rename = "origin")]
    pub origin: String,
    #[doc = "The authentication scheme used, such as basic or digest"]
    #[serde(rename = "scheme")]
    pub scheme: String,
    #[doc = "The realm of the challenge. May be empty."]
    #[serde(rename = "realm")]
    pub realm: String,
}
#[doc = "Source of the authentication challenge."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthChallengeSource {
    #[serde(rename = "Server")]
    Server,
    #[serde(rename = "Proxy")]
    Proxy,
}
impl AuthChallenge {
    pub fn new(
        origin: impl Into<String>,
        scheme: impl Into<String>,
        realm: impl Into<String>,
    ) -> Self {
        Self {
            origin: origin.into(),
            scheme: scheme.into(),
            realm: realm.into(),
            source: None,
        }
    }
}
impl AuthChallenge {
    pub const IDENTIFIER: &'static str = "Network.AuthChallenge";
}
#[doc = "Response to an AuthChallenge.\n[AuthChallengeResponse](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-AuthChallengeResponse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthChallengeResponse {
    #[doc = "The decision on what to do in response to the authorization challenge.  Default means\ndeferring to the default behavior of the net stack, which will likely either the Cancel\nauthentication or display a popup dialog box."]
    #[serde(rename = "response")]
    pub response: AuthChallengeResponseResponse,
    #[doc = "The username to provide, possibly empty. Should only be set if response is\nProvideCredentials."]
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub username: Option<String>,
    #[doc = "The password to provide, possibly empty. Should only be set if response is\nProvideCredentials."]
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub password: Option<String>,
}
#[doc = "The decision on what to do in response to the authorization challenge.  Default means\ndeferring to the default behavior of the net stack, which will likely either the Cancel\nauthentication or display a popup dialog box."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthChallengeResponseResponse {
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "CancelAuth")]
    CancelAuth,
    #[serde(rename = "ProvideCredentials")]
    ProvideCredentials,
}
impl AuthChallengeResponse {
    pub fn new(response: impl Into<AuthChallengeResponseResponse>) -> Self {
        Self {
            response: response.into(),
            username: None,
            password: None,
        }
    }
}
impl AuthChallengeResponse {
    pub const IDENTIFIER: &'static str = "Network.AuthChallengeResponse";
}
#[doc = "Stages of the interception to begin intercepting. Request will intercept before the request is\nsent. Response will intercept after the response is received."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InterceptionStage {
    #[serde(rename = "Request")]
    Request,
    #[serde(rename = "HeadersReceived")]
    HeadersReceived,
}
#[doc = "Request pattern for interception.\n[RequestPattern](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-RequestPattern)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestPattern {
    #[doc = "Wildcards (`'*'` -> zero or more, `'?'` -> exactly one) are allowed. Escape character is\nbackslash. Omitting is equivalent to `\"*\"`."]
    #[serde(rename = "urlPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url_pattern: Option<String>,
    #[doc = "If set, only requests for matching resource types will be intercepted."]
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub resource_type: Option<ResourceType>,
    #[doc = "Stage at which to begin intercepting requests. Default is Request."]
    #[serde(rename = "interceptionStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub interception_stage: Option<InterceptionStage>,
}
impl RequestPattern {
    pub const IDENTIFIER: &'static str = "Network.RequestPattern";
}
#[doc = "Information about a signed exchange signature.\nhttps://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1\n[SignedExchangeSignature](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-SignedExchangeSignature)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignedExchangeSignature {
    #[doc = "Signed exchange signature label."]
    #[serde(rename = "label")]
    pub label: String,
    #[doc = "The hex string of signed exchange signature."]
    #[serde(rename = "signature")]
    pub signature: String,
    #[doc = "Signed exchange signature integrity."]
    #[serde(rename = "integrity")]
    pub integrity: String,
    #[doc = "Signed exchange signature cert Url."]
    #[serde(rename = "certUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cert_url: Option<String>,
    #[doc = "The hex string of signed exchange signature cert sha256."]
    #[serde(rename = "certSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cert_sha256: Option<String>,
    #[doc = "Signed exchange signature validity Url."]
    #[serde(rename = "validityUrl")]
    pub validity_url: String,
    #[doc = "Signed exchange signature date."]
    #[serde(rename = "date")]
    pub date: i64,
    #[doc = "Signed exchange signature expires."]
    #[serde(rename = "expires")]
    pub expires: i64,
    #[doc = "The encoded certificates."]
    #[serde(rename = "certificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub certificates: Option<Vec<String>>,
}
impl SignedExchangeSignature {
    pub const IDENTIFIER: &'static str = "Network.SignedExchangeSignature";
}
#[doc = "Information about a signed exchange header.\nhttps://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#cbor-representation\n[SignedExchangeHeader](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-SignedExchangeHeader)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignedExchangeHeader {
    #[doc = "Signed exchange request URL."]
    #[serde(rename = "requestUrl")]
    pub request_url: String,
    #[doc = "Signed exchange response code."]
    #[serde(rename = "responseCode")]
    pub response_code: i64,
    #[doc = "Signed exchange response headers."]
    #[serde(rename = "responseHeaders")]
    pub response_headers: Headers,
    #[doc = "Signed exchange response signature."]
    #[serde(rename = "signatures")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub signatures: Vec<SignedExchangeSignature>,
    #[doc = "Signed exchange header integrity hash in the form of `sha256-<base64-hash-value>`."]
    #[serde(rename = "headerIntegrity")]
    pub header_integrity: String,
}
impl SignedExchangeHeader {
    pub const IDENTIFIER: &'static str = "Network.SignedExchangeHeader";
}
#[doc = "Field type for a signed exchange related error."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignedExchangeErrorField {
    #[serde(rename = "signatureSig")]
    SignatureSig,
    #[serde(rename = "signatureIntegrity")]
    SignatureIntegrity,
    #[serde(rename = "signatureCertUrl")]
    SignatureCertUrl,
    #[serde(rename = "signatureCertSha256")]
    SignatureCertSha256,
    #[serde(rename = "signatureValidityUrl")]
    SignatureValidityUrl,
    #[serde(rename = "signatureTimestamps")]
    SignatureTimestamps,
}
#[doc = "Information about a signed exchange response.\n[SignedExchangeError](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-SignedExchangeError)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignedExchangeError {
    #[doc = "Error message."]
    #[serde(rename = "message")]
    pub message: String,
    #[doc = "The index of the signature which caused the error."]
    #[serde(rename = "signatureIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub signature_index: Option<i64>,
    #[doc = "The field which caused the error."]
    #[serde(rename = "errorField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub error_field: Option<SignedExchangeErrorField>,
}
impl SignedExchangeError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            signature_index: None,
            error_field: None,
        }
    }
}
impl<T: Into<String>> From<T> for SignedExchangeError {
    fn from(url: T) -> Self {
        SignedExchangeError::new(url)
    }
}
impl SignedExchangeError {
    pub const IDENTIFIER: &'static str = "Network.SignedExchangeError";
}
#[doc = "Information about a signed exchange response.\n[SignedExchangeInfo](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-SignedExchangeInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignedExchangeInfo {
    #[doc = "The outer response of signed HTTP exchange which was received from network."]
    #[serde(rename = "outerResponse")]
    pub outer_response: Response,
    #[doc = "Whether network response for the signed exchange was accompanied by\nextra headers."]
    #[serde(rename = "hasExtraInfo")]
    pub has_extra_info: bool,
    #[doc = "Information about the signed exchange header."]
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub header: Option<SignedExchangeHeader>,
    #[doc = "Security details for the signed exchange header."]
    #[serde(rename = "securityDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub security_details: Option<SecurityDetails>,
    #[doc = "Errors occurred while handling the signed exchange."]
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub errors: Option<Vec<SignedExchangeError>>,
}
impl SignedExchangeInfo {
    pub fn new(outer_response: impl Into<Response>, has_extra_info: impl Into<bool>) -> Self {
        Self {
            outer_response: outer_response.into(),
            has_extra_info: has_extra_info.into(),
            header: None,
            security_details: None,
            errors: None,
        }
    }
}
impl SignedExchangeInfo {
    pub const IDENTIFIER: &'static str = "Network.SignedExchangeInfo";
}
#[doc = "List of content encodings supported by the backend."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentEncoding {
    #[serde(rename = "deflate")]
    Deflate,
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "br")]
    Br,
    #[serde(rename = "zstd")]
    Zstd,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkConditions {
    #[doc = "Only matching requests will be affected by these conditions. Patterns use the URLPattern constructor string\nsyntax (https://urlpattern.spec.whatwg.org/) and must be absolute. If the pattern is empty, all requests are\nmatched (including p2p connections)."]
    #[serde(rename = "urlPattern")]
    pub url_pattern: String,
    #[doc = "Minimum latency from request sent to response headers received (ms)."]
    #[serde(rename = "latency")]
    pub latency: f64,
    #[doc = "Maximal aggregated download throughput (bytes/sec). -1 disables download throttling."]
    #[serde(rename = "downloadThroughput")]
    pub download_throughput: f64,
    #[doc = "Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling."]
    #[serde(rename = "uploadThroughput")]
    pub upload_throughput: f64,
    #[doc = "Connection type if known."]
    #[serde(rename = "connectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub connection_type: Option<ConnectionType>,
    #[doc = "WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets."]
    #[serde(rename = "packetLoss")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub packet_loss: Option<f64>,
    #[doc = "WebRTC packet queue length (packet). 0 removes any queue length limitations."]
    #[serde(rename = "packetQueueLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub packet_queue_length: Option<i64>,
    #[doc = "WebRTC packetReordering feature."]
    #[serde(rename = "packetReordering")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub packet_reordering: Option<bool>,
}
impl NetworkConditions {
    pub fn new(
        url_pattern: impl Into<String>,
        latency: impl Into<f64>,
        download_throughput: impl Into<f64>,
        upload_throughput: impl Into<f64>,
    ) -> Self {
        Self {
            url_pattern: url_pattern.into(),
            latency: latency.into(),
            download_throughput: download_throughput.into(),
            upload_throughput: upload_throughput.into(),
            connection_type: None,
            packet_loss: None,
            packet_queue_length: None,
            packet_reordering: None,
        }
    }
}
impl NetworkConditions {
    pub const IDENTIFIER: &'static str = "Network.NetworkConditions";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockPattern {
    #[doc = "URL pattern to match. Patterns use the URLPattern constructor string syntax\n(https://urlpattern.spec.whatwg.org/) and must be absolute. Example: `*://*:*/*.css`."]
    #[serde(rename = "urlPattern")]
    pub url_pattern: String,
    #[doc = "Whether or not to block the pattern. If false, a matching request will not be blocked even if it matches a later\n`BlockPattern`."]
    #[serde(rename = "block")]
    pub block: bool,
}
impl BlockPattern {
    pub fn new(url_pattern: impl Into<String>, block: impl Into<bool>) -> Self {
        Self {
            url_pattern: url_pattern.into(),
            block: block.into(),
        }
    }
}
impl BlockPattern {
    pub const IDENTIFIER: &'static str = "Network.BlockPattern";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DirectSocketDnsQueryType {
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectTcpSocketOptions {
    #[doc = "TCP_NODELAY option"]
    #[serde(rename = "noDelay")]
    pub no_delay: bool,
    #[doc = "Expected to be unsigned integer."]
    #[serde(rename = "keepAliveDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub keep_alive_delay: Option<f64>,
    #[doc = "Expected to be unsigned integer."]
    #[serde(rename = "sendBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub send_buffer_size: Option<f64>,
    #[doc = "Expected to be unsigned integer."]
    #[serde(rename = "receiveBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub receive_buffer_size: Option<f64>,
    #[serde(rename = "dnsQueryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dns_query_type: Option<DirectSocketDnsQueryType>,
}
impl DirectTcpSocketOptions {
    pub fn new(no_delay: impl Into<bool>) -> Self {
        Self {
            no_delay: no_delay.into(),
            keep_alive_delay: None,
            send_buffer_size: None,
            receive_buffer_size: None,
            dns_query_type: None,
        }
    }
}
impl DirectTcpSocketOptions {
    pub const IDENTIFIER: &'static str = "Network.DirectTCPSocketOptions";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DirectUdpSocketOptions {
    #[serde(rename = "remoteAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub remote_addr: Option<String>,
    #[doc = "Unsigned int 16."]
    #[serde(rename = "remotePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub remote_port: Option<i64>,
    #[serde(rename = "localAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub local_addr: Option<String>,
    #[doc = "Unsigned int 16."]
    #[serde(rename = "localPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub local_port: Option<i64>,
    #[serde(rename = "dnsQueryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dns_query_type: Option<DirectSocketDnsQueryType>,
    #[doc = "Expected to be unsigned integer."]
    #[serde(rename = "sendBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub send_buffer_size: Option<f64>,
    #[doc = "Expected to be unsigned integer."]
    #[serde(rename = "receiveBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub receive_buffer_size: Option<f64>,
    #[serde(rename = "multicastLoopback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub multicast_loopback: Option<bool>,
    #[doc = "Unsigned int 8."]
    #[serde(rename = "multicastTimeToLive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub multicast_time_to_live: Option<i64>,
    #[serde(rename = "multicastAllowAddressSharing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub multicast_allow_address_sharing: Option<bool>,
}
impl DirectUdpSocketOptions {
    pub const IDENTIFIER: &'static str = "Network.DirectUDPSocketOptions";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectUdpMessage {
    #[serde(rename = "data")]
    pub data: super::super::super::Binary,
    #[doc = "Null for connected mode."]
    #[serde(rename = "remoteAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub remote_addr: Option<String>,
    #[doc = "Null for connected mode.\nExpected to be unsigned integer."]
    #[serde(rename = "remotePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub remote_port: Option<i64>,
}
impl DirectUdpMessage {
    pub fn new(data: impl Into<super::super::super::Binary>) -> Self {
        Self {
            data: data.into(),
            remote_addr: None,
            remote_port: None,
        }
    }
}
impl DirectUdpMessage {
    pub const IDENTIFIER: &'static str = "Network.DirectUDPMessage";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LocalNetworkAccessRequestPolicy {
    #[serde(rename = "Allow")]
    Allow,
    #[serde(rename = "BlockFromInsecureToMorePrivate")]
    BlockFromInsecureToMorePrivate,
    #[serde(rename = "WarnFromInsecureToMorePrivate")]
    WarnFromInsecureToMorePrivate,
    #[serde(rename = "PermissionBlock")]
    PermissionBlock,
    #[serde(rename = "PermissionWarn")]
    PermissionWarn,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IpAddressSpace {
    #[serde(rename = "Loopback")]
    Loopback,
    #[serde(rename = "Local")]
    Local,
    #[serde(rename = "Public")]
    Public,
    #[serde(rename = "Unknown")]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectTiming {
    #[doc = "Timing's requestTime is a baseline in seconds, while the other numbers are ticks in\nmilliseconds relatively to this requestTime. Matches ResourceTiming's requestTime for\nthe same request (but not for redirected requests)."]
    #[serde(rename = "requestTime")]
    pub request_time: f64,
}
impl ConnectTiming {
    pub fn new(request_time: impl Into<f64>) -> Self {
        Self {
            request_time: request_time.into(),
        }
    }
}
impl ConnectTiming {
    pub const IDENTIFIER: &'static str = "Network.ConnectTiming";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientSecurityState {
    #[serde(rename = "initiatorIsSecureContext")]
    pub initiator_is_secure_context: bool,
    #[serde(rename = "initiatorIPAddressSpace")]
    pub initiator_ip_address_space: IpAddressSpace,
    #[serde(rename = "localNetworkAccessRequestPolicy")]
    pub local_network_access_request_policy: LocalNetworkAccessRequestPolicy,
}
impl ClientSecurityState {
    pub fn new(
        initiator_is_secure_context: impl Into<bool>,
        initiator_ip_address_space: impl Into<IpAddressSpace>,
        local_network_access_request_policy: impl Into<LocalNetworkAccessRequestPolicy>,
    ) -> Self {
        Self {
            initiator_is_secure_context: initiator_is_secure_context.into(),
            initiator_ip_address_space: initiator_ip_address_space.into(),
            local_network_access_request_policy: local_network_access_request_policy.into(),
        }
    }
}
impl ClientSecurityState {
    pub const IDENTIFIER: &'static str = "Network.ClientSecurityState";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CrossOriginOpenerPolicyValue {
    #[serde(rename = "SameOrigin")]
    SameOrigin,
    #[serde(rename = "SameOriginAllowPopups")]
    SameOriginAllowPopups,
    #[serde(rename = "RestrictProperties")]
    RestrictProperties,
    #[serde(rename = "UnsafeNone")]
    UnsafeNone,
    #[serde(rename = "SameOriginPlusCoep")]
    SameOriginPlusCoep,
    #[serde(rename = "RestrictPropertiesPlusCoep")]
    RestrictPropertiesPlusCoep,
    #[serde(rename = "NoopenerAllowPopups")]
    NoopenerAllowPopups,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossOriginOpenerPolicyStatus {
    #[serde(rename = "value")]
    pub value: CrossOriginOpenerPolicyValue,
    #[serde(rename = "reportOnlyValue")]
    pub report_only_value: CrossOriginOpenerPolicyValue,
    #[serde(rename = "reportingEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reporting_endpoint: Option<String>,
    #[serde(rename = "reportOnlyReportingEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub report_only_reporting_endpoint: Option<String>,
}
impl CrossOriginOpenerPolicyStatus {
    pub fn new(
        value: impl Into<CrossOriginOpenerPolicyValue>,
        report_only_value: impl Into<CrossOriginOpenerPolicyValue>,
    ) -> Self {
        Self {
            value: value.into(),
            report_only_value: report_only_value.into(),
            reporting_endpoint: None,
            report_only_reporting_endpoint: None,
        }
    }
}
impl CrossOriginOpenerPolicyStatus {
    pub const IDENTIFIER: &'static str = "Network.CrossOriginOpenerPolicyStatus";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CrossOriginEmbedderPolicyValue {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Credentialless")]
    Credentialless,
    #[serde(rename = "RequireCorp")]
    RequireCorp,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossOriginEmbedderPolicyStatus {
    #[serde(rename = "value")]
    pub value: CrossOriginEmbedderPolicyValue,
    #[serde(rename = "reportOnlyValue")]
    pub report_only_value: CrossOriginEmbedderPolicyValue,
    #[serde(rename = "reportingEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reporting_endpoint: Option<String>,
    #[serde(rename = "reportOnlyReportingEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub report_only_reporting_endpoint: Option<String>,
}
impl CrossOriginEmbedderPolicyStatus {
    pub fn new(
        value: impl Into<CrossOriginEmbedderPolicyValue>,
        report_only_value: impl Into<CrossOriginEmbedderPolicyValue>,
    ) -> Self {
        Self {
            value: value.into(),
            report_only_value: report_only_value.into(),
            reporting_endpoint: None,
            report_only_reporting_endpoint: None,
        }
    }
}
impl CrossOriginEmbedderPolicyStatus {
    pub const IDENTIFIER: &'static str = "Network.CrossOriginEmbedderPolicyStatus";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentSecurityPolicySource {
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "Meta")]
    Meta,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentSecurityPolicyStatus {
    #[serde(rename = "effectiveDirectives")]
    pub effective_directives: String,
    #[serde(rename = "isEnforced")]
    pub is_enforced: bool,
    #[serde(rename = "source")]
    pub source: ContentSecurityPolicySource,
}
impl ContentSecurityPolicyStatus {
    pub fn new(
        effective_directives: impl Into<String>,
        is_enforced: impl Into<bool>,
        source: impl Into<ContentSecurityPolicySource>,
    ) -> Self {
        Self {
            effective_directives: effective_directives.into(),
            is_enforced: is_enforced.into(),
            source: source.into(),
        }
    }
}
impl ContentSecurityPolicyStatus {
    pub const IDENTIFIER: &'static str = "Network.ContentSecurityPolicyStatus";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityIsolationStatus {
    #[serde(rename = "coop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub coop: Option<CrossOriginOpenerPolicyStatus>,
    #[serde(rename = "coep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub coep: Option<CrossOriginEmbedderPolicyStatus>,
    #[serde(rename = "csp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub csp: Option<Vec<ContentSecurityPolicyStatus>>,
}
impl SecurityIsolationStatus {
    pub const IDENTIFIER: &'static str = "Network.SecurityIsolationStatus";
}
#[doc = "The status of a Reporting API report."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReportStatus {
    #[doc = "Report has been queued and no attempt has been made to deliver it yet,\nor attempted previous upload failed (impermanently)."]
    #[serde(rename = "Queued")]
    Queued,
    #[doc = "There is an ongoing attempt to upload this report."]
    #[serde(rename = "Pending")]
    Pending,
    #[doc = "Deletion of this report was requested while it was pending, so it will\nbe removed after possibly outstanding upload attempts complete (successful\nor not)."]
    #[serde(rename = "MarkedForRemoval")]
    MarkedForRemoval,
    #[doc = "Successfully uploaded and MarkedForRemoval."]
    #[serde(rename = "Success")]
    Success,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct ReportId(String);
impl ReportId {
    pub fn new(val: impl Into<String>) -> Self {
        ReportId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for ReportId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<ReportId> for String {
    fn from(el: ReportId) -> String {
        el.0
    }
}
impl From<String> for ReportId {
    fn from(expr: String) -> Self {
        ReportId(expr)
    }
}
impl std::borrow::Borrow<str> for ReportId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl ReportId {
    pub const IDENTIFIER: &'static str = "Network.ReportId";
}
#[doc = "An object representing a report generated by the Reporting API.\n[ReportingApiReport](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-ReportingApiReport)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportingApiReport {
    #[serde(rename = "id")]
    pub id: ReportId,
    #[doc = "The URL of the document that triggered the report."]
    #[serde(rename = "initiatorUrl")]
    pub initiator_url: String,
    #[doc = "The name of the endpoint group that should be used to deliver the report."]
    #[serde(rename = "destination")]
    pub destination: String,
    #[doc = "The type of the report (specifies the set of data that is contained in the report body)."]
    #[serde(rename = "type")]
    pub r#type: String,
    #[doc = "When the report was generated."]
    #[serde(rename = "timestamp")]
    pub timestamp: TimeSinceEpoch,
    #[doc = "How many uploads deep the related request was."]
    #[serde(rename = "depth")]
    pub depth: i64,
    #[doc = "The number of delivery attempts made so far, not including an active attempt."]
    #[serde(rename = "completedAttempts")]
    pub completed_attempts: i64,
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(rename = "status")]
    pub status: ReportStatus,
}
impl ReportingApiReport {
    pub const IDENTIFIER: &'static str = "Network.ReportingApiReport";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportingApiEndpoint {
    #[doc = "The URL of the endpoint to which reports may be delivered."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Name of the endpoint group."]
    #[serde(rename = "groupName")]
    pub group_name: String,
}
impl ReportingApiEndpoint {
    pub fn new(url: impl Into<String>, group_name: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            group_name: group_name.into(),
        }
    }
}
impl ReportingApiEndpoint {
    pub const IDENTIFIER: &'static str = "Network.ReportingApiEndpoint";
}
#[doc = "Unique identifier for a device bound session.\n[DeviceBoundSessionKey](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-DeviceBoundSessionKey)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceBoundSessionKey {
    #[doc = "The site the session is set up for."]
    #[serde(rename = "site")]
    pub site: String,
    #[doc = "The id of the session."]
    #[serde(rename = "id")]
    pub id: String,
}
impl DeviceBoundSessionKey {
    pub fn new(site: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site: site.into(),
            id: id.into(),
        }
    }
}
impl DeviceBoundSessionKey {
    pub const IDENTIFIER: &'static str = "Network.DeviceBoundSessionKey";
}
#[doc = "How a device bound session was used during a request.\n[DeviceBoundSessionWithUsage](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-DeviceBoundSessionWithUsage)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceBoundSessionWithUsage {
    #[doc = "The key for the session."]
    #[serde(rename = "sessionKey")]
    pub session_key: DeviceBoundSessionKey,
    #[doc = "How the session was used (or not used)."]
    #[serde(rename = "usage")]
    pub usage: DeviceBoundSessionWithUsageUsage,
}
#[doc = "How the session was used (or not used)."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceBoundSessionWithUsageUsage {
    #[serde(rename = "NotInScope")]
    NotInScope,
    #[serde(rename = "InScopeRefreshNotYetNeeded")]
    InScopeRefreshNotYetNeeded,
    #[serde(rename = "InScopeRefreshNotAllowed")]
    InScopeRefreshNotAllowed,
    #[serde(rename = "ProactiveRefreshNotPossible")]
    ProactiveRefreshNotPossible,
    #[serde(rename = "ProactiveRefreshAttempted")]
    ProactiveRefreshAttempted,
    #[serde(rename = "Deferred")]
    Deferred,
}
impl DeviceBoundSessionWithUsage {
    pub fn new(
        session_key: impl Into<DeviceBoundSessionKey>,
        usage: impl Into<DeviceBoundSessionWithUsageUsage>,
    ) -> Self {
        Self {
            session_key: session_key.into(),
            usage: usage.into(),
        }
    }
}
impl DeviceBoundSessionWithUsage {
    pub const IDENTIFIER: &'static str = "Network.DeviceBoundSessionWithUsage";
}
#[doc = "A device bound session's cookie craving.\n[DeviceBoundSessionCookieCraving](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-DeviceBoundSessionCookieCraving)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceBoundSessionCookieCraving {
    #[doc = "The name of the craving."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The domain of the craving."]
    #[serde(rename = "domain")]
    pub domain: String,
    #[doc = "The path of the craving."]
    #[serde(rename = "path")]
    pub path: String,
    #[doc = "The `Secure` attribute of the craving attributes."]
    #[serde(rename = "secure")]
    pub secure: bool,
    #[doc = "The `HttpOnly` attribute of the craving attributes."]
    #[serde(rename = "httpOnly")]
    pub http_only: bool,
    #[doc = "The `SameSite` attribute of the craving attributes."]
    #[serde(rename = "sameSite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub same_site: Option<CookieSameSite>,
}
impl DeviceBoundSessionCookieCraving {
    pub const IDENTIFIER: &'static str = "Network.DeviceBoundSessionCookieCraving";
}
#[doc = "A device bound session's inclusion URL rule.\n[DeviceBoundSessionUrlRule](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-DeviceBoundSessionUrlRule)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceBoundSessionUrlRule {
    #[doc = "See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::rule_type`."]
    #[serde(rename = "ruleType")]
    pub rule_type: DeviceBoundSessionUrlRuleRuleType,
    #[doc = "See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::host_pattern`."]
    #[serde(rename = "hostPattern")]
    pub host_pattern: String,
    #[doc = "See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::path_prefix`."]
    #[serde(rename = "pathPrefix")]
    pub path_prefix: String,
}
#[doc = "See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::rule_type`."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceBoundSessionUrlRuleRuleType {
    #[serde(rename = "Exclude")]
    Exclude,
    #[serde(rename = "Include")]
    Include,
}
impl DeviceBoundSessionUrlRule {
    pub fn new(
        rule_type: impl Into<DeviceBoundSessionUrlRuleRuleType>,
        host_pattern: impl Into<String>,
        path_prefix: impl Into<String>,
    ) -> Self {
        Self {
            rule_type: rule_type.into(),
            host_pattern: host_pattern.into(),
            path_prefix: path_prefix.into(),
        }
    }
}
impl DeviceBoundSessionUrlRule {
    pub const IDENTIFIER: &'static str = "Network.DeviceBoundSessionUrlRule";
}
#[doc = "A device bound session's inclusion rules.\n[DeviceBoundSessionInclusionRules](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-DeviceBoundSessionInclusionRules)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceBoundSessionInclusionRules {
    #[doc = "See comments on `net::device_bound_sessions::SessionInclusionRules::origin_`."]
    #[serde(rename = "origin")]
    pub origin: String,
    #[doc = "Whether the whole site is included. See comments on\n`net::device_bound_sessions::SessionInclusionRules::include_site_` for more\ndetails; this boolean is true if that value is populated."]
    #[serde(rename = "includeSite")]
    pub include_site: bool,
    #[doc = "See comments on `net::device_bound_sessions::SessionInclusionRules::url_rules_`."]
    #[serde(rename = "urlRules")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url_rules: Vec<DeviceBoundSessionUrlRule>,
}
impl DeviceBoundSessionInclusionRules {
    pub fn new(
        origin: impl Into<String>,
        include_site: impl Into<bool>,
        url_rules: Vec<DeviceBoundSessionUrlRule>,
    ) -> Self {
        Self {
            origin: origin.into(),
            include_site: include_site.into(),
            url_rules,
        }
    }
}
impl DeviceBoundSessionInclusionRules {
    pub const IDENTIFIER: &'static str = "Network.DeviceBoundSessionInclusionRules";
}
#[doc = "A device bound session.\n[DeviceBoundSession](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-DeviceBoundSession)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceBoundSession {
    #[doc = "The site and session ID of the session."]
    #[serde(rename = "key")]
    pub key: DeviceBoundSessionKey,
    #[doc = "See comments on `net::device_bound_sessions::Session::refresh_url_`."]
    #[serde(rename = "refreshUrl")]
    pub refresh_url: String,
    #[doc = "See comments on `net::device_bound_sessions::Session::inclusion_rules_`."]
    #[serde(rename = "inclusionRules")]
    pub inclusion_rules: DeviceBoundSessionInclusionRules,
    #[doc = "See comments on `net::device_bound_sessions::Session::cookie_cravings_`."]
    #[serde(rename = "cookieCravings")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookie_cravings: Vec<DeviceBoundSessionCookieCraving>,
    #[doc = "See comments on `net::device_bound_sessions::Session::expiry_date_`."]
    #[serde(rename = "expiryDate")]
    pub expiry_date: TimeSinceEpoch,
    #[doc = "See comments on `net::device_bound_sessions::Session::cached_challenge__`."]
    #[serde(rename = "cachedChallenge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cached_challenge: Option<String>,
    #[doc = "See comments on `net::device_bound_sessions::Session::allowed_refresh_initiators_`."]
    #[serde(rename = "allowedRefreshInitiators")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_refresh_initiators: Vec<String>,
}
impl DeviceBoundSession {
    pub const IDENTIFIER: &'static str = "Network.DeviceBoundSession";
}
#[doc = "A unique identifier for a device bound session event.\n[DeviceBoundSessionEventId](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-DeviceBoundSessionEventId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct DeviceBoundSessionEventId(String);
impl DeviceBoundSessionEventId {
    pub fn new(val: impl Into<String>) -> Self {
        DeviceBoundSessionEventId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for DeviceBoundSessionEventId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<DeviceBoundSessionEventId> for String {
    fn from(el: DeviceBoundSessionEventId) -> String {
        el.0
    }
}
impl From<String> for DeviceBoundSessionEventId {
    fn from(expr: String) -> Self {
        DeviceBoundSessionEventId(expr)
    }
}
impl std::borrow::Borrow<str> for DeviceBoundSessionEventId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl DeviceBoundSessionEventId {
    pub const IDENTIFIER: &'static str = "Network.DeviceBoundSessionEventId";
}
#[doc = "A fetch result for a device bound session creation or refresh."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceBoundSessionFetchResult {
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "KeyError")]
    KeyError,
    #[serde(rename = "SigningError")]
    SigningError,
    #[serde(rename = "ServerRequestedTermination")]
    ServerRequestedTermination,
    #[serde(rename = "InvalidSessionId")]
    InvalidSessionId,
    #[serde(rename = "InvalidChallenge")]
    InvalidChallenge,
    #[serde(rename = "TooManyChallenges")]
    TooManyChallenges,
    #[serde(rename = "InvalidFetcherUrl")]
    InvalidFetcherUrl,
    #[serde(rename = "InvalidRefreshUrl")]
    InvalidRefreshUrl,
    #[serde(rename = "TransientHttpError")]
    TransientHttpError,
    #[serde(rename = "ScopeOriginSameSiteMismatch")]
    ScopeOriginSameSiteMismatch,
    #[serde(rename = "RefreshUrlSameSiteMismatch")]
    RefreshUrlSameSiteMismatch,
    #[serde(rename = "MismatchedSessionId")]
    MismatchedSessionId,
    #[serde(rename = "MissingScope")]
    MissingScope,
    #[serde(rename = "NoCredentials")]
    NoCredentials,
    #[serde(rename = "SubdomainRegistrationWellKnownUnavailable")]
    SubdomainRegistrationWellKnownUnavailable,
    #[serde(rename = "SubdomainRegistrationUnauthorized")]
    SubdomainRegistrationUnauthorized,
    #[serde(rename = "SubdomainRegistrationWellKnownMalformed")]
    SubdomainRegistrationWellKnownMalformed,
    #[serde(rename = "SessionProviderWellKnownUnavailable")]
    SessionProviderWellKnownUnavailable,
    #[serde(rename = "RelyingPartyWellKnownUnavailable")]
    RelyingPartyWellKnownUnavailable,
    #[serde(rename = "FederatedKeyThumbprintMismatch")]
    FederatedKeyThumbprintMismatch,
    #[serde(rename = "InvalidFederatedSessionUrl")]
    InvalidFederatedSessionUrl,
    #[serde(rename = "InvalidFederatedKey")]
    InvalidFederatedKey,
    #[serde(rename = "TooManyRelyingOriginLabels")]
    TooManyRelyingOriginLabels,
    #[serde(rename = "BoundCookieSetForbidden")]
    BoundCookieSetForbidden,
    #[serde(rename = "NetError")]
    NetError,
    #[serde(rename = "ProxyError")]
    ProxyError,
    #[serde(rename = "EmptySessionConfig")]
    EmptySessionConfig,
    #[serde(rename = "InvalidCredentialsConfig")]
    InvalidCredentialsConfig,
    #[serde(rename = "InvalidCredentialsType")]
    InvalidCredentialsType,
    #[serde(rename = "InvalidCredentialsEmptyName")]
    InvalidCredentialsEmptyName,
    #[serde(rename = "InvalidCredentialsCookie")]
    InvalidCredentialsCookie,
    #[serde(rename = "PersistentHttpError")]
    PersistentHttpError,
    #[serde(rename = "RegistrationAttemptedChallenge")]
    RegistrationAttemptedChallenge,
    #[serde(rename = "InvalidScopeOrigin")]
    InvalidScopeOrigin,
    #[serde(rename = "ScopeOriginContainsPath")]
    ScopeOriginContainsPath,
    #[serde(rename = "RefreshInitiatorNotString")]
    RefreshInitiatorNotString,
    #[serde(rename = "RefreshInitiatorInvalidHostPattern")]
    RefreshInitiatorInvalidHostPattern,
    #[serde(rename = "InvalidScopeSpecification")]
    InvalidScopeSpecification,
    #[serde(rename = "MissingScopeSpecificationType")]
    MissingScopeSpecificationType,
    #[serde(rename = "EmptyScopeSpecificationDomain")]
    EmptyScopeSpecificationDomain,
    #[serde(rename = "EmptyScopeSpecificationPath")]
    EmptyScopeSpecificationPath,
    #[serde(rename = "InvalidScopeSpecificationType")]
    InvalidScopeSpecificationType,
    #[serde(rename = "InvalidScopeIncludeSite")]
    InvalidScopeIncludeSite,
    #[serde(rename = "MissingScopeIncludeSite")]
    MissingScopeIncludeSite,
    #[serde(rename = "FederatedNotAuthorizedByProvider")]
    FederatedNotAuthorizedByProvider,
    #[serde(rename = "FederatedNotAuthorizedByRelyingParty")]
    FederatedNotAuthorizedByRelyingParty,
    #[serde(rename = "SessionProviderWellKnownMalformed")]
    SessionProviderWellKnownMalformed,
    #[serde(rename = "SessionProviderWellKnownHasProviderOrigin")]
    SessionProviderWellKnownHasProviderOrigin,
    #[serde(rename = "RelyingPartyWellKnownMalformed")]
    RelyingPartyWellKnownMalformed,
    #[serde(rename = "RelyingPartyWellKnownHasRelyingOrigins")]
    RelyingPartyWellKnownHasRelyingOrigins,
    #[serde(rename = "InvalidFederatedSessionProviderSessionMissing")]
    InvalidFederatedSessionProviderSessionMissing,
    #[serde(rename = "InvalidFederatedSessionWrongProviderOrigin")]
    InvalidFederatedSessionWrongProviderOrigin,
    #[serde(rename = "InvalidCredentialsCookieCreationTime")]
    InvalidCredentialsCookieCreationTime,
    #[serde(rename = "InvalidCredentialsCookieName")]
    InvalidCredentialsCookieName,
    #[serde(rename = "InvalidCredentialsCookieParsing")]
    InvalidCredentialsCookieParsing,
    #[serde(rename = "InvalidCredentialsCookieUnpermittedAttribute")]
    InvalidCredentialsCookieUnpermittedAttribute,
    #[serde(rename = "InvalidCredentialsCookieInvalidDomain")]
    InvalidCredentialsCookieInvalidDomain,
    #[serde(rename = "InvalidCredentialsCookiePrefix")]
    InvalidCredentialsCookiePrefix,
    #[serde(rename = "InvalidScopeRulePath")]
    InvalidScopeRulePath,
    #[serde(rename = "InvalidScopeRuleHostPattern")]
    InvalidScopeRuleHostPattern,
    #[serde(rename = "ScopeRuleOriginScopedHostPatternMismatch")]
    ScopeRuleOriginScopedHostPatternMismatch,
    #[serde(rename = "ScopeRuleSiteScopedHostPatternMismatch")]
    ScopeRuleSiteScopedHostPatternMismatch,
    #[serde(rename = "SigningQuotaExceeded")]
    SigningQuotaExceeded,
    #[serde(rename = "InvalidConfigJson")]
    InvalidConfigJson,
    #[serde(rename = "InvalidFederatedSessionProviderFailedToRestoreKey")]
    InvalidFederatedSessionProviderFailedToRestoreKey,
    #[serde(rename = "FailedToUnwrapKey")]
    FailedToUnwrapKey,
    #[serde(rename = "SessionDeletedDuringRefresh")]
    SessionDeletedDuringRefresh,
}
#[doc = "Details about a failed device bound session network request.\n[DeviceBoundSessionFailedRequest](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-DeviceBoundSessionFailedRequest)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceBoundSessionFailedRequest {
    #[doc = "The failed request URL."]
    #[serde(rename = "requestUrl")]
    pub request_url: String,
    #[doc = "The net error of the response if it was not OK."]
    #[serde(rename = "netError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub net_error: Option<String>,
    #[doc = "The response code if the net error was OK and the response code was not\n200."]
    #[serde(rename = "responseError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub response_error: Option<i64>,
    #[doc = "The body of the response if the net error was OK, the response code was\nnot 200, and the response body was not empty."]
    #[serde(rename = "responseErrorBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub response_error_body: Option<String>,
}
impl DeviceBoundSessionFailedRequest {
    pub fn new(request_url: impl Into<String>) -> Self {
        Self {
            request_url: request_url.into(),
            net_error: None,
            response_error: None,
            response_error_body: None,
        }
    }
}
impl<T: Into<String>> From<T> for DeviceBoundSessionFailedRequest {
    fn from(url: T) -> Self {
        DeviceBoundSessionFailedRequest::new(url)
    }
}
impl DeviceBoundSessionFailedRequest {
    pub const IDENTIFIER: &'static str = "Network.DeviceBoundSessionFailedRequest";
}
#[doc = "Session event details specific to creation.\n[CreationEventDetails](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-CreationEventDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreationEventDetails {
    #[doc = "The result of the fetch attempt."]
    #[serde(rename = "fetchResult")]
    pub fetch_result: DeviceBoundSessionFetchResult,
    #[doc = "The session if there was a newly created session. This is populated for\nall successful creation events."]
    #[serde(rename = "newSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub new_session: Option<DeviceBoundSession>,
    #[doc = "Details about a failed device bound session network request if there was\none."]
    #[serde(rename = "failedRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub failed_request: Option<DeviceBoundSessionFailedRequest>,
}
impl CreationEventDetails {
    pub fn new(fetch_result: impl Into<DeviceBoundSessionFetchResult>) -> Self {
        Self {
            fetch_result: fetch_result.into(),
            new_session: None,
            failed_request: None,
        }
    }
}
impl CreationEventDetails {
    pub const IDENTIFIER: &'static str = "Network.CreationEventDetails";
}
#[doc = "Session event details specific to refresh.\n[RefreshEventDetails](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-RefreshEventDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefreshEventDetails {
    #[doc = "The result of a refresh."]
    #[serde(rename = "refreshResult")]
    pub refresh_result: RefreshEventDetailsRefreshResult,
    #[doc = "If there was a fetch attempt, the result of that."]
    #[serde(rename = "fetchResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fetch_result: Option<DeviceBoundSessionFetchResult>,
    #[doc = "The session display if there was a newly created session. This is populated\nfor any refresh event that modifies the session config."]
    #[serde(rename = "newSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub new_session: Option<DeviceBoundSession>,
    #[doc = "See comments on `net::device_bound_sessions::RefreshEventResult::was_fully_proactive_refresh`."]
    #[serde(rename = "wasFullyProactiveRefresh")]
    pub was_fully_proactive_refresh: bool,
    #[doc = "Details about a failed device bound session network request if there was\none."]
    #[serde(rename = "failedRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub failed_request: Option<DeviceBoundSessionFailedRequest>,
}
#[doc = "The result of a refresh."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RefreshEventDetailsRefreshResult {
    #[serde(rename = "Refreshed")]
    Refreshed,
    #[serde(rename = "InitializedService")]
    InitializedService,
    #[serde(rename = "Unreachable")]
    Unreachable,
    #[serde(rename = "ServerError")]
    ServerError,
    #[serde(rename = "RefreshQuotaExceeded")]
    RefreshQuotaExceeded,
    #[serde(rename = "FatalError")]
    FatalError,
    #[serde(rename = "SigningQuotaExceeded")]
    SigningQuotaExceeded,
}
impl RefreshEventDetails {
    pub fn new(
        refresh_result: impl Into<RefreshEventDetailsRefreshResult>,
        was_fully_proactive_refresh: impl Into<bool>,
    ) -> Self {
        Self {
            refresh_result: refresh_result.into(),
            was_fully_proactive_refresh: was_fully_proactive_refresh.into(),
            fetch_result: None,
            new_session: None,
            failed_request: None,
        }
    }
}
impl RefreshEventDetails {
    pub const IDENTIFIER: &'static str = "Network.RefreshEventDetails";
}
#[doc = "Session event details specific to termination.\n[TerminationEventDetails](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-TerminationEventDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerminationEventDetails {
    #[doc = "The reason for a session being deleted."]
    #[serde(rename = "deletionReason")]
    pub deletion_reason: TerminationEventDetailsDeletionReason,
}
#[doc = "The reason for a session being deleted."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerminationEventDetailsDeletionReason {
    #[serde(rename = "Expired")]
    Expired,
    #[serde(rename = "FailedToRestoreKey")]
    FailedToRestoreKey,
    #[serde(rename = "FailedToUnwrapKey")]
    FailedToUnwrapKey,
    #[serde(rename = "StoragePartitionCleared")]
    StoragePartitionCleared,
    #[serde(rename = "ClearBrowsingData")]
    ClearBrowsingData,
    #[serde(rename = "ServerRequested")]
    ServerRequested,
    #[serde(rename = "InvalidSessionParams")]
    InvalidSessionParams,
    #[serde(rename = "RefreshFatalError")]
    RefreshFatalError,
}
impl TerminationEventDetails {
    pub fn new(deletion_reason: impl Into<TerminationEventDetailsDeletionReason>) -> Self {
        Self {
            deletion_reason: deletion_reason.into(),
        }
    }
}
impl TerminationEventDetails {
    pub const IDENTIFIER: &'static str = "Network.TerminationEventDetails";
}
#[doc = "Session event details specific to challenges.\n[ChallengeEventDetails](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-ChallengeEventDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChallengeEventDetails {
    #[doc = "The result of a challenge."]
    #[serde(rename = "challengeResult")]
    pub challenge_result: ChallengeEventDetailsChallengeResult,
    #[doc = "The challenge set."]
    #[serde(rename = "challenge")]
    pub challenge: String,
}
#[doc = "The result of a challenge."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChallengeEventDetailsChallengeResult {
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "NoSessionId")]
    NoSessionId,
    #[serde(rename = "NoSessionMatch")]
    NoSessionMatch,
    #[serde(rename = "CantSetBoundCookie")]
    CantSetBoundCookie,
}
impl ChallengeEventDetails {
    pub fn new(
        challenge_result: impl Into<ChallengeEventDetailsChallengeResult>,
        challenge: impl Into<String>,
    ) -> Self {
        Self {
            challenge_result: challenge_result.into(),
            challenge: challenge.into(),
        }
    }
}
impl ChallengeEventDetails {
    pub const IDENTIFIER: &'static str = "Network.ChallengeEventDetails";
}
#[doc = "An object providing the result of a network resource load.\n[LoadNetworkResourcePageResult](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-LoadNetworkResourcePageResult)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadNetworkResourcePageResult {
    #[serde(rename = "success")]
    pub success: bool,
    #[doc = "Optional values used for error reporting."]
    #[serde(rename = "netError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub net_error: Option<f64>,
    #[serde(rename = "netErrorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub net_error_name: Option<String>,
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub http_status_code: Option<f64>,
    #[doc = "If successful, one of the following two fields holds the result."]
    #[serde(rename = "stream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stream: Option<super::super::io::types::StreamHandle>,
    #[doc = "Response headers."]
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub headers: Option<Headers>,
}
impl LoadNetworkResourcePageResult {
    pub fn new(success: impl Into<bool>) -> Self {
        Self {
            success: success.into(),
            net_error: None,
            net_error_name: None,
            http_status_code: None,
            stream: None,
            headers: None,
        }
    }
}
impl LoadNetworkResourcePageResult {
    pub const IDENTIFIER: &'static str = "Network.LoadNetworkResourcePageResult";
}
#[doc = "An options object that may be extended later to better support CORS,\nCORB and streaming.\n[LoadNetworkResourceOptions](https://chromedevtools.github.io/devtools-protocol/tot/Network/#type-LoadNetworkResourceOptions)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadNetworkResourceOptions {
    #[serde(rename = "disableCache")]
    pub disable_cache: bool,
    #[serde(rename = "includeCredentials")]
    pub include_credentials: bool,
}
impl LoadNetworkResourceOptions {
    pub fn new(disable_cache: impl Into<bool>, include_credentials: impl Into<bool>) -> Self {
        Self {
            disable_cache: disable_cache.into(),
            include_credentials: include_credentials.into(),
        }
    }
}
impl LoadNetworkResourceOptions {
    pub const IDENTIFIER: &'static str = "Network.LoadNetworkResourceOptions";
}
group_enum ! (NetworkTypes { ResourceType (ResourceType) , LoaderId (LoaderId) , RequestId (RequestId) , InterceptionId (InterceptionId) , ErrorReason (ErrorReason) , TimeSinceEpoch (TimeSinceEpoch) , MonotonicTime (MonotonicTime) , Headers (Headers) , ConnectionType (ConnectionType) , CookieSameSite (CookieSameSite) , CookiePriority (CookiePriority) , CookieSourceScheme (CookieSourceScheme) , ResourceTiming (ResourceTiming) , ResourcePriority (ResourcePriority) , RenderBlockingBehavior (RenderBlockingBehavior) , PostDataEntry (PostDataEntry) , Request (Request) , SignedCertificateTimestamp (SignedCertificateTimestamp) , SecurityDetails (SecurityDetails) , CertificateTransparencyCompliance (CertificateTransparencyCompliance) , BlockedReason (BlockedReason) , CorsError (CorsError) , CorsErrorStatus (CorsErrorStatus) , ServiceWorkerResponseSource (ServiceWorkerResponseSource) , TrustTokenParams (TrustTokenParams) , TrustTokenOperationType (TrustTokenOperationType) , AlternateProtocolUsage (AlternateProtocolUsage) , ServiceWorkerRouterSource (ServiceWorkerRouterSource) , ServiceWorkerRouterInfo (ServiceWorkerRouterInfo) , Response (Response) , WebSocketRequest (WebSocketRequest) , WebSocketResponse (WebSocketResponse) , WebSocketFrame (WebSocketFrame) , CachedResource (CachedResource) , Initiator (Initiator) , CookiePartitionKey (CookiePartitionKey) , Cookie (Cookie) , SetCookieBlockedReason (SetCookieBlockedReason) , CookieBlockedReason (CookieBlockedReason) , CookieExemptionReason (CookieExemptionReason) , BlockedSetCookieWithReason (BlockedSetCookieWithReason) , ExemptedSetCookieWithReason (ExemptedSetCookieWithReason) , AssociatedCookie (AssociatedCookie) , CookieParam (CookieParam) , AuthChallenge (AuthChallenge) , AuthChallengeResponse (AuthChallengeResponse) , InterceptionStage (InterceptionStage) , RequestPattern (RequestPattern) , SignedExchangeSignature (SignedExchangeSignature) , SignedExchangeHeader (SignedExchangeHeader) , SignedExchangeErrorField (SignedExchangeErrorField) , SignedExchangeError (SignedExchangeError) , SignedExchangeInfo (SignedExchangeInfo) , ContentEncoding (ContentEncoding) , NetworkConditions (NetworkConditions) , BlockPattern (BlockPattern) , DirectSocketDnsQueryType (DirectSocketDnsQueryType) , DirectTcpSocketOptions (DirectTcpSocketOptions) , DirectUdpSocketOptions (DirectUdpSocketOptions) , DirectUdpMessage (DirectUdpMessage) , LocalNetworkAccessRequestPolicy (LocalNetworkAccessRequestPolicy) , IpAddressSpace (IpAddressSpace) , ConnectTiming (ConnectTiming) , ClientSecurityState (ClientSecurityState) , CrossOriginOpenerPolicyValue (CrossOriginOpenerPolicyValue) , CrossOriginOpenerPolicyStatus (CrossOriginOpenerPolicyStatus) , CrossOriginEmbedderPolicyValue (CrossOriginEmbedderPolicyValue) , CrossOriginEmbedderPolicyStatus (CrossOriginEmbedderPolicyStatus) , ContentSecurityPolicySource (ContentSecurityPolicySource) , ContentSecurityPolicyStatus (ContentSecurityPolicyStatus) , SecurityIsolationStatus (SecurityIsolationStatus) , ReportStatus (ReportStatus) , ReportId (ReportId) , ReportingApiReport (ReportingApiReport) , ReportingApiEndpoint (ReportingApiEndpoint) , DeviceBoundSessionKey (DeviceBoundSessionKey) , DeviceBoundSessionWithUsage (DeviceBoundSessionWithUsage) , DeviceBoundSessionCookieCraving (DeviceBoundSessionCookieCraving) , DeviceBoundSessionUrlRule (DeviceBoundSessionUrlRule) , DeviceBoundSessionInclusionRules (DeviceBoundSessionInclusionRules) , DeviceBoundSession (DeviceBoundSession) , DeviceBoundSessionEventId (DeviceBoundSessionEventId) , DeviceBoundSessionFetchResult (DeviceBoundSessionFetchResult) , DeviceBoundSessionFailedRequest (DeviceBoundSessionFailedRequest) , CreationEventDetails (CreationEventDetails) , RefreshEventDetails (RefreshEventDetails) , TerminationEventDetails (TerminationEventDetails) , ChallengeEventDetails (ChallengeEventDetails) , LoadNetworkResourcePageResult (LoadNetworkResourcePageResult) , LoadNetworkResourceOptions (LoadNetworkResourceOptions) });
