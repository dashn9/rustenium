use serde::{Deserialize, Serialize};
#[doc = "Notification sent after the virtual time budget for the current VirtualTimePolicy has run out.\n[virtualTimeBudgetExpired](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#event-virtualTimeBudgetExpired)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualTimeBudgetExpiredParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VirtualTimeBudgetExpiredMethod {
    #[serde(rename = "Emulation.virtualTimeBudgetExpired")]
    VirtualTimeBudgetExpired,
}
#[doc = "Notification sent after the virtual time budget for the current VirtualTimePolicy has run out.\n[virtualTimeBudgetExpired](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#event-virtualTimeBudgetExpired)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualTimeBudgetExpired {
    pub method: VirtualTimeBudgetExpiredMethod,
    pub params: VirtualTimeBudgetExpiredParams,
}
impl VirtualTimeBudgetExpired {
    pub const IDENTIFIER: &'static str = "Emulation.virtualTimeBudgetExpired";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (EmulationEvents { VirtualTimeBudgetExpired (VirtualTimeBudgetExpired) } + identifiable);
