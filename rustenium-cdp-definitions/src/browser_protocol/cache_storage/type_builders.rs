use super::types::*;
impl DataEntry {
    pub fn builder() -> DataEntryBuilder {
        DataEntryBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DataEntryBuilder {
    request_url: Option<String>,
    request_method: Option<String>,
    request_headers: Option<Vec<Header>>,
    response_time: Option<f64>,
    response_status: Option<i64>,
    response_status_text: Option<String>,
    response_type: Option<CachedResponseType>,
    response_headers: Option<Vec<Header>>,
}
impl DataEntryBuilder {
    pub fn request_url(mut self, request_url: impl Into<String>) -> Self {
        self.request_url = Some(request_url.into());
        self
    }
    pub fn request_method(mut self, request_method: impl Into<String>) -> Self {
        self.request_method = Some(request_method.into());
        self
    }
    pub fn request_header(mut self, request_header: impl Into<Header>) -> Self {
        let v = self.request_headers.get_or_insert(Vec::new());
        v.push(request_header.into());
        self
    }
    pub fn request_headers<I, S>(mut self, request_headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Header>,
    {
        let v = self.request_headers.get_or_insert(Vec::new());
        for val in request_headers {
            v.push(val.into());
        }
        self
    }
    pub fn response_time(mut self, response_time: impl Into<f64>) -> Self {
        self.response_time = Some(response_time.into());
        self
    }
    pub fn response_status(mut self, response_status: impl Into<i64>) -> Self {
        self.response_status = Some(response_status.into());
        self
    }
    pub fn response_status_text(mut self, response_status_text: impl Into<String>) -> Self {
        self.response_status_text = Some(response_status_text.into());
        self
    }
    pub fn response_type(mut self, response_type: impl Into<CachedResponseType>) -> Self {
        self.response_type = Some(response_type.into());
        self
    }
    pub fn response_header(mut self, response_header: impl Into<Header>) -> Self {
        let v = self.response_headers.get_or_insert(Vec::new());
        v.push(response_header.into());
        self
    }
    pub fn response_headers<I, S>(mut self, response_headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Header>,
    {
        let v = self.response_headers.get_or_insert(Vec::new());
        for val in response_headers {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<DataEntry, String> {
        Ok(DataEntry {
            request_url: self
                .request_url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request_url)))?,
            request_method: self.request_method.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(request_method))
            })?,
            request_headers: self.request_headers.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(request_headers))
            })?,
            response_time: self.response_time.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(response_time))
            })?,
            response_status: self.response_status.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(response_status))
            })?,
            response_status_text: self.response_status_text.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(response_status_text)
                )
            })?,
            response_type: self.response_type.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(response_type))
            })?,
            response_headers: self.response_headers.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(response_headers)
                )
            })?,
        })
    }
}
impl Cache {
    pub fn builder() -> CacheBuilder {
        CacheBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CacheBuilder {
    cache_id: Option<CacheId>,
    security_origin: Option<String>,
    storage_key: Option<String>,
    storage_bucket: Option<super::super::storage::types::StorageBucket>,
    cache_name: Option<String>,
}
impl CacheBuilder {
    pub fn cache_id(mut self, cache_id: impl Into<CacheId>) -> Self {
        self.cache_id = Some(cache_id.into());
        self
    }
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
        storage_bucket: impl Into<super::super::storage::types::StorageBucket>,
    ) -> Self {
        self.storage_bucket = Some(storage_bucket.into());
        self
    }
    pub fn cache_name(mut self, cache_name: impl Into<String>) -> Self {
        self.cache_name = Some(cache_name.into());
        self
    }
    pub fn build(self) -> Result<Cache, String> {
        Ok(Cache {
            cache_id: self
                .cache_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cache_id)))?,
            security_origin: self.security_origin.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(security_origin))
            })?,
            storage_key: self
                .storage_key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(storage_key)))?,
            storage_bucket: self.storage_bucket,
            cache_name: self
                .cache_name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cache_name)))?,
        })
    }
}
impl Header {
    pub fn builder() -> HeaderBuilder {
        HeaderBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct HeaderBuilder {
    name: Option<String>,
    value: Option<String>,
}
impl HeaderBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<Header, String> {
        Ok(Header {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl CachedResponse {
    pub fn builder() -> CachedResponseBuilder {
        CachedResponseBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CachedResponseBuilder {
    body: Option<super::super::super::Binary>,
}
impl CachedResponseBuilder {
    pub fn body(mut self, body: impl Into<super::super::super::Binary>) -> Self {
        self.body = Some(body.into());
        self
    }
    pub fn build(self) -> Result<CachedResponse, String> {
        Ok(CachedResponse {
            body: self
                .body
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(body)))?,
        })
    }
}
