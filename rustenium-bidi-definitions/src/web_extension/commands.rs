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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Install {
    pub method: InstallMethod,
    pub params: InstallParams,
}
impl Install {
    pub const IDENTIFIER: &'static str = "webExtension.install";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uninstall {
    pub method: UninstallMethod,
    pub params: UninstallParams,
}
impl Uninstall {
    pub const IDENTIFIER: &'static str = "webExtension.uninstall";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Uninstall {
    type Result = super::results::UninstallResult;
}
group_enum ! (WebExtensionCommand { Install (Install) , Uninstall (Uninstall) } + identifiable);
