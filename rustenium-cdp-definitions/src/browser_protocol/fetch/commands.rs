use serde::{Deserialize, Serialize};
#[doc = "Disables the fetch domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Fetch.disable")]
    Disable,
}
#[doc = "Disables the fetch domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "Fetch.disable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enables issuing of requestPaused events. A request will be paused until client\ncalls one of failRequest, fulfillRequest or continueRequest/continueWithAuth.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {
    #[doc = "If specified, only requests matching any of these patterns will produce\nfetchRequested event and will be paused until clients response. If not set,\nall requests will be affected."]
    #[serde(rename = "patterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub patterns: Option<Vec<super::types::RequestPattern>>,
    #[doc = "If true, authRequired events will be issued and requests will be paused\nexpecting a call to continueWithAuth."]
    #[serde(rename = "handleAuthRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle_auth_requests: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Fetch.enable")]
    Enable,
}
#[doc = "Enables issuing of requestPaused events. A request will be paused until client\ncalls one of failRequest, fulfillRequest or continueRequest/continueWithAuth.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "Fetch.enable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Causes the request to fail with specified reason.\n[failRequest](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-failRequest)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FailRequestParams {
    #[doc = "An id the client received in requestPaused event."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Causes the request to fail with the given reason."]
    #[serde(rename = "errorReason")]
    pub error_reason: crate::browser_protocol::network::types::ErrorReason,
}
impl FailRequestParams {
    pub fn new(
        request_id: impl Into<super::types::RequestId>,
        error_reason: impl Into<crate::browser_protocol::network::types::ErrorReason>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            error_reason: error_reason.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FailRequestMethod {
    #[serde(rename = "Fetch.failRequest")]
    FailRequest,
}
#[doc = "Causes the request to fail with specified reason.\n[failRequest](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-failRequest)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FailRequest {
    pub method: FailRequestMethod,
    pub params: FailRequestParams,
}
impl FailRequest {
    pub const IDENTIFIER: &'static str = "Fetch.failRequest";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for FailRequest {
    type Result = super::results::FailRequestResult;
}
#[doc = "Provides response to the request.\n[fulfillRequest](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-fulfillRequest)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FulfillRequestParams {
    #[doc = "An id the client received in requestPaused event."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "An HTTP response code."]
    #[serde(rename = "responseCode")]
    pub response_code: i64,
    #[doc = "Response headers."]
    #[serde(rename = "responseHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub response_headers: Option<Vec<super::types::HeaderEntry>>,
    #[doc = "Alternative way of specifying response headers as a \\0-separated\nseries of name: value pairs. Prefer the above method unless you\nneed to represent some non-UTF8 values that can't be transmitted\nover the protocol as text."]
    #[serde(rename = "binaryResponseHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub binary_response_headers: Option<crate::Binary>,
    #[doc = "A response body. If absent, original response body will be used if\nthe request is intercepted at the response stage and empty body\nwill be used if the request is intercepted at the request stage."]
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub body: Option<crate::Binary>,
    #[doc = "A textual representation of responseCode.\nIf absent, a standard phrase matching responseCode is used."]
    #[serde(rename = "responsePhrase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub response_phrase: Option<String>,
}
impl FulfillRequestParams {
    pub fn new(
        request_id: impl Into<super::types::RequestId>,
        response_code: impl Into<i64>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            response_code: response_code.into(),
            response_headers: None,
            binary_response_headers: None,
            body: None,
            response_phrase: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FulfillRequestMethod {
    #[serde(rename = "Fetch.fulfillRequest")]
    FulfillRequest,
}
#[doc = "Provides response to the request.\n[fulfillRequest](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-fulfillRequest)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FulfillRequest {
    pub method: FulfillRequestMethod,
    pub params: FulfillRequestParams,
}
impl FulfillRequest {
    pub const IDENTIFIER: &'static str = "Fetch.fulfillRequest";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for FulfillRequest {
    type Result = super::results::FulfillRequestResult;
}
#[doc = "Continues the request, optionally modifying some of its parameters.\n[continueRequest](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-continueRequest)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueRequestParams {
    #[doc = "An id the client received in requestPaused event."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "If set, the request url will be modified in a way that's not observable by page."]
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
    #[doc = "If set, the request method is overridden."]
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub method: Option<String>,
    #[doc = "If set, overrides the post data in the request."]
    #[serde(rename = "postData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub post_data: Option<crate::Binary>,
    #[doc = "If set, overrides the request headers. Note that the overrides do not\nextend to subsequent redirect hops, if a redirect happens. Another override\nmay be applied to a different request produced by a redirect."]
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub headers: Option<Vec<super::types::HeaderEntry>>,
    #[doc = "If set, overrides response interception behavior for this request."]
    #[serde(rename = "interceptResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub intercept_response: Option<bool>,
}
impl ContinueRequestParams {
    pub fn new(request_id: impl Into<super::types::RequestId>) -> Self {
        Self {
            request_id: request_id.into(),
            url: None,
            method: None,
            post_data: None,
            headers: None,
            intercept_response: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContinueRequestMethod {
    #[serde(rename = "Fetch.continueRequest")]
    ContinueRequest,
}
#[doc = "Continues the request, optionally modifying some of its parameters.\n[continueRequest](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-continueRequest)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueRequest {
    pub method: ContinueRequestMethod,
    pub params: ContinueRequestParams,
}
impl ContinueRequest {
    pub const IDENTIFIER: &'static str = "Fetch.continueRequest";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ContinueRequest {
    type Result = super::results::ContinueRequestResult;
}
#[doc = "Continues a request supplying authChallengeResponse following authRequired event.\n[continueWithAuth](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-continueWithAuth)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueWithAuthParams {
    #[doc = "An id the client received in authRequired event."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Response to  with an authChallenge."]
    #[serde(rename = "authChallengeResponse")]
    pub auth_challenge_response: super::types::AuthChallengeResponse,
}
impl ContinueWithAuthParams {
    pub fn new(
        request_id: impl Into<super::types::RequestId>,
        auth_challenge_response: impl Into<super::types::AuthChallengeResponse>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            auth_challenge_response: auth_challenge_response.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContinueWithAuthMethod {
    #[serde(rename = "Fetch.continueWithAuth")]
    ContinueWithAuth,
}
#[doc = "Continues a request supplying authChallengeResponse following authRequired event.\n[continueWithAuth](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-continueWithAuth)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueWithAuth {
    pub method: ContinueWithAuthMethod,
    pub params: ContinueWithAuthParams,
}
impl ContinueWithAuth {
    pub const IDENTIFIER: &'static str = "Fetch.continueWithAuth";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ContinueWithAuth {
    type Result = super::results::ContinueWithAuthResult;
}
#[doc = "Continues loading of the paused response, optionally modifying the\nresponse headers. If either responseCode or headers are modified, all of them\nmust be present.\n[continueResponse](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-continueResponse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueResponseParams {
    #[doc = "An id the client received in requestPaused event."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "An HTTP response code. If absent, original response code will be used."]
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub response_code: Option<i64>,
    #[doc = "A textual representation of responseCode.\nIf absent, a standard phrase matching responseCode is used."]
    #[serde(rename = "responsePhrase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub response_phrase: Option<String>,
    #[doc = "Response headers. If absent, original response headers will be used."]
    #[serde(rename = "responseHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub response_headers: Option<Vec<super::types::HeaderEntry>>,
    #[doc = "Alternative way of specifying response headers as a \\0-separated\nseries of name: value pairs. Prefer the above method unless you\nneed to represent some non-UTF8 values that can't be transmitted\nover the protocol as text."]
    #[serde(rename = "binaryResponseHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub binary_response_headers: Option<crate::Binary>,
}
impl ContinueResponseParams {
    pub fn new(request_id: impl Into<super::types::RequestId>) -> Self {
        Self {
            request_id: request_id.into(),
            response_code: None,
            response_phrase: None,
            response_headers: None,
            binary_response_headers: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContinueResponseMethod {
    #[serde(rename = "Fetch.continueResponse")]
    ContinueResponse,
}
#[doc = "Continues loading of the paused response, optionally modifying the\nresponse headers. If either responseCode or headers are modified, all of them\nmust be present.\n[continueResponse](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-continueResponse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueResponse {
    pub method: ContinueResponseMethod,
    pub params: ContinueResponseParams,
}
impl ContinueResponse {
    pub const IDENTIFIER: &'static str = "Fetch.continueResponse";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ContinueResponse {
    type Result = super::results::ContinueResponseResult;
}
#[doc = "Causes the body of the response to be received from the server and\nreturned as a single string. May only be issued for a request that\nis paused in the Response stage and is mutually exclusive with\ntakeResponseBodyForInterceptionAsStream. Calling other methods that\naffect the request or disabling fetch domain before body is received\nresults in an undefined behavior.\nNote that the response body is not available for redirects. Requests\npaused in the _redirect received_ state may be differentiated by\n`responseCode` and presence of `location` response header, see\ncomments to `requestPaused` for details.\n[getResponseBody](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-getResponseBody)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResponseBodyParams {
    #[doc = "Identifier for the intercepted request to get body for."]
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
    #[serde(rename = "Fetch.getResponseBody")]
    GetResponseBody,
}
#[doc = "Causes the body of the response to be received from the server and\nreturned as a single string. May only be issued for a request that\nis paused in the Response stage and is mutually exclusive with\ntakeResponseBodyForInterceptionAsStream. Calling other methods that\naffect the request or disabling fetch domain before body is received\nresults in an undefined behavior.\nNote that the response body is not available for redirects. Requests\npaused in the _redirect received_ state may be differentiated by\n`responseCode` and presence of `location` response header, see\ncomments to `requestPaused` for details.\n[getResponseBody](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-getResponseBody)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResponseBody {
    pub method: GetResponseBodyMethod,
    pub params: GetResponseBodyParams,
}
impl GetResponseBody {
    pub const IDENTIFIER: &'static str = "Fetch.getResponseBody";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetResponseBody {
    type Result = super::results::GetResponseBodyResult;
}
#[doc = "Returns a handle to the stream representing the response body.\nThe request must be paused in the HeadersReceived stage.\nNote that after this command the request can't be continued\nas is -- client either needs to cancel it or to provide the\nresponse body.\nThe stream only supports sequential read, IO.read will fail if the position\nis specified.\nThis method is mutually exclusive with getResponseBody.\nCalling other methods that affect the request or disabling fetch\ndomain before body is received results in an undefined behavior.\n[takeResponseBodyAsStream](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-takeResponseBodyAsStream)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeResponseBodyAsStreamParams {
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
}
impl TakeResponseBodyAsStreamParams {
    pub fn new(request_id: impl Into<super::types::RequestId>) -> Self {
        Self {
            request_id: request_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TakeResponseBodyAsStreamMethod {
    #[serde(rename = "Fetch.takeResponseBodyAsStream")]
    TakeResponseBodyAsStream,
}
#[doc = "Returns a handle to the stream representing the response body.\nThe request must be paused in the HeadersReceived stage.\nNote that after this command the request can't be continued\nas is -- client either needs to cancel it or to provide the\nresponse body.\nThe stream only supports sequential read, IO.read will fail if the position\nis specified.\nThis method is mutually exclusive with getResponseBody.\nCalling other methods that affect the request or disabling fetch\ndomain before body is received results in an undefined behavior.\n[takeResponseBodyAsStream](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#method-takeResponseBodyAsStream)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeResponseBodyAsStream {
    pub method: TakeResponseBodyAsStreamMethod,
    pub params: TakeResponseBodyAsStreamParams,
}
impl TakeResponseBodyAsStream {
    pub const IDENTIFIER: &'static str = "Fetch.takeResponseBodyAsStream";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for TakeResponseBodyAsStream {
    type Result = super::results::TakeResponseBodyAsStreamResult;
}
group_enum ! (FetchCommands { Disable (Disable) , Enable (Enable) , FailRequest (FailRequest) , FulfillRequest (FulfillRequest) , ContinueRequest (ContinueRequest) , ContinueWithAuth (ContinueWithAuth) , ContinueResponse (ContinueResponse) , GetResponseBody (GetResponseBody) , TakeResponseBodyAsStream (TakeResponseBodyAsStream) });
