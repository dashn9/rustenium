use serde::{Deserialize, Serialize};
#[doc = "Enable events in this domain.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "DeviceAccess.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "DeviceAccess.enable";
}
#[doc = "Enable events in this domain.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
#[doc = "Disable events in this domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "DeviceAccess.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "DeviceAccess.disable";
}
#[doc = "Disable events in this domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
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
impl SelectPromptMethod {
    pub const IDENTIFIER: &'static str = "DeviceAccess.selectPrompt";
}
#[doc = "Select a device in response to a DeviceAccess.deviceRequestPrompted event.\n[selectPrompt](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#method-selectPrompt)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SelectPrompt {
    pub method: SelectPromptMethod,
    pub params: SelectPromptParams,
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
impl CancelPromptMethod {
    pub const IDENTIFIER: &'static str = "DeviceAccess.cancelPrompt";
}
#[doc = "Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event.\n[cancelPrompt](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#method-cancelPrompt)"]
#[derive(Debug, Clone, PartialEq)]
pub struct CancelPrompt {
    pub method: CancelPromptMethod,
    pub params: CancelPromptParams,
}
group_enum ! (Command { Enable (Enable) , Disable (Disable) , SelectPrompt (SelectPrompt) , CancelPrompt (CancelPrompt) });
