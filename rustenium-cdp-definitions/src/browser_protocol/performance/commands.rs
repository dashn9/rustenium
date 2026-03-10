use serde::{Deserialize, Serialize};
#[doc = "Disable collecting and reporting metrics.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Performance/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Performance.disable")]
    Disable,
}
#[doc = "Disable collecting and reporting metrics.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Performance/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "Performance.disable";
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enable collecting and reporting metrics.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Performance/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {
    #[doc = "Time domain to use for collecting and reporting duration metrics."]
    #[serde(rename = "timeDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub time_domain: Option<EnableTimeDomain>,
}
#[doc = "Time domain to use for collecting and reporting duration metrics."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnableTimeDomain {
    #[doc = "Use monotonically increasing abstract time (default)."]
    #[serde(rename = "timeTicks")]
    TimeTicks,
    #[doc = "Use thread running time."]
    #[serde(rename = "threadTicks")]
    ThreadTicks,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Performance.enable")]
    Enable,
}
#[doc = "Enable collecting and reporting metrics.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Performance/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "Performance.enable";
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Retrieve current values of run-time metrics.\n[getMetrics](https://chromedevtools.github.io/devtools-protocol/tot/Performance/#method-getMetrics)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMetricsParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetMetricsMethod {
    #[serde(rename = "Performance.getMetrics")]
    GetMetrics,
}
#[doc = "Retrieve current values of run-time metrics.\n[getMetrics](https://chromedevtools.github.io/devtools-protocol/tot/Performance/#method-getMetrics)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMetrics {
    pub method: GetMetricsMethod,
    pub params: GetMetricsParams,
}
impl GetMetrics {
    pub const IDENTIFIER: &'static str = "Performance.getMetrics";
}
impl crate::CommandResult for GetMetrics {
    type Result = super::results::GetMetricsResult;
}
group_enum ! (PerformanceCommands { Disable (Disable) , Enable (Enable) , GetMetrics (GetMetrics) });
