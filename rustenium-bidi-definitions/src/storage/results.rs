use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCookiesResult {
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookies: Vec<crate::network::types::Cookie>,
    #[serde(rename = "partitionKey")]
    pub partition_key: super::types::PartitionKey,
}
impl TryFrom<serde_json::Value> for GetCookiesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCookieResult {
    #[serde(rename = "partitionKey")]
    pub partition_key: super::types::PartitionKey,
}
impl TryFrom<serde_json::Value> for SetCookieResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCookiesResult {
    #[serde(rename = "partitionKey")]
    pub partition_key: super::types::PartitionKey,
}
impl TryFrom<serde_json::Value> for DeleteCookiesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
