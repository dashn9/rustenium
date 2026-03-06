use super::types::*;
impl ExtensionPath {
    pub fn builder() -> ExtensionPathBuilder {
        ExtensionPathBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ExtensionPathBuilder {
    r#type: Option<String>,
    path: Option<String>,
}
impl ExtensionPathBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
    pub fn build(self) -> Result<ExtensionPath, String> {
        Ok(ExtensionPath {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            path: self
                .path
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(path)))?,
        })
    }
}
impl ExtensionArchivePath {
    pub fn builder() -> ExtensionArchivePathBuilder {
        ExtensionArchivePathBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ExtensionArchivePathBuilder {
    r#type: Option<String>,
    path: Option<String>,
}
impl ExtensionArchivePathBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
    pub fn build(self) -> Result<ExtensionArchivePath, String> {
        Ok(ExtensionArchivePath {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            path: self
                .path
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(path)))?,
        })
    }
}
impl ExtensionBase64Encoded {
    pub fn builder() -> ExtensionBase64EncodedBuilder {
        ExtensionBase64EncodedBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ExtensionBase64EncodedBuilder {
    r#type: Option<String>,
    value: Option<String>,
}
impl ExtensionBase64EncodedBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<ExtensionBase64Encoded, String> {
        Ok(ExtensionBase64Encoded {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
