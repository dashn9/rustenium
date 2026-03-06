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
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtensionData {
    ExtensionArchivePath(ExtensionArchivePath),
    ExtensionBase64Encoded(ExtensionBase64Encoded),
    ExtensionPath(ExtensionPath),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionPath {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "path")]
    pub path: String,
}
impl ExtensionPath {
    pub fn new(r#type: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            path: path.into(),
        }
    }
}
impl ExtensionPath {
    pub const IDENTIFIER: &'static str = "webExtension.ExtensionPath";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionArchivePath {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "path")]
    pub path: String,
}
impl ExtensionArchivePath {
    pub fn new(r#type: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            path: path.into(),
        }
    }
}
impl ExtensionArchivePath {
    pub const IDENTIFIER: &'static str = "webExtension.ExtensionArchivePath";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionBase64Encoded {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl ExtensionBase64Encoded {
    pub fn new(r#type: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl ExtensionBase64Encoded {
    pub const IDENTIFIER: &'static str = "webExtension.ExtensionBase64Encoded";
}
group_enum ! (WebExtensionTypes { Extension (Extension) , ExtensionData (ExtensionData) , ExtensionPath (ExtensionPath) , ExtensionArchivePath (ExtensionArchivePath) , ExtensionBase64Encoded (ExtensionBase64Encoded) });
