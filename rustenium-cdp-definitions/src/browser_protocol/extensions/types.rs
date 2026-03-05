use serde::{Deserialize, Serialize};
#[doc = "Storage areas."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageArea {
    #[serde(rename = "session")]
    Session,
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "sync")]
    Sync,
    #[serde(rename = "managed")]
    Managed,
}
#[doc = "Detailed information about an extension.\n[ExtensionInfo](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#type-ExtensionInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionInfo {
    #[doc = "Extension id."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "Extension name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Extension version."]
    #[serde(rename = "version")]
    pub version: String,
    #[doc = "The path from which the extension was loaded."]
    #[serde(rename = "path")]
    pub path: String,
    #[doc = "Extension enabled status."]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
impl ExtensionInfo {
    pub const IDENTIFIER: &'static str = "Extensions.ExtensionInfo";
}
group_enum ! (ExtensionsTypes { StorageArea (StorageArea) , ExtensionInfo (ExtensionInfo) });
