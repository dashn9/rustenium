use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadReturns {
    #[doc = "Set if the data is base64-encoded"]
    #[serde(rename = "base64Encoded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub base64_encoded: Option<bool>,
    #[doc = "Data that were read."]
    #[serde(rename = "data")]
    pub data: String,
    #[doc = "Set if the end-of-file condition occurred while reading."]
    #[serde(rename = "eof")]
    pub eof: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveBlobReturns {
    #[doc = "UUID of the specified Blob."]
    #[serde(rename = "uuid")]
    pub uuid: String,
}
