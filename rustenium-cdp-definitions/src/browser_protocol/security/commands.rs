use serde::{Deserialize, Serialize};
#[doc = "Disables tracking security state changes.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Security/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Security.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "Security.disable";
}
#[doc = "Disables tracking security state changes.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Security/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl super::super::super::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enables tracking security state changes.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Security/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Security.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "Security.enable";
}
#[doc = "Enables tracking security state changes.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Security/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl super::super::super::CommandResult for Enable {
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
impl SetIgnoreCertificateErrorsMethod {
    pub const IDENTIFIER: &'static str = "Security.setIgnoreCertificateErrors";
}
#[doc = "Enable/disable whether all certificate errors should be ignored.\n[setIgnoreCertificateErrors](https://chromedevtools.github.io/devtools-protocol/tot/Security/#method-setIgnoreCertificateErrors)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetIgnoreCertificateErrors {
    pub method: SetIgnoreCertificateErrorsMethod,
    pub params: SetIgnoreCertificateErrorsParams,
}
impl super::super::super::CommandResult for SetIgnoreCertificateErrors {
    type Result = super::results::SetIgnoreCertificateErrorsResult;
}
group_enum ! (SecurityCommands { Disable (Disable) , Enable (Enable) , SetIgnoreCertificateErrors (SetIgnoreCertificateErrors) });
