use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartitionKey {
    #[serde(rename = "userContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_context: Option<String>,
    #[serde(rename = "sourceOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_origin: Option<String>,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl PartitionKey {
    pub fn new(
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            extensible: extensible.into(),
            user_context: None,
            source_origin: None,
        }
    }
}
impl PartitionKey {
    pub const IDENTIFIER: &'static str = "storage.PartitionKey";
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookieFilter {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<crate::network::types::BytesValue>,
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub path: Option<String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(rename = "httpOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub http_only: Option<bool>,
    #[serde(rename = "secure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub secure: Option<bool>,
    #[serde(rename = "sameSite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub same_site: Option<crate::network::types::SameSite>,
    #[serde(rename = "expiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub expiry: Option<u64>,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl CookieFilter {
    pub fn new(
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            extensible: extensible.into(),
            name: None,
            value: None,
            domain: None,
            path: None,
            size: None,
            http_only: None,
            secure: None,
            same_site: None,
            expiry: None,
        }
    }
}
impl CookieFilter {
    pub const IDENTIFIER: &'static str = "storage.CookieFilter";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BrowsingContextPartitionDescriptor {
    #[serde(rename = "type")]
    pub r#type: BrowsingContextPartitionDescriptorType,
    #[serde(rename = "context")]
    pub context: crate::browsing_context::types::BrowsingContext,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BrowsingContextPartitionDescriptorType {
    #[serde(rename = "context")]
    Context,
}
impl BrowsingContextPartitionDescriptor {
    pub fn new(
        r#type: impl Into<BrowsingContextPartitionDescriptorType>,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            context: context.into(),
        }
    }
}
impl BrowsingContextPartitionDescriptor {
    pub const IDENTIFIER: &'static str = "storage.BrowsingContextPartitionDescriptor";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageKeyPartitionDescriptor {
    #[serde(rename = "type")]
    pub r#type: StorageKeyPartitionDescriptorType,
    #[serde(rename = "userContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_context: Option<String>,
    #[serde(rename = "sourceOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_origin: Option<String>,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageKeyPartitionDescriptorType {
    #[serde(rename = "storageKey")]
    StorageKey,
}
impl StorageKeyPartitionDescriptor {
    pub fn new(
        r#type: impl Into<StorageKeyPartitionDescriptorType>,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            extensible: extensible.into(),
            user_context: None,
            source_origin: None,
        }
    }
}
impl StorageKeyPartitionDescriptor {
    pub const IDENTIFIER: &'static str = "storage.StorageKeyPartitionDescriptor";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PartitionDescriptor {
    BrowsingContextPartitionDescriptor(BrowsingContextPartitionDescriptor),
    StorageKeyPartitionDescriptor(StorageKeyPartitionDescriptor),
}
impl From<BrowsingContextPartitionDescriptor> for PartitionDescriptor {
    fn from(v: BrowsingContextPartitionDescriptor) -> Self {
        PartitionDescriptor::BrowsingContextPartitionDescriptor(v)
    }
}
impl TryFrom<PartitionDescriptor> for BrowsingContextPartitionDescriptor {
    type Error = PartitionDescriptor;
    fn try_from(e: PartitionDescriptor) -> Result<Self, Self::Error> {
        match e {
            PartitionDescriptor::BrowsingContextPartitionDescriptor(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<StorageKeyPartitionDescriptor> for PartitionDescriptor {
    fn from(v: StorageKeyPartitionDescriptor) -> Self {
        PartitionDescriptor::StorageKeyPartitionDescriptor(v)
    }
}
impl TryFrom<PartitionDescriptor> for StorageKeyPartitionDescriptor {
    type Error = PartitionDescriptor;
    fn try_from(e: PartitionDescriptor) -> Result<Self, Self::Error> {
        match e {
            PartitionDescriptor::StorageKeyPartitionDescriptor(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartialCookie {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: crate::network::types::BytesValue,
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub path: Option<String>,
    #[serde(rename = "httpOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub http_only: Option<bool>,
    #[serde(rename = "secure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub secure: Option<bool>,
    #[serde(rename = "sameSite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub same_site: Option<crate::network::types::SameSite>,
    #[serde(rename = "expiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub expiry: Option<u64>,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl PartialCookie {
    pub fn new(
        name: impl Into<String>,
        value: impl Into<crate::network::types::BytesValue>,
        domain: impl Into<String>,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            domain: domain.into(),
            extensible: extensible.into(),
            path: None,
            http_only: None,
            secure: None,
            same_site: None,
            expiry: None,
        }
    }
}
impl PartialCookie {
    pub const IDENTIFIER: &'static str = "storage.PartialCookie";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (StorageType { PartitionKey (PartitionKey) , CookieFilter (CookieFilter) , PartitionDescriptor (PartitionDescriptor) , PartialCookie (PartialCookie) });
