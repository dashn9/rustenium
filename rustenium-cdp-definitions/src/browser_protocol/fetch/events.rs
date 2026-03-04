use serde::{Deserialize, Serialize};
#[doc = "Issued when the domain is enabled and the request URL matches the\nspecified filter. The request is paused until the client responds\nwith one of continueRequest, failRequest or fulfillRequest.\nThe stage of the request can be determined by presence of responseErrorReason\nand responseStatusCode -- the request is at the response stage if either\nof these fields is present and in the request stage otherwise.\nRedirect responses and subsequent requests are reported similarly to regular\nresponses and requests. Redirect responses may be distinguished by the value\nof `responseStatusCode` (which is one of 301, 302, 303, 307, 308) along with\npresence of the `location` header. Requests resulting from a redirect will\nhave `redirectedRequestId` field set.\n[requestPaused](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#event-requestPaused)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestPaused {
    #[doc = "Each request the page makes will have a unique id."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "The details of the request."]
    #[serde(rename = "request")]
    pub request: super::super::network::types::Request,
    #[doc = "The id of the frame that initiated the request."]
    #[serde(rename = "frameId")]
    pub frame_id: super::super::page::types::FrameId,
    #[doc = "How the requested resource will be used."]
    #[serde(rename = "resourceType")]
    pub resource_type: super::super::network::types::ResourceType,
    #[doc = "Response error if intercepted at response stage."]
    #[serde(rename = "responseErrorReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub response_error_reason: Option<super::super::network::types::ErrorReason>,
    #[doc = "Response code if intercepted at response stage."]
    #[serde(rename = "responseStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub response_status_code: Option<i64>,
    #[doc = "Response status text if intercepted at response stage."]
    #[serde(rename = "responseStatusText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub response_status_text: Option<String>,
    #[doc = "Response headers if intercepted at the response stage."]
    #[serde(rename = "responseHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub response_headers: Option<Vec<super::types::HeaderEntry>>,
    #[doc = "If the intercepted request had a corresponding Network.requestWillBeSent event fired for it,\nthen this networkId will be the same as the requestId present in the requestWillBeSent event."]
    #[serde(rename = "networkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub network_id: Option<super::super::network::types::RequestId>,
    #[doc = "If the request is due to a redirect response from the server, the id of the request that\nhas caused the redirect."]
    #[serde(rename = "redirectedRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub redirected_request_id: Option<super::types::RequestId>,
}
impl RequestPaused {
    pub const IDENTIFIER: &'static str = "Fetch.requestPaused";
}
#[doc = "Issued when the domain is enabled with handleAuthRequests set to true.\nThe request is paused until client responds with continueWithAuth.\n[authRequired](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#event-authRequired)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRequired {
    #[doc = "Each request the page makes will have a unique id."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "The details of the request."]
    #[serde(rename = "request")]
    pub request: super::super::network::types::Request,
    #[doc = "The id of the frame that initiated the request."]
    #[serde(rename = "frameId")]
    pub frame_id: super::super::page::types::FrameId,
    #[doc = "How the requested resource will be used."]
    #[serde(rename = "resourceType")]
    pub resource_type: super::super::network::types::ResourceType,
    #[doc = "Details of the Authorization Challenge encountered.\nIf this is set, client should respond with continueRequest that\ncontains AuthChallengeResponse."]
    #[serde(rename = "authChallenge")]
    pub auth_challenge: super::types::AuthChallenge,
}
impl AuthRequired {
    pub const IDENTIFIER: &'static str = "Fetch.authRequired";
}
group_enum ! (Event { RequestPaused (RequestPaused) , AuthRequired (AuthRequired) });
