use serde::{Deserialize, Serialize};
#[doc = "A cache's contents have been modified.\n[cacheStorageContentUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#event-cacheStorageContentUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CacheStorageContentUpdated {
    #[doc = "Origin to update."]
    #[serde(rename = "origin")]
    pub origin: String,
    #[doc = "Storage key to update."]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[doc = "Storage bucket to update."]
    #[serde(rename = "bucketId")]
    pub bucket_id: String,
    #[doc = "Name of cache in origin."]
    #[serde(rename = "cacheName")]
    pub cache_name: String,
}
impl CacheStorageContentUpdated {
    pub const IDENTIFIER: &'static str = "Storage.cacheStorageContentUpdated";
}
#[doc = "A cache has been added/deleted.\n[cacheStorageListUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#event-cacheStorageListUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CacheStorageListUpdated {
    #[doc = "Origin to update."]
    #[serde(rename = "origin")]
    pub origin: String,
    #[doc = "Storage key to update."]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[doc = "Storage bucket to update."]
    #[serde(rename = "bucketId")]
    pub bucket_id: String,
}
impl CacheStorageListUpdated {
    pub const IDENTIFIER: &'static str = "Storage.cacheStorageListUpdated";
}
#[doc = "The origin's IndexedDB object store has been modified.\n[indexedDBContentUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#event-indexedDBContentUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexedDbContentUpdated {
    #[doc = "Origin to update."]
    #[serde(rename = "origin")]
    pub origin: String,
    #[doc = "Storage key to update."]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[doc = "Storage bucket to update."]
    #[serde(rename = "bucketId")]
    pub bucket_id: String,
    #[doc = "Database to update."]
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[doc = "ObjectStore to update."]
    #[serde(rename = "objectStoreName")]
    pub object_store_name: String,
}
impl IndexedDbContentUpdated {
    pub const IDENTIFIER: &'static str = "Storage.indexedDBContentUpdated";
}
#[doc = "The origin's IndexedDB database list has been modified.\n[indexedDBListUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#event-indexedDBListUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexedDbListUpdated {
    #[doc = "Origin to update."]
    #[serde(rename = "origin")]
    pub origin: String,
    #[doc = "Storage key to update."]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[doc = "Storage bucket to update."]
    #[serde(rename = "bucketId")]
    pub bucket_id: String,
}
impl IndexedDbListUpdated {
    pub const IDENTIFIER: &'static str = "Storage.indexedDBListUpdated";
}
#[doc = "One of the interest groups was accessed. Note that these events are global\nto all targets sharing an interest group store.\n[interestGroupAccessed](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#event-interestGroupAccessed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterestGroupAccessed {
    #[serde(rename = "accessTime")]
    pub access_time: super::super::network::types::TimeSinceEpoch,
    #[serde(rename = "type")]
    pub r#type: super::types::InterestGroupAccessType,
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "For topLevelBid/topLevelAdditionalBid, and when appropriate,\nwin and additionalBidWin"]
    #[serde(rename = "componentSellerOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub component_seller_origin: Option<String>,
    #[doc = "For bid or somethingBid event, if done locally and not on a server."]
    #[serde(rename = "bid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bid: Option<f64>,
    #[serde(rename = "bidCurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bid_currency: Option<String>,
    #[doc = "For non-global events --- links to interestGroupAuctionEvent"]
    #[serde(rename = "uniqueAuctionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unique_auction_id: Option<super::types::InterestGroupAuctionId>,
}
impl InterestGroupAccessed {
    pub const IDENTIFIER: &'static str = "Storage.interestGroupAccessed";
}
#[doc = "An auction involving interest groups is taking place. These events are\ntarget-specific.\n[interestGroupAuctionEventOccurred](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#event-interestGroupAuctionEventOccurred)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterestGroupAuctionEventOccurred {
    #[serde(rename = "eventTime")]
    pub event_time: super::super::network::types::TimeSinceEpoch,
    #[serde(rename = "type")]
    pub r#type: super::types::InterestGroupAuctionEventType,
    #[serde(rename = "uniqueAuctionId")]
    pub unique_auction_id: super::types::InterestGroupAuctionId,
    #[doc = "Set for child auctions."]
    #[serde(rename = "parentAuctionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent_auction_id: Option<super::types::InterestGroupAuctionId>,
    #[doc = "Set for started and configResolved"]
    #[serde(rename = "auctionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub auction_config: Option<serde_json::Value>,
}
impl InterestGroupAuctionEventOccurred {
    pub const IDENTIFIER: &'static str = "Storage.interestGroupAuctionEventOccurred";
}
#[doc = "Specifies which auctions a particular network fetch may be related to, and\nin what role. Note that it is not ordered with respect to\nNetwork.requestWillBeSent (but will happen before loadingFinished\nloadingFailed).\n[interestGroupAuctionNetworkRequestCreated](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#event-interestGroupAuctionNetworkRequestCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterestGroupAuctionNetworkRequestCreated {
    #[serde(rename = "type")]
    pub r#type: super::types::InterestGroupAuctionFetchType,
    #[serde(rename = "requestId")]
    pub request_id: super::super::network::types::RequestId,
    #[doc = "This is the set of the auctions using the worklet that issued this\nrequest.  In the case of trusted signals, it's possible that only some of\nthem actually care about the keys being queried."]
    #[serde(rename = "auctions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub auctions: Vec<super::types::InterestGroupAuctionId>,
}
impl InterestGroupAuctionNetworkRequestCreated {
    pub const IDENTIFIER: &'static str = "Storage.interestGroupAuctionNetworkRequestCreated";
}
#[doc = "Shared storage was accessed by the associated page.\nThe following parameters are included in all events.\n[sharedStorageAccessed](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#event-sharedStorageAccessed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedStorageAccessed {
    #[doc = "Time of the access."]
    #[serde(rename = "accessTime")]
    pub access_time: super::super::network::types::TimeSinceEpoch,
    #[doc = "Enum value indicating the access scope."]
    #[serde(rename = "scope")]
    pub scope: super::types::SharedStorageAccessScope,
    #[doc = "Enum value indicating the Shared Storage API method invoked."]
    #[serde(rename = "method")]
    pub method: super::types::SharedStorageAccessMethod,
    #[doc = "DevTools Frame Token for the primary frame tree's root."]
    #[serde(rename = "mainFrameId")]
    pub main_frame_id: super::super::page::types::FrameId,
    #[doc = "Serialization of the origin owning the Shared Storage data."]
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
    #[doc = "Serialization of the site owning the Shared Storage data."]
    #[serde(rename = "ownerSite")]
    pub owner_site: String,
    #[doc = "The sub-parameters wrapped by `params` are all optional and their\npresence/absence depends on `type`."]
    #[serde(rename = "params")]
    pub params: super::types::SharedStorageAccessParams,
}
impl SharedStorageAccessed {
    pub const IDENTIFIER: &'static str = "Storage.sharedStorageAccessed";
}
#[doc = "A shared storage run or selectURL operation finished its execution.\nThe following parameters are included in all events.\n[sharedStorageWorkletOperationExecutionFinished](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#event-sharedStorageWorkletOperationExecutionFinished)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedStorageWorkletOperationExecutionFinished {
    #[doc = "Time that the operation finished."]
    #[serde(rename = "finishedTime")]
    pub finished_time: super::super::network::types::TimeSinceEpoch,
    #[doc = "Time, in microseconds, from start of shared storage JS API call until\nend of operation execution in the worklet."]
    #[serde(rename = "executionTime")]
    pub execution_time: i64,
    #[doc = "Enum value indicating the Shared Storage API method invoked."]
    #[serde(rename = "method")]
    pub method: super::types::SharedStorageAccessMethod,
    #[doc = "ID of the operation call."]
    #[serde(rename = "operationId")]
    pub operation_id: String,
    #[doc = "Hex representation of the DevTools token used as the TargetID for the\nassociated shared storage worklet."]
    #[serde(rename = "workletTargetId")]
    pub worklet_target_id: super::super::target::types::TargetId,
    #[doc = "DevTools Frame Token for the primary frame tree's root."]
    #[serde(rename = "mainFrameId")]
    pub main_frame_id: super::super::page::types::FrameId,
    #[doc = "Serialization of the origin owning the Shared Storage data."]
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
}
impl SharedStorageWorkletOperationExecutionFinished {
    pub const IDENTIFIER: &'static str = "Storage.sharedStorageWorkletOperationExecutionFinished";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageBucketCreatedOrUpdated {
    #[serde(rename = "bucketInfo")]
    pub bucket_info: super::types::StorageBucketInfo,
}
impl StorageBucketCreatedOrUpdated {
    pub const IDENTIFIER: &'static str = "Storage.storageBucketCreatedOrUpdated";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageBucketDeleted {
    #[serde(rename = "bucketId")]
    pub bucket_id: String,
}
impl StorageBucketDeleted {
    pub const IDENTIFIER: &'static str = "Storage.storageBucketDeleted";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingSourceRegistered {
    #[serde(rename = "registration")]
    pub registration: super::types::AttributionReportingSourceRegistration,
    #[serde(rename = "result")]
    pub result: super::types::AttributionReportingSourceRegistrationResult,
}
impl AttributionReportingSourceRegistered {
    pub const IDENTIFIER: &'static str = "Storage.attributionReportingSourceRegistered";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingTriggerRegistered {
    #[serde(rename = "registration")]
    pub registration: super::types::AttributionReportingTriggerRegistration,
    #[serde(rename = "eventLevel")]
    pub event_level: super::types::AttributionReportingEventLevelResult,
    #[serde(rename = "aggregatable")]
    pub aggregatable: super::types::AttributionReportingAggregatableResult,
}
impl AttributionReportingTriggerRegistered {
    pub const IDENTIFIER: &'static str = "Storage.attributionReportingTriggerRegistered";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingReportSent {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(rename = "result")]
    pub result: super::types::AttributionReportingReportResult,
    #[doc = "If result is `sent`, populated with net/HTTP status."]
    #[serde(rename = "netError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub net_error: Option<i64>,
    #[serde(rename = "netErrorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub net_error_name: Option<String>,
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub http_status_code: Option<i64>,
}
impl AttributionReportingReportSent {
    pub const IDENTIFIER: &'static str = "Storage.attributionReportingReportSent";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingVerboseDebugReportSent {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub body: Option<Vec<serde_json::Value>>,
    #[serde(rename = "netError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub net_error: Option<i64>,
    #[serde(rename = "netErrorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub net_error_name: Option<String>,
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub http_status_code: Option<i64>,
}
impl AttributionReportingVerboseDebugReportSent {
    pub const IDENTIFIER: &'static str = "Storage.attributionReportingVerboseDebugReportSent";
}
group_enum ! (StorageEvents { CacheStorageContentUpdated (CacheStorageContentUpdated) , CacheStorageListUpdated (CacheStorageListUpdated) , IndexedDbContentUpdated (IndexedDbContentUpdated) , IndexedDbListUpdated (IndexedDbListUpdated) , InterestGroupAccessed (InterestGroupAccessed) , InterestGroupAuctionEventOccurred (InterestGroupAuctionEventOccurred) , InterestGroupAuctionNetworkRequestCreated (InterestGroupAuctionNetworkRequestCreated) , SharedStorageAccessed (SharedStorageAccessed) , SharedStorageWorkletOperationExecutionFinished (SharedStorageWorkletOperationExecutionFinished) , StorageBucketCreatedOrUpdated (StorageBucketCreatedOrUpdated) , StorageBucketDeleted (StorageBucketDeleted) , AttributionReportingSourceRegistered (AttributionReportingSourceRegistered) , AttributionReportingTriggerRegistered (AttributionReportingTriggerRegistered) , AttributionReportingReportSent (AttributionReportingReportSent) , AttributionReportingVerboseDebugReportSent (AttributionReportingVerboseDebugReportSent) });
