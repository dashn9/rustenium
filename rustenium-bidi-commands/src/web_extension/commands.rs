// Generated commands for module

use serde::{Serialize, Deserialize};
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebExtensionCommand {
    Install(Install),
    Uninstall(Uninstall),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebExtensionInstallMethod {
    #[serde(rename = "webExtension.Install")]
    WebExtensionInstall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebExtensionUninstallMethod {
    #[serde(rename = "webExtension.Uninstall")]
    WebExtensionUninstall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallParameters {
    #[serde(rename = "extensionData")]
    pub extension_data: ExtensionData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UninstallParameters {
    #[serde(rename = "extension")]
    pub extension: Extension,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Install {
    #[serde(rename = "method")]
    pub method: WebExtensionInstallMethod,
    #[serde(rename = "params")]
    pub params: InstallParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uninstall {
    #[serde(rename = "method")]
    pub method: WebExtensionUninstallMethod,
    #[serde(rename = "params")]
    pub params: UninstallParameters,
}

// Generated results

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebExtensionResult {
    InstallResult(InstallResult),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallResult {
    #[serde(rename = "extension")]
    pub extension: Extension,
}

