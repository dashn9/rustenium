use serde::{Deserialize, Serialize};
#[doc = "A device request opened a user prompt to select a device. Respond with the\nselectPrompt or cancelPrompt command.\n[deviceRequestPrompted](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#event-deviceRequestPrompted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceRequestPrompted {
    #[serde(rename = "id")]
    pub id: super::types::RequestId,
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<super::types::PromptDevice>,
}
impl DeviceRequestPrompted {
    pub const IDENTIFIER: &'static str = "DeviceAccess.deviceRequestPrompted";
}
group_enum ! (DeviceAccessEvents { DeviceRequestPrompted (DeviceRequestPrompted) });
