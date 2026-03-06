use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCookiesResult {
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookies: Vec<crate::network::types::Cookie>,
    #[serde(rename = "partitionKey")]
    pub partition_key: super::types::PartitionKey,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCookieResult {
    #[serde(rename = "partitionKey")]
    pub partition_key: super::types::PartitionKey,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCookiesResult {
    #[serde(rename = "partitionKey")]
    pub partition_key: super::types::PartitionKey,
}
