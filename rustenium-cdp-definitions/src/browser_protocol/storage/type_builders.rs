use super::types::*;
impl UsageForType {
    pub fn builder() -> UsageForTypeBuilder {
        UsageForTypeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct UsageForTypeBuilder {
    storage_type: Option<StorageType>,
    usage: Option<f64>,
}
impl UsageForTypeBuilder {
    pub fn storage_type(mut self, storage_type: impl Into<StorageType>) -> Self {
        self.storage_type = Some(storage_type.into());
        self
    }
    pub fn usage(mut self, usage: impl Into<f64>) -> Self {
        self.usage = Some(usage.into());
        self
    }
    pub fn build(self) -> Result<UsageForType, String> {
        Ok(UsageForType {
            storage_type: self.storage_type.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(storage_type))
            })?,
            usage: self
                .usage
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(usage)))?,
        })
    }
}
impl TrustTokens {
    pub fn builder() -> TrustTokensBuilder {
        TrustTokensBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct TrustTokensBuilder {
    issuer_origin: Option<String>,
    count: Option<f64>,
}
impl TrustTokensBuilder {
    pub fn issuer_origin(mut self, issuer_origin: impl Into<String>) -> Self {
        self.issuer_origin = Some(issuer_origin.into());
        self
    }
    pub fn count(mut self, count: impl Into<f64>) -> Self {
        self.count = Some(count.into());
        self
    }
    pub fn build(self) -> Result<TrustTokens, String> {
        Ok(TrustTokens {
            issuer_origin: self.issuer_origin.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(issuer_origin))
            })?,
            count: self
                .count
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(count)))?,
        })
    }
}
impl SharedStorageEntry {
    pub fn builder() -> SharedStorageEntryBuilder {
        SharedStorageEntryBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SharedStorageEntryBuilder {
    key: Option<String>,
    value: Option<String>,
}
impl SharedStorageEntryBuilder {
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<SharedStorageEntry, String> {
        Ok(SharedStorageEntry {
            key: self
                .key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl SharedStorageMetadata {
    pub fn builder() -> SharedStorageMetadataBuilder {
        SharedStorageMetadataBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SharedStorageMetadataBuilder {
    creation_time: Option<super::super::network::types::TimeSinceEpoch>,
    length: Option<i64>,
    remaining_budget: Option<f64>,
    bytes_used: Option<i64>,
}
impl SharedStorageMetadataBuilder {
    pub fn creation_time(
        mut self,
        creation_time: impl Into<super::super::network::types::TimeSinceEpoch>,
    ) -> Self {
        self.creation_time = Some(creation_time.into());
        self
    }
    pub fn length(mut self, length: impl Into<i64>) -> Self {
        self.length = Some(length.into());
        self
    }
    pub fn remaining_budget(mut self, remaining_budget: impl Into<f64>) -> Self {
        self.remaining_budget = Some(remaining_budget.into());
        self
    }
    pub fn bytes_used(mut self, bytes_used: impl Into<i64>) -> Self {
        self.bytes_used = Some(bytes_used.into());
        self
    }
    pub fn build(self) -> Result<SharedStorageMetadata, String> {
        Ok(SharedStorageMetadata {
            creation_time: self.creation_time.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(creation_time))
            })?,
            length: self
                .length
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(length)))?,
            remaining_budget: self.remaining_budget.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(remaining_budget)
                )
            })?,
            bytes_used: self
                .bytes_used
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(bytes_used)))?,
        })
    }
}
impl SharedStoragePrivateAggregationConfig {
    pub fn builder() -> SharedStoragePrivateAggregationConfigBuilder {
        SharedStoragePrivateAggregationConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SharedStoragePrivateAggregationConfigBuilder {
    aggregation_coordinator_origin: Option<String>,
    context_id: Option<String>,
    filtering_id_max_bytes: Option<i64>,
    max_contributions: Option<i64>,
}
impl SharedStoragePrivateAggregationConfigBuilder {
    pub fn aggregation_coordinator_origin(
        mut self,
        aggregation_coordinator_origin: impl Into<String>,
    ) -> Self {
        self.aggregation_coordinator_origin = Some(aggregation_coordinator_origin.into());
        self
    }
    pub fn context_id(mut self, context_id: impl Into<String>) -> Self {
        self.context_id = Some(context_id.into());
        self
    }
    pub fn filtering_id_max_bytes(mut self, filtering_id_max_bytes: impl Into<i64>) -> Self {
        self.filtering_id_max_bytes = Some(filtering_id_max_bytes.into());
        self
    }
    pub fn max_contributions(mut self, max_contributions: impl Into<i64>) -> Self {
        self.max_contributions = Some(max_contributions.into());
        self
    }
    pub fn build(self) -> Result<SharedStoragePrivateAggregationConfig, String> {
        Ok(SharedStoragePrivateAggregationConfig {
            aggregation_coordinator_origin: self.aggregation_coordinator_origin,
            context_id: self.context_id,
            filtering_id_max_bytes: self.filtering_id_max_bytes.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(filtering_id_max_bytes)
                )
            })?,
            max_contributions: self.max_contributions,
        })
    }
}
impl SharedStorageReportingMetadata {
    pub fn builder() -> SharedStorageReportingMetadataBuilder {
        SharedStorageReportingMetadataBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SharedStorageReportingMetadataBuilder {
    event_type: Option<String>,
    reporting_url: Option<String>,
}
impl SharedStorageReportingMetadataBuilder {
    pub fn event_type(mut self, event_type: impl Into<String>) -> Self {
        self.event_type = Some(event_type.into());
        self
    }
    pub fn reporting_url(mut self, reporting_url: impl Into<String>) -> Self {
        self.reporting_url = Some(reporting_url.into());
        self
    }
    pub fn build(self) -> Result<SharedStorageReportingMetadata, String> {
        Ok(SharedStorageReportingMetadata {
            event_type: self
                .event_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(event_type)))?,
            reporting_url: self.reporting_url.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(reporting_url))
            })?,
        })
    }
}
impl SharedStorageUrlWithMetadata {
    pub fn builder() -> SharedStorageUrlWithMetadataBuilder {
        SharedStorageUrlWithMetadataBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SharedStorageUrlWithMetadataBuilder {
    url: Option<String>,
    reporting_metadata: Option<Vec<SharedStorageReportingMetadata>>,
}
impl SharedStorageUrlWithMetadataBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn reporting_metadata(
        mut self,
        reporting_metadata: impl Into<SharedStorageReportingMetadata>,
    ) -> Self {
        let v = self.reporting_metadata.get_or_insert(Vec::new());
        v.push(reporting_metadata.into());
        self
    }
    pub fn reporting_metadatas<I, S>(mut self, reporting_metadatas: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<SharedStorageReportingMetadata>,
    {
        let v = self.reporting_metadata.get_or_insert(Vec::new());
        for val in reporting_metadatas {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SharedStorageUrlWithMetadata, String> {
        Ok(SharedStorageUrlWithMetadata {
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            reporting_metadata: self.reporting_metadata.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(reporting_metadata)
                )
            })?,
        })
    }
}
impl SharedStorageAccessParams {
    pub fn builder() -> SharedStorageAccessParamsBuilder {
        SharedStorageAccessParamsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SharedStorageAccessParamsBuilder {
    script_source_url: Option<String>,
    data_origin: Option<String>,
    operation_name: Option<String>,
    operation_id: Option<String>,
    keep_alive: Option<bool>,
    private_aggregation_config: Option<SharedStoragePrivateAggregationConfig>,
    serialized_data: Option<String>,
    urls_with_metadata: Option<Vec<SharedStorageUrlWithMetadata>>,
    urn_uuid: Option<String>,
    key: Option<String>,
    value: Option<String>,
    ignore_if_present: Option<bool>,
    worklet_ordinal: Option<i64>,
    worklet_target_id: Option<super::super::target::types::TargetId>,
    with_lock: Option<String>,
    batch_update_id: Option<String>,
    batch_size: Option<i64>,
}
impl SharedStorageAccessParamsBuilder {
    pub fn script_source_url(mut self, script_source_url: impl Into<String>) -> Self {
        self.script_source_url = Some(script_source_url.into());
        self
    }
    pub fn data_origin(mut self, data_origin: impl Into<String>) -> Self {
        self.data_origin = Some(data_origin.into());
        self
    }
    pub fn operation_name(mut self, operation_name: impl Into<String>) -> Self {
        self.operation_name = Some(operation_name.into());
        self
    }
    pub fn operation_id(mut self, operation_id: impl Into<String>) -> Self {
        self.operation_id = Some(operation_id.into());
        self
    }
    pub fn keep_alive(mut self, keep_alive: impl Into<bool>) -> Self {
        self.keep_alive = Some(keep_alive.into());
        self
    }
    pub fn private_aggregation_config(
        mut self,
        private_aggregation_config: impl Into<SharedStoragePrivateAggregationConfig>,
    ) -> Self {
        self.private_aggregation_config = Some(private_aggregation_config.into());
        self
    }
    pub fn serialized_data(mut self, serialized_data: impl Into<String>) -> Self {
        self.serialized_data = Some(serialized_data.into());
        self
    }
    pub fn urls_with_metadata(
        mut self,
        urls_with_metadata: impl Into<SharedStorageUrlWithMetadata>,
    ) -> Self {
        let v = self.urls_with_metadata.get_or_insert(Vec::new());
        v.push(urls_with_metadata.into());
        self
    }
    pub fn urls_with_metadatas<I, S>(mut self, urls_with_metadatas: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<SharedStorageUrlWithMetadata>,
    {
        let v = self.urls_with_metadata.get_or_insert(Vec::new());
        for val in urls_with_metadatas {
            v.push(val.into());
        }
        self
    }
    pub fn urn_uuid(mut self, urn_uuid: impl Into<String>) -> Self {
        self.urn_uuid = Some(urn_uuid.into());
        self
    }
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn ignore_if_present(mut self, ignore_if_present: impl Into<bool>) -> Self {
        self.ignore_if_present = Some(ignore_if_present.into());
        self
    }
    pub fn worklet_ordinal(mut self, worklet_ordinal: impl Into<i64>) -> Self {
        self.worklet_ordinal = Some(worklet_ordinal.into());
        self
    }
    pub fn worklet_target_id(
        mut self,
        worklet_target_id: impl Into<super::super::target::types::TargetId>,
    ) -> Self {
        self.worklet_target_id = Some(worklet_target_id.into());
        self
    }
    pub fn with_lock(mut self, with_lock: impl Into<String>) -> Self {
        self.with_lock = Some(with_lock.into());
        self
    }
    pub fn batch_update_id(mut self, batch_update_id: impl Into<String>) -> Self {
        self.batch_update_id = Some(batch_update_id.into());
        self
    }
    pub fn batch_size(mut self, batch_size: impl Into<i64>) -> Self {
        self.batch_size = Some(batch_size.into());
        self
    }
    pub fn build(self) -> SharedStorageAccessParams {
        SharedStorageAccessParams {
            script_source_url: self.script_source_url,
            data_origin: self.data_origin,
            operation_name: self.operation_name,
            operation_id: self.operation_id,
            keep_alive: self.keep_alive,
            private_aggregation_config: self.private_aggregation_config,
            serialized_data: self.serialized_data,
            urls_with_metadata: self.urls_with_metadata,
            urn_uuid: self.urn_uuid,
            key: self.key,
            value: self.value,
            ignore_if_present: self.ignore_if_present,
            worklet_ordinal: self.worklet_ordinal,
            worklet_target_id: self.worklet_target_id,
            with_lock: self.with_lock,
            batch_update_id: self.batch_update_id,
            batch_size: self.batch_size,
        }
    }
}
impl StorageBucket {
    pub fn builder() -> StorageBucketBuilder {
        StorageBucketBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct StorageBucketBuilder {
    storage_key: Option<SerializedStorageKey>,
    name: Option<String>,
}
impl StorageBucketBuilder {
    pub fn storage_key(mut self, storage_key: impl Into<SerializedStorageKey>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn build(self) -> Result<StorageBucket, String> {
        Ok(StorageBucket {
            storage_key: self
                .storage_key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(storage_key)))?,
            name: self.name,
        })
    }
}
impl StorageBucketInfo {
    pub fn builder() -> StorageBucketInfoBuilder {
        StorageBucketInfoBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct StorageBucketInfoBuilder {
    bucket: Option<StorageBucket>,
    id: Option<String>,
    expiration: Option<super::super::network::types::TimeSinceEpoch>,
    quota: Option<f64>,
    persistent: Option<bool>,
    durability: Option<StorageBucketsDurability>,
}
impl StorageBucketInfoBuilder {
    pub fn bucket(mut self, bucket: impl Into<StorageBucket>) -> Self {
        self.bucket = Some(bucket.into());
        self
    }
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn expiration(
        mut self,
        expiration: impl Into<super::super::network::types::TimeSinceEpoch>,
    ) -> Self {
        self.expiration = Some(expiration.into());
        self
    }
    pub fn quota(mut self, quota: impl Into<f64>) -> Self {
        self.quota = Some(quota.into());
        self
    }
    pub fn persistent(mut self, persistent: impl Into<bool>) -> Self {
        self.persistent = Some(persistent.into());
        self
    }
    pub fn durability(mut self, durability: impl Into<StorageBucketsDurability>) -> Self {
        self.durability = Some(durability.into());
        self
    }
    pub fn build(self) -> Result<StorageBucketInfo, String> {
        Ok(StorageBucketInfo {
            bucket: self
                .bucket
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(bucket)))?,
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            expiration: self
                .expiration
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(expiration)))?,
            quota: self
                .quota
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(quota)))?,
            persistent: self
                .persistent
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(persistent)))?,
            durability: self
                .durability
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(durability)))?,
        })
    }
}
impl AttributionReportingFilterDataEntry {
    pub fn builder() -> AttributionReportingFilterDataEntryBuilder {
        AttributionReportingFilterDataEntryBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingFilterDataEntryBuilder {
    key: Option<String>,
    values: Option<Vec<String>>,
}
impl AttributionReportingFilterDataEntryBuilder {
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        let v = self.values.get_or_insert(Vec::new());
        v.push(value.into());
        self
    }
    pub fn values<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.values.get_or_insert(Vec::new());
        for val in values {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<AttributionReportingFilterDataEntry, String> {
        Ok(AttributionReportingFilterDataEntry {
            key: self
                .key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key)))?,
            values: self
                .values
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(values)))?,
        })
    }
}
impl AttributionReportingFilterConfig {
    pub fn builder() -> AttributionReportingFilterConfigBuilder {
        AttributionReportingFilterConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingFilterConfigBuilder {
    filter_values: Option<Vec<AttributionReportingFilterDataEntry>>,
    lookback_window: Option<i64>,
}
impl AttributionReportingFilterConfigBuilder {
    pub fn filter_value(
        mut self,
        filter_value: impl Into<AttributionReportingFilterDataEntry>,
    ) -> Self {
        let v = self.filter_values.get_or_insert(Vec::new());
        v.push(filter_value.into());
        self
    }
    pub fn filter_values<I, S>(mut self, filter_values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AttributionReportingFilterDataEntry>,
    {
        let v = self.filter_values.get_or_insert(Vec::new());
        for val in filter_values {
            v.push(val.into());
        }
        self
    }
    pub fn lookback_window(mut self, lookback_window: impl Into<i64>) -> Self {
        self.lookback_window = Some(lookback_window.into());
        self
    }
    pub fn build(self) -> Result<AttributionReportingFilterConfig, String> {
        Ok(AttributionReportingFilterConfig {
            filter_values: self.filter_values.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(filter_values))
            })?,
            lookback_window: self.lookback_window,
        })
    }
}
impl AttributionReportingFilterPair {
    pub fn builder() -> AttributionReportingFilterPairBuilder {
        AttributionReportingFilterPairBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingFilterPairBuilder {
    filters: Option<Vec<AttributionReportingFilterConfig>>,
    not_filters: Option<Vec<AttributionReportingFilterConfig>>,
}
impl AttributionReportingFilterPairBuilder {
    pub fn filter(mut self, filter: impl Into<AttributionReportingFilterConfig>) -> Self {
        let v = self.filters.get_or_insert(Vec::new());
        v.push(filter.into());
        self
    }
    pub fn filters<I, S>(mut self, filters: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AttributionReportingFilterConfig>,
    {
        let v = self.filters.get_or_insert(Vec::new());
        for val in filters {
            v.push(val.into());
        }
        self
    }
    pub fn not_filter(mut self, not_filter: impl Into<AttributionReportingFilterConfig>) -> Self {
        let v = self.not_filters.get_or_insert(Vec::new());
        v.push(not_filter.into());
        self
    }
    pub fn not_filters<I, S>(mut self, not_filters: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AttributionReportingFilterConfig>,
    {
        let v = self.not_filters.get_or_insert(Vec::new());
        for val in not_filters {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<AttributionReportingFilterPair, String> {
        Ok(AttributionReportingFilterPair {
            filters: self
                .filters
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(filters)))?,
            not_filters: self
                .not_filters
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(not_filters)))?,
        })
    }
}
impl AttributionReportingAggregationKeysEntry {
    pub fn builder() -> AttributionReportingAggregationKeysEntryBuilder {
        AttributionReportingAggregationKeysEntryBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingAggregationKeysEntryBuilder {
    key: Option<String>,
    value: Option<UnsignedInt128AsBase16>,
}
impl AttributionReportingAggregationKeysEntryBuilder {
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn value(mut self, value: impl Into<UnsignedInt128AsBase16>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<AttributionReportingAggregationKeysEntry, String> {
        Ok(AttributionReportingAggregationKeysEntry {
            key: self
                .key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl AttributionReportingEventReportWindows {
    pub fn builder() -> AttributionReportingEventReportWindowsBuilder {
        AttributionReportingEventReportWindowsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingEventReportWindowsBuilder {
    start: Option<i64>,
    ends: Option<Vec<i64>>,
}
impl AttributionReportingEventReportWindowsBuilder {
    pub fn start(mut self, start: impl Into<i64>) -> Self {
        self.start = Some(start.into());
        self
    }
    pub fn end(mut self, end: impl Into<i64>) -> Self {
        let v = self.ends.get_or_insert(Vec::new());
        v.push(end.into());
        self
    }
    pub fn ends<I, S>(mut self, ends: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.ends.get_or_insert(Vec::new());
        for val in ends {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<AttributionReportingEventReportWindows, String> {
        Ok(AttributionReportingEventReportWindows {
            start: self
                .start
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(start)))?,
            ends: self
                .ends
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(ends)))?,
        })
    }
}
impl AttributionReportingAggregatableDebugReportingData {
    pub fn builder() -> AttributionReportingAggregatableDebugReportingDataBuilder {
        AttributionReportingAggregatableDebugReportingDataBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingAggregatableDebugReportingDataBuilder {
    key_piece: Option<UnsignedInt128AsBase16>,
    value: Option<f64>,
    types: Option<Vec<String>>,
}
impl AttributionReportingAggregatableDebugReportingDataBuilder {
    pub fn key_piece(mut self, key_piece: impl Into<UnsignedInt128AsBase16>) -> Self {
        self.key_piece = Some(key_piece.into());
        self
    }
    pub fn value(mut self, value: impl Into<f64>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        let v = self.types.get_or_insert(Vec::new());
        v.push(r#type.into());
        self
    }
    pub fn types<I, S>(mut self, types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.types.get_or_insert(Vec::new());
        for val in types {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<AttributionReportingAggregatableDebugReportingData, String> {
        Ok(AttributionReportingAggregatableDebugReportingData {
            key_piece: self
                .key_piece
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key_piece)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            types: self
                .types
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(types)))?,
        })
    }
}
impl AttributionReportingAggregatableDebugReportingConfig {
    pub fn builder() -> AttributionReportingAggregatableDebugReportingConfigBuilder {
        AttributionReportingAggregatableDebugReportingConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingAggregatableDebugReportingConfigBuilder {
    budget: Option<f64>,
    key_piece: Option<UnsignedInt128AsBase16>,
    debug_data: Option<Vec<AttributionReportingAggregatableDebugReportingData>>,
    aggregation_coordinator_origin: Option<String>,
}
impl AttributionReportingAggregatableDebugReportingConfigBuilder {
    pub fn budget(mut self, budget: impl Into<f64>) -> Self {
        self.budget = Some(budget.into());
        self
    }
    pub fn key_piece(mut self, key_piece: impl Into<UnsignedInt128AsBase16>) -> Self {
        self.key_piece = Some(key_piece.into());
        self
    }
    pub fn debug_data(
        mut self,
        debug_data: impl Into<AttributionReportingAggregatableDebugReportingData>,
    ) -> Self {
        let v = self.debug_data.get_or_insert(Vec::new());
        v.push(debug_data.into());
        self
    }
    pub fn debug_datas<I, S>(mut self, debug_datas: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AttributionReportingAggregatableDebugReportingData>,
    {
        let v = self.debug_data.get_or_insert(Vec::new());
        for val in debug_datas {
            v.push(val.into());
        }
        self
    }
    pub fn aggregation_coordinator_origin(
        mut self,
        aggregation_coordinator_origin: impl Into<String>,
    ) -> Self {
        self.aggregation_coordinator_origin = Some(aggregation_coordinator_origin.into());
        self
    }
    pub fn build(self) -> Result<AttributionReportingAggregatableDebugReportingConfig, String> {
        Ok(AttributionReportingAggregatableDebugReportingConfig {
            budget: self.budget,
            key_piece: self
                .key_piece
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key_piece)))?,
            debug_data: self
                .debug_data
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(debug_data)))?,
            aggregation_coordinator_origin: self.aggregation_coordinator_origin,
        })
    }
}
impl AttributionScopesData {
    pub fn builder() -> AttributionScopesDataBuilder {
        AttributionScopesDataBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionScopesDataBuilder {
    values: Option<Vec<String>>,
    limit: Option<f64>,
    max_event_states: Option<f64>,
}
impl AttributionScopesDataBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        let v = self.values.get_or_insert(Vec::new());
        v.push(value.into());
        self
    }
    pub fn values<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.values.get_or_insert(Vec::new());
        for val in values {
            v.push(val.into());
        }
        self
    }
    pub fn limit(mut self, limit: impl Into<f64>) -> Self {
        self.limit = Some(limit.into());
        self
    }
    pub fn max_event_states(mut self, max_event_states: impl Into<f64>) -> Self {
        self.max_event_states = Some(max_event_states.into());
        self
    }
    pub fn build(self) -> Result<AttributionScopesData, String> {
        Ok(AttributionScopesData {
            values: self
                .values
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(values)))?,
            limit: self
                .limit
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(limit)))?,
            max_event_states: self.max_event_states.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(max_event_states)
                )
            })?,
        })
    }
}
impl AttributionReportingNamedBudgetDef {
    pub fn builder() -> AttributionReportingNamedBudgetDefBuilder {
        AttributionReportingNamedBudgetDefBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingNamedBudgetDefBuilder {
    name: Option<String>,
    budget: Option<i64>,
}
impl AttributionReportingNamedBudgetDefBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn budget(mut self, budget: impl Into<i64>) -> Self {
        self.budget = Some(budget.into());
        self
    }
    pub fn build(self) -> Result<AttributionReportingNamedBudgetDef, String> {
        Ok(AttributionReportingNamedBudgetDef {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            budget: self
                .budget
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(budget)))?,
        })
    }
}
impl AttributionReportingSourceRegistration {
    pub fn builder() -> AttributionReportingSourceRegistrationBuilder {
        AttributionReportingSourceRegistrationBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingSourceRegistrationBuilder {
    time: Option<super::super::network::types::TimeSinceEpoch>,
    expiry: Option<i64>,
    trigger_data: Option<Vec<f64>>,
    event_report_windows: Option<AttributionReportingEventReportWindows>,
    aggregatable_report_window: Option<i64>,
    r#type: Option<AttributionReportingSourceType>,
    source_origin: Option<String>,
    reporting_origin: Option<String>,
    destination_sites: Option<Vec<String>>,
    event_id: Option<UnsignedInt64AsBase10>,
    priority: Option<SignedInt64AsBase10>,
    filter_data: Option<Vec<AttributionReportingFilterDataEntry>>,
    aggregation_keys: Option<Vec<AttributionReportingAggregationKeysEntry>>,
    debug_key: Option<UnsignedInt64AsBase10>,
    trigger_data_matching: Option<AttributionReportingTriggerDataMatching>,
    destination_limit_priority: Option<SignedInt64AsBase10>,
    aggregatable_debug_reporting_config:
        Option<AttributionReportingAggregatableDebugReportingConfig>,
    scopes_data: Option<AttributionScopesData>,
    max_event_level_reports: Option<i64>,
    named_budgets: Option<Vec<AttributionReportingNamedBudgetDef>>,
    debug_reporting: Option<bool>,
    event_level_epsilon: Option<f64>,
}
impl AttributionReportingSourceRegistrationBuilder {
    pub fn time(mut self, time: impl Into<super::super::network::types::TimeSinceEpoch>) -> Self {
        self.time = Some(time.into());
        self
    }
    pub fn expiry(mut self, expiry: impl Into<i64>) -> Self {
        self.expiry = Some(expiry.into());
        self
    }
    pub fn trigger_data(mut self, trigger_data: impl Into<f64>) -> Self {
        let v = self.trigger_data.get_or_insert(Vec::new());
        v.push(trigger_data.into());
        self
    }
    pub fn trigger_datas<I, S>(mut self, trigger_datas: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        let v = self.trigger_data.get_or_insert(Vec::new());
        for val in trigger_datas {
            v.push(val.into());
        }
        self
    }
    pub fn event_report_windows(
        mut self,
        event_report_windows: impl Into<AttributionReportingEventReportWindows>,
    ) -> Self {
        self.event_report_windows = Some(event_report_windows.into());
        self
    }
    pub fn aggregatable_report_window(
        mut self,
        aggregatable_report_window: impl Into<i64>,
    ) -> Self {
        self.aggregatable_report_window = Some(aggregatable_report_window.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<AttributionReportingSourceType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn source_origin(mut self, source_origin: impl Into<String>) -> Self {
        self.source_origin = Some(source_origin.into());
        self
    }
    pub fn reporting_origin(mut self, reporting_origin: impl Into<String>) -> Self {
        self.reporting_origin = Some(reporting_origin.into());
        self
    }
    pub fn destination_site(mut self, destination_site: impl Into<String>) -> Self {
        let v = self.destination_sites.get_or_insert(Vec::new());
        v.push(destination_site.into());
        self
    }
    pub fn destination_sites<I, S>(mut self, destination_sites: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.destination_sites.get_or_insert(Vec::new());
        for val in destination_sites {
            v.push(val.into());
        }
        self
    }
    pub fn event_id(mut self, event_id: impl Into<UnsignedInt64AsBase10>) -> Self {
        self.event_id = Some(event_id.into());
        self
    }
    pub fn priority(mut self, priority: impl Into<SignedInt64AsBase10>) -> Self {
        self.priority = Some(priority.into());
        self
    }
    pub fn filter_data(
        mut self,
        filter_data: impl Into<AttributionReportingFilterDataEntry>,
    ) -> Self {
        let v = self.filter_data.get_or_insert(Vec::new());
        v.push(filter_data.into());
        self
    }
    pub fn filter_datas<I, S>(mut self, filter_datas: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AttributionReportingFilterDataEntry>,
    {
        let v = self.filter_data.get_or_insert(Vec::new());
        for val in filter_datas {
            v.push(val.into());
        }
        self
    }
    pub fn aggregation_key(
        mut self,
        aggregation_key: impl Into<AttributionReportingAggregationKeysEntry>,
    ) -> Self {
        let v = self.aggregation_keys.get_or_insert(Vec::new());
        v.push(aggregation_key.into());
        self
    }
    pub fn aggregation_keys<I, S>(mut self, aggregation_keys: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AttributionReportingAggregationKeysEntry>,
    {
        let v = self.aggregation_keys.get_or_insert(Vec::new());
        for val in aggregation_keys {
            v.push(val.into());
        }
        self
    }
    pub fn debug_key(mut self, debug_key: impl Into<UnsignedInt64AsBase10>) -> Self {
        self.debug_key = Some(debug_key.into());
        self
    }
    pub fn trigger_data_matching(
        mut self,
        trigger_data_matching: impl Into<AttributionReportingTriggerDataMatching>,
    ) -> Self {
        self.trigger_data_matching = Some(trigger_data_matching.into());
        self
    }
    pub fn destination_limit_priority(
        mut self,
        destination_limit_priority: impl Into<SignedInt64AsBase10>,
    ) -> Self {
        self.destination_limit_priority = Some(destination_limit_priority.into());
        self
    }
    pub fn aggregatable_debug_reporting_config(
        mut self,
        aggregatable_debug_reporting_config: impl Into<
            AttributionReportingAggregatableDebugReportingConfig,
        >,
    ) -> Self {
        self.aggregatable_debug_reporting_config = Some(aggregatable_debug_reporting_config.into());
        self
    }
    pub fn scopes_data(mut self, scopes_data: impl Into<AttributionScopesData>) -> Self {
        self.scopes_data = Some(scopes_data.into());
        self
    }
    pub fn max_event_level_reports(mut self, max_event_level_reports: impl Into<i64>) -> Self {
        self.max_event_level_reports = Some(max_event_level_reports.into());
        self
    }
    pub fn named_budget(
        mut self,
        named_budget: impl Into<AttributionReportingNamedBudgetDef>,
    ) -> Self {
        let v = self.named_budgets.get_or_insert(Vec::new());
        v.push(named_budget.into());
        self
    }
    pub fn named_budgets<I, S>(mut self, named_budgets: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AttributionReportingNamedBudgetDef>,
    {
        let v = self.named_budgets.get_or_insert(Vec::new());
        for val in named_budgets {
            v.push(val.into());
        }
        self
    }
    pub fn debug_reporting(mut self, debug_reporting: impl Into<bool>) -> Self {
        self.debug_reporting = Some(debug_reporting.into());
        self
    }
    pub fn event_level_epsilon(mut self, event_level_epsilon: impl Into<f64>) -> Self {
        self.event_level_epsilon = Some(event_level_epsilon.into());
        self
    }
    pub fn build(self) -> Result<AttributionReportingSourceRegistration, String> {
        Ok(AttributionReportingSourceRegistration {
            time: self
                .time
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(time)))?,
            expiry: self
                .expiry
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(expiry)))?,
            trigger_data: self.trigger_data.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(trigger_data))
            })?,
            event_report_windows: self.event_report_windows.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(event_report_windows)
                )
            })?,
            aggregatable_report_window: self.aggregatable_report_window.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(aggregatable_report_window)
                )
            })?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            source_origin: self.source_origin.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(source_origin))
            })?,
            reporting_origin: self.reporting_origin.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(reporting_origin)
                )
            })?,
            destination_sites: self.destination_sites.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(destination_sites)
                )
            })?,
            event_id: self
                .event_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(event_id)))?,
            priority: self
                .priority
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(priority)))?,
            filter_data: self
                .filter_data
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(filter_data)))?,
            aggregation_keys: self.aggregation_keys.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(aggregation_keys)
                )
            })?,
            debug_key: self.debug_key,
            trigger_data_matching: self.trigger_data_matching.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(trigger_data_matching)
                )
            })?,
            destination_limit_priority: self.destination_limit_priority.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(destination_limit_priority)
                )
            })?,
            aggregatable_debug_reporting_config: self
                .aggregatable_debug_reporting_config
                .ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(aggregatable_debug_reporting_config)
                    )
                })?,
            scopes_data: self.scopes_data,
            max_event_level_reports: self.max_event_level_reports.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(max_event_level_reports)
                )
            })?,
            named_budgets: self.named_budgets.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(named_budgets))
            })?,
            debug_reporting: self.debug_reporting.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(debug_reporting))
            })?,
            event_level_epsilon: self.event_level_epsilon.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(event_level_epsilon)
                )
            })?,
        })
    }
}
impl AttributionReportingAggregatableValueDictEntry {
    pub fn builder() -> AttributionReportingAggregatableValueDictEntryBuilder {
        AttributionReportingAggregatableValueDictEntryBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingAggregatableValueDictEntryBuilder {
    key: Option<String>,
    value: Option<f64>,
    filtering_id: Option<UnsignedInt64AsBase10>,
}
impl AttributionReportingAggregatableValueDictEntryBuilder {
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn value(mut self, value: impl Into<f64>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn filtering_id(mut self, filtering_id: impl Into<UnsignedInt64AsBase10>) -> Self {
        self.filtering_id = Some(filtering_id.into());
        self
    }
    pub fn build(self) -> Result<AttributionReportingAggregatableValueDictEntry, String> {
        Ok(AttributionReportingAggregatableValueDictEntry {
            key: self
                .key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            filtering_id: self.filtering_id.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(filtering_id))
            })?,
        })
    }
}
impl AttributionReportingAggregatableValueEntry {
    pub fn builder() -> AttributionReportingAggregatableValueEntryBuilder {
        AttributionReportingAggregatableValueEntryBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingAggregatableValueEntryBuilder {
    values: Option<Vec<AttributionReportingAggregatableValueDictEntry>>,
    filters: Option<AttributionReportingFilterPair>,
}
impl AttributionReportingAggregatableValueEntryBuilder {
    pub fn value(
        mut self,
        value: impl Into<AttributionReportingAggregatableValueDictEntry>,
    ) -> Self {
        let v = self.values.get_or_insert(Vec::new());
        v.push(value.into());
        self
    }
    pub fn values<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AttributionReportingAggregatableValueDictEntry>,
    {
        let v = self.values.get_or_insert(Vec::new());
        for val in values {
            v.push(val.into());
        }
        self
    }
    pub fn filters(mut self, filters: impl Into<AttributionReportingFilterPair>) -> Self {
        self.filters = Some(filters.into());
        self
    }
    pub fn build(self) -> Result<AttributionReportingAggregatableValueEntry, String> {
        Ok(AttributionReportingAggregatableValueEntry {
            values: self
                .values
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(values)))?,
            filters: self
                .filters
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(filters)))?,
        })
    }
}
impl AttributionReportingEventTriggerData {
    pub fn builder() -> AttributionReportingEventTriggerDataBuilder {
        AttributionReportingEventTriggerDataBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingEventTriggerDataBuilder {
    data: Option<UnsignedInt64AsBase10>,
    priority: Option<SignedInt64AsBase10>,
    dedup_key: Option<UnsignedInt64AsBase10>,
    filters: Option<AttributionReportingFilterPair>,
}
impl AttributionReportingEventTriggerDataBuilder {
    pub fn data(mut self, data: impl Into<UnsignedInt64AsBase10>) -> Self {
        self.data = Some(data.into());
        self
    }
    pub fn priority(mut self, priority: impl Into<SignedInt64AsBase10>) -> Self {
        self.priority = Some(priority.into());
        self
    }
    pub fn dedup_key(mut self, dedup_key: impl Into<UnsignedInt64AsBase10>) -> Self {
        self.dedup_key = Some(dedup_key.into());
        self
    }
    pub fn filters(mut self, filters: impl Into<AttributionReportingFilterPair>) -> Self {
        self.filters = Some(filters.into());
        self
    }
    pub fn build(self) -> Result<AttributionReportingEventTriggerData, String> {
        Ok(AttributionReportingEventTriggerData {
            data: self
                .data
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(data)))?,
            priority: self
                .priority
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(priority)))?,
            dedup_key: self.dedup_key,
            filters: self
                .filters
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(filters)))?,
        })
    }
}
impl AttributionReportingAggregatableTriggerData {
    pub fn builder() -> AttributionReportingAggregatableTriggerDataBuilder {
        AttributionReportingAggregatableTriggerDataBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingAggregatableTriggerDataBuilder {
    key_piece: Option<UnsignedInt128AsBase16>,
    source_keys: Option<Vec<String>>,
    filters: Option<AttributionReportingFilterPair>,
}
impl AttributionReportingAggregatableTriggerDataBuilder {
    pub fn key_piece(mut self, key_piece: impl Into<UnsignedInt128AsBase16>) -> Self {
        self.key_piece = Some(key_piece.into());
        self
    }
    pub fn source_key(mut self, source_key: impl Into<String>) -> Self {
        let v = self.source_keys.get_or_insert(Vec::new());
        v.push(source_key.into());
        self
    }
    pub fn source_keys<I, S>(mut self, source_keys: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.source_keys.get_or_insert(Vec::new());
        for val in source_keys {
            v.push(val.into());
        }
        self
    }
    pub fn filters(mut self, filters: impl Into<AttributionReportingFilterPair>) -> Self {
        self.filters = Some(filters.into());
        self
    }
    pub fn build(self) -> Result<AttributionReportingAggregatableTriggerData, String> {
        Ok(AttributionReportingAggregatableTriggerData {
            key_piece: self
                .key_piece
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key_piece)))?,
            source_keys: self
                .source_keys
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(source_keys)))?,
            filters: self
                .filters
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(filters)))?,
        })
    }
}
impl AttributionReportingAggregatableDedupKey {
    pub fn builder() -> AttributionReportingAggregatableDedupKeyBuilder {
        AttributionReportingAggregatableDedupKeyBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingAggregatableDedupKeyBuilder {
    dedup_key: Option<UnsignedInt64AsBase10>,
    filters: Option<AttributionReportingFilterPair>,
}
impl AttributionReportingAggregatableDedupKeyBuilder {
    pub fn dedup_key(mut self, dedup_key: impl Into<UnsignedInt64AsBase10>) -> Self {
        self.dedup_key = Some(dedup_key.into());
        self
    }
    pub fn filters(mut self, filters: impl Into<AttributionReportingFilterPair>) -> Self {
        self.filters = Some(filters.into());
        self
    }
    pub fn build(self) -> Result<AttributionReportingAggregatableDedupKey, String> {
        Ok(AttributionReportingAggregatableDedupKey {
            dedup_key: self.dedup_key,
            filters: self
                .filters
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(filters)))?,
        })
    }
}
impl AttributionReportingNamedBudgetCandidate {
    pub fn builder() -> AttributionReportingNamedBudgetCandidateBuilder {
        AttributionReportingNamedBudgetCandidateBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingNamedBudgetCandidateBuilder {
    name: Option<String>,
    filters: Option<AttributionReportingFilterPair>,
}
impl AttributionReportingNamedBudgetCandidateBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn filters(mut self, filters: impl Into<AttributionReportingFilterPair>) -> Self {
        self.filters = Some(filters.into());
        self
    }
    pub fn build(self) -> Result<AttributionReportingNamedBudgetCandidate, String> {
        Ok(AttributionReportingNamedBudgetCandidate {
            name: self.name,
            filters: self
                .filters
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(filters)))?,
        })
    }
}
impl AttributionReportingTriggerRegistration {
    pub fn builder() -> AttributionReportingTriggerRegistrationBuilder {
        AttributionReportingTriggerRegistrationBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingTriggerRegistrationBuilder {
    filters: Option<AttributionReportingFilterPair>,
    debug_key: Option<UnsignedInt64AsBase10>,
    aggregatable_dedup_keys: Option<Vec<AttributionReportingAggregatableDedupKey>>,
    event_trigger_data: Option<Vec<AttributionReportingEventTriggerData>>,
    aggregatable_trigger_data: Option<Vec<AttributionReportingAggregatableTriggerData>>,
    aggregatable_values: Option<Vec<AttributionReportingAggregatableValueEntry>>,
    aggregatable_filtering_id_max_bytes: Option<i64>,
    debug_reporting: Option<bool>,
    aggregation_coordinator_origin: Option<String>,
    source_registration_time_config: Option<AttributionReportingSourceRegistrationTimeConfig>,
    trigger_context_id: Option<String>,
    aggregatable_debug_reporting_config:
        Option<AttributionReportingAggregatableDebugReportingConfig>,
    scopes: Option<Vec<String>>,
    named_budgets: Option<Vec<AttributionReportingNamedBudgetCandidate>>,
}
impl AttributionReportingTriggerRegistrationBuilder {
    pub fn filters(mut self, filters: impl Into<AttributionReportingFilterPair>) -> Self {
        self.filters = Some(filters.into());
        self
    }
    pub fn debug_key(mut self, debug_key: impl Into<UnsignedInt64AsBase10>) -> Self {
        self.debug_key = Some(debug_key.into());
        self
    }
    pub fn aggregatable_dedup_key(
        mut self,
        aggregatable_dedup_key: impl Into<AttributionReportingAggregatableDedupKey>,
    ) -> Self {
        let v = self.aggregatable_dedup_keys.get_or_insert(Vec::new());
        v.push(aggregatable_dedup_key.into());
        self
    }
    pub fn aggregatable_dedup_keys<I, S>(mut self, aggregatable_dedup_keys: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AttributionReportingAggregatableDedupKey>,
    {
        let v = self.aggregatable_dedup_keys.get_or_insert(Vec::new());
        for val in aggregatable_dedup_keys {
            v.push(val.into());
        }
        self
    }
    pub fn event_trigger_data(
        mut self,
        event_trigger_data: impl Into<AttributionReportingEventTriggerData>,
    ) -> Self {
        let v = self.event_trigger_data.get_or_insert(Vec::new());
        v.push(event_trigger_data.into());
        self
    }
    pub fn event_trigger_datas<I, S>(mut self, event_trigger_datas: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AttributionReportingEventTriggerData>,
    {
        let v = self.event_trigger_data.get_or_insert(Vec::new());
        for val in event_trigger_datas {
            v.push(val.into());
        }
        self
    }
    pub fn aggregatable_trigger_data(
        mut self,
        aggregatable_trigger_data: impl Into<AttributionReportingAggregatableTriggerData>,
    ) -> Self {
        let v = self.aggregatable_trigger_data.get_or_insert(Vec::new());
        v.push(aggregatable_trigger_data.into());
        self
    }
    pub fn aggregatable_trigger_datas<I, S>(mut self, aggregatable_trigger_datas: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AttributionReportingAggregatableTriggerData>,
    {
        let v = self.aggregatable_trigger_data.get_or_insert(Vec::new());
        for val in aggregatable_trigger_datas {
            v.push(val.into());
        }
        self
    }
    pub fn aggregatable_value(
        mut self,
        aggregatable_value: impl Into<AttributionReportingAggregatableValueEntry>,
    ) -> Self {
        let v = self.aggregatable_values.get_or_insert(Vec::new());
        v.push(aggregatable_value.into());
        self
    }
    pub fn aggregatable_values<I, S>(mut self, aggregatable_values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AttributionReportingAggregatableValueEntry>,
    {
        let v = self.aggregatable_values.get_or_insert(Vec::new());
        for val in aggregatable_values {
            v.push(val.into());
        }
        self
    }
    pub fn aggregatable_filtering_id_max_bytes(
        mut self,
        aggregatable_filtering_id_max_bytes: impl Into<i64>,
    ) -> Self {
        self.aggregatable_filtering_id_max_bytes = Some(aggregatable_filtering_id_max_bytes.into());
        self
    }
    pub fn debug_reporting(mut self, debug_reporting: impl Into<bool>) -> Self {
        self.debug_reporting = Some(debug_reporting.into());
        self
    }
    pub fn aggregation_coordinator_origin(
        mut self,
        aggregation_coordinator_origin: impl Into<String>,
    ) -> Self {
        self.aggregation_coordinator_origin = Some(aggregation_coordinator_origin.into());
        self
    }
    pub fn source_registration_time_config(
        mut self,
        source_registration_time_config: impl Into<AttributionReportingSourceRegistrationTimeConfig>,
    ) -> Self {
        self.source_registration_time_config = Some(source_registration_time_config.into());
        self
    }
    pub fn trigger_context_id(mut self, trigger_context_id: impl Into<String>) -> Self {
        self.trigger_context_id = Some(trigger_context_id.into());
        self
    }
    pub fn aggregatable_debug_reporting_config(
        mut self,
        aggregatable_debug_reporting_config: impl Into<
            AttributionReportingAggregatableDebugReportingConfig,
        >,
    ) -> Self {
        self.aggregatable_debug_reporting_config = Some(aggregatable_debug_reporting_config.into());
        self
    }
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        let v = self.scopes.get_or_insert(Vec::new());
        v.push(scope.into());
        self
    }
    pub fn scopes<I, S>(mut self, scopes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.scopes.get_or_insert(Vec::new());
        for val in scopes {
            v.push(val.into());
        }
        self
    }
    pub fn named_budget(
        mut self,
        named_budget: impl Into<AttributionReportingNamedBudgetCandidate>,
    ) -> Self {
        let v = self.named_budgets.get_or_insert(Vec::new());
        v.push(named_budget.into());
        self
    }
    pub fn named_budgets<I, S>(mut self, named_budgets: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AttributionReportingNamedBudgetCandidate>,
    {
        let v = self.named_budgets.get_or_insert(Vec::new());
        for val in named_budgets {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<AttributionReportingTriggerRegistration, String> {
        Ok(AttributionReportingTriggerRegistration {
            filters: self
                .filters
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(filters)))?,
            debug_key: self.debug_key,
            aggregatable_dedup_keys: self.aggregatable_dedup_keys.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(aggregatable_dedup_keys)
                )
            })?,
            event_trigger_data: self.event_trigger_data.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(event_trigger_data)
                )
            })?,
            aggregatable_trigger_data: self.aggregatable_trigger_data.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(aggregatable_trigger_data)
                )
            })?,
            aggregatable_values: self.aggregatable_values.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(aggregatable_values)
                )
            })?,
            aggregatable_filtering_id_max_bytes: self
                .aggregatable_filtering_id_max_bytes
                .ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(aggregatable_filtering_id_max_bytes)
                    )
                })?,
            debug_reporting: self.debug_reporting.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(debug_reporting))
            })?,
            aggregation_coordinator_origin: self.aggregation_coordinator_origin,
            source_registration_time_config: self.source_registration_time_config.ok_or_else(
                || {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(source_registration_time_config)
                    )
                },
            )?,
            trigger_context_id: self.trigger_context_id,
            aggregatable_debug_reporting_config: self
                .aggregatable_debug_reporting_config
                .ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(aggregatable_debug_reporting_config)
                    )
                })?,
            scopes: self
                .scopes
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(scopes)))?,
            named_budgets: self.named_budgets.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(named_budgets))
            })?,
        })
    }
}
impl RelatedWebsiteSet {
    pub fn builder() -> RelatedWebsiteSetBuilder {
        RelatedWebsiteSetBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct RelatedWebsiteSetBuilder {
    primary_sites: Option<Vec<String>>,
    associated_sites: Option<Vec<String>>,
    service_sites: Option<Vec<String>>,
}
impl RelatedWebsiteSetBuilder {
    pub fn primary_site(mut self, primary_site: impl Into<String>) -> Self {
        let v = self.primary_sites.get_or_insert(Vec::new());
        v.push(primary_site.into());
        self
    }
    pub fn primary_sites<I, S>(mut self, primary_sites: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.primary_sites.get_or_insert(Vec::new());
        for val in primary_sites {
            v.push(val.into());
        }
        self
    }
    pub fn associated_site(mut self, associated_site: impl Into<String>) -> Self {
        let v = self.associated_sites.get_or_insert(Vec::new());
        v.push(associated_site.into());
        self
    }
    pub fn associated_sites<I, S>(mut self, associated_sites: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.associated_sites.get_or_insert(Vec::new());
        for val in associated_sites {
            v.push(val.into());
        }
        self
    }
    pub fn service_site(mut self, service_site: impl Into<String>) -> Self {
        let v = self.service_sites.get_or_insert(Vec::new());
        v.push(service_site.into());
        self
    }
    pub fn service_sites<I, S>(mut self, service_sites: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.service_sites.get_or_insert(Vec::new());
        for val in service_sites {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<RelatedWebsiteSet, String> {
        Ok(RelatedWebsiteSet {
            primary_sites: self.primary_sites.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(primary_sites))
            })?,
            associated_sites: self.associated_sites.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(associated_sites)
                )
            })?,
            service_sites: self.service_sites.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(service_sites))
            })?,
        })
    }
}
