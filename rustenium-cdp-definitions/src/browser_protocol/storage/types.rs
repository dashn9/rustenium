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
    pub const IDENTIFIER: &'static str = "Storage.SerializedStorageKey";
}
#[doc = "Enum of possible storage types."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageType {
    #[serde(rename = "cookies")]
    Cookies,
    #[serde(rename = "file_systems")]
    FileSystems,
    #[serde(rename = "indexeddb")]
    Indexeddb,
    #[serde(rename = "local_storage")]
    LocalStorage,
    #[serde(rename = "shader_cache")]
    ShaderCache,
    #[serde(rename = "websql")]
    Websql,
    #[serde(rename = "service_workers")]
    ServiceWorkers,
    #[serde(rename = "cache_storage")]
    CacheStorage,
    #[serde(rename = "interest_groups")]
    InterestGroups,
    #[serde(rename = "shared_storage")]
    SharedStorage,
    #[serde(rename = "storage_buckets")]
    StorageBuckets,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "other")]
    Other,
}
#[doc = "Usage for a storage type.\n[UsageForType](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-UsageForType)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsageForType {
    #[doc = "Name of storage type."]
    #[serde(rename = "storageType")]
    pub storage_type: StorageType,
    #[doc = "Storage usage (bytes)."]
    #[serde(rename = "usage")]
    pub usage: f64,
}
impl UsageForType {
    pub fn new(storage_type: impl Into<StorageType>, usage: impl Into<f64>) -> Self {
        Self {
            storage_type: storage_type.into(),
            usage: usage.into(),
        }
    }
}
impl UsageForType {
    pub const IDENTIFIER: &'static str = "Storage.UsageForType";
}
#[doc = "Pair of issuer origin and number of available (signed, but not used) Trust\nTokens from that issuer.\n[TrustTokens](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-TrustTokens)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrustTokens {
    #[serde(rename = "issuerOrigin")]
    pub issuer_origin: String,
    #[serde(rename = "count")]
    pub count: f64,
}
impl TrustTokens {
    pub fn new(issuer_origin: impl Into<String>, count: impl Into<f64>) -> Self {
        Self {
            issuer_origin: issuer_origin.into(),
            count: count.into(),
        }
    }
}
impl TrustTokens {
    pub const IDENTIFIER: &'static str = "Storage.TrustTokens";
}
#[doc = "Protected audience interest group auction identifier.\n[InterestGroupAuctionId](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-InterestGroupAuctionId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct InterestGroupAuctionId(String);
impl InterestGroupAuctionId {
    pub fn new(val: impl Into<String>) -> Self {
        InterestGroupAuctionId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for InterestGroupAuctionId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<InterestGroupAuctionId> for String {
    fn from(el: InterestGroupAuctionId) -> String {
        el.0
    }
}
impl From<String> for InterestGroupAuctionId {
    fn from(expr: String) -> Self {
        InterestGroupAuctionId(expr)
    }
}
impl std::borrow::Borrow<str> for InterestGroupAuctionId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl InterestGroupAuctionId {
    pub const IDENTIFIER: &'static str = "Storage.InterestGroupAuctionId";
}
#[doc = "Enum of interest group access types."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InterestGroupAccessType {
    #[serde(rename = "join")]
    Join,
    #[serde(rename = "leave")]
    Leave,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "loaded")]
    Loaded,
    #[serde(rename = "bid")]
    Bid,
    #[serde(rename = "win")]
    Win,
    #[serde(rename = "additionalBid")]
    AdditionalBid,
    #[serde(rename = "additionalBidWin")]
    AdditionalBidWin,
    #[serde(rename = "topLevelBid")]
    TopLevelBid,
    #[serde(rename = "topLevelAdditionalBid")]
    TopLevelAdditionalBid,
    #[serde(rename = "clear")]
    Clear,
}
#[doc = "Enum of auction events."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InterestGroupAuctionEventType {
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "configResolved")]
    ConfigResolved,
}
#[doc = "Enum of network fetches auctions can do."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InterestGroupAuctionFetchType {
    #[serde(rename = "bidderJs")]
    BidderJs,
    #[serde(rename = "bidderWasm")]
    BidderWasm,
    #[serde(rename = "sellerJs")]
    SellerJs,
    #[serde(rename = "bidderTrustedSignals")]
    BidderTrustedSignals,
    #[serde(rename = "sellerTrustedSignals")]
    SellerTrustedSignals,
}
#[doc = "Enum of shared storage access scopes."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SharedStorageAccessScope {
    #[serde(rename = "window")]
    Window,
    #[serde(rename = "sharedStorageWorklet")]
    SharedStorageWorklet,
    #[serde(rename = "protectedAudienceWorklet")]
    ProtectedAudienceWorklet,
    #[serde(rename = "header")]
    Header,
}
#[doc = "Enum of shared storage access methods."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SharedStorageAccessMethod {
    #[serde(rename = "addModule")]
    AddModule,
    #[serde(rename = "createWorklet")]
    CreateWorklet,
    #[serde(rename = "selectURL")]
    SelectUrl,
    #[serde(rename = "run")]
    Run,
    #[serde(rename = "batchUpdate")]
    BatchUpdate,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "append")]
    Append,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "get")]
    Get,
    #[serde(rename = "keys")]
    Keys,
    #[serde(rename = "values")]
    Values,
    #[serde(rename = "entries")]
    Entries,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "remainingBudget")]
    RemainingBudget,
}
#[doc = "Struct for a single key-value pair in an origin's shared storage.\n[SharedStorageEntry](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SharedStorageEntry)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedStorageEntry {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl SharedStorageEntry {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}
impl SharedStorageEntry {
    pub const IDENTIFIER: &'static str = "Storage.SharedStorageEntry";
}
#[doc = "Details for an origin's shared storage.\n[SharedStorageMetadata](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SharedStorageMetadata)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedStorageMetadata {
    #[doc = "Time when the origin's shared storage was last created."]
    #[serde(rename = "creationTime")]
    pub creation_time: super::super::network::types::TimeSinceEpoch,
    #[doc = "Number of key-value pairs stored in origin's shared storage."]
    #[serde(rename = "length")]
    pub length: i64,
    #[doc = "Current amount of bits of entropy remaining in the navigation budget."]
    #[serde(rename = "remainingBudget")]
    pub remaining_budget: f64,
    #[doc = "Total number of bytes stored as key-value pairs in origin's shared\nstorage."]
    #[serde(rename = "bytesUsed")]
    pub bytes_used: i64,
}
impl SharedStorageMetadata {
    pub fn new(
        creation_time: impl Into<super::super::network::types::TimeSinceEpoch>,
        length: impl Into<i64>,
        remaining_budget: impl Into<f64>,
        bytes_used: impl Into<i64>,
    ) -> Self {
        Self {
            creation_time: creation_time.into(),
            length: length.into(),
            remaining_budget: remaining_budget.into(),
            bytes_used: bytes_used.into(),
        }
    }
}
impl SharedStorageMetadata {
    pub const IDENTIFIER: &'static str = "Storage.SharedStorageMetadata";
}
#[doc = "Represents a dictionary object passed in as privateAggregationConfig to\nrun or selectURL.\n[SharedStoragePrivateAggregationConfig](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SharedStoragePrivateAggregationConfig)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedStoragePrivateAggregationConfig {
    #[doc = "The chosen aggregation service deployment."]
    #[serde(rename = "aggregationCoordinatorOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub aggregation_coordinator_origin: Option<String>,
    #[doc = "The context ID provided."]
    #[serde(rename = "contextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub context_id: Option<String>,
    #[doc = "Configures the maximum size allowed for filtering IDs."]
    #[serde(rename = "filteringIdMaxBytes")]
    pub filtering_id_max_bytes: i64,
    #[doc = "The limit on the number of contributions in the final report."]
    #[serde(rename = "maxContributions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_contributions: Option<i64>,
}
impl SharedStoragePrivateAggregationConfig {
    pub fn new(filtering_id_max_bytes: impl Into<i64>) -> Self {
        Self {
            filtering_id_max_bytes: filtering_id_max_bytes.into(),
            aggregation_coordinator_origin: None,
            context_id: None,
            max_contributions: None,
        }
    }
}
impl SharedStoragePrivateAggregationConfig {
    pub const IDENTIFIER: &'static str = "Storage.SharedStoragePrivateAggregationConfig";
}
#[doc = "Pair of reporting metadata details for a candidate URL for `selectURL()`.\n[SharedStorageReportingMetadata](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SharedStorageReportingMetadata)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedStorageReportingMetadata {
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(rename = "reportingUrl")]
    pub reporting_url: String,
}
impl SharedStorageReportingMetadata {
    pub fn new(event_type: impl Into<String>, reporting_url: impl Into<String>) -> Self {
        Self {
            event_type: event_type.into(),
            reporting_url: reporting_url.into(),
        }
    }
}
impl SharedStorageReportingMetadata {
    pub const IDENTIFIER: &'static str = "Storage.SharedStorageReportingMetadata";
}
#[doc = "Bundles a candidate URL with its reporting metadata.\n[SharedStorageUrlWithMetadata](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SharedStorageUrlWithMetadata)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedStorageUrlWithMetadata {
    #[doc = "Spec of candidate URL."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Any associated reporting metadata."]
    #[serde(rename = "reportingMetadata")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reporting_metadata: Vec<SharedStorageReportingMetadata>,
}
impl SharedStorageUrlWithMetadata {
    pub fn new(
        url: impl Into<String>,
        reporting_metadata: Vec<SharedStorageReportingMetadata>,
    ) -> Self {
        Self {
            url: url.into(),
            reporting_metadata,
        }
    }
}
impl SharedStorageUrlWithMetadata {
    pub const IDENTIFIER: &'static str = "Storage.SharedStorageUrlWithMetadata";
}
#[doc = "Bundles the parameters for shared storage access events whose\npresence/absence can vary according to SharedStorageAccessType.\n[SharedStorageAccessParams](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SharedStorageAccessParams)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SharedStorageAccessParams {
    #[doc = "Spec of the module script URL.\nPresent only for SharedStorageAccessMethods: addModule and\ncreateWorklet."]
    #[serde(rename = "scriptSourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub script_source_url: Option<String>,
    #[doc = "String denoting \"context-origin\", \"script-origin\", or a custom\norigin to be used as the worklet's data origin.\nPresent only for SharedStorageAccessMethod: createWorklet."]
    #[serde(rename = "dataOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data_origin: Option<String>,
    #[doc = "Name of the registered operation to be run.\nPresent only for SharedStorageAccessMethods: run and selectURL."]
    #[serde(rename = "operationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub operation_name: Option<String>,
    #[doc = "ID of the operation call.\nPresent only for SharedStorageAccessMethods: run and selectURL."]
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub operation_id: Option<String>,
    #[doc = "Whether or not to keep the worket alive for future run or selectURL\ncalls.\nPresent only for SharedStorageAccessMethods: run and selectURL."]
    #[serde(rename = "keepAlive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub keep_alive: Option<bool>,
    #[doc = "Configures the private aggregation options.\nPresent only for SharedStorageAccessMethods: run and selectURL."]
    #[serde(rename = "privateAggregationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub private_aggregation_config: Option<SharedStoragePrivateAggregationConfig>,
    #[doc = "The operation's serialized data in bytes (converted to a string).\nPresent only for SharedStorageAccessMethods: run and selectURL.\nTODO(crbug.com/401011862): Consider updating this parameter to binary."]
    #[serde(rename = "serializedData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub serialized_data: Option<String>,
    #[doc = "Array of candidate URLs' specs, along with any associated metadata.\nPresent only for SharedStorageAccessMethod: selectURL."]
    #[serde(rename = "urlsWithMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub urls_with_metadata: Option<Vec<SharedStorageUrlWithMetadata>>,
    #[doc = "Spec of the URN:UUID generated for a selectURL call.\nPresent only for SharedStorageAccessMethod: selectURL."]
    #[serde(rename = "urnUuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub urn_uuid: Option<String>,
    #[doc = "Key for a specific entry in an origin's shared storage.\nPresent only for SharedStorageAccessMethods: set, append, delete, and\nget."]
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub key: Option<String>,
    #[doc = "Value for a specific entry in an origin's shared storage.\nPresent only for SharedStorageAccessMethods: set and append."]
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<String>,
    #[doc = "Whether or not to set an entry for a key if that key is already present.\nPresent only for SharedStorageAccessMethod: set."]
    #[serde(rename = "ignoreIfPresent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ignore_if_present: Option<bool>,
    #[doc = "A number denoting the (0-based) order of the worklet's\ncreation relative to all other shared storage worklets created by\ndocuments using the current storage partition.\nPresent only for SharedStorageAccessMethods: addModule, createWorklet."]
    #[serde(rename = "workletOrdinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub worklet_ordinal: Option<i64>,
    #[doc = "Hex representation of the DevTools token used as the TargetID for the\nassociated shared storage worklet.\nPresent only for SharedStorageAccessMethods: addModule, createWorklet,\nrun, selectURL, and any other SharedStorageAccessMethod when the\nSharedStorageAccessScope is sharedStorageWorklet."]
    #[serde(rename = "workletTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub worklet_target_id: Option<super::super::target::types::TargetId>,
    #[doc = "Name of the lock to be acquired, if present.\nOptionally present only for SharedStorageAccessMethods: batchUpdate,\nset, append, delete, and clear."]
    #[serde(rename = "withLock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub with_lock: Option<String>,
    #[doc = "If the method has been called as part of a batchUpdate, then this\nnumber identifies the batch to which it belongs.\nOptionally present only for SharedStorageAccessMethods:\nbatchUpdate (required), set, append, delete, and clear."]
    #[serde(rename = "batchUpdateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub batch_update_id: Option<String>,
    #[doc = "Number of modifier methods sent in batch.\nPresent only for SharedStorageAccessMethod: batchUpdate."]
    #[serde(rename = "batchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub batch_size: Option<i64>,
}
impl SharedStorageAccessParams {
    pub const IDENTIFIER: &'static str = "Storage.SharedStorageAccessParams";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageBucketsDurability {
    #[serde(rename = "relaxed")]
    Relaxed,
    #[serde(rename = "strict")]
    Strict,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageBucket {
    #[serde(rename = "storageKey")]
    pub storage_key: SerializedStorageKey,
    #[doc = "If not specified, it is the default bucket of the storageKey."]
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
}
impl StorageBucket {
    pub fn new(storage_key: impl Into<SerializedStorageKey>) -> Self {
        Self {
            storage_key: storage_key.into(),
            name: None,
        }
    }
}
impl StorageBucket {
    pub const IDENTIFIER: &'static str = "Storage.StorageBucket";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageBucketInfo {
    #[serde(rename = "bucket")]
    pub bucket: StorageBucket,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "expiration")]
    pub expiration: super::super::network::types::TimeSinceEpoch,
    #[doc = "Storage quota (bytes)."]
    #[serde(rename = "quota")]
    pub quota: f64,
    #[serde(rename = "persistent")]
    pub persistent: bool,
    #[serde(rename = "durability")]
    pub durability: StorageBucketsDurability,
}
impl StorageBucketInfo {
    pub const IDENTIFIER: &'static str = "Storage.StorageBucketInfo";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributionReportingSourceType {
    #[serde(rename = "navigation")]
    Navigation,
    #[serde(rename = "event")]
    Event,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct UnsignedInt64AsBase10(String);
impl UnsignedInt64AsBase10 {
    pub fn new(val: impl Into<String>) -> Self {
        UnsignedInt64AsBase10(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for UnsignedInt64AsBase10 {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<UnsignedInt64AsBase10> for String {
    fn from(el: UnsignedInt64AsBase10) -> String {
        el.0
    }
}
impl From<String> for UnsignedInt64AsBase10 {
    fn from(expr: String) -> Self {
        UnsignedInt64AsBase10(expr)
    }
}
impl UnsignedInt64AsBase10 {
    pub const IDENTIFIER: &'static str = "Storage.UnsignedInt64AsBase10";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct UnsignedInt128AsBase16(String);
impl UnsignedInt128AsBase16 {
    pub fn new(val: impl Into<String>) -> Self {
        UnsignedInt128AsBase16(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for UnsignedInt128AsBase16 {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<UnsignedInt128AsBase16> for String {
    fn from(el: UnsignedInt128AsBase16) -> String {
        el.0
    }
}
impl From<String> for UnsignedInt128AsBase16 {
    fn from(expr: String) -> Self {
        UnsignedInt128AsBase16(expr)
    }
}
impl UnsignedInt128AsBase16 {
    pub const IDENTIFIER: &'static str = "Storage.UnsignedInt128AsBase16";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct SignedInt64AsBase10(String);
impl SignedInt64AsBase10 {
    pub fn new(val: impl Into<String>) -> Self {
        SignedInt64AsBase10(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for SignedInt64AsBase10 {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<SignedInt64AsBase10> for String {
    fn from(el: SignedInt64AsBase10) -> String {
        el.0
    }
}
impl From<String> for SignedInt64AsBase10 {
    fn from(expr: String) -> Self {
        SignedInt64AsBase10(expr)
    }
}
impl SignedInt64AsBase10 {
    pub const IDENTIFIER: &'static str = "Storage.SignedInt64AsBase10";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingFilterDataEntry {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}
impl AttributionReportingFilterDataEntry {
    pub fn new(key: impl Into<String>, values: Vec<String>) -> Self {
        Self {
            key: key.into(),
            values,
        }
    }
}
impl AttributionReportingFilterDataEntry {
    pub const IDENTIFIER: &'static str = "Storage.AttributionReportingFilterDataEntry";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingFilterConfig {
    #[serde(rename = "filterValues")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub filter_values: Vec<AttributionReportingFilterDataEntry>,
    #[doc = "duration in seconds"]
    #[serde(rename = "lookbackWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub lookback_window: Option<i64>,
}
impl AttributionReportingFilterConfig {
    pub fn new(filter_values: Vec<AttributionReportingFilterDataEntry>) -> Self {
        Self {
            filter_values,
            lookback_window: None,
        }
    }
}
impl AttributionReportingFilterConfig {
    pub const IDENTIFIER: &'static str = "Storage.AttributionReportingFilterConfig";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingFilterPair {
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<AttributionReportingFilterConfig>,
    #[serde(rename = "notFilters")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub not_filters: Vec<AttributionReportingFilterConfig>,
}
impl AttributionReportingFilterPair {
    pub fn new(
        filters: Vec<AttributionReportingFilterConfig>,
        not_filters: Vec<AttributionReportingFilterConfig>,
    ) -> Self {
        Self {
            filters,
            not_filters,
        }
    }
}
impl AttributionReportingFilterPair {
    pub const IDENTIFIER: &'static str = "Storage.AttributionReportingFilterPair";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingAggregationKeysEntry {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: UnsignedInt128AsBase16,
}
impl AttributionReportingAggregationKeysEntry {
    pub fn new(key: impl Into<String>, value: impl Into<UnsignedInt128AsBase16>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}
impl AttributionReportingAggregationKeysEntry {
    pub const IDENTIFIER: &'static str = "Storage.AttributionReportingAggregationKeysEntry";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingEventReportWindows {
    #[doc = "duration in seconds"]
    #[serde(rename = "start")]
    pub start: i64,
    #[doc = "duration in seconds"]
    #[serde(rename = "ends")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ends: Vec<i64>,
}
impl AttributionReportingEventReportWindows {
    pub fn new(start: impl Into<i64>, ends: Vec<i64>) -> Self {
        Self {
            start: start.into(),
            ends,
        }
    }
}
impl AttributionReportingEventReportWindows {
    pub const IDENTIFIER: &'static str = "Storage.AttributionReportingEventReportWindows";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributionReportingTriggerDataMatching {
    #[serde(rename = "exact")]
    Exact,
    #[serde(rename = "modulus")]
    Modulus,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingAggregatableDebugReportingData {
    #[serde(rename = "keyPiece")]
    pub key_piece: UnsignedInt128AsBase16,
    #[doc = "number instead of integer because not all uint32 can be represented by\nint"]
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "types")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<String>,
}
impl AttributionReportingAggregatableDebugReportingData {
    pub fn new(
        key_piece: impl Into<UnsignedInt128AsBase16>,
        value: impl Into<f64>,
        types: Vec<String>,
    ) -> Self {
        Self {
            key_piece: key_piece.into(),
            value: value.into(),
            types,
        }
    }
}
impl AttributionReportingAggregatableDebugReportingData {
    pub const IDENTIFIER: &'static str =
        "Storage.AttributionReportingAggregatableDebugReportingData";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingAggregatableDebugReportingConfig {
    #[doc = "number instead of integer because not all uint32 can be represented by\nint, only present for source registrations"]
    #[serde(rename = "budget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub budget: Option<f64>,
    #[serde(rename = "keyPiece")]
    pub key_piece: UnsignedInt128AsBase16,
    #[serde(rename = "debugData")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub debug_data: Vec<AttributionReportingAggregatableDebugReportingData>,
    #[serde(rename = "aggregationCoordinatorOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub aggregation_coordinator_origin: Option<String>,
}
impl AttributionReportingAggregatableDebugReportingConfig {
    pub fn new(
        key_piece: impl Into<UnsignedInt128AsBase16>,
        debug_data: Vec<AttributionReportingAggregatableDebugReportingData>,
    ) -> Self {
        Self {
            key_piece: key_piece.into(),
            debug_data,
            budget: None,
            aggregation_coordinator_origin: None,
        }
    }
}
impl AttributionReportingAggregatableDebugReportingConfig {
    pub const IDENTIFIER: &'static str =
        "Storage.AttributionReportingAggregatableDebugReportingConfig";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionScopesData {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[doc = "number instead of integer because not all uint32 can be represented by\nint"]
    #[serde(rename = "limit")]
    pub limit: f64,
    #[serde(rename = "maxEventStates")]
    pub max_event_states: f64,
}
impl AttributionScopesData {
    pub fn new(
        values: Vec<String>,
        limit: impl Into<f64>,
        max_event_states: impl Into<f64>,
    ) -> Self {
        Self {
            values,
            limit: limit.into(),
            max_event_states: max_event_states.into(),
        }
    }
}
impl AttributionScopesData {
    pub const IDENTIFIER: &'static str = "Storage.AttributionScopesData";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingNamedBudgetDef {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "budget")]
    pub budget: i64,
}
impl AttributionReportingNamedBudgetDef {
    pub fn new(name: impl Into<String>, budget: impl Into<i64>) -> Self {
        Self {
            name: name.into(),
            budget: budget.into(),
        }
    }
}
impl AttributionReportingNamedBudgetDef {
    pub const IDENTIFIER: &'static str = "Storage.AttributionReportingNamedBudgetDef";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingSourceRegistration {
    #[serde(rename = "time")]
    pub time: super::super::network::types::TimeSinceEpoch,
    #[doc = "duration in seconds"]
    #[serde(rename = "expiry")]
    pub expiry: i64,
    #[doc = "number instead of integer because not all uint32 can be represented by\nint"]
    #[serde(rename = "triggerData")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub trigger_data: Vec<f64>,
    #[serde(rename = "eventReportWindows")]
    pub event_report_windows: AttributionReportingEventReportWindows,
    #[doc = "duration in seconds"]
    #[serde(rename = "aggregatableReportWindow")]
    pub aggregatable_report_window: i64,
    #[serde(rename = "type")]
    pub r#type: AttributionReportingSourceType,
    #[serde(rename = "sourceOrigin")]
    pub source_origin: String,
    #[serde(rename = "reportingOrigin")]
    pub reporting_origin: String,
    #[serde(rename = "destinationSites")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub destination_sites: Vec<String>,
    #[serde(rename = "eventId")]
    pub event_id: UnsignedInt64AsBase10,
    #[serde(rename = "priority")]
    pub priority: SignedInt64AsBase10,
    #[serde(rename = "filterData")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub filter_data: Vec<AttributionReportingFilterDataEntry>,
    #[serde(rename = "aggregationKeys")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aggregation_keys: Vec<AttributionReportingAggregationKeysEntry>,
    #[serde(rename = "debugKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub debug_key: Option<UnsignedInt64AsBase10>,
    #[serde(rename = "triggerDataMatching")]
    pub trigger_data_matching: AttributionReportingTriggerDataMatching,
    #[serde(rename = "destinationLimitPriority")]
    pub destination_limit_priority: SignedInt64AsBase10,
    #[serde(rename = "aggregatableDebugReportingConfig")]
    pub aggregatable_debug_reporting_config: AttributionReportingAggregatableDebugReportingConfig,
    #[serde(rename = "scopesData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scopes_data: Option<AttributionScopesData>,
    #[serde(rename = "maxEventLevelReports")]
    pub max_event_level_reports: i64,
    #[serde(rename = "namedBudgets")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub named_budgets: Vec<AttributionReportingNamedBudgetDef>,
    #[serde(rename = "debugReporting")]
    pub debug_reporting: bool,
    #[serde(rename = "eventLevelEpsilon")]
    pub event_level_epsilon: f64,
}
impl AttributionReportingSourceRegistration {
    pub const IDENTIFIER: &'static str = "Storage.AttributionReportingSourceRegistration";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributionReportingSourceRegistrationResult {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "internalError")]
    InternalError,
    #[serde(rename = "insufficientSourceCapacity")]
    InsufficientSourceCapacity,
    #[serde(rename = "insufficientUniqueDestinationCapacity")]
    InsufficientUniqueDestinationCapacity,
    #[serde(rename = "excessiveReportingOrigins")]
    ExcessiveReportingOrigins,
    #[serde(rename = "prohibitedByBrowserPolicy")]
    ProhibitedByBrowserPolicy,
    #[serde(rename = "successNoised")]
    SuccessNoised,
    #[serde(rename = "destinationReportingLimitReached")]
    DestinationReportingLimitReached,
    #[serde(rename = "destinationGlobalLimitReached")]
    DestinationGlobalLimitReached,
    #[serde(rename = "destinationBothLimitsReached")]
    DestinationBothLimitsReached,
    #[serde(rename = "reportingOriginsPerSiteLimitReached")]
    ReportingOriginsPerSiteLimitReached,
    #[serde(rename = "exceedsMaxChannelCapacity")]
    ExceedsMaxChannelCapacity,
    #[serde(rename = "exceedsMaxScopesChannelCapacity")]
    ExceedsMaxScopesChannelCapacity,
    #[serde(rename = "exceedsMaxTriggerStateCardinality")]
    ExceedsMaxTriggerStateCardinality,
    #[serde(rename = "exceedsMaxEventStatesLimit")]
    ExceedsMaxEventStatesLimit,
    #[serde(rename = "destinationPerDayReportingLimitReached")]
    DestinationPerDayReportingLimitReached,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributionReportingSourceRegistrationTimeConfig {
    #[serde(rename = "include")]
    Include,
    #[serde(rename = "exclude")]
    Exclude,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingAggregatableValueDictEntry {
    #[serde(rename = "key")]
    pub key: String,
    #[doc = "number instead of integer because not all uint32 can be represented by\nint"]
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "filteringId")]
    pub filtering_id: UnsignedInt64AsBase10,
}
impl AttributionReportingAggregatableValueDictEntry {
    pub fn new(
        key: impl Into<String>,
        value: impl Into<f64>,
        filtering_id: impl Into<UnsignedInt64AsBase10>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            filtering_id: filtering_id.into(),
        }
    }
}
impl AttributionReportingAggregatableValueDictEntry {
    pub const IDENTIFIER: &'static str = "Storage.AttributionReportingAggregatableValueDictEntry";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingAggregatableValueEntry {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<AttributionReportingAggregatableValueDictEntry>,
    #[serde(rename = "filters")]
    pub filters: AttributionReportingFilterPair,
}
impl AttributionReportingAggregatableValueEntry {
    pub fn new(
        values: Vec<AttributionReportingAggregatableValueDictEntry>,
        filters: impl Into<AttributionReportingFilterPair>,
    ) -> Self {
        Self {
            values,
            filters: filters.into(),
        }
    }
}
impl AttributionReportingAggregatableValueEntry {
    pub const IDENTIFIER: &'static str = "Storage.AttributionReportingAggregatableValueEntry";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingEventTriggerData {
    #[serde(rename = "data")]
    pub data: UnsignedInt64AsBase10,
    #[serde(rename = "priority")]
    pub priority: SignedInt64AsBase10,
    #[serde(rename = "dedupKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dedup_key: Option<UnsignedInt64AsBase10>,
    #[serde(rename = "filters")]
    pub filters: AttributionReportingFilterPair,
}
impl AttributionReportingEventTriggerData {
    pub fn new(
        data: impl Into<UnsignedInt64AsBase10>,
        priority: impl Into<SignedInt64AsBase10>,
        filters: impl Into<AttributionReportingFilterPair>,
    ) -> Self {
        Self {
            data: data.into(),
            priority: priority.into(),
            filters: filters.into(),
            dedup_key: None,
        }
    }
}
impl AttributionReportingEventTriggerData {
    pub const IDENTIFIER: &'static str = "Storage.AttributionReportingEventTriggerData";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingAggregatableTriggerData {
    #[serde(rename = "keyPiece")]
    pub key_piece: UnsignedInt128AsBase16,
    #[serde(rename = "sourceKeys")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source_keys: Vec<String>,
    #[serde(rename = "filters")]
    pub filters: AttributionReportingFilterPair,
}
impl AttributionReportingAggregatableTriggerData {
    pub fn new(
        key_piece: impl Into<UnsignedInt128AsBase16>,
        source_keys: Vec<String>,
        filters: impl Into<AttributionReportingFilterPair>,
    ) -> Self {
        Self {
            key_piece: key_piece.into(),
            source_keys,
            filters: filters.into(),
        }
    }
}
impl AttributionReportingAggregatableTriggerData {
    pub const IDENTIFIER: &'static str = "Storage.AttributionReportingAggregatableTriggerData";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingAggregatableDedupKey {
    #[serde(rename = "dedupKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dedup_key: Option<UnsignedInt64AsBase10>,
    #[serde(rename = "filters")]
    pub filters: AttributionReportingFilterPair,
}
impl AttributionReportingAggregatableDedupKey {
    pub fn new(filters: impl Into<AttributionReportingFilterPair>) -> Self {
        Self {
            filters: filters.into(),
            dedup_key: None,
        }
    }
}
impl AttributionReportingAggregatableDedupKey {
    pub const IDENTIFIER: &'static str = "Storage.AttributionReportingAggregatableDedupKey";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingNamedBudgetCandidate {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "filters")]
    pub filters: AttributionReportingFilterPair,
}
impl AttributionReportingNamedBudgetCandidate {
    pub fn new(filters: impl Into<AttributionReportingFilterPair>) -> Self {
        Self {
            filters: filters.into(),
            name: None,
        }
    }
}
impl AttributionReportingNamedBudgetCandidate {
    pub const IDENTIFIER: &'static str = "Storage.AttributionReportingNamedBudgetCandidate";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributionReportingTriggerRegistration {
    #[serde(rename = "filters")]
    pub filters: AttributionReportingFilterPair,
    #[serde(rename = "debugKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub debug_key: Option<UnsignedInt64AsBase10>,
    #[serde(rename = "aggregatableDedupKeys")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aggregatable_dedup_keys: Vec<AttributionReportingAggregatableDedupKey>,
    #[serde(rename = "eventTriggerData")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub event_trigger_data: Vec<AttributionReportingEventTriggerData>,
    #[serde(rename = "aggregatableTriggerData")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aggregatable_trigger_data: Vec<AttributionReportingAggregatableTriggerData>,
    #[serde(rename = "aggregatableValues")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aggregatable_values: Vec<AttributionReportingAggregatableValueEntry>,
    #[serde(rename = "aggregatableFilteringIdMaxBytes")]
    pub aggregatable_filtering_id_max_bytes: i64,
    #[serde(rename = "debugReporting")]
    pub debug_reporting: bool,
    #[serde(rename = "aggregationCoordinatorOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub aggregation_coordinator_origin: Option<String>,
    #[serde(rename = "sourceRegistrationTimeConfig")]
    pub source_registration_time_config: AttributionReportingSourceRegistrationTimeConfig,
    #[serde(rename = "triggerContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub trigger_context_id: Option<String>,
    #[serde(rename = "aggregatableDebugReportingConfig")]
    pub aggregatable_debug_reporting_config: AttributionReportingAggregatableDebugReportingConfig,
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(rename = "namedBudgets")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub named_budgets: Vec<AttributionReportingNamedBudgetCandidate>,
}
impl AttributionReportingTriggerRegistration {
    pub const IDENTIFIER: &'static str = "Storage.AttributionReportingTriggerRegistration";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributionReportingEventLevelResult {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "successDroppedLowerPriority")]
    SuccessDroppedLowerPriority,
    #[serde(rename = "internalError")]
    InternalError,
    #[serde(rename = "noCapacityForAttributionDestination")]
    NoCapacityForAttributionDestination,
    #[serde(rename = "noMatchingSources")]
    NoMatchingSources,
    #[serde(rename = "deduplicated")]
    Deduplicated,
    #[serde(rename = "excessiveAttributions")]
    ExcessiveAttributions,
    #[serde(rename = "priorityTooLow")]
    PriorityTooLow,
    #[serde(rename = "neverAttributedSource")]
    NeverAttributedSource,
    #[serde(rename = "excessiveReportingOrigins")]
    ExcessiveReportingOrigins,
    #[serde(rename = "noMatchingSourceFilterData")]
    NoMatchingSourceFilterData,
    #[serde(rename = "prohibitedByBrowserPolicy")]
    ProhibitedByBrowserPolicy,
    #[serde(rename = "noMatchingConfigurations")]
    NoMatchingConfigurations,
    #[serde(rename = "excessiveReports")]
    ExcessiveReports,
    #[serde(rename = "falselyAttributedSource")]
    FalselyAttributedSource,
    #[serde(rename = "reportWindowPassed")]
    ReportWindowPassed,
    #[serde(rename = "notRegistered")]
    NotRegistered,
    #[serde(rename = "reportWindowNotStarted")]
    ReportWindowNotStarted,
    #[serde(rename = "noMatchingTriggerData")]
    NoMatchingTriggerData,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributionReportingAggregatableResult {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "internalError")]
    InternalError,
    #[serde(rename = "noCapacityForAttributionDestination")]
    NoCapacityForAttributionDestination,
    #[serde(rename = "noMatchingSources")]
    NoMatchingSources,
    #[serde(rename = "excessiveAttributions")]
    ExcessiveAttributions,
    #[serde(rename = "excessiveReportingOrigins")]
    ExcessiveReportingOrigins,
    #[serde(rename = "noHistograms")]
    NoHistograms,
    #[serde(rename = "insufficientBudget")]
    InsufficientBudget,
    #[serde(rename = "insufficientNamedBudget")]
    InsufficientNamedBudget,
    #[serde(rename = "noMatchingSourceFilterData")]
    NoMatchingSourceFilterData,
    #[serde(rename = "notRegistered")]
    NotRegistered,
    #[serde(rename = "prohibitedByBrowserPolicy")]
    ProhibitedByBrowserPolicy,
    #[serde(rename = "deduplicated")]
    Deduplicated,
    #[serde(rename = "reportWindowPassed")]
    ReportWindowPassed,
    #[serde(rename = "excessiveReports")]
    ExcessiveReports,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributionReportingReportResult {
    #[doc = "A network request was attempted for the report."]
    #[serde(rename = "sent")]
    Sent,
    #[doc = "No request was attempted because of browser policy."]
    #[serde(rename = "prohibited")]
    Prohibited,
    #[doc = "No request was attempted because of an error in report assembly,\ne.g. the aggregation service was unavailable."]
    #[serde(rename = "failedToAssemble")]
    FailedToAssemble,
    #[doc = "No request was attempted because the report's expiry passed."]
    #[serde(rename = "expired")]
    Expired,
}
#[doc = "A single Related Website Set object.\n[RelatedWebsiteSet](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-RelatedWebsiteSet)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelatedWebsiteSet {
    #[doc = "The primary site of this set, along with the ccTLDs if there is any."]
    #[serde(rename = "primarySites")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub primary_sites: Vec<String>,
    #[doc = "The associated sites of this set, along with the ccTLDs if there is any."]
    #[serde(rename = "associatedSites")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub associated_sites: Vec<String>,
    #[doc = "The service sites of this set, along with the ccTLDs if there is any."]
    #[serde(rename = "serviceSites")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub service_sites: Vec<String>,
}
impl RelatedWebsiteSet {
    pub fn new(
        primary_sites: Vec<String>,
        associated_sites: Vec<String>,
        service_sites: Vec<String>,
    ) -> Self {
        Self {
            primary_sites,
            associated_sites,
            service_sites,
        }
    }
}
impl RelatedWebsiteSet {
    pub const IDENTIFIER: &'static str = "Storage.RelatedWebsiteSet";
}
group_enum ! (StorageTypes { SerializedStorageKey (SerializedStorageKey) , StorageType (StorageType) , UsageForType (UsageForType) , TrustTokens (TrustTokens) , InterestGroupAuctionId (InterestGroupAuctionId) , InterestGroupAccessType (InterestGroupAccessType) , InterestGroupAuctionEventType (InterestGroupAuctionEventType) , InterestGroupAuctionFetchType (InterestGroupAuctionFetchType) , SharedStorageAccessScope (SharedStorageAccessScope) , SharedStorageAccessMethod (SharedStorageAccessMethod) , SharedStorageEntry (SharedStorageEntry) , SharedStorageMetadata (SharedStorageMetadata) , SharedStoragePrivateAggregationConfig (SharedStoragePrivateAggregationConfig) , SharedStorageReportingMetadata (SharedStorageReportingMetadata) , SharedStorageUrlWithMetadata (SharedStorageUrlWithMetadata) , SharedStorageAccessParams (SharedStorageAccessParams) , StorageBucketsDurability (StorageBucketsDurability) , StorageBucket (StorageBucket) , StorageBucketInfo (StorageBucketInfo) , AttributionReportingSourceType (AttributionReportingSourceType) , UnsignedInt64AsBase10 (UnsignedInt64AsBase10) , UnsignedInt128AsBase16 (UnsignedInt128AsBase16) , SignedInt64AsBase10 (SignedInt64AsBase10) , AttributionReportingFilterDataEntry (AttributionReportingFilterDataEntry) , AttributionReportingFilterConfig (AttributionReportingFilterConfig) , AttributionReportingFilterPair (AttributionReportingFilterPair) , AttributionReportingAggregationKeysEntry (AttributionReportingAggregationKeysEntry) , AttributionReportingEventReportWindows (AttributionReportingEventReportWindows) , AttributionReportingTriggerDataMatching (AttributionReportingTriggerDataMatching) , AttributionReportingAggregatableDebugReportingData (AttributionReportingAggregatableDebugReportingData) , AttributionReportingAggregatableDebugReportingConfig (AttributionReportingAggregatableDebugReportingConfig) , AttributionScopesData (AttributionScopesData) , AttributionReportingNamedBudgetDef (AttributionReportingNamedBudgetDef) , AttributionReportingSourceRegistration (AttributionReportingSourceRegistration) , AttributionReportingSourceRegistrationResult (AttributionReportingSourceRegistrationResult) , AttributionReportingSourceRegistrationTimeConfig (AttributionReportingSourceRegistrationTimeConfig) , AttributionReportingAggregatableValueDictEntry (AttributionReportingAggregatableValueDictEntry) , AttributionReportingAggregatableValueEntry (AttributionReportingAggregatableValueEntry) , AttributionReportingEventTriggerData (AttributionReportingEventTriggerData) , AttributionReportingAggregatableTriggerData (AttributionReportingAggregatableTriggerData) , AttributionReportingAggregatableDedupKey (AttributionReportingAggregatableDedupKey) , AttributionReportingNamedBudgetCandidate (AttributionReportingNamedBudgetCandidate) , AttributionReportingTriggerRegistration (AttributionReportingTriggerRegistration) , AttributionReportingEventLevelResult (AttributionReportingEventLevelResult) , AttributionReportingAggregatableResult (AttributionReportingAggregatableResult) , AttributionReportingReportResult (AttributionReportingReportResult) , RelatedWebsiteSet (RelatedWebsiteSet) });
