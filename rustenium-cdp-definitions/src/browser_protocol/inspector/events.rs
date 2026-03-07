use serde::{Deserialize, Serialize};
#[doc = "Fired when remote debugging connection is about to be terminated. Contains detach reason.\n[detached](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#event-detached)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetachedParams {
    #[doc = "The reason why connection has been terminated."]
    #[serde(rename = "reason")]
    pub reason: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DetachedMethod {
    #[serde(rename = "Inspector.detached")]
    Detached,
}
#[doc = "Fired when remote debugging connection is about to be terminated. Contains detach reason.\n[detached](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#event-detached)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Detached {
    pub method: DetachedMethod,
    pub params: DetachedParams,
}
impl Detached {
    pub const IDENTIFIER: &'static str = "Inspector.detached";
}
#[doc = "Fired when debugging target has crashed\n[targetCrashed](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#event-targetCrashed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetCrashedParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TargetCrashedMethod {
    #[serde(rename = "Inspector.targetCrashed")]
    TargetCrashed,
}
#[doc = "Fired when debugging target has crashed\n[targetCrashed](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#event-targetCrashed)"]
#[derive(Debug, Clone, PartialEq)]
pub struct TargetCrashed {
    pub method: TargetCrashedMethod,
    pub params: TargetCrashedParams,
}
impl TargetCrashed {
    pub const IDENTIFIER: &'static str = "Inspector.targetCrashed";
}
#[doc = "Fired when debugging target has reloaded after crash\n[targetReloadedAfterCrash](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#event-targetReloadedAfterCrash)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetReloadedAfterCrashParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TargetReloadedAfterCrashMethod {
    #[serde(rename = "Inspector.targetReloadedAfterCrash")]
    TargetReloadedAfterCrash,
}
#[doc = "Fired when debugging target has reloaded after crash\n[targetReloadedAfterCrash](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#event-targetReloadedAfterCrash)"]
#[derive(Debug, Clone, PartialEq)]
pub struct TargetReloadedAfterCrash {
    pub method: TargetReloadedAfterCrashMethod,
    pub params: TargetReloadedAfterCrashParams,
}
impl TargetReloadedAfterCrash {
    pub const IDENTIFIER: &'static str = "Inspector.targetReloadedAfterCrash";
}
#[doc = "Fired on worker targets when main worker script and any imported scripts have been evaluated.\n[workerScriptLoaded](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#event-workerScriptLoaded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerScriptLoadedParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkerScriptLoadedMethod {
    #[serde(rename = "Inspector.workerScriptLoaded")]
    WorkerScriptLoaded,
}
#[doc = "Fired on worker targets when main worker script and any imported scripts have been evaluated.\n[workerScriptLoaded](https://chromedevtools.github.io/devtools-protocol/tot/Inspector/#event-workerScriptLoaded)"]
#[derive(Debug, Clone, PartialEq)]
pub struct WorkerScriptLoaded {
    pub method: WorkerScriptLoadedMethod,
    pub params: WorkerScriptLoadedParams,
}
impl WorkerScriptLoaded {
    pub const IDENTIFIER: &'static str = "Inspector.workerScriptLoaded";
}
group_enum ! (InspectorEvents { Detached (Detached) , TargetCrashed (TargetCrashed) , TargetReloadedAfterCrash (TargetReloadedAfterCrash) , WorkerScriptLoaded (WorkerScriptLoaded) });
