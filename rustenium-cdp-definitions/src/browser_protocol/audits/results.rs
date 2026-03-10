use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEncodedResponseResult {
    #[doc = "The encoded body as a base64 string. Omitted if sizeOnly is true."]
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub body: Option<crate::Binary>,
    #[doc = "Size before re-encoding."]
    #[serde(rename = "originalSize")]
    pub original_size: i64,
    #[doc = "Size after re-encoding."]
    #[serde(rename = "encodedSize")]
    pub encoded_size: i64,
}
impl TryFrom<serde_json::Value> for GetEncodedResponseResult {
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
pub struct EnableResult {}
impl TryFrom<serde_json::Value> for EnableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CheckContrastResult {}
impl TryFrom<serde_json::Value> for CheckContrastResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckFormsIssuesResult {
    #[serde(rename = "formIssues")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub form_issues: Vec<super::types::GenericIssueDetails>,
}
impl TryFrom<serde_json::Value> for CheckFormsIssuesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
