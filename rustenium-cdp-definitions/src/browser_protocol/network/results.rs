use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAcceptedEncodingsResult {}
impl TryFrom<serde_json::Value> for SetAcceptedEncodingsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearAcceptedEncodingsOverrideResult {}
impl TryFrom<serde_json::Value> for ClearAcceptedEncodingsOverrideResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanClearBrowserCacheResult {
    #[doc = "True if browser cache can be cleared."]
    #[serde(rename = "result")]
    pub result: bool,
}
impl TryFrom<serde_json::Value> for CanClearBrowserCacheResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanClearBrowserCookiesResult {
    #[doc = "True if browser cookies can be cleared."]
    #[serde(rename = "result")]
    pub result: bool,
}
impl TryFrom<serde_json::Value> for CanClearBrowserCookiesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanEmulateNetworkConditionsResult {
    #[doc = "True if emulation of network conditions is supported."]
    #[serde(rename = "result")]
    pub result: bool,
}
impl TryFrom<serde_json::Value> for CanEmulateNetworkConditionsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearBrowserCacheResult {}
impl TryFrom<serde_json::Value> for ClearBrowserCacheResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearBrowserCookiesResult {}
impl TryFrom<serde_json::Value> for ClearBrowserCookiesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ContinueInterceptedRequestResult {}
impl TryFrom<serde_json::Value> for ContinueInterceptedRequestResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteCookiesResult {}
impl TryFrom<serde_json::Value> for DeleteCookiesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
impl TryFrom<serde_json::Value> for DisableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EmulateNetworkConditionsResult {}
impl TryFrom<serde_json::Value> for EmulateNetworkConditionsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmulateNetworkConditionsByRuleResult {
    #[doc = "An id for each entry in matchedNetworkConditions. The id will be included in the requestWillBeSentExtraInfo for\nrequests affected by a rule."]
    #[serde(rename = "ruleIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rule_ids: Vec<String>,
}
impl TryFrom<serde_json::Value> for EmulateNetworkConditionsByRuleResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OverrideNetworkStateResult {}
impl TryFrom<serde_json::Value> for OverrideNetworkStateResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
impl TryFrom<serde_json::Value> for EnableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfigureDurableMessagesResult {}
impl TryFrom<serde_json::Value> for ConfigureDurableMessagesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAllCookiesResult {
    #[doc = "Array of cookie objects."]
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookies: Vec<super::types::Cookie>,
}
impl TryFrom<serde_json::Value> for GetAllCookiesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCertificateResult {
    #[serde(rename = "tableNames")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub table_names: Vec<String>,
}
impl TryFrom<serde_json::Value> for GetCertificateResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCookiesResult {
    #[doc = "Array of cookie objects."]
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookies: Vec<super::types::Cookie>,
}
impl TryFrom<serde_json::Value> for GetCookiesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
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
impl TryFrom<serde_json::Value> for GetResponseBodyResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
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
impl TryFrom<serde_json::Value> for GetRequestPostDataResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
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
impl TryFrom<serde_json::Value> for GetResponseBodyForInterceptionResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeResponseBodyForInterceptionAsStreamResult {
    #[serde(rename = "stream")]
    pub stream: crate::browser_protocol::io::types::StreamHandle,
}
impl TryFrom<serde_json::Value> for TakeResponseBodyForInterceptionAsStreamResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplayXhrResult {}
impl TryFrom<serde_json::Value> for ReplayXhrResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchInResponseBodyResult {
    #[doc = "List of search matches."]
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<crate::js_protocol::debugger::types::SearchMatch>,
}
impl TryFrom<serde_json::Value> for SearchInResponseBodyResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetBlockedUrLsResult {}
impl TryFrom<serde_json::Value> for SetBlockedUrLsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetBypassServiceWorkerResult {}
impl TryFrom<serde_json::Value> for SetBypassServiceWorkerResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetCacheDisabledResult {}
impl TryFrom<serde_json::Value> for SetCacheDisabledResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetCookieResult {}
impl TryFrom<serde_json::Value> for SetCookieResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetCookiesResult {}
impl TryFrom<serde_json::Value> for SetCookiesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetExtraHttpHeadersResult {}
impl TryFrom<serde_json::Value> for SetExtraHttpHeadersResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAttachDebugStackResult {}
impl TryFrom<serde_json::Value> for SetAttachDebugStackResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetRequestInterceptionResult {}
impl TryFrom<serde_json::Value> for SetRequestInterceptionResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetUserAgentOverrideResult {}
impl TryFrom<serde_json::Value> for SetUserAgentOverrideResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamResourceContentResult {
    #[doc = "Data that has been buffered until streaming is enabled."]
    #[serde(rename = "bufferedData")]
    pub buffered_data: crate::Binary,
}
impl TryFrom<serde_json::Value> for StreamResourceContentResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSecurityIsolationStatusResult {
    #[serde(rename = "status")]
    pub status: super::types::SecurityIsolationStatus,
}
impl TryFrom<serde_json::Value> for GetSecurityIsolationStatusResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableReportingApiResult {}
impl TryFrom<serde_json::Value> for EnableReportingApiResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableDeviceBoundSessionsResult {}
impl TryFrom<serde_json::Value> for EnableDeviceBoundSessionsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FetchSchemefulSiteResult {
    #[doc = "The corresponding schemeful site."]
    #[serde(rename = "schemefulSite")]
    pub schemeful_site: String,
}
impl TryFrom<serde_json::Value> for FetchSchemefulSiteResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadNetworkResourceResult {
    #[serde(rename = "resource")]
    pub resource: super::types::LoadNetworkResourcePageResult,
}
impl TryFrom<serde_json::Value> for LoadNetworkResourceResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetCookieControlsResult {}
impl TryFrom<serde_json::Value> for SetCookieControlsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
