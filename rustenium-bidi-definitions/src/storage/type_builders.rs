use super::types::*;
impl PartitionKey {
    pub fn builder() -> PartitionKeyBuilder {
        PartitionKeyBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PartitionKeyBuilder {
    user_context: Option<String>,
    source_origin: Option<String>,
    extensible: Option<std::collections::HashMap<String, serde_json::Value>>,
}
impl PartitionKeyBuilder {
    pub fn user_context(mut self, user_context: impl Into<String>) -> Self {
        self.user_context = Some(user_context.into());
        self
    }
    pub fn source_origin(mut self, source_origin: impl Into<String>) -> Self {
        self.source_origin = Some(source_origin.into());
        self
    }
    pub fn extensible(
        mut self,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.extensible = Some(extensible.into());
        self
    }
    pub fn build(self) -> Result<PartitionKey, String> {
        Ok(PartitionKey {
            user_context: self.user_context,
            source_origin: self.source_origin,
            extensible: self
                .extensible
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(extensible)))?,
        })
    }
}
impl CookieFilter {
    pub fn builder() -> CookieFilterBuilder {
        CookieFilterBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CookieFilterBuilder {
    name: Option<String>,
    value: Option<crate::network::types::BytesValue>,
    domain: Option<String>,
    path: Option<String>,
    size: Option<u64>,
    http_only: Option<bool>,
    secure: Option<bool>,
    same_site: Option<crate::network::types::SameSite>,
    expiry: Option<u64>,
    extensible: Option<std::collections::HashMap<String, serde_json::Value>>,
}
impl CookieFilterBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<crate::network::types::BytesValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
    pub fn size(mut self, size: impl Into<u64>) -> Self {
        self.size = Some(size.into());
        self
    }
    pub fn http_only(mut self, http_only: impl Into<bool>) -> Self {
        self.http_only = Some(http_only.into());
        self
    }
    pub fn secure(mut self, secure: impl Into<bool>) -> Self {
        self.secure = Some(secure.into());
        self
    }
    pub fn same_site(mut self, same_site: impl Into<crate::network::types::SameSite>) -> Self {
        self.same_site = Some(same_site.into());
        self
    }
    pub fn expiry(mut self, expiry: impl Into<u64>) -> Self {
        self.expiry = Some(expiry.into());
        self
    }
    pub fn extensible(
        mut self,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.extensible = Some(extensible.into());
        self
    }
    pub fn build(self) -> Result<CookieFilter, String> {
        Ok(CookieFilter {
            name: self.name,
            value: self.value,
            domain: self.domain,
            path: self.path,
            size: self.size,
            http_only: self.http_only,
            secure: self.secure,
            same_site: self.same_site,
            expiry: self.expiry,
            extensible: self
                .extensible
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(extensible)))?,
        })
    }
}
impl BrowsingContextPartitionDescriptor {
    pub fn builder() -> BrowsingContextPartitionDescriptorBuilder {
        BrowsingContextPartitionDescriptorBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct BrowsingContextPartitionDescriptorBuilder {
    r#type: Option<String>,
    context: Option<crate::browsing_context::types::BrowsingContext>,
}
impl BrowsingContextPartitionDescriptorBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn build(self) -> Result<BrowsingContextPartitionDescriptor, String> {
        Ok(BrowsingContextPartitionDescriptor {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            context: self
                .context
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
        })
    }
}
impl StorageKeyPartitionDescriptor {
    pub fn builder() -> StorageKeyPartitionDescriptorBuilder {
        StorageKeyPartitionDescriptorBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct StorageKeyPartitionDescriptorBuilder {
    r#type: Option<String>,
    user_context: Option<String>,
    source_origin: Option<String>,
    extensible: Option<std::collections::HashMap<String, serde_json::Value>>,
}
impl StorageKeyPartitionDescriptorBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn user_context(mut self, user_context: impl Into<String>) -> Self {
        self.user_context = Some(user_context.into());
        self
    }
    pub fn source_origin(mut self, source_origin: impl Into<String>) -> Self {
        self.source_origin = Some(source_origin.into());
        self
    }
    pub fn extensible(
        mut self,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.extensible = Some(extensible.into());
        self
    }
    pub fn build(self) -> Result<StorageKeyPartitionDescriptor, String> {
        Ok(StorageKeyPartitionDescriptor {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            user_context: self.user_context,
            source_origin: self.source_origin,
            extensible: self
                .extensible
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(extensible)))?,
        })
    }
}
impl PartialCookie {
    pub fn builder() -> PartialCookieBuilder {
        PartialCookieBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PartialCookieBuilder {
    name: Option<String>,
    value: Option<crate::network::types::BytesValue>,
    domain: Option<String>,
    path: Option<String>,
    http_only: Option<bool>,
    secure: Option<bool>,
    same_site: Option<crate::network::types::SameSite>,
    expiry: Option<u64>,
    extensible: Option<std::collections::HashMap<String, serde_json::Value>>,
}
impl PartialCookieBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<crate::network::types::BytesValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
    pub fn http_only(mut self, http_only: impl Into<bool>) -> Self {
        self.http_only = Some(http_only.into());
        self
    }
    pub fn secure(mut self, secure: impl Into<bool>) -> Self {
        self.secure = Some(secure.into());
        self
    }
    pub fn same_site(mut self, same_site: impl Into<crate::network::types::SameSite>) -> Self {
        self.same_site = Some(same_site.into());
        self
    }
    pub fn expiry(mut self, expiry: impl Into<u64>) -> Self {
        self.expiry = Some(expiry.into());
        self
    }
    pub fn extensible(
        mut self,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.extensible = Some(extensible.into());
        self
    }
    pub fn build(self) -> Result<PartialCookie, String> {
        Ok(PartialCookie {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            domain: self
                .domain
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(domain)))?,
            path: self.path,
            http_only: self.http_only,
            secure: self.secure,
            same_site: self.same_site,
            expiry: self.expiry,
            extensible: self
                .extensible
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(extensible)))?,
        })
    }
}
