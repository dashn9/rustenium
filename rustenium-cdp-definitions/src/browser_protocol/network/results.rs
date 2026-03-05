use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAcceptedEncodingsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearAcceptedEncodingsOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearBrowserCacheResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearBrowserCookiesResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteCookiesResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OverrideNetworkStateResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfigureDurableMessagesResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplayXhrResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetBlockedUrLsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetBypassServiceWorkerResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetCacheDisabledResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetCookiesResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetExtraHttpHeadersResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAttachDebugStackResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetUserAgentOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableReportingApiResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableDeviceBoundSessionsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetCookieControlsResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanClearBrowserCacheResult {
    #[doc = "True if browser cache can be cleared."]
    #[serde(rename = "result")]
    pub result: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanClearBrowserCookiesResult {
    #[doc = "True if browser cookies can be cleared."]
    #[serde(rename = "result")]
    pub result: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanEmulateNetworkConditionsResult {
    #[doc = "True if emulation of network conditions is supported."]
    #[serde(rename = "result")]
    pub result: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmulateNetworkConditionsByRuleResult {
    #[doc = "An id for each entry in matchedNetworkConditions. The id will be included in the requestWillBeSentExtraInfo for\nrequests affected by a rule."]
    #[serde(rename = "ruleIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rule_ids: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAllCookiesResult {
    #[doc = "Array of cookie objects."]
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookies: Vec<super::types::Cookie>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCertificateResult {
    #[serde(rename = "tableNames")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub table_names: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCookiesResult {
    #[doc = "Array of cookie objects."]
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookies: Vec<super::types::Cookie>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResponseBodyResult {
    #[doc = "Response body."]
    #[serde(rename = "body")]
    pub body: String,
    #[doc = "True, if content was sent as base64."]
    #[serde(rename = "base64Encoded")]
    pub base64_encoded: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRequestPostDataResult {
    #[doc = "Request body string, omitting files from multipart requests"]
    #[serde(rename = "postData")]
    pub post_data: String,
    #[doc = "True, if content was sent as base64."]
    #[serde(rename = "base64Encoded")]
    pub base64_encoded: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResponseBodyForInterceptionResult {
    #[doc = "Response body."]
    #[serde(rename = "body")]
    pub body: String,
    #[doc = "True, if content was sent as base64."]
    #[serde(rename = "base64Encoded")]
    pub base64_encoded: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeResponseBodyForInterceptionAsStreamResult {
    #[serde(rename = "stream")]
    pub stream: super::super::io::types::StreamHandle,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchInResponseBodyResult {
    #[doc = "List of search matches."]
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<super::super::super::js_protocol::debugger::types::SearchMatch>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetCookieResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamResourceContentResult {
    #[doc = "Data that has been buffered until streaming is enabled."]
    #[serde(rename = "bufferedData")]
    pub buffered_data: super::super::super::Binary,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSecurityIsolationStatusResult {
    #[serde(rename = "status")]
    pub status: super::types::SecurityIsolationStatus,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FetchSchemefulSiteResult {
    #[doc = "The corresponding schemeful site."]
    #[serde(rename = "schemefulSite")]
    pub schemeful_site: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadNetworkResourceResult {
    #[serde(rename = "resource")]
    pub resource: super::types::LoadNetworkResourcePageResult,
}
