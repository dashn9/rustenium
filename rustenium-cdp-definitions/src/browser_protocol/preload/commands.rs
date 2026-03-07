use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Preload.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "Preload.enable";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Preload.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "Preload.disable";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
group_enum ! (PreloadCommands { Enable (Enable) , Disable (Disable) });
