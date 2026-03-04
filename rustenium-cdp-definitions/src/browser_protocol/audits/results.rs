use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEncodedResponseReturns {
    #[doc = "The encoded body as a base64 string. Omitted if sizeOnly is true."]
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub body: Option<super::super::super::Binary>,
    #[doc = "Size before re-encoding."]
    #[serde(rename = "originalSize")]
    pub original_size: i64,
    #[doc = "Size after re-encoding."]
    #[serde(rename = "encodedSize")]
    pub encoded_size: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckFormsIssuesReturns {
    #[serde(rename = "formIssues")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub form_issues: Vec<super::types::GenericIssueDetails>,
}
