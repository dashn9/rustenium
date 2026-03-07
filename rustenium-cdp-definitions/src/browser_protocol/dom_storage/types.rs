use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct SerializedStorageKey(String);
impl SerializedStorageKey {
    pub fn new(val: impl Into<String>) -> Self {
        SerializedStorageKey(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for SerializedStorageKey {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<SerializedStorageKey> for String {
    fn from(el: SerializedStorageKey) -> String {
        el.0
    }
}
impl From<String> for SerializedStorageKey {
    fn from(expr: String) -> Self {
        SerializedStorageKey(expr)
    }
}
impl SerializedStorageKey {
    pub const IDENTIFIER: &'static str = "DOMStorage.SerializedStorageKey";
}
#[doc = "DOM Storage identifier.\n[StorageId](https://chromedevtools.github.io/devtools-protocol/tot/DOMStorage/#type-StorageId)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageId {
    #[doc = "Security origin for the storage."]
    #[serde(rename = "securityOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub security_origin: Option<String>,
    #[doc = "Represents a key by which DOM Storage keys its CachedStorageAreas"]
    #[serde(rename = "storageKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_key: Option<SerializedStorageKey>,
    #[doc = "Whether the storage is local storage (not session storage)."]
    #[serde(rename = "isLocalStorage")]
    pub is_local_storage: bool,
}
impl StorageId {
    pub fn new(is_local_storage: impl Into<bool>) -> Self {
        Self {
            is_local_storage: is_local_storage.into(),
            security_origin: None,
            storage_key: None,
        }
    }
}
impl StorageId {
    pub const IDENTIFIER: &'static str = "DOMStorage.StorageId";
}
#[doc = "DOM Storage item.\n[Item](https://chromedevtools.github.io/devtools-protocol/tot/DOMStorage/#type-Item)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item(Vec<String>);
impl Item {
    pub fn new(val: impl Into<Vec<String>>) -> Self {
        Item(val.into())
    }
    pub fn inner(&self) -> &Vec<String> {
        &self.0
    }
}
impl Item {
    pub const IDENTIFIER: &'static str = "DOMStorage.Item";
}
group_enum ! (DomStorageTypes { SerializedStorageKey (SerializedStorageKey) , StorageId (StorageId) , Item (Item) });
