use serde::{Deserialize, Serialize};
#[doc = "Disables inspector domain notifications.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Inspector.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "Inspector.disable";
}
#[doc = "Disables inspector domain notifications.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
#[doc = "Enables inspector domain notifications.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Inspector.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "Inspector.enable";
}
#[doc = "Enables inspector domain notifications.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
group_enum ! (InspectorCommands { Disable (Disable) , Enable (Enable) });
