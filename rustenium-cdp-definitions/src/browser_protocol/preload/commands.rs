use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Preload.enable")]
    Enable,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "Preload.enable";
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "Preload.disable";
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
group_enum ! (PreloadCommands { Enable (Enable) , Disable (Disable) });
