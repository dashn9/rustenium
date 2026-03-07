use super::commands::*;
impl DeleteCache {
    pub fn builder() -> DeleteCacheBuilder {
        <DeleteCacheBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeleteCacheBuilder {
    cache_id: Option<super::types::CacheId>,
}
impl DeleteCacheBuilder {
    pub fn cache_id(mut self, cache_id: impl Into<super::types::CacheId>) -> Self {
        self.cache_id = Some(cache_id.into());
        self
    }
    pub fn build(self) -> Result<DeleteCache, String> {
        Ok(DeleteCache {
            method: DeleteCacheMethod::DeleteCache,
            params: DeleteCacheParams {
                cache_id: self.cache_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(cache_id))
                })?,
            },
        })
    }
}
impl DeleteEntry {
    pub fn builder() -> DeleteEntryBuilder {
        <DeleteEntryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeleteEntryBuilder {
    cache_id: Option<super::types::CacheId>,
    request: Option<String>,
}
impl DeleteEntryBuilder {
    pub fn cache_id(mut self, cache_id: impl Into<super::types::CacheId>) -> Self {
        self.cache_id = Some(cache_id.into());
        self
    }
    pub fn request(mut self, request: impl Into<String>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn build(self) -> Result<DeleteEntry, String> {
        Ok(DeleteEntry {
            method: DeleteEntryMethod::DeleteEntry,
            params: DeleteEntryParams {
                cache_id: self.cache_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(cache_id))
                })?,
                request: self
                    .request
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request)))?,
            },
        })
    }
}
impl RequestCacheNames {
    pub fn builder() -> RequestCacheNamesBuilder {
        <RequestCacheNamesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RequestCacheNamesBuilder {
    security_origin: Option<String>,
    storage_key: Option<String>,
    storage_bucket: Option<crate::browser_protocol::storage::types::StorageBucket>,
}
impl RequestCacheNamesBuilder {
    pub fn security_origin(mut self, security_origin: impl Into<String>) -> Self {
        self.security_origin = Some(security_origin.into());
        self
    }
    pub fn storage_key(mut self, storage_key: impl Into<String>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn storage_bucket(
        mut self,
        storage_bucket: impl Into<crate::browser_protocol::storage::types::StorageBucket>,
    ) -> Self {
        self.storage_bucket = Some(storage_bucket.into());
        self
    }
    pub fn build(self) -> RequestCacheNames {
        RequestCacheNames {
            method: RequestCacheNamesMethod::RequestCacheNames,
            params: RequestCacheNamesParams {
                security_origin: self.security_origin,
                storage_key: self.storage_key,
                storage_bucket: self.storage_bucket,
            },
        }
    }
}
impl RequestCachedResponse {
    pub fn builder() -> RequestCachedResponseBuilder {
        <RequestCachedResponseBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RequestCachedResponseBuilder {
    cache_id: Option<super::types::CacheId>,
    request_url: Option<String>,
    request_headers: Option<Vec<super::types::Header>>,
}
impl RequestCachedResponseBuilder {
    pub fn cache_id(mut self, cache_id: impl Into<super::types::CacheId>) -> Self {
        self.cache_id = Some(cache_id.into());
        self
    }
    pub fn request_url(mut self, request_url: impl Into<String>) -> Self {
        self.request_url = Some(request_url.into());
        self
    }
    pub fn request_header(mut self, request_header: impl Into<super::types::Header>) -> Self {
        let v = self.request_headers.get_or_insert(Vec::new());
        v.push(request_header.into());
        self
    }
    pub fn request_headers<I, S>(mut self, request_headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::Header>,
    {
        let v = self.request_headers.get_or_insert(Vec::new());
        for val in request_headers {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<RequestCachedResponse, String> {
        Ok(RequestCachedResponse {
            method: RequestCachedResponseMethod::RequestCachedResponse,
            params: RequestCachedResponseParams {
                cache_id: self.cache_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(cache_id))
                })?,
                request_url: self.request_url.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(request_url))
                })?,
                request_headers: self.request_headers.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(request_headers))
                })?,
            },
        })
    }
}
impl RequestEntries {
    pub fn builder() -> RequestEntriesBuilder {
        <RequestEntriesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RequestEntriesBuilder {
    cache_id: Option<super::types::CacheId>,
    skip_count: Option<i64>,
    page_size: Option<i64>,
    path_filter: Option<String>,
}
impl RequestEntriesBuilder {
    pub fn cache_id(mut self, cache_id: impl Into<super::types::CacheId>) -> Self {
        self.cache_id = Some(cache_id.into());
        self
    }
    pub fn skip_count(mut self, skip_count: impl Into<i64>) -> Self {
        self.skip_count = Some(skip_count.into());
        self
    }
    pub fn page_size(mut self, page_size: impl Into<i64>) -> Self {
        self.page_size = Some(page_size.into());
        self
    }
    pub fn path_filter(mut self, path_filter: impl Into<String>) -> Self {
        self.path_filter = Some(path_filter.into());
        self
    }
    pub fn build(self) -> Result<RequestEntries, String> {
        Ok(RequestEntries {
            method: RequestEntriesMethod::RequestEntries,
            params: RequestEntriesParams {
                cache_id: self.cache_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(cache_id))
                })?,
                skip_count: self.skip_count,
                page_size: self.page_size,
                path_filter: self.path_filter,
            },
        })
    }
}
