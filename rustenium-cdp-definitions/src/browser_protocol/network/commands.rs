use serde::{Deserialize, Serialize};
#[doc = "Sets a list of content encodings that will be accepted. Empty list means no encoding is accepted.\n[setAcceptedEncodings](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setAcceptedEncodings)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAcceptedEncodingsParams {
    #[doc = "List of accepted content encodings."]
    #[serde(rename = "encodings")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub encodings: Vec<super::types::ContentEncoding>,
}
impl SetAcceptedEncodingsParams {
    pub fn new(encodings: Vec<super::types::ContentEncoding>) -> Self {
        Self { encodings }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetAcceptedEncodingsMethod {
    #[serde(rename = "Network.setAcceptedEncodings")]
    SetAcceptedEncodings,
}
impl SetAcceptedEncodingsMethod {
    pub const IDENTIFIER: &'static str = "Network.setAcceptedEncodings";
}
#[doc = "Sets a list of content encodings that will be accepted. Empty list means no encoding is accepted.\n[setAcceptedEncodings](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setAcceptedEncodings)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetAcceptedEncodings {
    pub method: SetAcceptedEncodingsMethod,
    pub params: SetAcceptedEncodingsParams,
}
impl crate::CommandResult for SetAcceptedEncodings {
    type Result = super::results::SetAcceptedEncodingsResult;
}
#[doc = "Clears accepted encodings set by setAcceptedEncodings\n[clearAcceptedEncodingsOverride](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-clearAcceptedEncodingsOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearAcceptedEncodingsOverrideParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearAcceptedEncodingsOverrideMethod {
    #[serde(rename = "Network.clearAcceptedEncodingsOverride")]
    ClearAcceptedEncodingsOverride,
}
impl ClearAcceptedEncodingsOverrideMethod {
    pub const IDENTIFIER: &'static str = "Network.clearAcceptedEncodingsOverride";
}
#[doc = "Clears accepted encodings set by setAcceptedEncodings\n[clearAcceptedEncodingsOverride](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-clearAcceptedEncodingsOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ClearAcceptedEncodingsOverride {
    pub method: ClearAcceptedEncodingsOverrideMethod,
    pub params: ClearAcceptedEncodingsOverrideParams,
}
impl crate::CommandResult for ClearAcceptedEncodingsOverride {
    type Result = super::results::ClearAcceptedEncodingsOverrideResult;
}
#[doc = "Clears browser cache.\n[clearBrowserCache](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-clearBrowserCache)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearBrowserCacheParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearBrowserCacheMethod {
    #[serde(rename = "Network.clearBrowserCache")]
    ClearBrowserCache,
}
impl ClearBrowserCacheMethod {
    pub const IDENTIFIER: &'static str = "Network.clearBrowserCache";
}
#[doc = "Clears browser cache.\n[clearBrowserCache](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-clearBrowserCache)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ClearBrowserCache {
    pub method: ClearBrowserCacheMethod,
    pub params: ClearBrowserCacheParams,
}
impl crate::CommandResult for ClearBrowserCache {
    type Result = super::results::ClearBrowserCacheResult;
}
#[doc = "Clears browser cookies.\n[clearBrowserCookies](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-clearBrowserCookies)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearBrowserCookiesParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearBrowserCookiesMethod {
    #[serde(rename = "Network.clearBrowserCookies")]
    ClearBrowserCookies,
}
impl ClearBrowserCookiesMethod {
    pub const IDENTIFIER: &'static str = "Network.clearBrowserCookies";
}
#[doc = "Clears browser cookies.\n[clearBrowserCookies](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-clearBrowserCookies)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ClearBrowserCookies {
    pub method: ClearBrowserCookiesMethod,
    pub params: ClearBrowserCookiesParams,
}
impl crate::CommandResult for ClearBrowserCookies {
    type Result = super::results::ClearBrowserCookiesResult;
}
#[doc = "Deletes browser cookies with matching name and url or domain/path/partitionKey pair.\n[deleteCookies](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-deleteCookies)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCookiesParams {
    #[doc = "Name of the cookies to remove."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "If specified, deletes all the cookies with the given name where domain and path match\nprovided URL."]
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
    #[doc = "If specified, deletes only cookies with the exact domain."]
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub domain: Option<String>,
    #[doc = "If specified, deletes only cookies with the exact path."]
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub path: Option<String>,
    #[doc = "If specified, deletes only cookies with the the given name and partitionKey where\nall partition key attributes match the cookie partition key attribute."]
    #[serde(rename = "partitionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub partition_key: Option<super::types::CookiePartitionKey>,
}
impl DeleteCookiesParams {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: None,
            domain: None,
            path: None,
            partition_key: None,
        }
    }
}
impl<T: Into<String>> From<T> for DeleteCookiesParams {
    fn from(url: T) -> Self {
        DeleteCookiesParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeleteCookiesMethod {
    #[serde(rename = "Network.deleteCookies")]
    DeleteCookies,
}
impl DeleteCookiesMethod {
    pub const IDENTIFIER: &'static str = "Network.deleteCookies";
}
#[doc = "Deletes browser cookies with matching name and url or domain/path/partitionKey pair.\n[deleteCookies](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-deleteCookies)"]
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteCookies {
    pub method: DeleteCookiesMethod,
    pub params: DeleteCookiesParams,
}
impl crate::CommandResult for DeleteCookies {
    type Result = super::results::DeleteCookiesResult;
}
#[doc = "Disables network tracking, prevents network events from being sent to the client.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Network.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "Network.disable";
}
#[doc = "Disables network tracking, prevents network events from being sent to the client.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Activates emulation of network conditions for individual requests using URL match patterns. Unlike the deprecated\nNetwork.emulateNetworkConditions this method does not affect `navigator` state. Use Network.overrideNetworkState to\nexplicitly modify `navigator` behavior.\n[emulateNetworkConditionsByRule](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-emulateNetworkConditionsByRule)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmulateNetworkConditionsByRuleParams {
    #[doc = "True to emulate internet disconnection."]
    #[serde(rename = "offline")]
    pub offline: bool,
    #[doc = "Configure conditions for matching requests. If multiple entries match a request, the first entry wins.  Global\nconditions can be configured by leaving the urlPattern for the conditions empty. These global conditions are\nalso applied for throttling of p2p connections."]
    #[serde(rename = "matchedNetworkConditions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub matched_network_conditions: Vec<super::types::NetworkConditions>,
}
impl EmulateNetworkConditionsByRuleParams {
    pub fn new(
        offline: impl Into<bool>,
        matched_network_conditions: Vec<super::types::NetworkConditions>,
    ) -> Self {
        Self {
            offline: offline.into(),
            matched_network_conditions,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EmulateNetworkConditionsByRuleMethod {
    #[serde(rename = "Network.emulateNetworkConditionsByRule")]
    EmulateNetworkConditionsByRule,
}
impl EmulateNetworkConditionsByRuleMethod {
    pub const IDENTIFIER: &'static str = "Network.emulateNetworkConditionsByRule";
}
#[doc = "Activates emulation of network conditions for individual requests using URL match patterns. Unlike the deprecated\nNetwork.emulateNetworkConditions this method does not affect `navigator` state. Use Network.overrideNetworkState to\nexplicitly modify `navigator` behavior.\n[emulateNetworkConditionsByRule](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-emulateNetworkConditionsByRule)"]
#[derive(Debug, Clone, PartialEq)]
pub struct EmulateNetworkConditionsByRule {
    pub method: EmulateNetworkConditionsByRuleMethod,
    pub params: EmulateNetworkConditionsByRuleParams,
}
impl crate::CommandResult for EmulateNetworkConditionsByRule {
    type Result = super::results::EmulateNetworkConditionsByRuleResult;
}
#[doc = "Override the state of navigator.onLine and navigator.connection.\n[overrideNetworkState](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-overrideNetworkState)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverrideNetworkStateParams {
    #[doc = "True to emulate internet disconnection."]
    #[serde(rename = "offline")]
    pub offline: bool,
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
    pub connection_type: Option<super::types::ConnectionType>,
}
impl OverrideNetworkStateParams {
    pub fn new(
        offline: impl Into<bool>,
        latency: impl Into<f64>,
        download_throughput: impl Into<f64>,
        upload_throughput: impl Into<f64>,
    ) -> Self {
        Self {
            offline: offline.into(),
            latency: latency.into(),
            download_throughput: download_throughput.into(),
            upload_throughput: upload_throughput.into(),
            connection_type: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OverrideNetworkStateMethod {
    #[serde(rename = "Network.overrideNetworkState")]
    OverrideNetworkState,
}
impl OverrideNetworkStateMethod {
    pub const IDENTIFIER: &'static str = "Network.overrideNetworkState";
}
#[doc = "Override the state of navigator.onLine and navigator.connection.\n[overrideNetworkState](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-overrideNetworkState)"]
#[derive(Debug, Clone, PartialEq)]
pub struct OverrideNetworkState {
    pub method: OverrideNetworkStateMethod,
    pub params: OverrideNetworkStateParams,
}
impl crate::CommandResult for OverrideNetworkState {
    type Result = super::results::OverrideNetworkStateResult;
}
#[doc = "Enables network tracking, network events will now be delivered to the client.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {
    #[doc = "Buffer size in bytes to use when preserving network payloads (XHRs, etc).\nThis is the maximum number of bytes that will be collected by this\nDevTools session."]
    #[serde(rename = "maxTotalBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_total_buffer_size: Option<i64>,
    #[doc = "Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc)."]
    #[serde(rename = "maxResourceBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_resource_buffer_size: Option<i64>,
    #[doc = "Longest post body size (in bytes) that would be included in requestWillBeSent notification"]
    #[serde(rename = "maxPostDataSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_post_data_size: Option<i64>,
    #[doc = "Whether DirectSocket chunk send/receive events should be reported."]
    #[serde(rename = "reportDirectSocketTraffic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub report_direct_socket_traffic: Option<bool>,
    #[doc = "Enable storing response bodies outside of renderer, so that these survive\na cross-process navigation. Requires maxTotalBufferSize to be set.\nCurrently defaults to false. This field is being deprecated in favor of the dedicated\nconfigureDurableMessages command, due to the possibility of deadlocks when awaiting\nNetwork.enable before issuing Runtime.runIfWaitingForDebugger."]
    #[serde(rename = "enableDurableMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enable_durable_messages: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Network.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "Network.enable";
}
#[doc = "Enables network tracking, network events will now be delivered to the client.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Configures storing response bodies outside of renderer, so that these survive\na cross-process navigation.\nIf maxTotalBufferSize is not set, durable messages are disabled.\n[configureDurableMessages](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-configureDurableMessages)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfigureDurableMessagesParams {
    #[doc = "Buffer size in bytes to use when preserving network payloads (XHRs, etc)."]
    #[serde(rename = "maxTotalBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_total_buffer_size: Option<i64>,
    #[doc = "Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc)."]
    #[serde(rename = "maxResourceBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_resource_buffer_size: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConfigureDurableMessagesMethod {
    #[serde(rename = "Network.configureDurableMessages")]
    ConfigureDurableMessages,
}
impl ConfigureDurableMessagesMethod {
    pub const IDENTIFIER: &'static str = "Network.configureDurableMessages";
}
#[doc = "Configures storing response bodies outside of renderer, so that these survive\na cross-process navigation.\nIf maxTotalBufferSize is not set, durable messages are disabled.\n[configureDurableMessages](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-configureDurableMessages)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigureDurableMessages {
    pub method: ConfigureDurableMessagesMethod,
    pub params: ConfigureDurableMessagesParams,
}
impl crate::CommandResult for ConfigureDurableMessages {
    type Result = super::results::ConfigureDurableMessagesResult;
}
#[doc = "Returns the DER-encoded certificate.\n[getCertificate](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-getCertificate)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCertificateParams {
    #[doc = "Origin to get certificate for."]
    #[serde(rename = "origin")]
    pub origin: String,
}
impl GetCertificateParams {
    pub fn new(origin: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
        }
    }
}
impl<T: Into<String>> From<T> for GetCertificateParams {
    fn from(url: T) -> Self {
        GetCertificateParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetCertificateMethod {
    #[serde(rename = "Network.getCertificate")]
    GetCertificate,
}
impl GetCertificateMethod {
    pub const IDENTIFIER: &'static str = "Network.getCertificate";
}
#[doc = "Returns the DER-encoded certificate.\n[getCertificate](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-getCertificate)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetCertificate {
    pub method: GetCertificateMethod,
    pub params: GetCertificateParams,
}
impl crate::CommandResult for GetCertificate {
    type Result = super::results::GetCertificateResult;
}
#[doc = "Returns all browser cookies for the current URL. Depending on the backend support, will return\ndetailed cookie information in the `cookies` field.\n[getCookies](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-getCookies)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCookiesParams {
    #[doc = "The list of URLs for which applicable cookies will be fetched.\nIf not specified, it's assumed to be set to the list containing\nthe URLs of the page and all of its subframes."]
    #[serde(rename = "urls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub urls: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetCookiesMethod {
    #[serde(rename = "Network.getCookies")]
    GetCookies,
}
impl GetCookiesMethod {
    pub const IDENTIFIER: &'static str = "Network.getCookies";
}
#[doc = "Returns all browser cookies for the current URL. Depending on the backend support, will return\ndetailed cookie information in the `cookies` field.\n[getCookies](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-getCookies)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetCookies {
    pub method: GetCookiesMethod,
    pub params: GetCookiesParams,
}
impl crate::CommandResult for GetCookies {
    type Result = super::results::GetCookiesResult;
}
#[doc = "Returns content served for the given request.\n[getResponseBody](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-getResponseBody)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResponseBodyParams {
    #[doc = "Identifier of the network request to get content for."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
}
impl GetResponseBodyParams {
    pub fn new(request_id: impl Into<super::types::RequestId>) -> Self {
        Self {
            request_id: request_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetResponseBodyMethod {
    #[serde(rename = "Network.getResponseBody")]
    GetResponseBody,
}
impl GetResponseBodyMethod {
    pub const IDENTIFIER: &'static str = "Network.getResponseBody";
}
#[doc = "Returns content served for the given request.\n[getResponseBody](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-getResponseBody)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetResponseBody {
    pub method: GetResponseBodyMethod,
    pub params: GetResponseBodyParams,
}
impl crate::CommandResult for GetResponseBody {
    type Result = super::results::GetResponseBodyResult;
}
#[doc = "Returns post data sent with the request. Returns an error when no data was sent with the request.\n[getRequestPostData](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-getRequestPostData)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRequestPostDataParams {
    #[doc = "Identifier of the network request to get content for."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
}
impl GetRequestPostDataParams {
    pub fn new(request_id: impl Into<super::types::RequestId>) -> Self {
        Self {
            request_id: request_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetRequestPostDataMethod {
    #[serde(rename = "Network.getRequestPostData")]
    GetRequestPostData,
}
impl GetRequestPostDataMethod {
    pub const IDENTIFIER: &'static str = "Network.getRequestPostData";
}
#[doc = "Returns post data sent with the request. Returns an error when no data was sent with the request.\n[getRequestPostData](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-getRequestPostData)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetRequestPostData {
    pub method: GetRequestPostDataMethod,
    pub params: GetRequestPostDataParams,
}
impl crate::CommandResult for GetRequestPostData {
    type Result = super::results::GetRequestPostDataResult;
}
#[doc = "Returns content served for the given currently intercepted request.\n[getResponseBodyForInterception](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-getResponseBodyForInterception)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResponseBodyForInterceptionParams {
    #[doc = "Identifier for the intercepted request to get body for."]
    #[serde(rename = "interceptionId")]
    pub interception_id: super::types::InterceptionId,
}
impl GetResponseBodyForInterceptionParams {
    pub fn new(interception_id: impl Into<super::types::InterceptionId>) -> Self {
        Self {
            interception_id: interception_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetResponseBodyForInterceptionMethod {
    #[serde(rename = "Network.getResponseBodyForInterception")]
    GetResponseBodyForInterception,
}
impl GetResponseBodyForInterceptionMethod {
    pub const IDENTIFIER: &'static str = "Network.getResponseBodyForInterception";
}
#[doc = "Returns content served for the given currently intercepted request.\n[getResponseBodyForInterception](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-getResponseBodyForInterception)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetResponseBodyForInterception {
    pub method: GetResponseBodyForInterceptionMethod,
    pub params: GetResponseBodyForInterceptionParams,
}
impl crate::CommandResult for GetResponseBodyForInterception {
    type Result = super::results::GetResponseBodyForInterceptionResult;
}
#[doc = "Returns a handle to the stream representing the response body. Note that after this command,\nthe intercepted request can't be continued as is -- you either need to cancel it or to provide\nthe response body. The stream only supports sequential read, IO.read will fail if the position\nis specified.\n[takeResponseBodyForInterceptionAsStream](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-takeResponseBodyForInterceptionAsStream)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeResponseBodyForInterceptionAsStreamParams {
    #[serde(rename = "interceptionId")]
    pub interception_id: super::types::InterceptionId,
}
impl TakeResponseBodyForInterceptionAsStreamParams {
    pub fn new(interception_id: impl Into<super::types::InterceptionId>) -> Self {
        Self {
            interception_id: interception_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TakeResponseBodyForInterceptionAsStreamMethod {
    #[serde(rename = "Network.takeResponseBodyForInterceptionAsStream")]
    TakeResponseBodyForInterceptionAsStream,
}
impl TakeResponseBodyForInterceptionAsStreamMethod {
    pub const IDENTIFIER: &'static str = "Network.takeResponseBodyForInterceptionAsStream";
}
#[doc = "Returns a handle to the stream representing the response body. Note that after this command,\nthe intercepted request can't be continued as is -- you either need to cancel it or to provide\nthe response body. The stream only supports sequential read, IO.read will fail if the position\nis specified.\n[takeResponseBodyForInterceptionAsStream](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-takeResponseBodyForInterceptionAsStream)"]
#[derive(Debug, Clone, PartialEq)]
pub struct TakeResponseBodyForInterceptionAsStream {
    pub method: TakeResponseBodyForInterceptionAsStreamMethod,
    pub params: TakeResponseBodyForInterceptionAsStreamParams,
}
impl crate::CommandResult for TakeResponseBodyForInterceptionAsStream {
    type Result = super::results::TakeResponseBodyForInterceptionAsStreamResult;
}
#[doc = "This method sends a new XMLHttpRequest which is identical to the original one. The following\nparameters should be identical: method, url, async, request body, extra headers, withCredentials\nattribute, user, password.\n[replayXHR](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-replayXHR)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplayXhrParams {
    #[doc = "Identifier of XHR to replay."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
}
impl ReplayXhrParams {
    pub fn new(request_id: impl Into<super::types::RequestId>) -> Self {
        Self {
            request_id: request_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReplayXhrMethod {
    #[serde(rename = "Network.replayXHR")]
    ReplayXhr,
}
impl ReplayXhrMethod {
    pub const IDENTIFIER: &'static str = "Network.replayXHR";
}
#[doc = "This method sends a new XMLHttpRequest which is identical to the original one. The following\nparameters should be identical: method, url, async, request body, extra headers, withCredentials\nattribute, user, password.\n[replayXHR](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-replayXHR)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ReplayXhr {
    pub method: ReplayXhrMethod,
    pub params: ReplayXhrParams,
}
impl crate::CommandResult for ReplayXhr {
    type Result = super::results::ReplayXhrResult;
}
#[doc = "Searches for given string in response content.\n[searchInResponseBody](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-searchInResponseBody)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchInResponseBodyParams {
    #[doc = "Identifier of the network response to search."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "String to search for."]
    #[serde(rename = "query")]
    pub query: String,
    #[doc = "If true, search is case sensitive."]
    #[serde(rename = "caseSensitive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub case_sensitive: Option<bool>,
    #[doc = "If true, treats string parameter as regex."]
    #[serde(rename = "isRegex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_regex: Option<bool>,
}
impl SearchInResponseBodyParams {
    pub fn new(request_id: impl Into<super::types::RequestId>, query: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            query: query.into(),
            case_sensitive: None,
            is_regex: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SearchInResponseBodyMethod {
    #[serde(rename = "Network.searchInResponseBody")]
    SearchInResponseBody,
}
impl SearchInResponseBodyMethod {
    pub const IDENTIFIER: &'static str = "Network.searchInResponseBody";
}
#[doc = "Searches for given string in response content.\n[searchInResponseBody](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-searchInResponseBody)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SearchInResponseBody {
    pub method: SearchInResponseBodyMethod,
    pub params: SearchInResponseBodyParams,
}
impl crate::CommandResult for SearchInResponseBody {
    type Result = super::results::SearchInResponseBodyResult;
}
#[doc = "Blocks URLs from loading.\n[setBlockedURLs](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setBlockedURLs)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetBlockedUrLsParams {
    #[doc = "Patterns to match in the order in which they are given. These patterns\nalso take precedence over any wildcard patterns defined in `urls`."]
    #[serde(rename = "urlPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url_patterns: Option<Vec<super::types::BlockPattern>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetBlockedUrLsMethod {
    #[serde(rename = "Network.setBlockedURLs")]
    SetBlockedUrLs,
}
impl SetBlockedUrLsMethod {
    pub const IDENTIFIER: &'static str = "Network.setBlockedURLs";
}
#[doc = "Blocks URLs from loading.\n[setBlockedURLs](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setBlockedURLs)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetBlockedUrLs {
    pub method: SetBlockedUrLsMethod,
    pub params: SetBlockedUrLsParams,
}
impl crate::CommandResult for SetBlockedUrLs {
    type Result = super::results::SetBlockedUrLsResult;
}
#[doc = "Toggles ignoring of service worker for each request.\n[setBypassServiceWorker](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setBypassServiceWorker)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBypassServiceWorkerParams {
    #[doc = "Bypass service worker and load from network."]
    #[serde(rename = "bypass")]
    pub bypass: bool,
}
impl SetBypassServiceWorkerParams {
    pub fn new(bypass: impl Into<bool>) -> Self {
        Self {
            bypass: bypass.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetBypassServiceWorkerMethod {
    #[serde(rename = "Network.setBypassServiceWorker")]
    SetBypassServiceWorker,
}
impl SetBypassServiceWorkerMethod {
    pub const IDENTIFIER: &'static str = "Network.setBypassServiceWorker";
}
#[doc = "Toggles ignoring of service worker for each request.\n[setBypassServiceWorker](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setBypassServiceWorker)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetBypassServiceWorker {
    pub method: SetBypassServiceWorkerMethod,
    pub params: SetBypassServiceWorkerParams,
}
impl crate::CommandResult for SetBypassServiceWorker {
    type Result = super::results::SetBypassServiceWorkerResult;
}
#[doc = "Toggles ignoring cache for each request. If `true`, cache will not be used.\n[setCacheDisabled](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setCacheDisabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCacheDisabledParams {
    #[doc = "Cache disabled state."]
    #[serde(rename = "cacheDisabled")]
    pub cache_disabled: bool,
}
impl SetCacheDisabledParams {
    pub fn new(cache_disabled: impl Into<bool>) -> Self {
        Self {
            cache_disabled: cache_disabled.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetCacheDisabledMethod {
    #[serde(rename = "Network.setCacheDisabled")]
    SetCacheDisabled,
}
impl SetCacheDisabledMethod {
    pub const IDENTIFIER: &'static str = "Network.setCacheDisabled";
}
#[doc = "Toggles ignoring cache for each request. If `true`, cache will not be used.\n[setCacheDisabled](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setCacheDisabled)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetCacheDisabled {
    pub method: SetCacheDisabledMethod,
    pub params: SetCacheDisabledParams,
}
impl crate::CommandResult for SetCacheDisabled {
    type Result = super::results::SetCacheDisabledResult;
}
#[doc = "Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.\n[setCookie](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setCookie)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCookieParams {
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
    pub same_site: Option<super::types::CookieSameSite>,
    #[doc = "Cookie expiration date, session cookie if not set"]
    #[serde(rename = "expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub expires: Option<super::types::TimeSinceEpoch>,
    #[doc = "Cookie Priority type."]
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub priority: Option<super::types::CookiePriority>,
    #[doc = "Cookie source scheme type."]
    #[serde(rename = "sourceScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_scheme: Option<super::types::CookieSourceScheme>,
    #[doc = "Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.\nAn unspecified port value allows protocol clients to emulate legacy cookie scope for the port.\nThis is a temporary ability and it will be removed in the future."]
    #[serde(rename = "sourcePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_port: Option<i64>,
    #[doc = "Cookie partition key. If not set, the cookie will be set as not partitioned."]
    #[serde(rename = "partitionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub partition_key: Option<super::types::CookiePartitionKey>,
}
impl SetCookieParams {
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetCookieMethod {
    #[serde(rename = "Network.setCookie")]
    SetCookie,
}
impl SetCookieMethod {
    pub const IDENTIFIER: &'static str = "Network.setCookie";
}
#[doc = "Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.\n[setCookie](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setCookie)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetCookie {
    pub method: SetCookieMethod,
    pub params: SetCookieParams,
}
impl crate::CommandResult for SetCookie {
    type Result = super::results::SetCookieResult;
}
#[doc = "Sets given cookies.\n[setCookies](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setCookies)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCookiesParams {
    #[doc = "Cookies to be set."]
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookies: Vec<super::types::CookieParam>,
}
impl SetCookiesParams {
    pub fn new(cookies: Vec<super::types::CookieParam>) -> Self {
        Self { cookies }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetCookiesMethod {
    #[serde(rename = "Network.setCookies")]
    SetCookies,
}
impl SetCookiesMethod {
    pub const IDENTIFIER: &'static str = "Network.setCookies";
}
#[doc = "Sets given cookies.\n[setCookies](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setCookies)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetCookies {
    pub method: SetCookiesMethod,
    pub params: SetCookiesParams,
}
impl crate::CommandResult for SetCookies {
    type Result = super::results::SetCookiesResult;
}
#[doc = "Specifies whether to always send extra HTTP headers with the requests from this page.\n[setExtraHTTPHeaders](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setExtraHTTPHeaders)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetExtraHttpHeadersParams {
    #[doc = "Map with extra HTTP headers."]
    #[serde(rename = "headers")]
    pub headers: super::types::Headers,
}
impl SetExtraHttpHeadersParams {
    pub fn new(headers: impl Into<super::types::Headers>) -> Self {
        Self {
            headers: headers.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetExtraHttpHeadersMethod {
    #[serde(rename = "Network.setExtraHTTPHeaders")]
    SetExtraHttpHeaders,
}
impl SetExtraHttpHeadersMethod {
    pub const IDENTIFIER: &'static str = "Network.setExtraHTTPHeaders";
}
#[doc = "Specifies whether to always send extra HTTP headers with the requests from this page.\n[setExtraHTTPHeaders](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setExtraHTTPHeaders)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetExtraHttpHeaders {
    pub method: SetExtraHttpHeadersMethod,
    pub params: SetExtraHttpHeadersParams,
}
impl crate::CommandResult for SetExtraHttpHeaders {
    type Result = super::results::SetExtraHttpHeadersResult;
}
#[doc = "Specifies whether to attach a page script stack id in requests\n[setAttachDebugStack](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setAttachDebugStack)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAttachDebugStackParams {
    #[doc = "Whether to attach a page script stack for debugging purpose."]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
impl SetAttachDebugStackParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self {
            enabled: enabled.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetAttachDebugStackMethod {
    #[serde(rename = "Network.setAttachDebugStack")]
    SetAttachDebugStack,
}
impl SetAttachDebugStackMethod {
    pub const IDENTIFIER: &'static str = "Network.setAttachDebugStack";
}
#[doc = "Specifies whether to attach a page script stack id in requests\n[setAttachDebugStack](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setAttachDebugStack)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetAttachDebugStack {
    pub method: SetAttachDebugStackMethod,
    pub params: SetAttachDebugStackParams,
}
impl crate::CommandResult for SetAttachDebugStack {
    type Result = super::results::SetAttachDebugStackResult;
}
#[doc = "Allows overriding user agent with the given string.\n[setUserAgentOverride](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setUserAgentOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetUserAgentOverrideParams {
    #[doc = "User agent to use."]
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    #[doc = "Browser language to emulate."]
    #[serde(rename = "acceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub accept_language: Option<String>,
    #[doc = "The platform navigator.platform should return."]
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub platform: Option<String>,
    #[doc = "To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData"]
    #[serde(rename = "userAgentMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_agent_metadata: Option<crate::browser_protocol::emulation::types::UserAgentMetadata>,
}
impl SetUserAgentOverrideParams {
    pub fn new(user_agent: impl Into<String>) -> Self {
        Self {
            user_agent: user_agent.into(),
            accept_language: None,
            platform: None,
            user_agent_metadata: None,
        }
    }
}
impl<T: Into<String>> From<T> for SetUserAgentOverrideParams {
    fn from(url: T) -> Self {
        SetUserAgentOverrideParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetUserAgentOverrideMethod {
    #[serde(rename = "Network.setUserAgentOverride")]
    SetUserAgentOverride,
}
impl SetUserAgentOverrideMethod {
    pub const IDENTIFIER: &'static str = "Network.setUserAgentOverride";
}
#[doc = "Allows overriding user agent with the given string.\n[setUserAgentOverride](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setUserAgentOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetUserAgentOverride {
    pub method: SetUserAgentOverrideMethod,
    pub params: SetUserAgentOverrideParams,
}
impl crate::CommandResult for SetUserAgentOverride {
    type Result = super::results::SetUserAgentOverrideResult;
}
#[doc = "Enables streaming of the response for the given requestId.\nIf enabled, the dataReceived event contains the data that was received during streaming.\n[streamResourceContent](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-streamResourceContent)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamResourceContentParams {
    #[doc = "Identifier of the request to stream."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
}
impl StreamResourceContentParams {
    pub fn new(request_id: impl Into<super::types::RequestId>) -> Self {
        Self {
            request_id: request_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StreamResourceContentMethod {
    #[serde(rename = "Network.streamResourceContent")]
    StreamResourceContent,
}
impl StreamResourceContentMethod {
    pub const IDENTIFIER: &'static str = "Network.streamResourceContent";
}
#[doc = "Enables streaming of the response for the given requestId.\nIf enabled, the dataReceived event contains the data that was received during streaming.\n[streamResourceContent](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-streamResourceContent)"]
#[derive(Debug, Clone, PartialEq)]
pub struct StreamResourceContent {
    pub method: StreamResourceContentMethod,
    pub params: StreamResourceContentParams,
}
impl crate::CommandResult for StreamResourceContent {
    type Result = super::results::StreamResourceContentResult;
}
#[doc = "Returns information about the COEP/COOP isolation status.\n[getSecurityIsolationStatus](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-getSecurityIsolationStatus)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetSecurityIsolationStatusParams {
    #[doc = "If no frameId is provided, the status of the target is provided."]
    #[serde(rename = "frameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_id: Option<crate::browser_protocol::page::types::FrameId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetSecurityIsolationStatusMethod {
    #[serde(rename = "Network.getSecurityIsolationStatus")]
    GetSecurityIsolationStatus,
}
impl GetSecurityIsolationStatusMethod {
    pub const IDENTIFIER: &'static str = "Network.getSecurityIsolationStatus";
}
#[doc = "Returns information about the COEP/COOP isolation status.\n[getSecurityIsolationStatus](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-getSecurityIsolationStatus)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetSecurityIsolationStatus {
    pub method: GetSecurityIsolationStatusMethod,
    pub params: GetSecurityIsolationStatusParams,
}
impl crate::CommandResult for GetSecurityIsolationStatus {
    type Result = super::results::GetSecurityIsolationStatusResult;
}
#[doc = "Enables tracking for the Reporting API, events generated by the Reporting API will now be delivered to the client.\nEnabling triggers 'reportingApiReportAdded' for all existing reports.\n[enableReportingApi](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-enableReportingApi)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableReportingApiParams {
    #[doc = "Whether to enable or disable events for the Reporting API"]
    #[serde(rename = "enable")]
    pub enable: bool,
}
impl EnableReportingApiParams {
    pub fn new(enable: impl Into<bool>) -> Self {
        Self {
            enable: enable.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableReportingApiMethod {
    #[serde(rename = "Network.enableReportingApi")]
    EnableReportingApi,
}
impl EnableReportingApiMethod {
    pub const IDENTIFIER: &'static str = "Network.enableReportingApi";
}
#[doc = "Enables tracking for the Reporting API, events generated by the Reporting API will now be delivered to the client.\nEnabling triggers 'reportingApiReportAdded' for all existing reports.\n[enableReportingApi](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-enableReportingApi)"]
#[derive(Debug, Clone, PartialEq)]
pub struct EnableReportingApi {
    pub method: EnableReportingApiMethod,
    pub params: EnableReportingApiParams,
}
impl crate::CommandResult for EnableReportingApi {
    type Result = super::results::EnableReportingApiResult;
}
#[doc = "Sets up tracking device bound sessions and fetching of initial set of sessions.\n[enableDeviceBoundSessions](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-enableDeviceBoundSessions)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableDeviceBoundSessionsParams {
    #[doc = "Whether to enable or disable events."]
    #[serde(rename = "enable")]
    pub enable: bool,
}
impl EnableDeviceBoundSessionsParams {
    pub fn new(enable: impl Into<bool>) -> Self {
        Self {
            enable: enable.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableDeviceBoundSessionsMethod {
    #[serde(rename = "Network.enableDeviceBoundSessions")]
    EnableDeviceBoundSessions,
}
impl EnableDeviceBoundSessionsMethod {
    pub const IDENTIFIER: &'static str = "Network.enableDeviceBoundSessions";
}
#[doc = "Sets up tracking device bound sessions and fetching of initial set of sessions.\n[enableDeviceBoundSessions](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-enableDeviceBoundSessions)"]
#[derive(Debug, Clone, PartialEq)]
pub struct EnableDeviceBoundSessions {
    pub method: EnableDeviceBoundSessionsMethod,
    pub params: EnableDeviceBoundSessionsParams,
}
impl crate::CommandResult for EnableDeviceBoundSessions {
    type Result = super::results::EnableDeviceBoundSessionsResult;
}
#[doc = "Fetches the schemeful site for a specific origin.\n[fetchSchemefulSite](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-fetchSchemefulSite)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FetchSchemefulSiteParams {
    #[doc = "The URL origin."]
    #[serde(rename = "origin")]
    pub origin: String,
}
impl FetchSchemefulSiteParams {
    pub fn new(origin: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
        }
    }
}
impl<T: Into<String>> From<T> for FetchSchemefulSiteParams {
    fn from(url: T) -> Self {
        FetchSchemefulSiteParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FetchSchemefulSiteMethod {
    #[serde(rename = "Network.fetchSchemefulSite")]
    FetchSchemefulSite,
}
impl FetchSchemefulSiteMethod {
    pub const IDENTIFIER: &'static str = "Network.fetchSchemefulSite";
}
#[doc = "Fetches the schemeful site for a specific origin.\n[fetchSchemefulSite](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-fetchSchemefulSite)"]
#[derive(Debug, Clone, PartialEq)]
pub struct FetchSchemefulSite {
    pub method: FetchSchemefulSiteMethod,
    pub params: FetchSchemefulSiteParams,
}
impl crate::CommandResult for FetchSchemefulSite {
    type Result = super::results::FetchSchemefulSiteResult;
}
#[doc = "Fetches the resource and returns the content.\n[loadNetworkResource](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-loadNetworkResource)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadNetworkResourceParams {
    #[doc = "Frame id to get the resource for. Mandatory for frame targets, and\nshould be omitted for worker targets."]
    #[serde(rename = "frameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_id: Option<crate::browser_protocol::page::types::FrameId>,
    #[doc = "URL of the resource to get content for."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Options for the request."]
    #[serde(rename = "options")]
    pub options: super::types::LoadNetworkResourceOptions,
}
impl LoadNetworkResourceParams {
    pub fn new(
        url: impl Into<String>,
        options: impl Into<super::types::LoadNetworkResourceOptions>,
    ) -> Self {
        Self {
            url: url.into(),
            options: options.into(),
            frame_id: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoadNetworkResourceMethod {
    #[serde(rename = "Network.loadNetworkResource")]
    LoadNetworkResource,
}
impl LoadNetworkResourceMethod {
    pub const IDENTIFIER: &'static str = "Network.loadNetworkResource";
}
#[doc = "Fetches the resource and returns the content.\n[loadNetworkResource](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-loadNetworkResource)"]
#[derive(Debug, Clone, PartialEq)]
pub struct LoadNetworkResource {
    pub method: LoadNetworkResourceMethod,
    pub params: LoadNetworkResourceParams,
}
impl crate::CommandResult for LoadNetworkResource {
    type Result = super::results::LoadNetworkResourceResult;
}
#[doc = "Sets Controls for third-party cookie access\nPage reload is required before the new cookie behavior will be observed\n[setCookieControls](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setCookieControls)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCookieControlsParams {
    #[doc = "Whether 3pc restriction is enabled."]
    #[serde(rename = "enableThirdPartyCookieRestriction")]
    pub enable_third_party_cookie_restriction: bool,
    #[doc = "Whether 3pc grace period exception should be enabled; false by default."]
    #[serde(rename = "disableThirdPartyCookieMetadata")]
    pub disable_third_party_cookie_metadata: bool,
    #[doc = "Whether 3pc heuristics exceptions should be enabled; false by default."]
    #[serde(rename = "disableThirdPartyCookieHeuristics")]
    pub disable_third_party_cookie_heuristics: bool,
}
impl SetCookieControlsParams {
    pub fn new(
        enable_third_party_cookie_restriction: impl Into<bool>,
        disable_third_party_cookie_metadata: impl Into<bool>,
        disable_third_party_cookie_heuristics: impl Into<bool>,
    ) -> Self {
        Self {
            enable_third_party_cookie_restriction: enable_third_party_cookie_restriction.into(),
            disable_third_party_cookie_metadata: disable_third_party_cookie_metadata.into(),
            disable_third_party_cookie_heuristics: disable_third_party_cookie_heuristics.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetCookieControlsMethod {
    #[serde(rename = "Network.setCookieControls")]
    SetCookieControls,
}
impl SetCookieControlsMethod {
    pub const IDENTIFIER: &'static str = "Network.setCookieControls";
}
#[doc = "Sets Controls for third-party cookie access\nPage reload is required before the new cookie behavior will be observed\n[setCookieControls](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-setCookieControls)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetCookieControls {
    pub method: SetCookieControlsMethod,
    pub params: SetCookieControlsParams,
}
impl crate::CommandResult for SetCookieControls {
    type Result = super::results::SetCookieControlsResult;
}
group_enum ! (NetworkCommands { SetAcceptedEncodings (SetAcceptedEncodings) , ClearAcceptedEncodingsOverride (ClearAcceptedEncodingsOverride) , ClearBrowserCache (ClearBrowserCache) , ClearBrowserCookies (ClearBrowserCookies) , DeleteCookies (DeleteCookies) , Disable (Disable) , EmulateNetworkConditionsByRule (EmulateNetworkConditionsByRule) , OverrideNetworkState (OverrideNetworkState) , Enable (Enable) , ConfigureDurableMessages (ConfigureDurableMessages) , GetCertificate (GetCertificate) , GetCookies (GetCookies) , GetResponseBody (GetResponseBody) , GetRequestPostData (GetRequestPostData) , GetResponseBodyForInterception (GetResponseBodyForInterception) , TakeResponseBodyForInterceptionAsStream (TakeResponseBodyForInterceptionAsStream) , ReplayXhr (ReplayXhr) , SearchInResponseBody (SearchInResponseBody) , SetBlockedUrLs (SetBlockedUrLs) , SetBypassServiceWorker (SetBypassServiceWorker) , SetCacheDisabled (SetCacheDisabled) , SetCookie (SetCookie) , SetCookies (SetCookies) , SetExtraHttpHeaders (SetExtraHttpHeaders) , SetAttachDebugStack (SetAttachDebugStack) , SetUserAgentOverride (SetUserAgentOverride) , StreamResourceContent (StreamResourceContent) , GetSecurityIsolationStatus (GetSecurityIsolationStatus) , EnableReportingApi (EnableReportingApi) , EnableDeviceBoundSessions (EnableDeviceBoundSessions) , FetchSchemefulSite (FetchSchemefulSite) , LoadNetworkResource (LoadNetworkResource) , SetCookieControls (SetCookieControls) });
