use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FailRequestResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FulfillRequestResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ContinueRequestResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ContinueWithAuthResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ContinueResponseResult {}
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
pub struct TakeResponseBodyAsStreamResult {
    #[serde(rename = "stream")]
    pub stream: crate::browser_protocol::io::types::StreamHandle,
}
