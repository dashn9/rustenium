use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestCacheNamesReturns {
    #[doc = "Caches for the security origin."]
    #[serde(rename = "caches")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caches: Vec<super::types::Cache>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestCachedResponseReturns {
    #[doc = "Response read from the cache."]
    #[serde(rename = "response")]
    pub response: super::types::CachedResponse,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestEntriesReturns {
    #[doc = "Array of object store data entries."]
    #[serde(rename = "cacheDataEntries")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cache_data_entries: Vec<super::types::DataEntry>,
    #[doc = "Count of returned entries from this storage. If pathFilter is empty, it\nis the count of all entries from this storage."]
    #[serde(rename = "returnCount")]
    pub return_count: f64,
}
