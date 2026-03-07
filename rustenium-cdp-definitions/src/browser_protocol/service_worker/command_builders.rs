use super::commands::*;
impl DeliverPushMessage {
    pub fn builder() -> DeliverPushMessageBuilder {
        <DeliverPushMessageBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeliverPushMessageBuilder {
    origin: Option<String>,
    registration_id: Option<super::types::RegistrationId>,
    data: Option<String>,
}
impl DeliverPushMessageBuilder {
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn registration_id(
        mut self,
        registration_id: impl Into<super::types::RegistrationId>,
    ) -> Self {
        self.registration_id = Some(registration_id.into());
        self
    }
    pub fn data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }
    pub fn build(self) -> Result<DeliverPushMessage, String> {
        Ok(DeliverPushMessage {
            method: DeliverPushMessageMethod::DeliverPushMessage,
            params: DeliverPushMessageParams {
                origin: self
                    .origin
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
                registration_id: self.registration_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(registration_id))
                })?,
                data: self
                    .data
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(data)))?,
            },
        })
    }
}
impl DispatchSyncEvent {
    pub fn builder() -> DispatchSyncEventBuilder {
        <DispatchSyncEventBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DispatchSyncEventBuilder {
    origin: Option<String>,
    registration_id: Option<super::types::RegistrationId>,
    tag: Option<String>,
    last_chance: Option<bool>,
}
impl DispatchSyncEventBuilder {
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn registration_id(
        mut self,
        registration_id: impl Into<super::types::RegistrationId>,
    ) -> Self {
        self.registration_id = Some(registration_id.into());
        self
    }
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
    pub fn last_chance(mut self, last_chance: impl Into<bool>) -> Self {
        self.last_chance = Some(last_chance.into());
        self
    }
    pub fn build(self) -> Result<DispatchSyncEvent, String> {
        Ok(DispatchSyncEvent {
            method: DispatchSyncEventMethod::DispatchSyncEvent,
            params: DispatchSyncEventParams {
                origin: self
                    .origin
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
                registration_id: self.registration_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(registration_id))
                })?,
                tag: self
                    .tag
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(tag)))?,
                last_chance: self.last_chance.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(last_chance))
                })?,
            },
        })
    }
}
impl DispatchPeriodicSyncEvent {
    pub fn builder() -> DispatchPeriodicSyncEventBuilder {
        <DispatchPeriodicSyncEventBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DispatchPeriodicSyncEventBuilder {
    origin: Option<String>,
    registration_id: Option<super::types::RegistrationId>,
    tag: Option<String>,
}
impl DispatchPeriodicSyncEventBuilder {
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn registration_id(
        mut self,
        registration_id: impl Into<super::types::RegistrationId>,
    ) -> Self {
        self.registration_id = Some(registration_id.into());
        self
    }
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
    pub fn build(self) -> Result<DispatchPeriodicSyncEvent, String> {
        Ok(DispatchPeriodicSyncEvent {
            method: DispatchPeriodicSyncEventMethod::DispatchPeriodicSyncEvent,
            params: DispatchPeriodicSyncEventParams {
                origin: self
                    .origin
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
                registration_id: self.registration_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(registration_id))
                })?,
                tag: self
                    .tag
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(tag)))?,
            },
        })
    }
}
impl SetForceUpdateOnPageLoad {
    pub fn builder() -> SetForceUpdateOnPageLoadBuilder {
        <SetForceUpdateOnPageLoadBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetForceUpdateOnPageLoadBuilder {
    force_update_on_page_load: Option<bool>,
}
impl SetForceUpdateOnPageLoadBuilder {
    pub fn force_update_on_page_load(mut self, force_update_on_page_load: impl Into<bool>) -> Self {
        self.force_update_on_page_load = Some(force_update_on_page_load.into());
        self
    }
    pub fn build(self) -> Result<SetForceUpdateOnPageLoad, String> {
        Ok(SetForceUpdateOnPageLoad {
            method: SetForceUpdateOnPageLoadMethod::SetForceUpdateOnPageLoad,
            params: SetForceUpdateOnPageLoadParams {
                force_update_on_page_load: self.force_update_on_page_load.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(force_update_on_page_load)
                    )
                })?,
            },
        })
    }
}
impl SkipWaiting {
    pub fn builder() -> SkipWaitingBuilder {
        <SkipWaitingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SkipWaitingBuilder {
    scope_url: Option<String>,
}
impl SkipWaitingBuilder {
    pub fn scope_url(mut self, scope_url: impl Into<String>) -> Self {
        self.scope_url = Some(scope_url.into());
        self
    }
    pub fn build(self) -> Result<SkipWaiting, String> {
        Ok(SkipWaiting {
            method: SkipWaitingMethod::SkipWaiting,
            params: SkipWaitingParams {
                scope_url: self.scope_url.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(scope_url))
                })?,
            },
        })
    }
}
impl StartWorker {
    pub fn builder() -> StartWorkerBuilder {
        <StartWorkerBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StartWorkerBuilder {
    scope_url: Option<String>,
}
impl StartWorkerBuilder {
    pub fn scope_url(mut self, scope_url: impl Into<String>) -> Self {
        self.scope_url = Some(scope_url.into());
        self
    }
    pub fn build(self) -> Result<StartWorker, String> {
        Ok(StartWorker {
            method: StartWorkerMethod::StartWorker,
            params: StartWorkerParams {
                scope_url: self.scope_url.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(scope_url))
                })?,
            },
        })
    }
}
impl StopWorker {
    pub fn builder() -> StopWorkerBuilder {
        <StopWorkerBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StopWorkerBuilder {
    version_id: Option<String>,
}
impl StopWorkerBuilder {
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }
    pub fn build(self) -> Result<StopWorker, String> {
        Ok(StopWorker {
            method: StopWorkerMethod::StopWorker,
            params: StopWorkerParams {
                version_id: self.version_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(version_id))
                })?,
            },
        })
    }
}
impl Unregister {
    pub fn builder() -> UnregisterBuilder {
        <UnregisterBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UnregisterBuilder {
    scope_url: Option<String>,
}
impl UnregisterBuilder {
    pub fn scope_url(mut self, scope_url: impl Into<String>) -> Self {
        self.scope_url = Some(scope_url.into());
        self
    }
    pub fn build(self) -> Result<Unregister, String> {
        Ok(Unregister {
            method: UnregisterMethod::Unregister,
            params: UnregisterParams {
                scope_url: self.scope_url.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(scope_url))
                })?,
            },
        })
    }
}
impl UpdateRegistration {
    pub fn builder() -> UpdateRegistrationBuilder {
        <UpdateRegistrationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UpdateRegistrationBuilder {
    scope_url: Option<String>,
}
impl UpdateRegistrationBuilder {
    pub fn scope_url(mut self, scope_url: impl Into<String>) -> Self {
        self.scope_url = Some(scope_url.into());
        self
    }
    pub fn build(self) -> Result<UpdateRegistration, String> {
        Ok(UpdateRegistration {
            method: UpdateRegistrationMethod::UpdateRegistration,
            params: UpdateRegistrationParams {
                scope_url: self.scope_url.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(scope_url))
                })?,
            },
        })
    }
}
