use serde::{Deserialize, Serialize};
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
pub struct TakeResponseBodyAsStreamReturns {
    #[serde(rename = "stream")]
    pub stream: super::super::io::types::StreamHandle,
}
