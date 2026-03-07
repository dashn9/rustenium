use super::commands::*;
impl GetStorageKey {
    pub fn builder() -> GetStorageKeyBuilder {
        <GetStorageKeyBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetStorageKeyBuilder {
    frame_id: Option<crate::browser_protocol::page::types::FrameId>,
}
impl GetStorageKeyBuilder {
    pub fn frame_id(
        mut self,
        frame_id: impl Into<crate::browser_protocol::page::types::FrameId>,
    ) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn build(self) -> GetStorageKey {
        GetStorageKey {
            method: GetStorageKeyMethod::GetStorageKey,
            params: GetStorageKeyParams {
                frame_id: self.frame_id,
            },
        }
    }
}
impl ClearDataForOrigin {
    pub fn builder() -> ClearDataForOriginBuilder {
        <ClearDataForOriginBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ClearDataForOriginBuilder {
    origin: Option<String>,
    storage_types: Option<String>,
}
impl ClearDataForOriginBuilder {
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn storage_types(mut self, storage_types: impl Into<String>) -> Self {
        self.storage_types = Some(storage_types.into());
        self
    }
    pub fn build(self) -> Result<ClearDataForOrigin, String> {
        Ok(ClearDataForOrigin {
            method: ClearDataForOriginMethod::ClearDataForOrigin,
            params: ClearDataForOriginParams {
                origin: self
                    .origin
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
                storage_types: self.storage_types.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_types))
                })?,
            },
        })
    }
}
impl ClearDataForStorageKey {
    pub fn builder() -> ClearDataForStorageKeyBuilder {
        <ClearDataForStorageKeyBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ClearDataForStorageKeyBuilder {
    storage_key: Option<String>,
    storage_types: Option<String>,
}
impl ClearDataForStorageKeyBuilder {
    pub fn storage_key(mut self, storage_key: impl Into<String>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn storage_types(mut self, storage_types: impl Into<String>) -> Self {
        self.storage_types = Some(storage_types.into());
        self
    }
    pub fn build(self) -> Result<ClearDataForStorageKey, String> {
        Ok(ClearDataForStorageKey {
            method: ClearDataForStorageKeyMethod::ClearDataForStorageKey,
            params: ClearDataForStorageKeyParams {
                storage_key: self.storage_key.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_key))
                })?,
                storage_types: self.storage_types.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_types))
                })?,
            },
        })
    }
}
impl GetCookies {
    pub fn builder() -> GetCookiesBuilder {
        <GetCookiesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetCookiesBuilder {
    browser_context_id: Option<crate::browser_protocol::browser::types::BrowserContextId>,
}
impl GetCookiesBuilder {
    pub fn browser_context_id(
        mut self,
        browser_context_id: impl Into<crate::browser_protocol::browser::types::BrowserContextId>,
    ) -> Self {
        self.browser_context_id = Some(browser_context_id.into());
        self
    }
    pub fn build(self) -> GetCookies {
        GetCookies {
            method: GetCookiesMethod::GetCookies,
            params: GetCookiesParams {
                browser_context_id: self.browser_context_id,
            },
        }
    }
}
impl SetCookies {
    pub fn builder() -> SetCookiesBuilder {
        <SetCookiesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetCookiesBuilder {
    cookies: Option<Vec<crate::browser_protocol::network::types::CookieParam>>,
    browser_context_id: Option<crate::browser_protocol::browser::types::BrowserContextId>,
}
impl SetCookiesBuilder {
    pub fn cookie(
        mut self,
        cookie: impl Into<crate::browser_protocol::network::types::CookieParam>,
    ) -> Self {
        let v = self.cookies.get_or_insert(Vec::new());
        v.push(cookie.into());
        self
    }
    pub fn cookies<I, S>(mut self, cookies: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser_protocol::network::types::CookieParam>,
    {
        let v = self.cookies.get_or_insert(Vec::new());
        for val in cookies {
            v.push(val.into());
        }
        self
    }
    pub fn browser_context_id(
        mut self,
        browser_context_id: impl Into<crate::browser_protocol::browser::types::BrowserContextId>,
    ) -> Self {
        self.browser_context_id = Some(browser_context_id.into());
        self
    }
    pub fn build(self) -> Result<SetCookies, String> {
        Ok(SetCookies {
            method: SetCookiesMethod::SetCookies,
            params: SetCookiesParams {
                cookies: self
                    .cookies
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cookies)))?,
                browser_context_id: self.browser_context_id,
            },
        })
    }
}
impl ClearCookies {
    pub fn builder() -> ClearCookiesBuilder {
        <ClearCookiesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ClearCookiesBuilder {
    browser_context_id: Option<crate::browser_protocol::browser::types::BrowserContextId>,
}
impl ClearCookiesBuilder {
    pub fn browser_context_id(
        mut self,
        browser_context_id: impl Into<crate::browser_protocol::browser::types::BrowserContextId>,
    ) -> Self {
        self.browser_context_id = Some(browser_context_id.into());
        self
    }
    pub fn build(self) -> ClearCookies {
        ClearCookies {
            method: ClearCookiesMethod::ClearCookies,
            params: ClearCookiesParams {
                browser_context_id: self.browser_context_id,
            },
        }
    }
}
impl GetUsageAndQuota {
    pub fn builder() -> GetUsageAndQuotaBuilder {
        <GetUsageAndQuotaBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetUsageAndQuotaBuilder {
    origin: Option<String>,
}
impl GetUsageAndQuotaBuilder {
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn build(self) -> Result<GetUsageAndQuota, String> {
        Ok(GetUsageAndQuota {
            method: GetUsageAndQuotaMethod::GetUsageAndQuota,
            params: GetUsageAndQuotaParams {
                origin: self
                    .origin
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            },
        })
    }
}
impl OverrideQuotaForOrigin {
    pub fn builder() -> OverrideQuotaForOriginBuilder {
        <OverrideQuotaForOriginBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct OverrideQuotaForOriginBuilder {
    origin: Option<String>,
    quota_size: Option<f64>,
}
impl OverrideQuotaForOriginBuilder {
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn quota_size(mut self, quota_size: impl Into<f64>) -> Self {
        self.quota_size = Some(quota_size.into());
        self
    }
    pub fn build(self) -> Result<OverrideQuotaForOrigin, String> {
        Ok(OverrideQuotaForOrigin {
            method: OverrideQuotaForOriginMethod::OverrideQuotaForOrigin,
            params: OverrideQuotaForOriginParams {
                origin: self
                    .origin
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
                quota_size: self.quota_size,
            },
        })
    }
}
impl TrackCacheStorageForOrigin {
    pub fn builder() -> TrackCacheStorageForOriginBuilder {
        <TrackCacheStorageForOriginBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct TrackCacheStorageForOriginBuilder {
    origin: Option<String>,
}
impl TrackCacheStorageForOriginBuilder {
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn build(self) -> Result<TrackCacheStorageForOrigin, String> {
        Ok(TrackCacheStorageForOrigin {
            method: TrackCacheStorageForOriginMethod::TrackCacheStorageForOrigin,
            params: TrackCacheStorageForOriginParams {
                origin: self
                    .origin
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            },
        })
    }
}
impl TrackCacheStorageForStorageKey {
    pub fn builder() -> TrackCacheStorageForStorageKeyBuilder {
        <TrackCacheStorageForStorageKeyBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct TrackCacheStorageForStorageKeyBuilder {
    storage_key: Option<String>,
}
impl TrackCacheStorageForStorageKeyBuilder {
    pub fn storage_key(mut self, storage_key: impl Into<String>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn build(self) -> Result<TrackCacheStorageForStorageKey, String> {
        Ok(TrackCacheStorageForStorageKey {
            method: TrackCacheStorageForStorageKeyMethod::TrackCacheStorageForStorageKey,
            params: TrackCacheStorageForStorageKeyParams {
                storage_key: self.storage_key.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_key))
                })?,
            },
        })
    }
}
impl TrackIndexedDbForOrigin {
    pub fn builder() -> TrackIndexedDbForOriginBuilder {
        <TrackIndexedDbForOriginBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct TrackIndexedDbForOriginBuilder {
    origin: Option<String>,
}
impl TrackIndexedDbForOriginBuilder {
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn build(self) -> Result<TrackIndexedDbForOrigin, String> {
        Ok(TrackIndexedDbForOrigin {
            method: TrackIndexedDbForOriginMethod::TrackIndexedDbForOrigin,
            params: TrackIndexedDbForOriginParams {
                origin: self
                    .origin
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            },
        })
    }
}
impl TrackIndexedDbForStorageKey {
    pub fn builder() -> TrackIndexedDbForStorageKeyBuilder {
        <TrackIndexedDbForStorageKeyBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct TrackIndexedDbForStorageKeyBuilder {
    storage_key: Option<String>,
}
impl TrackIndexedDbForStorageKeyBuilder {
    pub fn storage_key(mut self, storage_key: impl Into<String>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn build(self) -> Result<TrackIndexedDbForStorageKey, String> {
        Ok(TrackIndexedDbForStorageKey {
            method: TrackIndexedDbForStorageKeyMethod::TrackIndexedDbForStorageKey,
            params: TrackIndexedDbForStorageKeyParams {
                storage_key: self.storage_key.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_key))
                })?,
            },
        })
    }
}
impl UntrackCacheStorageForOrigin {
    pub fn builder() -> UntrackCacheStorageForOriginBuilder {
        <UntrackCacheStorageForOriginBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UntrackCacheStorageForOriginBuilder {
    origin: Option<String>,
}
impl UntrackCacheStorageForOriginBuilder {
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn build(self) -> Result<UntrackCacheStorageForOrigin, String> {
        Ok(UntrackCacheStorageForOrigin {
            method: UntrackCacheStorageForOriginMethod::UntrackCacheStorageForOrigin,
            params: UntrackCacheStorageForOriginParams {
                origin: self
                    .origin
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            },
        })
    }
}
impl UntrackCacheStorageForStorageKey {
    pub fn builder() -> UntrackCacheStorageForStorageKeyBuilder {
        <UntrackCacheStorageForStorageKeyBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UntrackCacheStorageForStorageKeyBuilder {
    storage_key: Option<String>,
}
impl UntrackCacheStorageForStorageKeyBuilder {
    pub fn storage_key(mut self, storage_key: impl Into<String>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn build(self) -> Result<UntrackCacheStorageForStorageKey, String> {
        Ok(UntrackCacheStorageForStorageKey {
            method: UntrackCacheStorageForStorageKeyMethod::UntrackCacheStorageForStorageKey,
            params: UntrackCacheStorageForStorageKeyParams {
                storage_key: self.storage_key.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_key))
                })?,
            },
        })
    }
}
impl UntrackIndexedDbForOrigin {
    pub fn builder() -> UntrackIndexedDbForOriginBuilder {
        <UntrackIndexedDbForOriginBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UntrackIndexedDbForOriginBuilder {
    origin: Option<String>,
}
impl UntrackIndexedDbForOriginBuilder {
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn build(self) -> Result<UntrackIndexedDbForOrigin, String> {
        Ok(UntrackIndexedDbForOrigin {
            method: UntrackIndexedDbForOriginMethod::UntrackIndexedDbForOrigin,
            params: UntrackIndexedDbForOriginParams {
                origin: self
                    .origin
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            },
        })
    }
}
impl UntrackIndexedDbForStorageKey {
    pub fn builder() -> UntrackIndexedDbForStorageKeyBuilder {
        <UntrackIndexedDbForStorageKeyBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UntrackIndexedDbForStorageKeyBuilder {
    storage_key: Option<String>,
}
impl UntrackIndexedDbForStorageKeyBuilder {
    pub fn storage_key(mut self, storage_key: impl Into<String>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn build(self) -> Result<UntrackIndexedDbForStorageKey, String> {
        Ok(UntrackIndexedDbForStorageKey {
            method: UntrackIndexedDbForStorageKeyMethod::UntrackIndexedDbForStorageKey,
            params: UntrackIndexedDbForStorageKeyParams {
                storage_key: self.storage_key.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_key))
                })?,
            },
        })
    }
}
impl ClearTrustTokens {
    pub fn builder() -> ClearTrustTokensBuilder {
        <ClearTrustTokensBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ClearTrustTokensBuilder {
    issuer_origin: Option<String>,
}
impl ClearTrustTokensBuilder {
    pub fn issuer_origin(mut self, issuer_origin: impl Into<String>) -> Self {
        self.issuer_origin = Some(issuer_origin.into());
        self
    }
    pub fn build(self) -> Result<ClearTrustTokens, String> {
        Ok(ClearTrustTokens {
            method: ClearTrustTokensMethod::ClearTrustTokens,
            params: ClearTrustTokensParams {
                issuer_origin: self.issuer_origin.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(issuer_origin))
                })?,
            },
        })
    }
}
impl GetInterestGroupDetails {
    pub fn builder() -> GetInterestGroupDetailsBuilder {
        <GetInterestGroupDetailsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetInterestGroupDetailsBuilder {
    owner_origin: Option<String>,
    name: Option<String>,
}
impl GetInterestGroupDetailsBuilder {
    pub fn owner_origin(mut self, owner_origin: impl Into<String>) -> Self {
        self.owner_origin = Some(owner_origin.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn build(self) -> Result<GetInterestGroupDetails, String> {
        Ok(GetInterestGroupDetails {
            method: GetInterestGroupDetailsMethod::GetInterestGroupDetails,
            params: GetInterestGroupDetailsParams {
                owner_origin: self.owner_origin.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(owner_origin))
                })?,
                name: self
                    .name
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            },
        })
    }
}
impl SetInterestGroupTracking {
    pub fn builder() -> SetInterestGroupTrackingBuilder {
        <SetInterestGroupTrackingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetInterestGroupTrackingBuilder {
    enable: Option<bool>,
}
impl SetInterestGroupTrackingBuilder {
    pub fn enable(mut self, enable: impl Into<bool>) -> Self {
        self.enable = Some(enable.into());
        self
    }
    pub fn build(self) -> Result<SetInterestGroupTracking, String> {
        Ok(SetInterestGroupTracking {
            method: SetInterestGroupTrackingMethod::SetInterestGroupTracking,
            params: SetInterestGroupTrackingParams {
                enable: self
                    .enable
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enable)))?,
            },
        })
    }
}
impl SetInterestGroupAuctionTracking {
    pub fn builder() -> SetInterestGroupAuctionTrackingBuilder {
        <SetInterestGroupAuctionTrackingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetInterestGroupAuctionTrackingBuilder {
    enable: Option<bool>,
}
impl SetInterestGroupAuctionTrackingBuilder {
    pub fn enable(mut self, enable: impl Into<bool>) -> Self {
        self.enable = Some(enable.into());
        self
    }
    pub fn build(self) -> Result<SetInterestGroupAuctionTracking, String> {
        Ok(SetInterestGroupAuctionTracking {
            method: SetInterestGroupAuctionTrackingMethod::SetInterestGroupAuctionTracking,
            params: SetInterestGroupAuctionTrackingParams {
                enable: self
                    .enable
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enable)))?,
            },
        })
    }
}
impl GetSharedStorageMetadata {
    pub fn builder() -> GetSharedStorageMetadataBuilder {
        <GetSharedStorageMetadataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetSharedStorageMetadataBuilder {
    owner_origin: Option<String>,
}
impl GetSharedStorageMetadataBuilder {
    pub fn owner_origin(mut self, owner_origin: impl Into<String>) -> Self {
        self.owner_origin = Some(owner_origin.into());
        self
    }
    pub fn build(self) -> Result<GetSharedStorageMetadata, String> {
        Ok(GetSharedStorageMetadata {
            method: GetSharedStorageMetadataMethod::GetSharedStorageMetadata,
            params: GetSharedStorageMetadataParams {
                owner_origin: self.owner_origin.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(owner_origin))
                })?,
            },
        })
    }
}
impl GetSharedStorageEntries {
    pub fn builder() -> GetSharedStorageEntriesBuilder {
        <GetSharedStorageEntriesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetSharedStorageEntriesBuilder {
    owner_origin: Option<String>,
}
impl GetSharedStorageEntriesBuilder {
    pub fn owner_origin(mut self, owner_origin: impl Into<String>) -> Self {
        self.owner_origin = Some(owner_origin.into());
        self
    }
    pub fn build(self) -> Result<GetSharedStorageEntries, String> {
        Ok(GetSharedStorageEntries {
            method: GetSharedStorageEntriesMethod::GetSharedStorageEntries,
            params: GetSharedStorageEntriesParams {
                owner_origin: self.owner_origin.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(owner_origin))
                })?,
            },
        })
    }
}
impl SetSharedStorageEntry {
    pub fn builder() -> SetSharedStorageEntryBuilder {
        <SetSharedStorageEntryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetSharedStorageEntryBuilder {
    owner_origin: Option<String>,
    key: Option<String>,
    value: Option<String>,
    ignore_if_present: Option<bool>,
}
impl SetSharedStorageEntryBuilder {
    pub fn owner_origin(mut self, owner_origin: impl Into<String>) -> Self {
        self.owner_origin = Some(owner_origin.into());
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
    pub fn build(self) -> Result<SetSharedStorageEntry, String> {
        Ok(SetSharedStorageEntry {
            method: SetSharedStorageEntryMethod::SetSharedStorageEntry,
            params: SetSharedStorageEntryParams {
                owner_origin: self.owner_origin.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(owner_origin))
                })?,
                key: self
                    .key
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key)))?,
                value: self
                    .value
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
                ignore_if_present: self.ignore_if_present,
            },
        })
    }
}
impl DeleteSharedStorageEntry {
    pub fn builder() -> DeleteSharedStorageEntryBuilder {
        <DeleteSharedStorageEntryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeleteSharedStorageEntryBuilder {
    owner_origin: Option<String>,
    key: Option<String>,
}
impl DeleteSharedStorageEntryBuilder {
    pub fn owner_origin(mut self, owner_origin: impl Into<String>) -> Self {
        self.owner_origin = Some(owner_origin.into());
        self
    }
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn build(self) -> Result<DeleteSharedStorageEntry, String> {
        Ok(DeleteSharedStorageEntry {
            method: DeleteSharedStorageEntryMethod::DeleteSharedStorageEntry,
            params: DeleteSharedStorageEntryParams {
                owner_origin: self.owner_origin.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(owner_origin))
                })?,
                key: self
                    .key
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key)))?,
            },
        })
    }
}
impl ClearSharedStorageEntries {
    pub fn builder() -> ClearSharedStorageEntriesBuilder {
        <ClearSharedStorageEntriesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ClearSharedStorageEntriesBuilder {
    owner_origin: Option<String>,
}
impl ClearSharedStorageEntriesBuilder {
    pub fn owner_origin(mut self, owner_origin: impl Into<String>) -> Self {
        self.owner_origin = Some(owner_origin.into());
        self
    }
    pub fn build(self) -> Result<ClearSharedStorageEntries, String> {
        Ok(ClearSharedStorageEntries {
            method: ClearSharedStorageEntriesMethod::ClearSharedStorageEntries,
            params: ClearSharedStorageEntriesParams {
                owner_origin: self.owner_origin.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(owner_origin))
                })?,
            },
        })
    }
}
impl ResetSharedStorageBudget {
    pub fn builder() -> ResetSharedStorageBudgetBuilder {
        <ResetSharedStorageBudgetBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ResetSharedStorageBudgetBuilder {
    owner_origin: Option<String>,
}
impl ResetSharedStorageBudgetBuilder {
    pub fn owner_origin(mut self, owner_origin: impl Into<String>) -> Self {
        self.owner_origin = Some(owner_origin.into());
        self
    }
    pub fn build(self) -> Result<ResetSharedStorageBudget, String> {
        Ok(ResetSharedStorageBudget {
            method: ResetSharedStorageBudgetMethod::ResetSharedStorageBudget,
            params: ResetSharedStorageBudgetParams {
                owner_origin: self.owner_origin.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(owner_origin))
                })?,
            },
        })
    }
}
impl SetSharedStorageTracking {
    pub fn builder() -> SetSharedStorageTrackingBuilder {
        <SetSharedStorageTrackingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetSharedStorageTrackingBuilder {
    enable: Option<bool>,
}
impl SetSharedStorageTrackingBuilder {
    pub fn enable(mut self, enable: impl Into<bool>) -> Self {
        self.enable = Some(enable.into());
        self
    }
    pub fn build(self) -> Result<SetSharedStorageTracking, String> {
        Ok(SetSharedStorageTracking {
            method: SetSharedStorageTrackingMethod::SetSharedStorageTracking,
            params: SetSharedStorageTrackingParams {
                enable: self
                    .enable
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enable)))?,
            },
        })
    }
}
impl SetStorageBucketTracking {
    pub fn builder() -> SetStorageBucketTrackingBuilder {
        <SetStorageBucketTrackingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetStorageBucketTrackingBuilder {
    storage_key: Option<String>,
    enable: Option<bool>,
}
impl SetStorageBucketTrackingBuilder {
    pub fn storage_key(mut self, storage_key: impl Into<String>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn enable(mut self, enable: impl Into<bool>) -> Self {
        self.enable = Some(enable.into());
        self
    }
    pub fn build(self) -> Result<SetStorageBucketTracking, String> {
        Ok(SetStorageBucketTracking {
            method: SetStorageBucketTrackingMethod::SetStorageBucketTracking,
            params: SetStorageBucketTrackingParams {
                storage_key: self.storage_key.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_key))
                })?,
                enable: self
                    .enable
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enable)))?,
            },
        })
    }
}
impl DeleteStorageBucket {
    pub fn builder() -> DeleteStorageBucketBuilder {
        <DeleteStorageBucketBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeleteStorageBucketBuilder {
    bucket: Option<super::types::StorageBucket>,
}
impl DeleteStorageBucketBuilder {
    pub fn bucket(mut self, bucket: impl Into<super::types::StorageBucket>) -> Self {
        self.bucket = Some(bucket.into());
        self
    }
    pub fn build(self) -> Result<DeleteStorageBucket, String> {
        Ok(DeleteStorageBucket {
            method: DeleteStorageBucketMethod::DeleteStorageBucket,
            params: DeleteStorageBucketParams {
                bucket: self
                    .bucket
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(bucket)))?,
            },
        })
    }
}
impl SetAttributionReportingLocalTestingMode {
    pub fn builder() -> SetAttributionReportingLocalTestingModeBuilder {
        <SetAttributionReportingLocalTestingModeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetAttributionReportingLocalTestingModeBuilder {
    enabled: Option<bool>,
}
impl SetAttributionReportingLocalTestingModeBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn build(self) -> Result<SetAttributionReportingLocalTestingMode, String> {
        Ok (SetAttributionReportingLocalTestingMode { method : SetAttributionReportingLocalTestingModeMethod :: SetAttributionReportingLocalTestingMode , params : SetAttributionReportingLocalTestingModeParams { enabled : self . enabled . ok_or_else (|| format ! ("Field `{}` is mandatory." , std :: stringify ! (enabled))) ? , } , })
    }
}
impl SetAttributionReportingTracking {
    pub fn builder() -> SetAttributionReportingTrackingBuilder {
        <SetAttributionReportingTrackingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetAttributionReportingTrackingBuilder {
    enable: Option<bool>,
}
impl SetAttributionReportingTrackingBuilder {
    pub fn enable(mut self, enable: impl Into<bool>) -> Self {
        self.enable = Some(enable.into());
        self
    }
    pub fn build(self) -> Result<SetAttributionReportingTracking, String> {
        Ok(SetAttributionReportingTracking {
            method: SetAttributionReportingTrackingMethod::SetAttributionReportingTracking,
            params: SetAttributionReportingTrackingParams {
                enable: self
                    .enable
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enable)))?,
            },
        })
    }
}
impl GetAffectedUrlsForThirdPartyCookieMetadata {
    pub fn builder() -> GetAffectedUrlsForThirdPartyCookieMetadataBuilder {
        <GetAffectedUrlsForThirdPartyCookieMetadataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetAffectedUrlsForThirdPartyCookieMetadataBuilder {
    first_party_url: Option<String>,
    third_party_urls: Option<Vec<String>>,
}
impl GetAffectedUrlsForThirdPartyCookieMetadataBuilder {
    pub fn first_party_url(mut self, first_party_url: impl Into<String>) -> Self {
        self.first_party_url = Some(first_party_url.into());
        self
    }
    pub fn third_party_url(mut self, third_party_url: impl Into<String>) -> Self {
        let v = self.third_party_urls.get_or_insert(Vec::new());
        v.push(third_party_url.into());
        self
    }
    pub fn third_party_urls<I, S>(mut self, third_party_urls: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.third_party_urls.get_or_insert(Vec::new());
        for val in third_party_urls {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<GetAffectedUrlsForThirdPartyCookieMetadata, String> {
        Ok (GetAffectedUrlsForThirdPartyCookieMetadata { method : GetAffectedUrlsForThirdPartyCookieMetadataMethod :: GetAffectedUrlsForThirdPartyCookieMetadata , params : GetAffectedUrlsForThirdPartyCookieMetadataParams { first_party_url : self . first_party_url . ok_or_else (|| format ! ("Field `{}` is mandatory." , std :: stringify ! (first_party_url))) ? , third_party_urls : self . third_party_urls . ok_or_else (|| format ! ("Field `{}` is mandatory." , std :: stringify ! (third_party_urls))) ? , } , })
    }
}
impl SetProtectedAudienceKAnonymity {
    pub fn builder() -> SetProtectedAudienceKAnonymityBuilder {
        <SetProtectedAudienceKAnonymityBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetProtectedAudienceKAnonymityBuilder {
    owner: Option<String>,
    name: Option<String>,
    hashes: Option<Vec<crate::Binary>>,
}
impl SetProtectedAudienceKAnonymityBuilder {
    pub fn owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn hashe(mut self, hashe: impl Into<crate::Binary>) -> Self {
        let v = self.hashes.get_or_insert(Vec::new());
        v.push(hashe.into());
        self
    }
    pub fn hashes<I, S>(mut self, hashes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::Binary>,
    {
        let v = self.hashes.get_or_insert(Vec::new());
        for val in hashes {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetProtectedAudienceKAnonymity, String> {
        Ok(SetProtectedAudienceKAnonymity {
            method: SetProtectedAudienceKAnonymityMethod::SetProtectedAudienceKAnonymity,
            params: SetProtectedAudienceKAnonymityParams {
                owner: self
                    .owner
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(owner)))?,
                name: self
                    .name
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
                hashes: self
                    .hashes
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(hashes)))?,
            },
        })
    }
}
