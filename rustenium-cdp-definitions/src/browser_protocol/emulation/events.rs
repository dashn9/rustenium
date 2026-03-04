use serde::{Deserialize, Serialize};
#[doc = "Notification sent after the virtual time budget for the current VirtualTimePolicy has run out.\n[virtualTimeBudgetExpired](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#event-virtualTimeBudgetExpired)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VirtualTimeBudgetExpired {}
impl VirtualTimeBudgetExpired {
    pub const IDENTIFIER: &'static str = "Emulation.virtualTimeBudgetExpired";
}
group_enum ! (Event { VirtualTimeBudgetExpired (VirtualTimeBudgetExpired) });
