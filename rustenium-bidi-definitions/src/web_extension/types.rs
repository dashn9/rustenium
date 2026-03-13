use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct Extension(String);
impl Extension {
    pub fn new(val: impl Into<String>) -> Self {
        Extension(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for Extension {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<Extension> for String {
    fn from(el: Extension) -> String {
        el.0
    }
}
impl From<String> for Extension {
    fn from(expr: String) -> Self {
        Extension(expr)
    }
}
impl Extension {
    pub const IDENTIFIER: &'static str = "webExtension.Extension";
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtensionData {
    ExtensionArchivePath(ExtensionArchivePath),
    ExtensionBase64Encoded(ExtensionBase64Encoded),
    ExtensionPath(ExtensionPath),
}
impl From<ExtensionArchivePath> for ExtensionData {
    fn from(v: ExtensionArchivePath) -> Self {
        ExtensionData::ExtensionArchivePath(v)
    }
}
impl TryFrom<ExtensionData> for ExtensionArchivePath {
    type Error = ExtensionData;
    fn try_from(e: ExtensionData) -> Result<Self, Self::Error> {
        match e {
            ExtensionData::ExtensionArchivePath(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<ExtensionBase64Encoded> for ExtensionData {
    fn from(v: ExtensionBase64Encoded) -> Self {
        ExtensionData::ExtensionBase64Encoded(v)
    }
}
impl TryFrom<ExtensionData> for ExtensionBase64Encoded {
    type Error = ExtensionData;
    fn try_from(e: ExtensionData) -> Result<Self, Self::Error> {
        match e {
            ExtensionData::ExtensionBase64Encoded(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<ExtensionPath> for ExtensionData {
    fn from(v: ExtensionPath) -> Self {
        ExtensionData::ExtensionPath(v)
    }
}
impl TryFrom<ExtensionData> for ExtensionPath {
    type Error = ExtensionData;
    fn try_from(e: ExtensionData) -> Result<Self, Self::Error> {
        match e {
            ExtensionData::ExtensionPath(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionPath {
    #[serde(rename = "type")]
    pub r#type: ExtensionPathType,
    #[serde(rename = "path")]
    pub path: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExtensionPathType {
    #[serde(rename = "path")]
    Path,
}
impl ExtensionPath {
    pub fn new(r#type: impl Into<ExtensionPathType>, path: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            path: path.into(),
        }
    }
}
impl ExtensionPath {
    pub const IDENTIFIER: &'static str = "webExtension.ExtensionPath";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionArchivePath {
    #[serde(rename = "type")]
    pub r#type: ExtensionArchivePathType,
    #[serde(rename = "path")]
    pub path: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExtensionArchivePathType {
    #[serde(rename = "archivePath")]
    ArchivePath,
}
impl ExtensionArchivePath {
    pub fn new(r#type: impl Into<ExtensionArchivePathType>, path: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            path: path.into(),
        }
    }
}
impl ExtensionArchivePath {
    pub const IDENTIFIER: &'static str = "webExtension.ExtensionArchivePath";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionBase64Encoded {
    #[serde(rename = "type")]
    pub r#type: ExtensionBase64EncodedType,
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExtensionBase64EncodedType {
    #[serde(rename = "base64")]
    Base64,
}
impl ExtensionBase64Encoded {
    pub fn new(r#type: impl Into<ExtensionBase64EncodedType>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl ExtensionBase64Encoded {
    pub const IDENTIFIER: &'static str = "webExtension.ExtensionBase64Encoded";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (WebExtensionType { Extension (Extension) , ExtensionData (ExtensionData) });
