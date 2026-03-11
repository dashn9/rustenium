use serde::{Deserialize, Serialize};
#[doc = "Request browser port binding.\n[bind](https://chromedevtools.github.io/devtools-protocol/tot/Tethering/#method-bind)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BindParams {
    #[doc = "Port number to bind."]
    #[serde(rename = "port")]
    pub port: i64,
}
impl BindParams {
    pub fn new(port: impl Into<i64>) -> Self {
        Self { port: port.into() }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BindMethod {
    #[serde(rename = "Tethering.bind")]
    Bind,
}
#[doc = "Request browser port binding.\n[bind](https://chromedevtools.github.io/devtools-protocol/tot/Tethering/#method-bind)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bind {
    pub method: BindMethod,
    pub params: BindParams,
}
impl Bind {
    pub const IDENTIFIER: &'static str = "Tethering.bind";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Bind {
    type Result = super::results::BindResult;
}
#[doc = "Request browser port unbinding.\n[unbind](https://chromedevtools.github.io/devtools-protocol/tot/Tethering/#method-unbind)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnbindParams {
    #[doc = "Port number to unbind."]
    #[serde(rename = "port")]
    pub port: i64,
}
impl UnbindParams {
    pub fn new(port: impl Into<i64>) -> Self {
        Self { port: port.into() }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnbindMethod {
    #[serde(rename = "Tethering.unbind")]
    Unbind,
}
#[doc = "Request browser port unbinding.\n[unbind](https://chromedevtools.github.io/devtools-protocol/tot/Tethering/#method-unbind)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Unbind {
    pub method: UnbindMethod,
    pub params: UnbindParams,
}
impl Unbind {
    pub const IDENTIFIER: &'static str = "Tethering.unbind";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Unbind {
    type Result = super::results::UnbindResult;
}
group_enum ! (TetheringCommands { Bind (Bind) , Unbind (Unbind) } + identifiable);
