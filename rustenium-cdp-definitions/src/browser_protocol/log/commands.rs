use serde::{Deserialize, Serialize};
#[doc = "Clears the log.\n[clear](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-clear)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearMethod {
    #[serde(rename = "Log.clear")]
    Clear,
}
impl ClearMethod {
    pub const IDENTIFIER: &'static str = "Log.clear";
}
#[doc = "Clears the log.\n[clear](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-clear)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Clear {
    pub method: ClearMethod,
    pub params: ClearParams,
}
#[doc = "Disables log domain, prevents further log entries from being reported to the client.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Log.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "Log.disable";
}
#[doc = "Disables log domain, prevents further log entries from being reported to the client.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
#[doc = "Enables log domain, sends the entries collected so far to the client by means of the\n`entryAdded` notification.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Log.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "Log.enable";
}
#[doc = "Enables log domain, sends the entries collected so far to the client by means of the\n`entryAdded` notification.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
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
impl StartViolationsReportMethod {
    pub const IDENTIFIER: &'static str = "Log.startViolationsReport";
}
#[doc = "start violation reporting.\n[startViolationsReport](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-startViolationsReport)"]
#[derive(Debug, Clone, PartialEq)]
pub struct StartViolationsReport {
    pub method: StartViolationsReportMethod,
    pub params: StartViolationsReportParams,
}
#[doc = "Stop violation reporting.\n[stopViolationsReport](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-stopViolationsReport)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopViolationsReportParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopViolationsReportMethod {
    #[serde(rename = "Log.stopViolationsReport")]
    StopViolationsReport,
}
impl StopViolationsReportMethod {
    pub const IDENTIFIER: &'static str = "Log.stopViolationsReport";
}
#[doc = "Stop violation reporting.\n[stopViolationsReport](https://chromedevtools.github.io/devtools-protocol/tot/Log/#method-stopViolationsReport)"]
#[derive(Debug, Clone, PartialEq)]
pub struct StopViolationsReport {
    pub method: StopViolationsReportMethod,
    pub params: StopViolationsReportParams,
}
group_enum ! (Command { Clear (Clear) , Disable (Disable) , Enable (Enable) , StartViolationsReport (StartViolationsReport) , StopViolationsReport (StopViolationsReport) });
