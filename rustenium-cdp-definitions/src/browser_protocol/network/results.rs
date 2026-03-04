use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanClearBrowserCacheReturns {
    #[doc = "True if browser cache can be cleared."]
    #[serde(rename = "result")]
    pub result: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanClearBrowserCookiesReturns {
    #[doc = "True if browser cookies can be cleared."]
    #[serde(rename = "result")]
    pub result: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanEmulateNetworkConditionsReturns {
    #[doc = "True if emulation of network conditions is supported."]
    #[serde(rename = "result")]
    pub result: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmulateNetworkConditionsByRuleReturns {
    #[doc = "An id for each entry in matchedNetworkConditions. The id will be included in the requestWillBeSentExtraInfo for\nrequests affected by a rule."]
    #[serde(rename = "ruleIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rule_ids: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAllCookiesReturns {
    #[doc = "Array of cookie objects."]
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookies: Vec<super::types::Cookie>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCertificateReturns {
    #[serde(rename = "tableNames")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub table_names: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCookiesReturns {
    #[doc = "Array of cookie objects."]
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookies: Vec<super::types::Cookie>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResponseBodyReturns {
    #[doc = "Response body."]
    #[serde(rename = "body")]
    pub body: String,
    #[doc = "True, if content was sent as base64."]
    #[serde(rename = "base64Encoded")]
    pub base64_encoded: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRequestPostDataReturns {
    #[doc = "Request body string, omitting files from multipart requests"]
    #[serde(rename = "postData")]
    pub post_data: String,
    #[doc = "True, if content was sent as base64."]
    #[serde(rename = "base64Encoded")]
    pub base64_encoded: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResponseBodyForInterceptionReturns {
    #[doc = "Response body."]
    #[serde(rename = "body")]
    pub body: String,
    #[doc = "True, if content was sent as base64."]
    #[serde(rename = "base64Encoded")]
    pub base64_encoded: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeResponseBodyForInterceptionAsStreamReturns {
    #[serde(rename = "stream")]
    pub stream: super::super::io::types::StreamHandle,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchInResponseBodyReturns {
    #[doc = "List of search matches."]
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<super::super::super::js_protocol::debugger::types::SearchMatch>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetCookieReturns {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamResourceContentReturns {
    #[doc = "Data that has been buffered until streaming is enabled."]
    #[serde(rename = "bufferedData")]
    pub buffered_data: super::super::super::Binary,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSecurityIsolationStatusReturns {
    #[serde(rename = "status")]
    pub status: super::types::SecurityIsolationStatus,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FetchSchemefulSiteReturns {
    #[doc = "The corresponding schemeful site."]
    #[serde(rename = "schemefulSite")]
    pub schemeful_site: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadNetworkResourceReturns {
    #[serde(rename = "resource")]
    pub resource: super::types::LoadNetworkResourcePageResult,
}
