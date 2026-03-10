use serde::{Deserialize, Serialize};
#[doc = "Enable events in this domain.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "DeviceAccess.enable")]
    Enable,
}
#[doc = "Enable events in this domain.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "DeviceAccess.enable";
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Disable events in this domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "DeviceAccess.disable")]
    Disable,
}
#[doc = "Disable events in this domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "DeviceAccess.disable";
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Select a device in response to a DeviceAccess.deviceRequestPrompted event.\n[selectPrompt](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#method-selectPrompt)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectPromptParams {
    #[serde(rename = "id")]
    pub id: super::types::RequestId,
    #[serde(rename = "deviceId")]
    pub device_id: super::types::DeviceId,
}
impl SelectPromptParams {
    pub fn new(
        id: impl Into<super::types::RequestId>,
        device_id: impl Into<super::types::DeviceId>,
    ) -> Self {
        Self {
            id: id.into(),
            device_id: device_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SelectPromptMethod {
    #[serde(rename = "DeviceAccess.selectPrompt")]
    SelectPrompt,
}
#[doc = "Select a device in response to a DeviceAccess.deviceRequestPrompted event.\n[selectPrompt](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#method-selectPrompt)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectPrompt {
    pub method: SelectPromptMethod,
    pub params: SelectPromptParams,
}
impl SelectPrompt {
    pub const IDENTIFIER: &'static str = "DeviceAccess.selectPrompt";
}
impl crate::CommandResult for SelectPrompt {
    type Result = super::results::SelectPromptResult;
}
#[doc = "Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event.\n[cancelPrompt](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#method-cancelPrompt)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelPromptParams {
    #[serde(rename = "id")]
    pub id: super::types::RequestId,
}
impl CancelPromptParams {
    pub fn new(id: impl Into<super::types::RequestId>) -> Self {
        Self { id: id.into() }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CancelPromptMethod {
    #[serde(rename = "DeviceAccess.cancelPrompt")]
    CancelPrompt,
}
#[doc = "Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event.\n[cancelPrompt](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#method-cancelPrompt)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelPrompt {
    pub method: CancelPromptMethod,
    pub params: CancelPromptParams,
}
impl CancelPrompt {
    pub const IDENTIFIER: &'static str = "DeviceAccess.cancelPrompt";
}
impl crate::CommandResult for CancelPrompt {
    type Result = super::results::CancelPromptResult;
}
group_enum ! (DeviceAccessCommands { Enable (Enable) , Disable (Disable) , SelectPrompt (SelectPrompt) , CancelPrompt (CancelPrompt) });
