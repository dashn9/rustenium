use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearObjectStoreResult {}
impl TryFrom<serde_json::Value> for ClearObjectStoreResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteDatabaseResult {}
impl TryFrom<serde_json::Value> for DeleteDatabaseResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteObjectStoreEntriesResult {}
impl TryFrom<serde_json::Value> for DeleteObjectStoreEntriesResult {
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestDataResult {
    #[doc = "Array of object store data entries."]
    #[serde(rename = "objectStoreDataEntries")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub object_store_data_entries: Vec<super::types::DataEntry>,
    #[doc = "If true, there are more entries to fetch in the given range."]
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}
impl TryFrom<serde_json::Value> for RequestDataResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMetadataResult {
    #[doc = "the entries count"]
    #[serde(rename = "entriesCount")]
    pub entries_count: f64,
    #[doc = "the current value of key generator, to become the next inserted\nkey into the object store. Valid if objectStore.autoIncrement\nis true."]
    #[serde(rename = "keyGeneratorValue")]
    pub key_generator_value: f64,
}
impl TryFrom<serde_json::Value> for GetMetadataResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestDatabaseResult {
    #[doc = "Database with an array of object stores."]
    #[serde(rename = "databaseWithObjectStores")]
    pub database_with_object_stores: super::types::DatabaseWithObjectStores,
}
impl TryFrom<serde_json::Value> for RequestDatabaseResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestDatabaseNamesResult {
    #[doc = "Database names for origin."]
    #[serde(rename = "databaseNames")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub database_names: Vec<String>,
}
impl TryFrom<serde_json::Value> for RequestDatabaseNamesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
