use serde::{Deserialize, Serialize};
#[doc = "Runs an extension default action.\nAvailable if the client is connected using the --remote-debugging-pipe\nflag and the --enable-unsafe-extension-debugging flag is set.\n[triggerAction](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-triggerAction)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerActionParams {
    #[doc = "Extension id."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "A tab target ID to trigger the default extension action on."]
    #[serde(rename = "targetId")]
    pub target_id: String,
}
impl TriggerActionParams {
    pub fn new(id: impl Into<String>, target_id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            target_id: target_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriggerActionMethod {
    #[serde(rename = "Extensions.triggerAction")]
    TriggerAction,
}
#[doc = "Runs an extension default action.\nAvailable if the client is connected using the --remote-debugging-pipe\nflag and the --enable-unsafe-extension-debugging flag is set.\n[triggerAction](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-triggerAction)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerAction {
    pub method: TriggerActionMethod,
    pub params: TriggerActionParams,
}
impl TriggerAction {
    pub const IDENTIFIER: &'static str = "Extensions.triggerAction";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for TriggerAction {
    type Result = super::results::TriggerActionResult;
}
#[doc = "Installs an unpacked extension from the filesystem similar to\n--load-extension CLI flags. Returns extension ID once the extension\nhas been installed. Available if the client is connected using the\n--remote-debugging-pipe flag and the --enable-unsafe-extension-debugging\nflag is set.\n[loadUnpacked](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-loadUnpacked)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadUnpackedParams {
    #[doc = "Absolute file path."]
    #[serde(rename = "path")]
    pub path: String,
    #[doc = "Enable the extension in incognito"]
    #[serde(rename = "enableInIncognito")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enable_in_incognito: Option<bool>,
}
impl LoadUnpackedParams {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            enable_in_incognito: None,
        }
    }
}
impl<T: Into<String>> From<T> for LoadUnpackedParams {
    fn from(url: T) -> Self {
        LoadUnpackedParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoadUnpackedMethod {
    #[serde(rename = "Extensions.loadUnpacked")]
    LoadUnpacked,
}
#[doc = "Installs an unpacked extension from the filesystem similar to\n--load-extension CLI flags. Returns extension ID once the extension\nhas been installed. Available if the client is connected using the\n--remote-debugging-pipe flag and the --enable-unsafe-extension-debugging\nflag is set.\n[loadUnpacked](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-loadUnpacked)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadUnpacked {
    pub method: LoadUnpackedMethod,
    pub params: LoadUnpackedParams,
}
impl LoadUnpacked {
    pub const IDENTIFIER: &'static str = "Extensions.loadUnpacked";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for LoadUnpacked {
    type Result = super::results::LoadUnpackedResult;
}
#[doc = "Gets a list of all unpacked extensions.\nAvailable if the client is connected using the --remote-debugging-pipe flag\nand the --enable-unsafe-extension-debugging flag is set.\n[getExtensions](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-getExtensions)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetExtensionsParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetExtensionsMethod {
    #[serde(rename = "Extensions.getExtensions")]
    GetExtensions,
}
#[doc = "Gets a list of all unpacked extensions.\nAvailable if the client is connected using the --remote-debugging-pipe flag\nand the --enable-unsafe-extension-debugging flag is set.\n[getExtensions](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-getExtensions)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetExtensions {
    pub method: GetExtensionsMethod,
    pub params: GetExtensionsParams,
}
impl GetExtensions {
    pub const IDENTIFIER: &'static str = "Extensions.getExtensions";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetExtensions {
    type Result = super::results::GetExtensionsResult;
}
#[doc = "Uninstalls an unpacked extension (others not supported) from the profile.\nAvailable if the client is connected using the --remote-debugging-pipe flag\nand the --enable-unsafe-extension-debugging.\n[uninstall](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-uninstall)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UninstallParams {
    #[doc = "Extension id."]
    #[serde(rename = "id")]
    pub id: String,
}
impl UninstallParams {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}
impl<T: Into<String>> From<T> for UninstallParams {
    fn from(url: T) -> Self {
        UninstallParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UninstallMethod {
    #[serde(rename = "Extensions.uninstall")]
    Uninstall,
}
#[doc = "Uninstalls an unpacked extension (others not supported) from the profile.\nAvailable if the client is connected using the --remote-debugging-pipe flag\nand the --enable-unsafe-extension-debugging.\n[uninstall](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-uninstall)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uninstall {
    pub method: UninstallMethod,
    pub params: UninstallParams,
}
impl Uninstall {
    pub const IDENTIFIER: &'static str = "Extensions.uninstall";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Uninstall {
    type Result = super::results::UninstallResult;
}
#[doc = "Gets data from extension storage in the given `storageArea`. If `keys` is\nspecified, these are used to filter the result.\n[getStorageItems](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-getStorageItems)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStorageItemsParams {
    #[doc = "ID of extension."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "StorageArea to retrieve data from."]
    #[serde(rename = "storageArea")]
    pub storage_area: super::types::StorageArea,
    #[doc = "Keys to retrieve."]
    #[serde(rename = "keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub keys: Option<Vec<String>>,
}
impl GetStorageItemsParams {
    pub fn new(id: impl Into<String>, storage_area: impl Into<super::types::StorageArea>) -> Self {
        Self {
            id: id.into(),
            storage_area: storage_area.into(),
            keys: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetStorageItemsMethod {
    #[serde(rename = "Extensions.getStorageItems")]
    GetStorageItems,
}
#[doc = "Gets data from extension storage in the given `storageArea`. If `keys` is\nspecified, these are used to filter the result.\n[getStorageItems](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-getStorageItems)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStorageItems {
    pub method: GetStorageItemsMethod,
    pub params: GetStorageItemsParams,
}
impl GetStorageItems {
    pub const IDENTIFIER: &'static str = "Extensions.getStorageItems";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetStorageItems {
    type Result = super::results::GetStorageItemsResult;
}
#[doc = "Removes `keys` from extension storage in the given `storageArea`.\n[removeStorageItems](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-removeStorageItems)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveStorageItemsParams {
    #[doc = "ID of extension."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "StorageArea to remove data from."]
    #[serde(rename = "storageArea")]
    pub storage_area: super::types::StorageArea,
    #[doc = "Keys to remove."]
    #[serde(rename = "keys")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<String>,
}
impl RemoveStorageItemsParams {
    pub fn new(
        id: impl Into<String>,
        storage_area: impl Into<super::types::StorageArea>,
        keys: Vec<String>,
    ) -> Self {
        Self {
            id: id.into(),
            storage_area: storage_area.into(),
            keys,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveStorageItemsMethod {
    #[serde(rename = "Extensions.removeStorageItems")]
    RemoveStorageItems,
}
#[doc = "Removes `keys` from extension storage in the given `storageArea`.\n[removeStorageItems](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-removeStorageItems)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveStorageItems {
    pub method: RemoveStorageItemsMethod,
    pub params: RemoveStorageItemsParams,
}
impl RemoveStorageItems {
    pub const IDENTIFIER: &'static str = "Extensions.removeStorageItems";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for RemoveStorageItems {
    type Result = super::results::RemoveStorageItemsResult;
}
#[doc = "Clears extension storage in the given `storageArea`.\n[clearStorageItems](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-clearStorageItems)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearStorageItemsParams {
    #[doc = "ID of extension."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "StorageArea to remove data from."]
    #[serde(rename = "storageArea")]
    pub storage_area: super::types::StorageArea,
}
impl ClearStorageItemsParams {
    pub fn new(id: impl Into<String>, storage_area: impl Into<super::types::StorageArea>) -> Self {
        Self {
            id: id.into(),
            storage_area: storage_area.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearStorageItemsMethod {
    #[serde(rename = "Extensions.clearStorageItems")]
    ClearStorageItems,
}
#[doc = "Clears extension storage in the given `storageArea`.\n[clearStorageItems](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-clearStorageItems)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearStorageItems {
    pub method: ClearStorageItemsMethod,
    pub params: ClearStorageItemsParams,
}
impl ClearStorageItems {
    pub const IDENTIFIER: &'static str = "Extensions.clearStorageItems";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ClearStorageItems {
    type Result = super::results::ClearStorageItemsResult;
}
#[doc = "Sets `values` in extension storage in the given `storageArea`. The provided `values`\nwill be merged with existing values in the storage area.\n[setStorageItems](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-setStorageItems)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStorageItemsParams {
    #[doc = "ID of extension."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "StorageArea to set data in."]
    #[serde(rename = "storageArea")]
    pub storage_area: super::types::StorageArea,
    #[doc = "Values to set."]
    #[serde(rename = "values")]
    pub values: serde_json::Value,
}
impl SetStorageItemsParams {
    pub fn new(
        id: impl Into<String>,
        storage_area: impl Into<super::types::StorageArea>,
        values: impl Into<serde_json::Value>,
    ) -> Self {
        Self {
            id: id.into(),
            storage_area: storage_area.into(),
            values: values.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetStorageItemsMethod {
    #[serde(rename = "Extensions.setStorageItems")]
    SetStorageItems,
}
#[doc = "Sets `values` in extension storage in the given `storageArea`. The provided `values`\nwill be merged with existing values in the storage area.\n[setStorageItems](https://chromedevtools.github.io/devtools-protocol/tot/Extensions/#method-setStorageItems)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStorageItems {
    pub method: SetStorageItemsMethod,
    pub params: SetStorageItemsParams,
}
impl SetStorageItems {
    pub const IDENTIFIER: &'static str = "Extensions.setStorageItems";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetStorageItems {
    type Result = super::results::SetStorageItemsResult;
}
group_enum ! (ExtensionsCommands { TriggerAction (TriggerAction) , LoadUnpacked (LoadUnpacked) , GetExtensions (GetExtensions) , Uninstall (Uninstall) , GetStorageItems (GetStorageItems) , RemoveStorageItems (RemoveStorageItems) , ClearStorageItems (ClearStorageItems) , SetStorageItems (SetStorageItems) });
