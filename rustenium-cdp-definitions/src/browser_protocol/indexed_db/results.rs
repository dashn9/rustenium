use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestDataReturns {
    #[doc = "Array of object store data entries."]
    #[serde(rename = "objectStoreDataEntries")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub object_store_data_entries: Vec<super::types::DataEntry>,
    #[doc = "If true, there are more entries to fetch in the given range."]
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMetadataReturns {
    #[doc = "the entries count"]
    #[serde(rename = "entriesCount")]
    pub entries_count: f64,
    #[doc = "the current value of key generator, to become the next inserted\nkey into the object store. Valid if objectStore.autoIncrement\nis true."]
    #[serde(rename = "keyGeneratorValue")]
    pub key_generator_value: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestDatabaseReturns {
    #[doc = "Database with an array of object stores."]
    #[serde(rename = "databaseWithObjectStores")]
    pub database_with_object_stores: super::types::DatabaseWithObjectStores,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestDatabaseNamesReturns {
    #[doc = "Database names for origin."]
    #[serde(rename = "databaseNames")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub database_names: Vec<String>,
}
