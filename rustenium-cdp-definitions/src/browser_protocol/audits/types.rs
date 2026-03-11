use serde::{Deserialize, Serialize};
#[doc = "Information about a cookie that is affected by an inspector issue.\n[AffectedCookie](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-AffectedCookie)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectedCookie {
    #[doc = "The following three properties uniquely identify a cookie"]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "domain")]
    pub domain: String,
}
impl AffectedCookie {
    pub fn new(
        name: impl Into<String>,
        path: impl Into<String>,
        domain: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            domain: domain.into(),
        }
    }
}
impl AffectedCookie {
    pub const IDENTIFIER: &'static str = "Audits.AffectedCookie";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Information about a request that is affected by an inspector issue.\n[AffectedRequest](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-AffectedRequest)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectedRequest {
    #[doc = "The unique request id."]
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request_id: Option<crate::browser_protocol::network::types::RequestId>,
    #[serde(rename = "url")]
    pub url: String,
}
impl AffectedRequest {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            request_id: None,
        }
    }
}
impl<T: Into<String>> From<T> for AffectedRequest {
    fn from(url: T) -> Self {
        AffectedRequest::new(url)
    }
}
impl AffectedRequest {
    pub const IDENTIFIER: &'static str = "Audits.AffectedRequest";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Information about the frame affected by an inspector issue.\n[AffectedFrame](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-AffectedFrame)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectedFrame {
    #[serde(rename = "frameId")]
    pub frame_id: crate::browser_protocol::page::types::FrameId,
}
impl AffectedFrame {
    pub fn new(frame_id: impl Into<crate::browser_protocol::page::types::FrameId>) -> Self {
        Self {
            frame_id: frame_id.into(),
        }
    }
}
impl AffectedFrame {
    pub const IDENTIFIER: &'static str = "Audits.AffectedFrame";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CookieExclusionReason {
    #[serde(rename = "ExcludeSameSiteUnspecifiedTreatedAsLax")]
    ExcludeSameSiteUnspecifiedTreatedAsLax,
    #[serde(rename = "ExcludeSameSiteNoneInsecure")]
    ExcludeSameSiteNoneInsecure,
    #[serde(rename = "ExcludeSameSiteLax")]
    ExcludeSameSiteLax,
    #[serde(rename = "ExcludeSameSiteStrict")]
    ExcludeSameSiteStrict,
    #[serde(rename = "ExcludeDomainNonASCII")]
    ExcludeDomainNonAscii,
    #[serde(rename = "ExcludeThirdPartyCookieBlockedInFirstPartySet")]
    ExcludeThirdPartyCookieBlockedInFirstPartySet,
    #[serde(rename = "ExcludeThirdPartyPhaseout")]
    ExcludeThirdPartyPhaseout,
    #[serde(rename = "ExcludePortMismatch")]
    ExcludePortMismatch,
    #[serde(rename = "ExcludeSchemeMismatch")]
    ExcludeSchemeMismatch,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CookieWarningReason {
    #[serde(rename = "WarnSameSiteUnspecifiedCrossSiteContext")]
    WarnSameSiteUnspecifiedCrossSiteContext,
    #[serde(rename = "WarnSameSiteNoneInsecure")]
    WarnSameSiteNoneInsecure,
    #[serde(rename = "WarnSameSiteUnspecifiedLaxAllowUnsafe")]
    WarnSameSiteUnspecifiedLaxAllowUnsafe,
    #[serde(rename = "WarnSameSiteStrictLaxDowngradeStrict")]
    WarnSameSiteStrictLaxDowngradeStrict,
    #[serde(rename = "WarnSameSiteStrictCrossDowngradeStrict")]
    WarnSameSiteStrictCrossDowngradeStrict,
    #[serde(rename = "WarnSameSiteStrictCrossDowngradeLax")]
    WarnSameSiteStrictCrossDowngradeLax,
    #[serde(rename = "WarnSameSiteLaxCrossDowngradeStrict")]
    WarnSameSiteLaxCrossDowngradeStrict,
    #[serde(rename = "WarnSameSiteLaxCrossDowngradeLax")]
    WarnSameSiteLaxCrossDowngradeLax,
    #[serde(rename = "WarnAttributeValueExceedsMaxSize")]
    WarnAttributeValueExceedsMaxSize,
    #[serde(rename = "WarnDomainNonASCII")]
    WarnDomainNonAscii,
    #[serde(rename = "WarnThirdPartyPhaseout")]
    WarnThirdPartyPhaseout,
    #[serde(rename = "WarnCrossSiteRedirectDowngradeChangesInclusion")]
    WarnCrossSiteRedirectDowngradeChangesInclusion,
    #[serde(rename = "WarnDeprecationTrialMetadata")]
    WarnDeprecationTrialMetadata,
    #[serde(rename = "WarnThirdPartyCookieHeuristic")]
    WarnThirdPartyCookieHeuristic,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CookieOperation {
    #[serde(rename = "SetCookie")]
    SetCookie,
    #[serde(rename = "ReadCookie")]
    ReadCookie,
}
#[doc = "Represents the category of insight that a cookie issue falls under."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InsightType {
    #[doc = "Cookie domain has an entry in third-party cookie migration readiness\nlist:\nhttps://github.com/privacysandbox/privacy-sandbox-dev-support/blob/main/3pc-migration-readiness.md"]
    #[serde(rename = "GitHubResource")]
    GitHubResource,
    #[doc = "Cookie is exempted due to a grace period:\nhttps://developers.google.com/privacy-sandbox/cookies/temporary-exceptions/grace-period"]
    #[serde(rename = "GracePeriod")]
    GracePeriod,
    #[doc = "Cookie is exempted due a heuristics-based exemptiuon:\nhttps://developers.google.com/privacy-sandbox/cookies/temporary-exceptions/heuristics-based-exception"]
    #[serde(rename = "Heuristics")]
    Heuristics,
}
#[doc = "Information about the suggested solution to a cookie issue.\n[CookieIssueInsight](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-CookieIssueInsight)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookieIssueInsight {
    #[serde(rename = "type")]
    pub r#type: InsightType,
    #[doc = "Link to table entry in third-party cookie migration readiness list."]
    #[serde(rename = "tableEntryUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub table_entry_url: Option<String>,
}
impl CookieIssueInsight {
    pub fn new(r#type: impl Into<InsightType>) -> Self {
        Self {
            r#type: r#type.into(),
            table_entry_url: None,
        }
    }
}
impl CookieIssueInsight {
    pub const IDENTIFIER: &'static str = "Audits.CookieIssueInsight";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "This information is currently necessary, as the front-end has a difficult\ntime finding a specific cookie. With this, we can convey specific error\ninformation without the cookie.\n[CookieIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-CookieIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookieIssueDetails {
    #[doc = "If AffectedCookie is not set then rawCookieLine contains the raw\nSet-Cookie header string. This hints at a problem where the\ncookie line is syntactically or semantically malformed in a way\nthat no valid cookie could be created."]
    #[serde(rename = "cookie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cookie: Option<AffectedCookie>,
    #[serde(rename = "rawCookieLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub raw_cookie_line: Option<String>,
    #[serde(rename = "cookieWarningReasons")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookie_warning_reasons: Vec<CookieWarningReason>,
    #[serde(rename = "cookieExclusionReasons")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookie_exclusion_reasons: Vec<CookieExclusionReason>,
    #[doc = "Optionally identifies the site-for-cookies and the cookie url, which\nmay be used by the front-end as additional context."]
    #[serde(rename = "operation")]
    pub operation: CookieOperation,
    #[serde(rename = "siteForCookies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub site_for_cookies: Option<String>,
    #[serde(rename = "cookieUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cookie_url: Option<String>,
    #[serde(rename = "request")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request: Option<AffectedRequest>,
    #[doc = "The recommended solution to the issue."]
    #[serde(rename = "insight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub insight: Option<CookieIssueInsight>,
}
impl CookieIssueDetails {
    pub fn new(
        cookie_warning_reasons: Vec<CookieWarningReason>,
        cookie_exclusion_reasons: Vec<CookieExclusionReason>,
        operation: impl Into<CookieOperation>,
    ) -> Self {
        Self {
            cookie_warning_reasons,
            cookie_exclusion_reasons,
            operation: operation.into(),
            cookie: None,
            raw_cookie_line: None,
            site_for_cookies: None,
            cookie_url: None,
            request: None,
            insight: None,
        }
    }
}
impl CookieIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.CookieIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerformanceIssueType {
    #[serde(rename = "DocumentCookie")]
    DocumentCookie,
}
#[doc = "Details for a performance issue.\n[PerformanceIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-PerformanceIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceIssueDetails {
    #[serde(rename = "performanceIssueType")]
    pub performance_issue_type: PerformanceIssueType,
    #[serde(rename = "sourceCodeLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_code_location: Option<SourceCodeLocation>,
}
impl PerformanceIssueDetails {
    pub fn new(performance_issue_type: impl Into<PerformanceIssueType>) -> Self {
        Self {
            performance_issue_type: performance_issue_type.into(),
            source_code_location: None,
        }
    }
}
impl PerformanceIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.PerformanceIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MixedContentResolutionStatus {
    #[serde(rename = "MixedContentBlocked")]
    MixedContentBlocked,
    #[serde(rename = "MixedContentAutomaticallyUpgraded")]
    MixedContentAutomaticallyUpgraded,
    #[serde(rename = "MixedContentWarning")]
    MixedContentWarning,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MixedContentResourceType {
    #[serde(rename = "AttributionSrc")]
    AttributionSrc,
    #[serde(rename = "Audio")]
    Audio,
    #[serde(rename = "Beacon")]
    Beacon,
    #[serde(rename = "CSPReport")]
    CspReport,
    #[serde(rename = "Download")]
    Download,
    #[serde(rename = "EventSource")]
    EventSource,
    #[serde(rename = "Favicon")]
    Favicon,
    #[serde(rename = "Font")]
    Font,
    #[serde(rename = "Form")]
    Form,
    #[serde(rename = "Frame")]
    Frame,
    #[serde(rename = "Image")]
    Image,
    #[serde(rename = "Import")]
    Import,
    #[serde(rename = "JSON")]
    Json,
    #[serde(rename = "Manifest")]
    Manifest,
    #[serde(rename = "Ping")]
    Ping,
    #[serde(rename = "PluginData")]
    PluginData,
    #[serde(rename = "PluginResource")]
    PluginResource,
    #[serde(rename = "Prefetch")]
    Prefetch,
    #[serde(rename = "Resource")]
    Resource,
    #[serde(rename = "Script")]
    Script,
    #[serde(rename = "ServiceWorker")]
    ServiceWorker,
    #[serde(rename = "SharedWorker")]
    SharedWorker,
    #[serde(rename = "SpeculationRules")]
    SpeculationRules,
    #[serde(rename = "Stylesheet")]
    Stylesheet,
    #[serde(rename = "Track")]
    Track,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Worker")]
    Worker,
    #[serde(rename = "XMLHttpRequest")]
    XmlHttpRequest,
    #[serde(rename = "XSLT")]
    Xslt,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MixedContentIssueDetails {
    #[doc = "The type of resource causing the mixed content issue (css, js, iframe,\nform,...). Marked as optional because it is mapped to from\nblink::mojom::RequestContextType, which will be replaced\nby network::mojom::RequestDestination"]
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub resource_type: Option<MixedContentResourceType>,
    #[doc = "The way the mixed content issue is being resolved."]
    #[serde(rename = "resolutionStatus")]
    pub resolution_status: MixedContentResolutionStatus,
    #[doc = "The unsafe http url causing the mixed content issue."]
    #[serde(rename = "insecureURL")]
    pub insecure_url: String,
    #[doc = "The url responsible for the call to an unsafe url."]
    #[serde(rename = "mainResourceURL")]
    pub main_resource_url: String,
    #[doc = "The mixed content request.\nDoes not always exist (e.g. for unsafe form submission urls)."]
    #[serde(rename = "request")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request: Option<AffectedRequest>,
    #[doc = "Optional because not every mixed content issue is necessarily linked to a frame."]
    #[serde(rename = "frame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame: Option<AffectedFrame>,
}
impl MixedContentIssueDetails {
    pub fn new(
        resolution_status: impl Into<MixedContentResolutionStatus>,
        insecure_url: impl Into<String>,
        main_resource_url: impl Into<String>,
    ) -> Self {
        Self {
            resolution_status: resolution_status.into(),
            insecure_url: insecure_url.into(),
            main_resource_url: main_resource_url.into(),
            resource_type: None,
            request: None,
            frame: None,
        }
    }
}
impl MixedContentIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.MixedContentIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Enum indicating the reason a response has been blocked. These reasons are\nrefinements of the net error BLOCKED_BY_RESPONSE."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlockedByResponseReason {
    #[serde(rename = "CoepFrameResourceNeedsCoepHeader")]
    CoepFrameResourceNeedsCoepHeader,
    #[serde(rename = "CoopSandboxedIFrameCannotNavigateToCoopPage")]
    CoopSandboxedIFrameCannotNavigateToCoopPage,
    #[serde(rename = "CorpNotSameOrigin")]
    CorpNotSameOrigin,
    #[serde(rename = "CorpNotSameOriginAfterDefaultedToSameOriginByCoep")]
    CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
    #[serde(rename = "CorpNotSameOriginAfterDefaultedToSameOriginByDip")]
    CorpNotSameOriginAfterDefaultedToSameOriginByDip,
    #[serde(rename = "CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip")]
    CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip,
    #[serde(rename = "CorpNotSameSite")]
    CorpNotSameSite,
    #[serde(rename = "SRIMessageSignatureMismatch")]
    SriMessageSignatureMismatch,
}
#[doc = "Details for a request that has been blocked with the BLOCKED_BY_RESPONSE\ncode. Currently only used for COEP/COOP, but may be extended to include\nsome CSP errors in the future.\n[BlockedByResponseIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-BlockedByResponseIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockedByResponseIssueDetails {
    #[serde(rename = "request")]
    pub request: AffectedRequest,
    #[serde(rename = "parentFrame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent_frame: Option<AffectedFrame>,
    #[serde(rename = "blockedFrame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub blocked_frame: Option<AffectedFrame>,
    #[serde(rename = "reason")]
    pub reason: BlockedByResponseReason,
}
impl BlockedByResponseIssueDetails {
    pub fn new(
        request: impl Into<AffectedRequest>,
        reason: impl Into<BlockedByResponseReason>,
    ) -> Self {
        Self {
            request: request.into(),
            reason: reason.into(),
            parent_frame: None,
            blocked_frame: None,
        }
    }
}
impl BlockedByResponseIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.BlockedByResponseIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeavyAdResolutionStatus {
    #[serde(rename = "HeavyAdBlocked")]
    HeavyAdBlocked,
    #[serde(rename = "HeavyAdWarning")]
    HeavyAdWarning,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeavyAdReason {
    #[serde(rename = "NetworkTotalLimit")]
    NetworkTotalLimit,
    #[serde(rename = "CpuTotalLimit")]
    CpuTotalLimit,
    #[serde(rename = "CpuPeakLimit")]
    CpuPeakLimit,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeavyAdIssueDetails {
    #[doc = "The resolution status, either blocking the content or warning."]
    #[serde(rename = "resolution")]
    pub resolution: HeavyAdResolutionStatus,
    #[doc = "The reason the ad was blocked, total network or cpu or peak cpu."]
    #[serde(rename = "reason")]
    pub reason: HeavyAdReason,
    #[doc = "The frame that was blocked."]
    #[serde(rename = "frame")]
    pub frame: AffectedFrame,
}
impl HeavyAdIssueDetails {
    pub fn new(
        resolution: impl Into<HeavyAdResolutionStatus>,
        reason: impl Into<HeavyAdReason>,
        frame: impl Into<AffectedFrame>,
    ) -> Self {
        Self {
            resolution: resolution.into(),
            reason: reason.into(),
            frame: frame.into(),
        }
    }
}
impl HeavyAdIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.HeavyAdIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentSecurityPolicyViolationType {
    #[serde(rename = "kInlineViolation")]
    KInlineViolation,
    #[serde(rename = "kEvalViolation")]
    KEvalViolation,
    #[serde(rename = "kURLViolation")]
    KUrlViolation,
    #[serde(rename = "kSRIViolation")]
    KSriViolation,
    #[serde(rename = "kTrustedTypesSinkViolation")]
    KTrustedTypesSinkViolation,
    #[serde(rename = "kTrustedTypesPolicyViolation")]
    KTrustedTypesPolicyViolation,
    #[serde(rename = "kWasmEvalViolation")]
    KWasmEvalViolation,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceCodeLocation {
    #[serde(rename = "scriptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub script_id: Option<crate::js_protocol::runtime::types::ScriptId>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    #[serde(rename = "columnNumber")]
    pub column_number: i64,
}
impl SourceCodeLocation {
    pub fn new(
        url: impl Into<String>,
        line_number: impl Into<i64>,
        column_number: impl Into<i64>,
    ) -> Self {
        Self {
            url: url.into(),
            line_number: line_number.into(),
            column_number: column_number.into(),
            script_id: None,
        }
    }
}
impl SourceCodeLocation {
    pub const IDENTIFIER: &'static str = "Audits.SourceCodeLocation";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentSecurityPolicyIssueDetails {
    #[doc = "The url not included in allowed sources."]
    #[serde(rename = "blockedURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub blocked_url: Option<String>,
    #[doc = "Specific directive that is violated, causing the CSP issue."]
    #[serde(rename = "violatedDirective")]
    pub violated_directive: String,
    #[serde(rename = "isReportOnly")]
    pub is_report_only: bool,
    #[serde(rename = "contentSecurityPolicyViolationType")]
    pub content_security_policy_violation_type: ContentSecurityPolicyViolationType,
    #[serde(rename = "frameAncestor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_ancestor: Option<AffectedFrame>,
    #[serde(rename = "sourceCodeLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_code_location: Option<SourceCodeLocation>,
    #[serde(rename = "violatingNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub violating_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
}
impl ContentSecurityPolicyIssueDetails {
    pub fn new(
        violated_directive: impl Into<String>,
        is_report_only: impl Into<bool>,
        content_security_policy_violation_type: impl Into<ContentSecurityPolicyViolationType>,
    ) -> Self {
        Self {
            violated_directive: violated_directive.into(),
            is_report_only: is_report_only.into(),
            content_security_policy_violation_type: content_security_policy_violation_type.into(),
            blocked_url: None,
            frame_ancestor: None,
            source_code_location: None,
            violating_node_id: None,
        }
    }
}
impl ContentSecurityPolicyIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.ContentSecurityPolicyIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SharedArrayBufferIssueType {
    #[serde(rename = "TransferIssue")]
    TransferIssue,
    #[serde(rename = "CreationIssue")]
    CreationIssue,
}
#[doc = "Details for a issue arising from an SAB being instantiated in, or\ntransferred to a context that is not cross-origin isolated.\n[SharedArrayBufferIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-SharedArrayBufferIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedArrayBufferIssueDetails {
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: SourceCodeLocation,
    #[serde(rename = "isWarning")]
    pub is_warning: bool,
    #[serde(rename = "type")]
    pub r#type: SharedArrayBufferIssueType,
}
impl SharedArrayBufferIssueDetails {
    pub fn new(
        source_code_location: impl Into<SourceCodeLocation>,
        is_warning: impl Into<bool>,
        r#type: impl Into<SharedArrayBufferIssueType>,
    ) -> Self {
        Self {
            source_code_location: source_code_location.into(),
            is_warning: is_warning.into(),
            r#type: r#type.into(),
        }
    }
}
impl SharedArrayBufferIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.SharedArrayBufferIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LowTextContrastIssueDetails {
    #[serde(rename = "violatingNodeId")]
    pub violating_node_id: crate::browser_protocol::dom::types::BackendNodeId,
    #[serde(rename = "violatingNodeSelector")]
    pub violating_node_selector: String,
    #[serde(rename = "contrastRatio")]
    pub contrast_ratio: f64,
    #[serde(rename = "thresholdAA")]
    pub threshold_aa: f64,
    #[serde(rename = "thresholdAAA")]
    pub threshold_aaa: f64,
    #[serde(rename = "fontSize")]
    pub font_size: String,
    #[serde(rename = "fontWeight")]
    pub font_weight: String,
}
impl LowTextContrastIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.LowTextContrastIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Details for a CORS related issue, e.g. a warning or error related to\nCORS RFC1918 enforcement.\n[CorsIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-CorsIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorsIssueDetails {
    #[serde(rename = "corsErrorStatus")]
    pub cors_error_status: crate::browser_protocol::network::types::CorsErrorStatus,
    #[serde(rename = "isWarning")]
    pub is_warning: bool,
    #[serde(rename = "request")]
    pub request: AffectedRequest,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub location: Option<SourceCodeLocation>,
    #[serde(rename = "initiatorOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub initiator_origin: Option<String>,
    #[serde(rename = "resourceIPAddressSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub resource_ip_address_space: Option<crate::browser_protocol::network::types::IpAddressSpace>,
    #[serde(rename = "clientSecurityState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub client_security_state: Option<crate::browser_protocol::network::types::ClientSecurityState>,
}
impl CorsIssueDetails {
    pub fn new(
        cors_error_status: impl Into<crate::browser_protocol::network::types::CorsErrorStatus>,
        is_warning: impl Into<bool>,
        request: impl Into<AffectedRequest>,
    ) -> Self {
        Self {
            cors_error_status: cors_error_status.into(),
            is_warning: is_warning.into(),
            request: request.into(),
            location: None,
            initiator_origin: None,
            resource_ip_address_space: None,
            client_security_state: None,
        }
    }
}
impl CorsIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.CorsIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributionReportingIssueType {
    #[serde(rename = "PermissionPolicyDisabled")]
    PermissionPolicyDisabled,
    #[serde(rename = "UntrustworthyReportingOrigin")]
    UntrustworthyReportingOrigin,
    #[serde(rename = "InsecureContext")]
    InsecureContext,
    #[doc = "TODO(apaseltiner): Rename this to InvalidRegisterSourceHeader"]
    #[serde(rename = "InvalidHeader")]
    InvalidHeader,
    #[serde(rename = "InvalidRegisterTriggerHeader")]
    InvalidRegisterTriggerHeader,
    #[serde(rename = "SourceAndTriggerHeaders")]
    SourceAndTriggerHeaders,
    #[serde(rename = "SourceIgnored")]
    SourceIgnored,
    #[serde(rename = "TriggerIgnored")]
    TriggerIgnored,
    #[serde(rename = "OsSourceIgnored")]
    OsSourceIgnored,
    #[serde(rename = "OsTriggerIgnored")]
    OsTriggerIgnored,
    #[serde(rename = "InvalidRegisterOsSourceHeader")]
    InvalidRegisterOsSourceHeader,
    #[serde(rename = "InvalidRegisterOsTriggerHeader")]
    InvalidRegisterOsTriggerHeader,
    #[serde(rename = "WebAndOsHeaders")]
    WebAndOsHeaders,
    #[serde(rename = "NoWebOrOsSupport")]
    NoWebOrOsSupport,
    #[serde(rename = "NavigationRegistrationWithoutTransientUserActivation")]
    NavigationRegistrationWithoutTransientUserActivation,
    #[serde(rename = "InvalidInfoHeader")]
    InvalidInfoHeader,
    #[serde(rename = "NoRegisterSourceHeader")]
    NoRegisterSourceHeader,
    #[serde(rename = "NoRegisterTriggerHeader")]
    NoRegisterTriggerHeader,
    #[serde(rename = "NoRegisterOsSourceHeader")]
    NoRegisterOsSourceHeader,
    #[serde(rename = "NoRegisterOsTriggerHeader")]
    NoRegisterOsTriggerHeader,
    #[serde(rename = "NavigationRegistrationUniqueScopeAlreadySet")]
    NavigationRegistrationUniqueScopeAlreadySet,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SharedDictionaryError {
    #[serde(rename = "UseErrorCrossOriginNoCorsRequest")]
    UseErrorCrossOriginNoCorsRequest,
    #[serde(rename = "UseErrorDictionaryLoadFailure")]
    UseErrorDictionaryLoadFailure,
    #[serde(rename = "UseErrorMatchingDictionaryNotUsed")]
    UseErrorMatchingDictionaryNotUsed,
    #[serde(rename = "UseErrorUnexpectedContentDictionaryHeader")]
    UseErrorUnexpectedContentDictionaryHeader,
    #[serde(rename = "WriteErrorCossOriginNoCorsRequest")]
    WriteErrorCossOriginNoCorsRequest,
    #[serde(rename = "WriteErrorDisallowedBySettings")]
    WriteErrorDisallowedBySettings,
    #[serde(rename = "WriteErrorExpiredResponse")]
    WriteErrorExpiredResponse,
    #[serde(rename = "WriteErrorFeatureDisabled")]
    WriteErrorFeatureDisabled,
    #[serde(rename = "WriteErrorInsufficientResources")]
    WriteErrorInsufficientResources,
    #[serde(rename = "WriteErrorInvalidMatchField")]
    WriteErrorInvalidMatchField,
    #[serde(rename = "WriteErrorInvalidStructuredHeader")]
    WriteErrorInvalidStructuredHeader,
    #[serde(rename = "WriteErrorInvalidTTLField")]
    WriteErrorInvalidTtlField,
    #[serde(rename = "WriteErrorNavigationRequest")]
    WriteErrorNavigationRequest,
    #[serde(rename = "WriteErrorNoMatchField")]
    WriteErrorNoMatchField,
    #[serde(rename = "WriteErrorNonIntegerTTLField")]
    WriteErrorNonIntegerTtlField,
    #[serde(rename = "WriteErrorNonListMatchDestField")]
    WriteErrorNonListMatchDestField,
    #[serde(rename = "WriteErrorNonSecureContext")]
    WriteErrorNonSecureContext,
    #[serde(rename = "WriteErrorNonStringIdField")]
    WriteErrorNonStringIdField,
    #[serde(rename = "WriteErrorNonStringInMatchDestList")]
    WriteErrorNonStringInMatchDestList,
    #[serde(rename = "WriteErrorNonStringMatchField")]
    WriteErrorNonStringMatchField,
    #[serde(rename = "WriteErrorNonTokenTypeField")]
    WriteErrorNonTokenTypeField,
    #[serde(rename = "WriteErrorRequestAborted")]
    WriteErrorRequestAborted,
    #[serde(rename = "WriteErrorShuttingDown")]
    WriteErrorShuttingDown,
    #[serde(rename = "WriteErrorTooLongIdField")]
    WriteErrorTooLongIdField,
    #[serde(rename = "WriteErrorUnsupportedType")]
    WriteErrorUnsupportedType,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SriMessageSignatureError {
    #[serde(rename = "MissingSignatureHeader")]
    MissingSignatureHeader,
    #[serde(rename = "MissingSignatureInputHeader")]
    MissingSignatureInputHeader,
    #[serde(rename = "InvalidSignatureHeader")]
    InvalidSignatureHeader,
    #[serde(rename = "InvalidSignatureInputHeader")]
    InvalidSignatureInputHeader,
    #[serde(rename = "SignatureHeaderValueIsNotByteSequence")]
    SignatureHeaderValueIsNotByteSequence,
    #[serde(rename = "SignatureHeaderValueIsParameterized")]
    SignatureHeaderValueIsParameterized,
    #[serde(rename = "SignatureHeaderValueIsIncorrectLength")]
    SignatureHeaderValueIsIncorrectLength,
    #[serde(rename = "SignatureInputHeaderMissingLabel")]
    SignatureInputHeaderMissingLabel,
    #[serde(rename = "SignatureInputHeaderValueNotInnerList")]
    SignatureInputHeaderValueNotInnerList,
    #[serde(rename = "SignatureInputHeaderValueMissingComponents")]
    SignatureInputHeaderValueMissingComponents,
    #[serde(rename = "SignatureInputHeaderInvalidComponentType")]
    SignatureInputHeaderInvalidComponentType,
    #[serde(rename = "SignatureInputHeaderInvalidComponentName")]
    SignatureInputHeaderInvalidComponentName,
    #[serde(rename = "SignatureInputHeaderInvalidHeaderComponentParameter")]
    SignatureInputHeaderInvalidHeaderComponentParameter,
    #[serde(rename = "SignatureInputHeaderInvalidDerivedComponentParameter")]
    SignatureInputHeaderInvalidDerivedComponentParameter,
    #[serde(rename = "SignatureInputHeaderKeyIdLength")]
    SignatureInputHeaderKeyIdLength,
    #[serde(rename = "SignatureInputHeaderInvalidParameter")]
    SignatureInputHeaderInvalidParameter,
    #[serde(rename = "SignatureInputHeaderMissingRequiredParameters")]
    SignatureInputHeaderMissingRequiredParameters,
    #[serde(rename = "ValidationFailedSignatureExpired")]
    ValidationFailedSignatureExpired,
    #[serde(rename = "ValidationFailedInvalidLength")]
    ValidationFailedInvalidLength,
    #[serde(rename = "ValidationFailedSignatureMismatch")]
    ValidationFailedSignatureMismatch,
    #[serde(rename = "ValidationFailedIntegrityMismatch")]
    ValidationFailedIntegrityMismatch,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnencodedDigestError {
    #[serde(rename = "MalformedDictionary")]
    MalformedDictionary,
    #[serde(rename = "UnknownAlgorithm")]
    UnknownAlgorithm,
    #[serde(rename = "IncorrectDigestType")]
    IncorrectDigestType,
    #[serde(rename = "IncorrectDigestLength")]
    IncorrectDigestLength,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConnectionAllowlistError {
    #[serde(rename = "InvalidHeader")]
    InvalidHeader,
    #[serde(rename = "MoreThanOneList")]
    MoreThanOneList,
    #[serde(rename = "ItemNotInnerList")]
    ItemNotInnerList,
    #[serde(rename = "InvalidAllowlistItemType")]
    InvalidAllowlistItemType,
    #[serde(rename = "ReportingEndpointNotToken")]
    ReportingEndpointNotToken,
    #[serde(rename = "InvalidUrlPattern")]
    InvalidUrlPattern,
}
#[doc = "Details for issues around \"Attribution Reporting API\" usage.\nExplainer: https://github.com/WICG/attribution-reporting-api\n[AttributionReportingIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-AttributionReportingIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingIssueDetails {
    #[serde(rename = "violationType")]
    pub violation_type: AttributionReportingIssueType,
    #[serde(rename = "request")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request: Option<AffectedRequest>,
    #[serde(rename = "violatingNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub violating_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
    #[serde(rename = "invalidParameter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub invalid_parameter: Option<String>,
}
impl AttributionReportingIssueDetails {
    pub fn new(violation_type: impl Into<AttributionReportingIssueType>) -> Self {
        Self {
            violation_type: violation_type.into(),
            request: None,
            violating_node_id: None,
            invalid_parameter: None,
        }
    }
}
impl AttributionReportingIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.AttributionReportingIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Details for issues about documents in Quirks Mode\nor Limited Quirks Mode that affects page layouting.\n[QuirksModeIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-QuirksModeIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuirksModeIssueDetails {
    #[doc = "If false, it means the document's mode is \"quirks\"\ninstead of \"limited-quirks\"."]
    #[serde(rename = "isLimitedQuirksMode")]
    pub is_limited_quirks_mode: bool,
    #[serde(rename = "documentNodeId")]
    pub document_node_id: crate::browser_protocol::dom::types::BackendNodeId,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "frameId")]
    pub frame_id: crate::browser_protocol::page::types::FrameId,
    #[serde(rename = "loaderId")]
    pub loader_id: crate::browser_protocol::network::types::LoaderId,
}
impl QuirksModeIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.QuirksModeIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedDictionaryIssueDetails {
    #[serde(rename = "sharedDictionaryError")]
    pub shared_dictionary_error: SharedDictionaryError,
    #[serde(rename = "request")]
    pub request: AffectedRequest,
}
impl SharedDictionaryIssueDetails {
    pub fn new(
        shared_dictionary_error: impl Into<SharedDictionaryError>,
        request: impl Into<AffectedRequest>,
    ) -> Self {
        Self {
            shared_dictionary_error: shared_dictionary_error.into(),
            request: request.into(),
        }
    }
}
impl SharedDictionaryIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.SharedDictionaryIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SriMessageSignatureIssueDetails {
    #[serde(rename = "error")]
    pub error: SriMessageSignatureError,
    #[serde(rename = "signatureBase")]
    pub signature_base: String,
    #[serde(rename = "integrityAssertions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub integrity_assertions: Vec<String>,
    #[serde(rename = "request")]
    pub request: AffectedRequest,
}
impl SriMessageSignatureIssueDetails {
    pub fn new(
        error: impl Into<SriMessageSignatureError>,
        signature_base: impl Into<String>,
        integrity_assertions: Vec<String>,
        request: impl Into<AffectedRequest>,
    ) -> Self {
        Self {
            error: error.into(),
            signature_base: signature_base.into(),
            integrity_assertions,
            request: request.into(),
        }
    }
}
impl SriMessageSignatureIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.SRIMessageSignatureIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnencodedDigestIssueDetails {
    #[serde(rename = "error")]
    pub error: UnencodedDigestError,
    #[serde(rename = "request")]
    pub request: AffectedRequest,
}
impl UnencodedDigestIssueDetails {
    pub fn new(
        error: impl Into<UnencodedDigestError>,
        request: impl Into<AffectedRequest>,
    ) -> Self {
        Self {
            error: error.into(),
            request: request.into(),
        }
    }
}
impl UnencodedDigestIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.UnencodedDigestIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionAllowlistIssueDetails {
    #[serde(rename = "error")]
    pub error: ConnectionAllowlistError,
    #[serde(rename = "request")]
    pub request: AffectedRequest,
}
impl ConnectionAllowlistIssueDetails {
    pub fn new(
        error: impl Into<ConnectionAllowlistError>,
        request: impl Into<AffectedRequest>,
    ) -> Self {
        Self {
            error: error.into(),
            request: request.into(),
        }
    }
}
impl ConnectionAllowlistIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.ConnectionAllowlistIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GenericIssueErrorType {
    #[serde(rename = "FormLabelForNameError")]
    FormLabelForNameError,
    #[serde(rename = "FormDuplicateIdForInputError")]
    FormDuplicateIdForInputError,
    #[serde(rename = "FormInputWithNoLabelError")]
    FormInputWithNoLabelError,
    #[serde(rename = "FormAutocompleteAttributeEmptyError")]
    FormAutocompleteAttributeEmptyError,
    #[serde(rename = "FormEmptyIdAndNameAttributesForInputError")]
    FormEmptyIdAndNameAttributesForInputError,
    #[serde(rename = "FormAriaLabelledByToNonExistingIdError")]
    FormAriaLabelledByToNonExistingIdError,
    #[serde(rename = "FormInputAssignedAutocompleteValueToIdOrNameAttributeError")]
    FormInputAssignedAutocompleteValueToIdOrNameAttributeError,
    #[serde(rename = "FormLabelHasNeitherForNorNestedInputError")]
    FormLabelHasNeitherForNorNestedInputError,
    #[serde(rename = "FormLabelForMatchesNonExistingIdError")]
    FormLabelForMatchesNonExistingIdError,
    #[serde(rename = "FormInputHasWrongButWellIntendedAutocompleteValueError")]
    FormInputHasWrongButWellIntendedAutocompleteValueError,
    #[serde(rename = "ResponseWasBlockedByORB")]
    ResponseWasBlockedByOrb,
    #[serde(rename = "NavigationEntryMarkedSkippable")]
    NavigationEntryMarkedSkippable,
    #[serde(rename = "AutofillAndManualTextPolicyControlledFeaturesInfo")]
    AutofillAndManualTextPolicyControlledFeaturesInfo,
    #[serde(rename = "AutofillPolicyControlledFeatureInfo")]
    AutofillPolicyControlledFeatureInfo,
    #[serde(rename = "ManualTextPolicyControlledFeatureInfo")]
    ManualTextPolicyControlledFeatureInfo,
}
#[doc = "Depending on the concrete errorType, different properties are set.\n[GenericIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-GenericIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericIssueDetails {
    #[doc = "Issues with the same errorType are aggregated in the frontend."]
    #[serde(rename = "errorType")]
    pub error_type: GenericIssueErrorType,
    #[serde(rename = "frameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_id: Option<crate::browser_protocol::page::types::FrameId>,
    #[serde(rename = "violatingNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub violating_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
    #[serde(rename = "violatingNodeAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub violating_node_attribute: Option<String>,
    #[serde(rename = "request")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request: Option<AffectedRequest>,
}
impl GenericIssueDetails {
    pub fn new(error_type: impl Into<GenericIssueErrorType>) -> Self {
        Self {
            error_type: error_type.into(),
            frame_id: None,
            violating_node_id: None,
            violating_node_attribute: None,
            request: None,
        }
    }
}
impl GenericIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.GenericIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "This issue tracks information needed to print a deprecation message.\nhttps://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/frame/third_party/blink/renderer/core/frame/deprecation/README.md\n[DeprecationIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-DeprecationIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeprecationIssueDetails {
    #[serde(rename = "affectedFrame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub affected_frame: Option<AffectedFrame>,
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: SourceCodeLocation,
    #[doc = "One of the deprecation names from third_party/blink/renderer/core/frame/deprecation/deprecation.json5"]
    #[serde(rename = "type")]
    pub r#type: String,
}
impl DeprecationIssueDetails {
    pub fn new(
        source_code_location: impl Into<SourceCodeLocation>,
        r#type: impl Into<String>,
    ) -> Self {
        Self {
            source_code_location: source_code_location.into(),
            r#type: r#type.into(),
            affected_frame: None,
        }
    }
}
impl DeprecationIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.DeprecationIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "This issue warns about sites in the redirect chain of a finished navigation\nthat may be flagged as trackers and have their state cleared if they don't\nreceive a user interaction. Note that in this context 'site' means eTLD+1.\nFor example, if the URL `https://example.test:80/bounce` was in the\nredirect chain, the site reported would be `example.test`.\n[BounceTrackingIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-BounceTrackingIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BounceTrackingIssueDetails {
    #[serde(rename = "trackingSites")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tracking_sites: Vec<String>,
}
impl BounceTrackingIssueDetails {
    pub fn new(tracking_sites: Vec<String>) -> Self {
        Self { tracking_sites }
    }
}
impl BounceTrackingIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.BounceTrackingIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "This issue warns about third-party sites that are accessing cookies on the\ncurrent page, and have been permitted due to having a global metadata grant.\nNote that in this context 'site' means eTLD+1. For example, if the URL\n`https://example.test:80/web_page` was accessing cookies, the site reported\nwould be `example.test`.\n[CookieDeprecationMetadataIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-CookieDeprecationMetadataIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookieDeprecationMetadataIssueDetails {
    #[serde(rename = "allowedSites")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_sites: Vec<String>,
    #[serde(rename = "optOutPercentage")]
    pub opt_out_percentage: f64,
    #[serde(rename = "isOptOutTopLevel")]
    pub is_opt_out_top_level: bool,
    #[serde(rename = "operation")]
    pub operation: CookieOperation,
}
impl CookieDeprecationMetadataIssueDetails {
    pub fn new(
        allowed_sites: Vec<String>,
        opt_out_percentage: impl Into<f64>,
        is_opt_out_top_level: impl Into<bool>,
        operation: impl Into<CookieOperation>,
    ) -> Self {
        Self {
            allowed_sites,
            opt_out_percentage: opt_out_percentage.into(),
            is_opt_out_top_level: is_opt_out_top_level.into(),
            operation: operation.into(),
        }
    }
}
impl CookieDeprecationMetadataIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.CookieDeprecationMetadataIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClientHintIssueReason {
    #[doc = "Items in the accept-ch meta tag allow list must be valid origins.\nNo special values (e.g. self, none, and *) are permitted."]
    #[serde(rename = "MetaTagAllowListInvalidOrigin")]
    MetaTagAllowListInvalidOrigin,
    #[doc = "Only accept-ch meta tags in the original HTML sent from the server\nare respected. Any injected via javascript (or other means) are ignored."]
    #[serde(rename = "MetaTagModifiedHTML")]
    MetaTagModifiedHtml,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FederatedAuthRequestIssueDetails {
    #[serde(rename = "federatedAuthRequestIssueReason")]
    pub federated_auth_request_issue_reason: FederatedAuthRequestIssueReason,
}
impl FederatedAuthRequestIssueDetails {
    pub fn new(
        federated_auth_request_issue_reason: impl Into<FederatedAuthRequestIssueReason>,
    ) -> Self {
        Self {
            federated_auth_request_issue_reason: federated_auth_request_issue_reason.into(),
        }
    }
}
impl FederatedAuthRequestIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.FederatedAuthRequestIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Represents the failure reason when a federated authentication reason fails.\nShould be updated alongside RequestIdTokenStatus in\nthird_party/blink/public/mojom/devtools/inspector_issue.mojom to include\nall cases except for success."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FederatedAuthRequestIssueReason {
    #[serde(rename = "ShouldEmbargo")]
    ShouldEmbargo,
    #[serde(rename = "TooManyRequests")]
    TooManyRequests,
    #[serde(rename = "WellKnownHttpNotFound")]
    WellKnownHttpNotFound,
    #[serde(rename = "WellKnownNoResponse")]
    WellKnownNoResponse,
    #[serde(rename = "WellKnownInvalidResponse")]
    WellKnownInvalidResponse,
    #[serde(rename = "WellKnownListEmpty")]
    WellKnownListEmpty,
    #[serde(rename = "WellKnownInvalidContentType")]
    WellKnownInvalidContentType,
    #[serde(rename = "ConfigNotInWellKnown")]
    ConfigNotInWellKnown,
    #[serde(rename = "WellKnownTooBig")]
    WellKnownTooBig,
    #[serde(rename = "ConfigHttpNotFound")]
    ConfigHttpNotFound,
    #[serde(rename = "ConfigNoResponse")]
    ConfigNoResponse,
    #[serde(rename = "ConfigInvalidResponse")]
    ConfigInvalidResponse,
    #[serde(rename = "ConfigInvalidContentType")]
    ConfigInvalidContentType,
    #[serde(rename = "IdpNotPotentiallyTrustworthy")]
    IdpNotPotentiallyTrustworthy,
    #[serde(rename = "DisabledInSettings")]
    DisabledInSettings,
    #[serde(rename = "DisabledInFlags")]
    DisabledInFlags,
    #[serde(rename = "ErrorFetchingSignin")]
    ErrorFetchingSignin,
    #[serde(rename = "InvalidSigninResponse")]
    InvalidSigninResponse,
    #[serde(rename = "AccountsHttpNotFound")]
    AccountsHttpNotFound,
    #[serde(rename = "AccountsNoResponse")]
    AccountsNoResponse,
    #[serde(rename = "AccountsInvalidResponse")]
    AccountsInvalidResponse,
    #[serde(rename = "AccountsListEmpty")]
    AccountsListEmpty,
    #[serde(rename = "AccountsInvalidContentType")]
    AccountsInvalidContentType,
    #[serde(rename = "IdTokenHttpNotFound")]
    IdTokenHttpNotFound,
    #[serde(rename = "IdTokenNoResponse")]
    IdTokenNoResponse,
    #[serde(rename = "IdTokenInvalidResponse")]
    IdTokenInvalidResponse,
    #[serde(rename = "IdTokenIdpErrorResponse")]
    IdTokenIdpErrorResponse,
    #[serde(rename = "IdTokenCrossSiteIdpErrorResponse")]
    IdTokenCrossSiteIdpErrorResponse,
    #[serde(rename = "IdTokenInvalidRequest")]
    IdTokenInvalidRequest,
    #[serde(rename = "IdTokenInvalidContentType")]
    IdTokenInvalidContentType,
    #[serde(rename = "ErrorIdToken")]
    ErrorIdToken,
    #[serde(rename = "Canceled")]
    Canceled,
    #[serde(rename = "RpPageNotVisible")]
    RpPageNotVisible,
    #[serde(rename = "SilentMediationFailure")]
    SilentMediationFailure,
    #[serde(rename = "NotSignedInWithIdp")]
    NotSignedInWithIdp,
    #[serde(rename = "MissingTransientUserActivation")]
    MissingTransientUserActivation,
    #[serde(rename = "ReplacedByActiveMode")]
    ReplacedByActiveMode,
    #[serde(rename = "RelyingPartyOriginIsOpaque")]
    RelyingPartyOriginIsOpaque,
    #[serde(rename = "TypeNotMatching")]
    TypeNotMatching,
    #[serde(rename = "UiDismissedNoEmbargo")]
    UiDismissedNoEmbargo,
    #[serde(rename = "CorsError")]
    CorsError,
    #[serde(rename = "SuppressedBySegmentationPlatform")]
    SuppressedBySegmentationPlatform,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FederatedAuthUserInfoRequestIssueDetails {
    #[serde(rename = "federatedAuthUserInfoRequestIssueReason")]
    pub federated_auth_user_info_request_issue_reason: FederatedAuthUserInfoRequestIssueReason,
}
impl FederatedAuthUserInfoRequestIssueDetails {
    pub fn new(
        federated_auth_user_info_request_issue_reason: impl Into<
            FederatedAuthUserInfoRequestIssueReason,
        >,
    ) -> Self {
        Self {
            federated_auth_user_info_request_issue_reason:
                federated_auth_user_info_request_issue_reason.into(),
        }
    }
}
impl FederatedAuthUserInfoRequestIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.FederatedAuthUserInfoRequestIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Represents the failure reason when a getUserInfo() call fails.\nShould be updated alongside FederatedAuthUserInfoRequestResult in\nthird_party/blink/public/mojom/devtools/inspector_issue.mojom."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FederatedAuthUserInfoRequestIssueReason {
    #[serde(rename = "NotSameOrigin")]
    NotSameOrigin,
    #[serde(rename = "NotIframe")]
    NotIframe,
    #[serde(rename = "NotPotentiallyTrustworthy")]
    NotPotentiallyTrustworthy,
    #[serde(rename = "NoApiPermission")]
    NoApiPermission,
    #[serde(rename = "NotSignedInWithIdp")]
    NotSignedInWithIdp,
    #[serde(rename = "NoAccountSharingPermission")]
    NoAccountSharingPermission,
    #[serde(rename = "InvalidConfigOrWellKnown")]
    InvalidConfigOrWellKnown,
    #[serde(rename = "InvalidAccountsResponse")]
    InvalidAccountsResponse,
    #[serde(rename = "NoReturningUserFromFetchedAccounts")]
    NoReturningUserFromFetchedAccounts,
}
#[doc = "This issue tracks client hints related issues. It's used to deprecate old\nfeatures, encourage the use of new ones, and provide general guidance.\n[ClientHintIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-ClientHintIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientHintIssueDetails {
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: SourceCodeLocation,
    #[serde(rename = "clientHintIssueReason")]
    pub client_hint_issue_reason: ClientHintIssueReason,
}
impl ClientHintIssueDetails {
    pub fn new(
        source_code_location: impl Into<SourceCodeLocation>,
        client_hint_issue_reason: impl Into<ClientHintIssueReason>,
    ) -> Self {
        Self {
            source_code_location: source_code_location.into(),
            client_hint_issue_reason: client_hint_issue_reason.into(),
        }
    }
}
impl ClientHintIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.ClientHintIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FailedRequestInfo {
    #[doc = "The URL that failed to load."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "The failure message for the failed request."]
    #[serde(rename = "failureMessage")]
    pub failure_message: String,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request_id: Option<crate::browser_protocol::network::types::RequestId>,
}
impl FailedRequestInfo {
    pub fn new(url: impl Into<String>, failure_message: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            failure_message: failure_message.into(),
            request_id: None,
        }
    }
}
impl FailedRequestInfo {
    pub const IDENTIFIER: &'static str = "Audits.FailedRequestInfo";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PartitioningBlobUrlInfo {
    #[serde(rename = "BlockedCrossPartitionFetching")]
    BlockedCrossPartitionFetching,
    #[serde(rename = "EnforceNoopenerForNavigation")]
    EnforceNoopenerForNavigation,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartitioningBlobUrlIssueDetails {
    #[doc = "The BlobURL that failed to load."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Additional information about the Partitioning Blob URL issue."]
    #[serde(rename = "partitioningBlobURLInfo")]
    pub partitioning_blob_url_info: PartitioningBlobUrlInfo,
}
impl PartitioningBlobUrlIssueDetails {
    pub fn new(
        url: impl Into<String>,
        partitioning_blob_url_info: impl Into<PartitioningBlobUrlInfo>,
    ) -> Self {
        Self {
            url: url.into(),
            partitioning_blob_url_info: partitioning_blob_url_info.into(),
        }
    }
}
impl PartitioningBlobUrlIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.PartitioningBlobURLIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementAccessibilityIssueReason {
    #[serde(rename = "DisallowedSelectChild")]
    DisallowedSelectChild,
    #[serde(rename = "DisallowedOptGroupChild")]
    DisallowedOptGroupChild,
    #[serde(rename = "NonPhrasingContentOptionChild")]
    NonPhrasingContentOptionChild,
    #[serde(rename = "InteractiveContentOptionChild")]
    InteractiveContentOptionChild,
    #[serde(rename = "InteractiveContentLegendChild")]
    InteractiveContentLegendChild,
    #[serde(rename = "InteractiveContentSummaryDescendant")]
    InteractiveContentSummaryDescendant,
}
#[doc = "This issue warns about errors in the select or summary element content model.\n[ElementAccessibilityIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-ElementAccessibilityIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElementAccessibilityIssueDetails {
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::BackendNodeId,
    #[serde(rename = "elementAccessibilityIssueReason")]
    pub element_accessibility_issue_reason: ElementAccessibilityIssueReason,
    #[serde(rename = "hasDisallowedAttributes")]
    pub has_disallowed_attributes: bool,
}
impl ElementAccessibilityIssueDetails {
    pub fn new(
        node_id: impl Into<crate::browser_protocol::dom::types::BackendNodeId>,
        element_accessibility_issue_reason: impl Into<ElementAccessibilityIssueReason>,
        has_disallowed_attributes: impl Into<bool>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            element_accessibility_issue_reason: element_accessibility_issue_reason.into(),
            has_disallowed_attributes: has_disallowed_attributes.into(),
        }
    }
}
impl ElementAccessibilityIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.ElementAccessibilityIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StyleSheetLoadingIssueReason {
    #[serde(rename = "LateImportRule")]
    LateImportRule,
    #[serde(rename = "RequestFailed")]
    RequestFailed,
}
#[doc = "This issue warns when a referenced stylesheet couldn't be loaded.\n[StylesheetLoadingIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-StylesheetLoadingIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StylesheetLoadingIssueDetails {
    #[doc = "Source code position that referenced the failing stylesheet."]
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: SourceCodeLocation,
    #[doc = "Reason why the stylesheet couldn't be loaded."]
    #[serde(rename = "styleSheetLoadingIssueReason")]
    pub style_sheet_loading_issue_reason: StyleSheetLoadingIssueReason,
    #[doc = "Contains additional info when the failure was due to a request."]
    #[serde(rename = "failedRequestInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub failed_request_info: Option<FailedRequestInfo>,
}
impl StylesheetLoadingIssueDetails {
    pub fn new(
        source_code_location: impl Into<SourceCodeLocation>,
        style_sheet_loading_issue_reason: impl Into<StyleSheetLoadingIssueReason>,
    ) -> Self {
        Self {
            source_code_location: source_code_location.into(),
            style_sheet_loading_issue_reason: style_sheet_loading_issue_reason.into(),
            failed_request_info: None,
        }
    }
}
impl StylesheetLoadingIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.StylesheetLoadingIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropertyRuleIssueReason {
    #[serde(rename = "InvalidSyntax")]
    InvalidSyntax,
    #[serde(rename = "InvalidInitialValue")]
    InvalidInitialValue,
    #[serde(rename = "InvalidInherits")]
    InvalidInherits,
    #[serde(rename = "InvalidName")]
    InvalidName,
}
#[doc = "This issue warns about errors in property rules that lead to property\nregistrations being ignored.\n[PropertyRuleIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-PropertyRuleIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyRuleIssueDetails {
    #[doc = "Source code position of the property rule."]
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: SourceCodeLocation,
    #[doc = "Reason why the property rule was discarded."]
    #[serde(rename = "propertyRuleIssueReason")]
    pub property_rule_issue_reason: PropertyRuleIssueReason,
    #[doc = "The value of the property rule property that failed to parse"]
    #[serde(rename = "propertyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub property_value: Option<String>,
}
impl PropertyRuleIssueDetails {
    pub fn new(
        source_code_location: impl Into<SourceCodeLocation>,
        property_rule_issue_reason: impl Into<PropertyRuleIssueReason>,
    ) -> Self {
        Self {
            source_code_location: source_code_location.into(),
            property_rule_issue_reason: property_rule_issue_reason.into(),
            property_value: None,
        }
    }
}
impl PropertyRuleIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.PropertyRuleIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserReidentificationIssueType {
    #[serde(rename = "BlockedFrameNavigation")]
    BlockedFrameNavigation,
    #[serde(rename = "BlockedSubresource")]
    BlockedSubresource,
    #[serde(rename = "NoisedCanvasReadback")]
    NoisedCanvasReadback,
}
#[doc = "This issue warns about uses of APIs that may be considered misuse to\nre-identify users.\n[UserReidentificationIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-UserReidentificationIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserReidentificationIssueDetails {
    #[serde(rename = "type")]
    pub r#type: UserReidentificationIssueType,
    #[doc = "Applies to BlockedFrameNavigation and BlockedSubresource issue types."]
    #[serde(rename = "request")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request: Option<AffectedRequest>,
    #[doc = "Applies to NoisedCanvasReadback issue type."]
    #[serde(rename = "sourceCodeLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_code_location: Option<SourceCodeLocation>,
}
impl UserReidentificationIssueDetails {
    pub fn new(r#type: impl Into<UserReidentificationIssueType>) -> Self {
        Self {
            r#type: r#type.into(),
            request: None,
            source_code_location: None,
        }
    }
}
impl UserReidentificationIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.UserReidentificationIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionElementIssueType {
    #[serde(rename = "InvalidType")]
    InvalidType,
    #[serde(rename = "FencedFrameDisallowed")]
    FencedFrameDisallowed,
    #[serde(rename = "CspFrameAncestorsMissing")]
    CspFrameAncestorsMissing,
    #[serde(rename = "PermissionsPolicyBlocked")]
    PermissionsPolicyBlocked,
    #[serde(rename = "PaddingRightUnsupported")]
    PaddingRightUnsupported,
    #[serde(rename = "PaddingBottomUnsupported")]
    PaddingBottomUnsupported,
    #[serde(rename = "InsetBoxShadowUnsupported")]
    InsetBoxShadowUnsupported,
    #[serde(rename = "RequestInProgress")]
    RequestInProgress,
    #[serde(rename = "UntrustedEvent")]
    UntrustedEvent,
    #[serde(rename = "RegistrationFailed")]
    RegistrationFailed,
    #[serde(rename = "TypeNotSupported")]
    TypeNotSupported,
    #[serde(rename = "InvalidTypeActivation")]
    InvalidTypeActivation,
    #[serde(rename = "SecurityChecksFailed")]
    SecurityChecksFailed,
    #[serde(rename = "ActivationDisabled")]
    ActivationDisabled,
    #[serde(rename = "GeolocationDeprecated")]
    GeolocationDeprecated,
    #[serde(rename = "InvalidDisplayStyle")]
    InvalidDisplayStyle,
    #[serde(rename = "NonOpaqueColor")]
    NonOpaqueColor,
    #[serde(rename = "LowContrast")]
    LowContrast,
    #[serde(rename = "FontSizeTooSmall")]
    FontSizeTooSmall,
    #[serde(rename = "FontSizeTooLarge")]
    FontSizeTooLarge,
    #[serde(rename = "InvalidSizeValue")]
    InvalidSizeValue,
}
#[doc = "This issue warns about improper usage of the <permission> element.\n[PermissionElementIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-PermissionElementIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionElementIssueDetails {
    #[serde(rename = "issueType")]
    pub issue_type: PermissionElementIssueType,
    #[doc = "The value of the type attribute."]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[doc = "The node ID of the <permission> element."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
    #[doc = "True if the issue is a warning, false if it is an error."]
    #[serde(rename = "isWarning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_warning: Option<bool>,
    #[doc = "Fields for message construction:\nUsed for messages that reference a specific permission name"]
    #[serde(rename = "permissionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub permission_name: Option<String>,
    #[doc = "Used for messages about occlusion"]
    #[serde(rename = "occluderNodeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub occluder_node_info: Option<String>,
    #[doc = "Used for messages about occluder's parent"]
    #[serde(rename = "occluderParentNodeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub occluder_parent_node_info: Option<String>,
    #[doc = "Used for messages about activation disabled reason"]
    #[serde(rename = "disableReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub disable_reason: Option<String>,
}
impl PermissionElementIssueDetails {
    pub fn new(issue_type: impl Into<PermissionElementIssueType>) -> Self {
        Self {
            issue_type: issue_type.into(),
            r#type: None,
            node_id: None,
            is_warning: None,
            permission_name: None,
            occluder_node_info: None,
            occluder_parent_node_info: None,
            disable_reason: None,
        }
    }
}
impl PermissionElementIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.PermissionElementIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Metadata about the ad script that was on the stack that caused the current\nscript in the `AdAncestry` to be considered ad related.\n[AdScriptIdentifier](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-AdScriptIdentifier)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdScriptIdentifier {
    #[doc = "The script's v8 identifier."]
    #[serde(rename = "scriptId")]
    pub script_id: crate::js_protocol::runtime::types::ScriptId,
    #[doc = "v8's debugging id for the v8::Context."]
    #[serde(rename = "debuggerId")]
    pub debugger_id: crate::js_protocol::runtime::types::UniqueDebuggerId,
    #[doc = "The script's url (or generated name based on id if inline script)."]
    #[serde(rename = "name")]
    pub name: String,
}
impl AdScriptIdentifier {
    pub fn new(
        script_id: impl Into<crate::js_protocol::runtime::types::ScriptId>,
        debugger_id: impl Into<crate::js_protocol::runtime::types::UniqueDebuggerId>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            script_id: script_id.into(),
            debugger_id: debugger_id.into(),
            name: name.into(),
        }
    }
}
impl AdScriptIdentifier {
    pub const IDENTIFIER: &'static str = "Audits.AdScriptIdentifier";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Providence about how an ad script was determined to be such. It is an ad\nbecause its url matched a filterlist rule, or because some other ad script\nwas on the stack when this script was loaded.\n[AdAncestry](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-AdAncestry)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdAncestry {
    #[doc = "The ad-script in the stack when the offending script was loaded. This is\nrecursive down to the root script that was tagged due to the filterlist\nrule."]
    #[serde(rename = "adAncestryChain")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ad_ancestry_chain: Vec<AdScriptIdentifier>,
    #[doc = "The filterlist rule that caused the root (last) script in\n`adAncestry` to be ad-tagged."]
    #[serde(rename = "rootScriptFilterlistRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub root_script_filterlist_rule: Option<String>,
}
impl AdAncestry {
    pub fn new(ad_ancestry_chain: Vec<AdScriptIdentifier>) -> Self {
        Self {
            ad_ancestry_chain,
            root_script_filterlist_rule: None,
        }
    }
}
impl AdAncestry {
    pub const IDENTIFIER: &'static str = "Audits.AdAncestry";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "The issue warns about blocked calls to privacy sensitive APIs via the\nSelective Permissions Intervention.\n[SelectivePermissionsInterventionIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-SelectivePermissionsInterventionIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectivePermissionsInterventionIssueDetails {
    #[doc = "Which API was intervened on."]
    #[serde(rename = "apiName")]
    pub api_name: String,
    #[doc = "Why the ad script using the API is considered an ad."]
    #[serde(rename = "adAncestry")]
    pub ad_ancestry: AdAncestry,
    #[doc = "The stack trace at the time of the intervention."]
    #[serde(rename = "stackTrace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stack_trace: Option<crate::js_protocol::runtime::types::StackTrace>,
}
impl SelectivePermissionsInterventionIssueDetails {
    pub fn new(api_name: impl Into<String>, ad_ancestry: impl Into<AdAncestry>) -> Self {
        Self {
            api_name: api_name.into(),
            ad_ancestry: ad_ancestry.into(),
            stack_trace: None,
        }
    }
}
impl SelectivePermissionsInterventionIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.SelectivePermissionsInterventionIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "A unique identifier for the type of issue. Each type may use one of the\noptional fields in InspectorIssueDetails to convey more specific\ninformation about the kind of issue."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InspectorIssueCode {
    #[serde(rename = "CookieIssue")]
    CookieIssue,
    #[serde(rename = "MixedContentIssue")]
    MixedContentIssue,
    #[serde(rename = "BlockedByResponseIssue")]
    BlockedByResponseIssue,
    #[serde(rename = "HeavyAdIssue")]
    HeavyAdIssue,
    #[serde(rename = "ContentSecurityPolicyIssue")]
    ContentSecurityPolicyIssue,
    #[serde(rename = "SharedArrayBufferIssue")]
    SharedArrayBufferIssue,
    #[serde(rename = "LowTextContrastIssue")]
    LowTextContrastIssue,
    #[serde(rename = "CorsIssue")]
    CorsIssue,
    #[serde(rename = "AttributionReportingIssue")]
    AttributionReportingIssue,
    #[serde(rename = "QuirksModeIssue")]
    QuirksModeIssue,
    #[serde(rename = "PartitioningBlobURLIssue")]
    PartitioningBlobUrlIssue,
    #[doc = "Deprecated"]
    #[serde(rename = "NavigatorUserAgentIssue")]
    NavigatorUserAgentIssue,
    #[serde(rename = "GenericIssue")]
    GenericIssue,
    #[serde(rename = "DeprecationIssue")]
    DeprecationIssue,
    #[serde(rename = "ClientHintIssue")]
    ClientHintIssue,
    #[serde(rename = "FederatedAuthRequestIssue")]
    FederatedAuthRequestIssue,
    #[serde(rename = "BounceTrackingIssue")]
    BounceTrackingIssue,
    #[serde(rename = "CookieDeprecationMetadataIssue")]
    CookieDeprecationMetadataIssue,
    #[serde(rename = "StylesheetLoadingIssue")]
    StylesheetLoadingIssue,
    #[serde(rename = "FederatedAuthUserInfoRequestIssue")]
    FederatedAuthUserInfoRequestIssue,
    #[serde(rename = "PropertyRuleIssue")]
    PropertyRuleIssue,
    #[serde(rename = "SharedDictionaryIssue")]
    SharedDictionaryIssue,
    #[serde(rename = "ElementAccessibilityIssue")]
    ElementAccessibilityIssue,
    #[serde(rename = "SRIMessageSignatureIssue")]
    SriMessageSignatureIssue,
    #[serde(rename = "UnencodedDigestIssue")]
    UnencodedDigestIssue,
    #[serde(rename = "ConnectionAllowlistIssue")]
    ConnectionAllowlistIssue,
    #[serde(rename = "UserReidentificationIssue")]
    UserReidentificationIssue,
    #[serde(rename = "PermissionElementIssue")]
    PermissionElementIssue,
    #[serde(rename = "PerformanceIssue")]
    PerformanceIssue,
    #[serde(rename = "SelectivePermissionsInterventionIssue")]
    SelectivePermissionsInterventionIssue,
}
#[doc = "This struct holds a list of optional fields with additional information\nspecific to the kind of issue. When adding a new issue code, please also\nadd a new optional field to this type.\n[InspectorIssueDetails](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-InspectorIssueDetails)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InspectorIssueDetails {
    #[serde(rename = "cookieIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cookie_issue_details: Option<CookieIssueDetails>,
    #[serde(rename = "mixedContentIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mixed_content_issue_details: Option<MixedContentIssueDetails>,
    #[serde(rename = "blockedByResponseIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub blocked_by_response_issue_details: Option<BlockedByResponseIssueDetails>,
    #[serde(rename = "heavyAdIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub heavy_ad_issue_details: Option<HeavyAdIssueDetails>,
    #[serde(rename = "contentSecurityPolicyIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub content_security_policy_issue_details: Option<ContentSecurityPolicyIssueDetails>,
    #[serde(rename = "sharedArrayBufferIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub shared_array_buffer_issue_details: Option<SharedArrayBufferIssueDetails>,
    #[serde(rename = "lowTextContrastIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub low_text_contrast_issue_details: Option<LowTextContrastIssueDetails>,
    #[serde(rename = "corsIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cors_issue_details: Option<CorsIssueDetails>,
    #[serde(rename = "attributionReportingIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub attribution_reporting_issue_details: Option<AttributionReportingIssueDetails>,
    #[serde(rename = "quirksModeIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub quirks_mode_issue_details: Option<QuirksModeIssueDetails>,
    #[serde(rename = "partitioningBlobURLIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub partitioning_blob_url_issue_details: Option<PartitioningBlobUrlIssueDetails>,
    #[serde(rename = "genericIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub generic_issue_details: Option<GenericIssueDetails>,
    #[serde(rename = "deprecationIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub deprecation_issue_details: Option<DeprecationIssueDetails>,
    #[serde(rename = "clientHintIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub client_hint_issue_details: Option<ClientHintIssueDetails>,
    #[serde(rename = "federatedAuthRequestIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub federated_auth_request_issue_details: Option<FederatedAuthRequestIssueDetails>,
    #[serde(rename = "bounceTrackingIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bounce_tracking_issue_details: Option<BounceTrackingIssueDetails>,
    #[serde(rename = "cookieDeprecationMetadataIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cookie_deprecation_metadata_issue_details: Option<CookieDeprecationMetadataIssueDetails>,
    #[serde(rename = "stylesheetLoadingIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stylesheet_loading_issue_details: Option<StylesheetLoadingIssueDetails>,
    #[serde(rename = "propertyRuleIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub property_rule_issue_details: Option<PropertyRuleIssueDetails>,
    #[serde(rename = "federatedAuthUserInfoRequestIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub federated_auth_user_info_request_issue_details:
        Option<FederatedAuthUserInfoRequestIssueDetails>,
    #[serde(rename = "sharedDictionaryIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub shared_dictionary_issue_details: Option<SharedDictionaryIssueDetails>,
    #[serde(rename = "elementAccessibilityIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub element_accessibility_issue_details: Option<ElementAccessibilityIssueDetails>,
    #[serde(rename = "sriMessageSignatureIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sri_message_signature_issue_details: Option<SriMessageSignatureIssueDetails>,
    #[serde(rename = "unencodedDigestIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unencoded_digest_issue_details: Option<UnencodedDigestIssueDetails>,
    #[serde(rename = "connectionAllowlistIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub connection_allowlist_issue_details: Option<ConnectionAllowlistIssueDetails>,
    #[serde(rename = "userReidentificationIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_reidentification_issue_details: Option<UserReidentificationIssueDetails>,
    #[serde(rename = "permissionElementIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub permission_element_issue_details: Option<PermissionElementIssueDetails>,
    #[serde(rename = "performanceIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub performance_issue_details: Option<PerformanceIssueDetails>,
    #[serde(rename = "selectivePermissionsInterventionIssueDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub selective_permissions_intervention_issue_details:
        Option<SelectivePermissionsInterventionIssueDetails>,
}
impl InspectorIssueDetails {
    pub const IDENTIFIER: &'static str = "Audits.InspectorIssueDetails";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "A unique id for a DevTools inspector issue. Allows other entities (e.g.\nexceptions, CDP message, console messages, etc.) to reference an issue.\n[IssueId](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-IssueId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct IssueId(String);
impl IssueId {
    pub fn new(val: impl Into<String>) -> Self {
        IssueId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for IssueId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<IssueId> for String {
    fn from(el: IssueId) -> String {
        el.0
    }
}
impl From<String> for IssueId {
    fn from(expr: String) -> Self {
        IssueId(expr)
    }
}
impl std::borrow::Borrow<str> for IssueId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl IssueId {
    pub const IDENTIFIER: &'static str = "Audits.IssueId";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "An inspector issue reported from the back-end.\n[InspectorIssue](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-InspectorIssue)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectorIssue {
    #[serde(rename = "code")]
    pub code: InspectorIssueCode,
    #[serde(rename = "details")]
    pub details: InspectorIssueDetails,
    #[doc = "A unique id for this issue. May be omitted if no other entity (e.g.\nexception, CDP message, etc.) is referencing this issue."]
    #[serde(rename = "issueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub issue_id: Option<IssueId>,
}
impl InspectorIssue {
    pub fn new(
        code: impl Into<InspectorIssueCode>,
        details: impl Into<InspectorIssueDetails>,
    ) -> Self {
        Self {
            code: code.into(),
            details: details.into(),
            issue_id: None,
        }
    }
}
impl InspectorIssue {
    pub const IDENTIFIER: &'static str = "Audits.InspectorIssue";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (AuditsTypes { AffectedCookie (AffectedCookie) , AffectedRequest (AffectedRequest) , AffectedFrame (AffectedFrame) , CookieExclusionReason (CookieExclusionReason) , CookieWarningReason (CookieWarningReason) , CookieOperation (CookieOperation) , InsightType (InsightType) , CookieIssueInsight (CookieIssueInsight) , CookieIssueDetails (CookieIssueDetails) , PerformanceIssueType (PerformanceIssueType) , PerformanceIssueDetails (PerformanceIssueDetails) , MixedContentResolutionStatus (MixedContentResolutionStatus) , MixedContentResourceType (MixedContentResourceType) , MixedContentIssueDetails (MixedContentIssueDetails) , BlockedByResponseReason (BlockedByResponseReason) , BlockedByResponseIssueDetails (BlockedByResponseIssueDetails) , HeavyAdResolutionStatus (HeavyAdResolutionStatus) , HeavyAdReason (HeavyAdReason) , HeavyAdIssueDetails (HeavyAdIssueDetails) , ContentSecurityPolicyViolationType (ContentSecurityPolicyViolationType) , SourceCodeLocation (SourceCodeLocation) , ContentSecurityPolicyIssueDetails (ContentSecurityPolicyIssueDetails) , SharedArrayBufferIssueType (SharedArrayBufferIssueType) , SharedArrayBufferIssueDetails (SharedArrayBufferIssueDetails) , LowTextContrastIssueDetails (LowTextContrastIssueDetails) , CorsIssueDetails (CorsIssueDetails) , AttributionReportingIssueType (AttributionReportingIssueType) , SharedDictionaryError (SharedDictionaryError) , SriMessageSignatureError (SriMessageSignatureError) , UnencodedDigestError (UnencodedDigestError) , ConnectionAllowlistError (ConnectionAllowlistError) , AttributionReportingIssueDetails (AttributionReportingIssueDetails) , QuirksModeIssueDetails (QuirksModeIssueDetails) , SharedDictionaryIssueDetails (SharedDictionaryIssueDetails) , SriMessageSignatureIssueDetails (SriMessageSignatureIssueDetails) , UnencodedDigestIssueDetails (UnencodedDigestIssueDetails) , ConnectionAllowlistIssueDetails (ConnectionAllowlistIssueDetails) , GenericIssueErrorType (GenericIssueErrorType) , GenericIssueDetails (GenericIssueDetails) , DeprecationIssueDetails (DeprecationIssueDetails) , BounceTrackingIssueDetails (BounceTrackingIssueDetails) , CookieDeprecationMetadataIssueDetails (CookieDeprecationMetadataIssueDetails) , ClientHintIssueReason (ClientHintIssueReason) , FederatedAuthRequestIssueDetails (FederatedAuthRequestIssueDetails) , FederatedAuthRequestIssueReason (FederatedAuthRequestIssueReason) , FederatedAuthUserInfoRequestIssueDetails (FederatedAuthUserInfoRequestIssueDetails) , FederatedAuthUserInfoRequestIssueReason (FederatedAuthUserInfoRequestIssueReason) , ClientHintIssueDetails (ClientHintIssueDetails) , FailedRequestInfo (FailedRequestInfo) , PartitioningBlobUrlInfo (PartitioningBlobUrlInfo) , PartitioningBlobUrlIssueDetails (PartitioningBlobUrlIssueDetails) , ElementAccessibilityIssueReason (ElementAccessibilityIssueReason) , ElementAccessibilityIssueDetails (ElementAccessibilityIssueDetails) , StyleSheetLoadingIssueReason (StyleSheetLoadingIssueReason) , StylesheetLoadingIssueDetails (StylesheetLoadingIssueDetails) , PropertyRuleIssueReason (PropertyRuleIssueReason) , PropertyRuleIssueDetails (PropertyRuleIssueDetails) , UserReidentificationIssueType (UserReidentificationIssueType) , UserReidentificationIssueDetails (UserReidentificationIssueDetails) , PermissionElementIssueType (PermissionElementIssueType) , PermissionElementIssueDetails (PermissionElementIssueDetails) , AdScriptIdentifier (AdScriptIdentifier) , AdAncestry (AdAncestry) , SelectivePermissionsInterventionIssueDetails (SelectivePermissionsInterventionIssueDetails) , InspectorIssueCode (InspectorIssueCode) , InspectorIssueDetails (InspectorIssueDetails) , IssueId (IssueId) , InspectorIssue (InspectorIssue) });
