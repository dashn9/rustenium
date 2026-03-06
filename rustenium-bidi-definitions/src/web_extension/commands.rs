use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstallParams {
    #[serde(rename = "extensionData")]
    pub extension_data: super::types::ExtensionData,
}
impl InstallParams {
    pub fn new(extension_data: impl Into<super::types::ExtensionData>) -> Self {
        Self {
            extension_data: extension_data.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InstallMethod {
    #[serde(rename = "webExtension.install")]
    Install,
}
impl InstallMethod {
    pub const IDENTIFIER: &'static str = "webExtension.install";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Install {
    pub method: InstallMethod,
    pub params: InstallParams,
}
impl crate::CommandResult for Install {
    type Result = super::results::InstallResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UninstallParams {
    #[serde(rename = "extension")]
    pub extension: super::types::Extension,
}
impl UninstallParams {
    pub fn new(extension: impl Into<super::types::Extension>) -> Self {
        Self {
            extension: extension.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UninstallMethod {
    #[serde(rename = "webExtension.uninstall")]
    Uninstall,
}
impl UninstallMethod {
    pub const IDENTIFIER: &'static str = "webExtension.uninstall";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Uninstall {
    pub method: UninstallMethod,
    pub params: UninstallParams,
}
impl crate::CommandResult for Uninstall {
    type Result = super::results::UninstallResult;
}
group_enum ! (WebExtensionCommands { Install (Install) , Uninstall (Uninstall) });
