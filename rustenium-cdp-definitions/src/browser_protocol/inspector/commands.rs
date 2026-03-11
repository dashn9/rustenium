use serde::{Deserialize, Serialize};
#[doc = "Disables inspector domain notifications.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Inspector.disable")]
    Disable,
}
#[doc = "Disables inspector domain notifications.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "Inspector.disable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enables inspector domain notifications.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Inspector.enable")]
    Enable,
}
#[doc = "Enables inspector domain notifications.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "Inspector.enable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
group_enum ! (InspectorCommands { Disable (Disable) , Enable (Enable) } + identifiable);
