use serde::{Deserialize, Serialize};
#[doc = "A device request opened a user prompt to select a device. Respond with the\nselectPrompt or cancelPrompt command.\n[deviceRequestPrompted](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#event-deviceRequestPrompted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceRequestPromptedParams {
    #[serde(rename = "id")]
    pub id: super::types::RequestId,
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<super::types::PromptDevice>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeviceRequestPromptedMethod {
    #[serde(rename = "DeviceAccess.deviceRequestPrompted")]
    DeviceRequestPrompted,
}
#[doc = "A device request opened a user prompt to select a device. Respond with the\nselectPrompt or cancelPrompt command.\n[deviceRequestPrompted](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#event-deviceRequestPrompted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceRequestPrompted {
    pub method: DeviceRequestPromptedMethod,
    pub params: DeviceRequestPromptedParams,
}
impl DeviceRequestPrompted {
    pub const IDENTIFIER: &'static str = "DeviceAccess.deviceRequestPrompted";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (DeviceAccessEvents { DeviceRequestPrompted (DeviceRequestPrompted) } + identifiable);
