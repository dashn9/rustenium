// Generated types for module

use serde::{Serialize, Deserialize};
use crate::browsing_context::types::BrowsingContext;
use crate::network::types::BytesValue;
use crate::network::types::SameSite;
use crate::Extensible;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieFilter {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "value")]
    pub value: Option<BytesValue>,
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    #[serde(rename = "path")]
    pub path: Option<String>,
    #[serde(rename = "size")]
    pub size: Option<u64>,
    #[serde(rename = "httpOnly")]
    pub http_only: Option<bool>,
    #[serde(rename = "secure")]
    pub secure: Option<bool>,
    #[serde(rename = "sameSite")]
    pub same_site: Option<SameSite>,
    #[serde(rename = "expiry")]
    pub expiry: Option<u64>,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContextEnum {
    #[serde(rename = "context")]
    Context,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowsingContextPartitionDescriptor {
    #[serde(rename = "type")]
    pub r#type: ContextEnum,
    #[serde(rename = "context")]
    pub context: BrowsingContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StorageKeyEnum {
    #[serde(rename = "storageKey")]
    StorageKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageKeyPartitionDescriptor {
    #[serde(rename = "type")]
    pub r#type: StorageKeyEnum,
    #[serde(rename = "userContext")]
    pub user_context: Option<String>,
    #[serde(rename = "sourceOrigin")]
    pub source_origin: Option<String>,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PartitionDescriptor {
    BrowsingContextPartitionDescriptor(BrowsingContextPartitionDescriptor),
    StorageKeyPartitionDescriptor(StorageKeyPartitionDescriptor),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialCookie {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: BytesValue,
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "path")]
    pub path: Option<String>,
    #[serde(rename = "httpOnly")]
    pub http_only: Option<bool>,
    #[serde(rename = "secure")]
    pub secure: Option<bool>,
    #[serde(rename = "sameSite")]
    pub same_site: Option<SameSite>,
    #[serde(rename = "expiry")]
    pub expiry: Option<u64>,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionKey {
    #[serde(rename = "userContext")]
    pub user_context: Option<String>,
    #[serde(rename = "sourceOrigin")]
    pub source_origin: Option<String>,
    #[serde(flatten)]
    pub extensible: Extensible,
}

