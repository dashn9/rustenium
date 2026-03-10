use serde::{Deserialize, Serialize};
#[doc = "Clears the log.\n[clear](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-clear)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearMethod {
    #[serde(rename = "Log.clear")]
    Clear,
}
#[doc = "Clears the log.\n[clear](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-clear)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clear {
    pub method: ClearMethod,
    pub params: ClearParams,
}
impl Clear {
    pub const IDENTIFIER: &'static str = "Log.clear";
}
impl crate::CommandResult for Clear {
    type Result = super::results::ClearResult;
}
#[doc = "Disables log domain, prevents further log entries from being reported to the client.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Log.disable")]
    Disable,
}
#[doc = "Disables log domain, prevents further log entries from being reported to the client.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "Log.disable";
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enables log domain, sends the entries collected so far to the client by means of the\n`entryAdded` notification.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Log.enable")]
    Enable,
}
#[doc = "Enables log domain, sends the entries collected so far to the client by means of the\n`entryAdded` notification.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "Log.enable";
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "start violation reporting.\n[startViolationsReport](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-startViolationsReport)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartViolationsReportParams {
    #[doc = "Configuration for violations."]
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<super::types::ViolationSetting>,
}
impl StartViolationsReportParams {
    pub fn new(config: Vec<super::types::ViolationSetting>) -> Self {
        Self { config }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartViolationsReportMethod {
    #[serde(rename = "Log.startViolationsReport")]
    StartViolationsReport,
}
#[doc = "start violation reporting.\n[startViolationsReport](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-startViolationsReport)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartViolationsReport {
    pub method: StartViolationsReportMethod,
    pub params: StartViolationsReportParams,
}
impl StartViolationsReport {
    pub const IDENTIFIER: &'static str = "Log.startViolationsReport";
}
impl crate::CommandResult for StartViolationsReport {
    type Result = super::results::StartViolationsReportResult;
}
#[doc = "Stop violation reporting.\n[stopViolationsReport](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-stopViolationsReport)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopViolationsReportParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopViolationsReportMethod {
    #[serde(rename = "Log.stopViolationsReport")]
    StopViolationsReport,
}
#[doc = "Stop violation reporting.\n[stopViolationsReport](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-stopViolationsReport)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopViolationsReport {
    pub method: StopViolationsReportMethod,
    pub params: StopViolationsReportParams,
}
impl StopViolationsReport {
    pub const IDENTIFIER: &'static str = "Log.stopViolationsReport";
}
impl crate::CommandResult for StopViolationsReport {
    type Result = super::results::StopViolationsReportResult;
}
group_enum ! (LogCommands { Clear (Clear) , Disable (Disable) , Enable (Enable) , StartViolationsReport (StartViolationsReport) , StopViolationsReport (StopViolationsReport) });
