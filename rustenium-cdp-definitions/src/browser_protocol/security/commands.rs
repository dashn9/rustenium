use serde::{Deserialize, Serialize};
#[doc = "Disables tracking security state changes.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Security/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Security.disable")]
    Disable,
}
#[doc = "Disables tracking security state changes.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Security/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "Security.disable";
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enables tracking security state changes.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Security/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Security.enable")]
    Enable,
}
#[doc = "Enables tracking security state changes.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Security/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "Security.enable";
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Enable/disable whether all certificate errors should be ignored.\n[setIgnoreCertificateErrors](https://chromedevtools.github.io/devtools-protocol/tot/Security/#method-setIgnoreCertificateErrors)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetIgnoreCertificateErrorsParams {
    #[doc = "If true, all certificate errors will be ignored."]
    #[serde(rename = "ignore")]
    pub ignore: bool,
}
impl SetIgnoreCertificateErrorsParams {
    pub fn new(ignore: impl Into<bool>) -> Self {
        Self {
            ignore: ignore.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetIgnoreCertificateErrorsMethod {
    #[serde(rename = "Security.setIgnoreCertificateErrors")]
    SetIgnoreCertificateErrors,
}
#[doc = "Enable/disable whether all certificate errors should be ignored.\n[setIgnoreCertificateErrors](https://chromedevtools.github.io/devtools-protocol/tot/Security/#method-setIgnoreCertificateErrors)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetIgnoreCertificateErrors {
    pub method: SetIgnoreCertificateErrorsMethod,
    pub params: SetIgnoreCertificateErrorsParams,
}
impl SetIgnoreCertificateErrors {
    pub const IDENTIFIER: &'static str = "Security.setIgnoreCertificateErrors";
}
impl crate::CommandResult for SetIgnoreCertificateErrors {
    type Result = super::results::SetIgnoreCertificateErrorsResult;
}
group_enum ! (SecurityCommands { Disable (Disable) , Enable (Enable) , SetIgnoreCertificateErrors (SetIgnoreCertificateErrors) });
