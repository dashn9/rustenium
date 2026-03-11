use serde::{Deserialize, Serialize};
#[doc = "Enables the Media domain\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Media/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Media.enable")]
    Enable,
}
#[doc = "Enables the Media domain\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Media/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "Media.enable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Disables the Media domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Media/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Media.disable")]
    Disable,
}
#[doc = "Disables the Media domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Media/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "Media.disable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
group_enum ! (MediaCommands { Enable (Enable) , Disable (Disable) } + identifiable);
