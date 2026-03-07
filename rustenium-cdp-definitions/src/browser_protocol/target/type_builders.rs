use super::types::*;
impl TargetInfo {
    pub fn builder() -> TargetInfoBuilder {
        <TargetInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct TargetInfoBuilder {
    target_id: Option<TargetId>,
    r#type: Option<String>,
    title: Option<String>,
    url: Option<String>,
    attached: Option<bool>,
    opener_id: Option<TargetId>,
    can_access_opener: Option<bool>,
    opener_frame_id: Option<crate::browser_protocol::page::types::FrameId>,
    parent_frame_id: Option<crate::browser_protocol::page::types::FrameId>,
    browser_context_id: Option<crate::browser_protocol::browser::types::BrowserContextId>,
    subtype: Option<String>,
}
impl TargetInfoBuilder {
    pub fn target_id(mut self, target_id: impl Into<TargetId>) -> Self {
        self.target_id = Some(target_id.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn attached(mut self, attached: impl Into<bool>) -> Self {
        self.attached = Some(attached.into());
        self
    }
    pub fn opener_id(mut self, opener_id: impl Into<TargetId>) -> Self {
        self.opener_id = Some(opener_id.into());
        self
    }
    pub fn can_access_opener(mut self, can_access_opener: impl Into<bool>) -> Self {
        self.can_access_opener = Some(can_access_opener.into());
        self
    }
    pub fn opener_frame_id(
        mut self,
        opener_frame_id: impl Into<crate::browser_protocol::page::types::FrameId>,
    ) -> Self {
        self.opener_frame_id = Some(opener_frame_id.into());
        self
    }
    pub fn parent_frame_id(
        mut self,
        parent_frame_id: impl Into<crate::browser_protocol::page::types::FrameId>,
    ) -> Self {
        self.parent_frame_id = Some(parent_frame_id.into());
        self
    }
    pub fn browser_context_id(
        mut self,
        browser_context_id: impl Into<crate::browser_protocol::browser::types::BrowserContextId>,
    ) -> Self {
        self.browser_context_id = Some(browser_context_id.into());
        self
    }
    pub fn subtype(mut self, subtype: impl Into<String>) -> Self {
        self.subtype = Some(subtype.into());
        self
    }
    pub fn build(self) -> Result<TargetInfo, String> {
        Ok(TargetInfo {
            target_id: self
                .target_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(target_id)))?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            title: self
                .title
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(title)))?,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            attached: self
                .attached
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(attached)))?,
            opener_id: self.opener_id,
            can_access_opener: self.can_access_opener.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(can_access_opener)
                )
            })?,
            opener_frame_id: self.opener_frame_id,
            parent_frame_id: self.parent_frame_id,
            browser_context_id: self.browser_context_id,
            subtype: self.subtype,
        })
    }
}
impl FilterEntry {
    pub fn builder() -> FilterEntryBuilder {
        <FilterEntryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct FilterEntryBuilder {
    exclude: Option<bool>,
    r#type: Option<String>,
}
impl FilterEntryBuilder {
    pub fn exclude(mut self, exclude: impl Into<bool>) -> Self {
        self.exclude = Some(exclude.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> FilterEntry {
        FilterEntry {
            exclude: self.exclude,
            r#type: self.r#type,
        }
    }
}
impl RemoteLocation {
    pub fn builder() -> RemoteLocationBuilder {
        <RemoteLocationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoteLocationBuilder {
    host: Option<String>,
    port: Option<i64>,
}
impl RemoteLocationBuilder {
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }
    pub fn port(mut self, port: impl Into<i64>) -> Self {
        self.port = Some(port.into());
        self
    }
    pub fn build(self) -> Result<RemoteLocation, String> {
        Ok(RemoteLocation {
            host: self
                .host
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(host)))?,
            port: self
                .port
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(port)))?,
        })
    }
}
