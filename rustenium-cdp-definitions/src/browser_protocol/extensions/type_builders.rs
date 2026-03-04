use super::types::*;
impl ExtensionInfo {
    pub fn builder() -> ExtensionInfoBuilder {
        ExtensionInfoBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ExtensionInfoBuilder {
    id: Option<String>,
    name: Option<String>,
    version: Option<String>,
    path: Option<String>,
    enabled: Option<bool>,
}
impl ExtensionInfoBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn build(self) -> Result<ExtensionInfo, String> {
        Ok(ExtensionInfo {
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            version: self
                .version
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(version)))?,
            path: self
                .path
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(path)))?,
            enabled: self
                .enabled
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
        })
    }
}
