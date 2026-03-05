use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeliverPushMessageParams {
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(rename = "registrationId")]
    pub registration_id: super::types::RegistrationId,
    #[serde(rename = "data")]
    pub data: String,
}
impl DeliverPushMessageParams {
    pub fn new(
        origin: impl Into<String>,
        registration_id: impl Into<super::types::RegistrationId>,
        data: impl Into<String>,
    ) -> Self {
        Self {
            origin: origin.into(),
            registration_id: registration_id.into(),
            data: data.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeliverPushMessageMethod {
    #[serde(rename = "ServiceWorker.deliverPushMessage")]
    DeliverPushMessage,
}
impl DeliverPushMessageMethod {
    pub const IDENTIFIER: &'static str = "ServiceWorker.deliverPushMessage";
}
#[derive(Debug, Clone, PartialEq)]
pub struct DeliverPushMessage {
    pub method: DeliverPushMessageMethod,
    pub params: DeliverPushMessageParams,
}
impl super::super::super::CommandResult for DeliverPushMessage {
    type Result = super::results::DeliverPushMessageResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "ServiceWorker.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "ServiceWorker.disable";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl super::super::super::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DispatchSyncEventParams {
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(rename = "registrationId")]
    pub registration_id: super::types::RegistrationId,
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "lastChance")]
    pub last_chance: bool,
}
impl DispatchSyncEventParams {
    pub fn new(
        origin: impl Into<String>,
        registration_id: impl Into<super::types::RegistrationId>,
        tag: impl Into<String>,
        last_chance: impl Into<bool>,
    ) -> Self {
        Self {
            origin: origin.into(),
            registration_id: registration_id.into(),
            tag: tag.into(),
            last_chance: last_chance.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DispatchSyncEventMethod {
    #[serde(rename = "ServiceWorker.dispatchSyncEvent")]
    DispatchSyncEvent,
}
impl DispatchSyncEventMethod {
    pub const IDENTIFIER: &'static str = "ServiceWorker.dispatchSyncEvent";
}
#[derive(Debug, Clone, PartialEq)]
pub struct DispatchSyncEvent {
    pub method: DispatchSyncEventMethod,
    pub params: DispatchSyncEventParams,
}
impl super::super::super::CommandResult for DispatchSyncEvent {
    type Result = super::results::DispatchSyncEventResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DispatchPeriodicSyncEventParams {
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(rename = "registrationId")]
    pub registration_id: super::types::RegistrationId,
    #[serde(rename = "tag")]
    pub tag: String,
}
impl DispatchPeriodicSyncEventParams {
    pub fn new(
        origin: impl Into<String>,
        registration_id: impl Into<super::types::RegistrationId>,
        tag: impl Into<String>,
    ) -> Self {
        Self {
            origin: origin.into(),
            registration_id: registration_id.into(),
            tag: tag.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DispatchPeriodicSyncEventMethod {
    #[serde(rename = "ServiceWorker.dispatchPeriodicSyncEvent")]
    DispatchPeriodicSyncEvent,
}
impl DispatchPeriodicSyncEventMethod {
    pub const IDENTIFIER: &'static str = "ServiceWorker.dispatchPeriodicSyncEvent";
}
#[derive(Debug, Clone, PartialEq)]
pub struct DispatchPeriodicSyncEvent {
    pub method: DispatchPeriodicSyncEventMethod,
    pub params: DispatchPeriodicSyncEventParams,
}
impl super::super::super::CommandResult for DispatchPeriodicSyncEvent {
    type Result = super::results::DispatchPeriodicSyncEventResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "ServiceWorker.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "ServiceWorker.enable";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl super::super::super::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetForceUpdateOnPageLoadParams {
    #[serde(rename = "forceUpdateOnPageLoad")]
    pub force_update_on_page_load: bool,
}
impl SetForceUpdateOnPageLoadParams {
    pub fn new(force_update_on_page_load: impl Into<bool>) -> Self {
        Self {
            force_update_on_page_load: force_update_on_page_load.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetForceUpdateOnPageLoadMethod {
    #[serde(rename = "ServiceWorker.setForceUpdateOnPageLoad")]
    SetForceUpdateOnPageLoad,
}
impl SetForceUpdateOnPageLoadMethod {
    pub const IDENTIFIER: &'static str = "ServiceWorker.setForceUpdateOnPageLoad";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetForceUpdateOnPageLoad {
    pub method: SetForceUpdateOnPageLoadMethod,
    pub params: SetForceUpdateOnPageLoadParams,
}
impl super::super::super::CommandResult for SetForceUpdateOnPageLoad {
    type Result = super::results::SetForceUpdateOnPageLoadResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkipWaitingParams {
    #[serde(rename = "scopeURL")]
    pub scope_url: String,
}
impl SkipWaitingParams {
    pub fn new(scope_url: impl Into<String>) -> Self {
        Self {
            scope_url: scope_url.into(),
        }
    }
}
impl<T: Into<String>> From<T> for SkipWaitingParams {
    fn from(url: T) -> Self {
        SkipWaitingParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SkipWaitingMethod {
    #[serde(rename = "ServiceWorker.skipWaiting")]
    SkipWaiting,
}
impl SkipWaitingMethod {
    pub const IDENTIFIER: &'static str = "ServiceWorker.skipWaiting";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SkipWaiting {
    pub method: SkipWaitingMethod,
    pub params: SkipWaitingParams,
}
impl super::super::super::CommandResult for SkipWaiting {
    type Result = super::results::SkipWaitingResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartWorkerParams {
    #[serde(rename = "scopeURL")]
    pub scope_url: String,
}
impl StartWorkerParams {
    pub fn new(scope_url: impl Into<String>) -> Self {
        Self {
            scope_url: scope_url.into(),
        }
    }
}
impl<T: Into<String>> From<T> for StartWorkerParams {
    fn from(url: T) -> Self {
        StartWorkerParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartWorkerMethod {
    #[serde(rename = "ServiceWorker.startWorker")]
    StartWorker,
}
impl StartWorkerMethod {
    pub const IDENTIFIER: &'static str = "ServiceWorker.startWorker";
}
#[derive(Debug, Clone, PartialEq)]
pub struct StartWorker {
    pub method: StartWorkerMethod,
    pub params: StartWorkerParams,
}
impl super::super::super::CommandResult for StartWorker {
    type Result = super::results::StartWorkerResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopAllWorkersParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopAllWorkersMethod {
    #[serde(rename = "ServiceWorker.stopAllWorkers")]
    StopAllWorkers,
}
impl StopAllWorkersMethod {
    pub const IDENTIFIER: &'static str = "ServiceWorker.stopAllWorkers";
}
#[derive(Debug, Clone, PartialEq)]
pub struct StopAllWorkers {
    pub method: StopAllWorkersMethod,
    pub params: StopAllWorkersParams,
}
impl super::super::super::CommandResult for StopAllWorkers {
    type Result = super::results::StopAllWorkersResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopWorkerParams {
    #[serde(rename = "versionId")]
    pub version_id: String,
}
impl StopWorkerParams {
    pub fn new(version_id: impl Into<String>) -> Self {
        Self {
            version_id: version_id.into(),
        }
    }
}
impl<T: Into<String>> From<T> for StopWorkerParams {
    fn from(url: T) -> Self {
        StopWorkerParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopWorkerMethod {
    #[serde(rename = "ServiceWorker.stopWorker")]
    StopWorker,
}
impl StopWorkerMethod {
    pub const IDENTIFIER: &'static str = "ServiceWorker.stopWorker";
}
#[derive(Debug, Clone, PartialEq)]
pub struct StopWorker {
    pub method: StopWorkerMethod,
    pub params: StopWorkerParams,
}
impl super::super::super::CommandResult for StopWorker {
    type Result = super::results::StopWorkerResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnregisterParams {
    #[serde(rename = "scopeURL")]
    pub scope_url: String,
}
impl UnregisterParams {
    pub fn new(scope_url: impl Into<String>) -> Self {
        Self {
            scope_url: scope_url.into(),
        }
    }
}
impl<T: Into<String>> From<T> for UnregisterParams {
    fn from(url: T) -> Self {
        UnregisterParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnregisterMethod {
    #[serde(rename = "ServiceWorker.unregister")]
    Unregister,
}
impl UnregisterMethod {
    pub const IDENTIFIER: &'static str = "ServiceWorker.unregister";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Unregister {
    pub method: UnregisterMethod,
    pub params: UnregisterParams,
}
impl super::super::super::CommandResult for Unregister {
    type Result = super::results::UnregisterResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateRegistrationParams {
    #[serde(rename = "scopeURL")]
    pub scope_url: String,
}
impl UpdateRegistrationParams {
    pub fn new(scope_url: impl Into<String>) -> Self {
        Self {
            scope_url: scope_url.into(),
        }
    }
}
impl<T: Into<String>> From<T> for UpdateRegistrationParams {
    fn from(url: T) -> Self {
        UpdateRegistrationParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UpdateRegistrationMethod {
    #[serde(rename = "ServiceWorker.updateRegistration")]
    UpdateRegistration,
}
impl UpdateRegistrationMethod {
    pub const IDENTIFIER: &'static str = "ServiceWorker.updateRegistration";
}
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateRegistration {
    pub method: UpdateRegistrationMethod,
    pub params: UpdateRegistrationParams,
}
impl super::super::super::CommandResult for UpdateRegistration {
    type Result = super::results::UpdateRegistrationResult;
}
group_enum ! (ServiceWorkerCommands { DeliverPushMessage (DeliverPushMessage) , Disable (Disable) , DispatchSyncEvent (DispatchSyncEvent) , DispatchPeriodicSyncEvent (DispatchPeriodicSyncEvent) , Enable (Enable) , SetForceUpdateOnPageLoad (SetForceUpdateOnPageLoad) , SkipWaiting (SkipWaiting) , StartWorker (StartWorker) , StopAllWorkers (StopAllWorkers) , StopWorker (StopWorker) , Unregister (Unregister) , UpdateRegistration (UpdateRegistration) });
