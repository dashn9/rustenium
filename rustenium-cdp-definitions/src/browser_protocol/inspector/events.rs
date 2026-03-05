use serde::{Deserialize, Serialize};
#[doc = "Fired when remote debugging connection is about to be terminated. Contains detach reason.\n[detached](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#event-detached)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Detached {
    #[doc = "The reason why connection has been terminated."]
    #[serde(rename = "reason")]
    pub reason: String,
}
impl Detached {
    pub const IDENTIFIER: &'static str = "Inspector.detached";
}
#[doc = "Fired when debugging target has crashed\n[targetCrashed](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#event-targetCrashed)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TargetCrashed {}
impl TargetCrashed {
    pub const IDENTIFIER: &'static str = "Inspector.targetCrashed";
}
#[doc = "Fired when debugging target has reloaded after crash\n[targetReloadedAfterCrash](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#event-targetReloadedAfterCrash)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TargetReloadedAfterCrash {}
impl TargetReloadedAfterCrash {
    pub const IDENTIFIER: &'static str = "Inspector.targetReloadedAfterCrash";
}
#[doc = "Fired on worker targets when main worker script and any imported scripts have been evaluated.\n[workerScriptLoaded](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#event-workerScriptLoaded)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkerScriptLoaded {}
impl WorkerScriptLoaded {
    pub const IDENTIFIER: &'static str = "Inspector.workerScriptLoaded";
}
group_enum ! (InspectorEvents { Detached (Detached) , TargetCrashed (TargetCrashed) , TargetReloadedAfterCrash (TargetReloadedAfterCrash) , WorkerScriptLoaded (WorkerScriptLoaded) });
