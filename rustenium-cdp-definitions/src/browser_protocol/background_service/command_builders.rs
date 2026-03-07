use super::commands::*;
impl StartObserving {
    pub fn builder() -> StartObservingBuilder {
        <StartObservingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StartObservingBuilder {
    service: Option<super::types::ServiceName>,
}
impl StartObservingBuilder {
    pub fn service(mut self, service: impl Into<super::types::ServiceName>) -> Self {
        self.service = Some(service.into());
        self
    }
    pub fn build(self) -> Result<StartObserving, String> {
        Ok(StartObserving {
            method: StartObservingMethod::StartObserving,
            params: StartObservingParams {
                service: self
                    .service
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(service)))?,
            },
        })
    }
}
impl StopObserving {
    pub fn builder() -> StopObservingBuilder {
        <StopObservingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StopObservingBuilder {
    service: Option<super::types::ServiceName>,
}
impl StopObservingBuilder {
    pub fn service(mut self, service: impl Into<super::types::ServiceName>) -> Self {
        self.service = Some(service.into());
        self
    }
    pub fn build(self) -> Result<StopObserving, String> {
        Ok(StopObserving {
            method: StopObservingMethod::StopObserving,
            params: StopObservingParams {
                service: self
                    .service
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(service)))?,
            },
        })
    }
}
impl SetRecording {
    pub fn builder() -> SetRecordingBuilder {
        <SetRecordingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetRecordingBuilder {
    should_record: Option<bool>,
    service: Option<super::types::ServiceName>,
}
impl SetRecordingBuilder {
    pub fn should_record(mut self, should_record: impl Into<bool>) -> Self {
        self.should_record = Some(should_record.into());
        self
    }
    pub fn service(mut self, service: impl Into<super::types::ServiceName>) -> Self {
        self.service = Some(service.into());
        self
    }
    pub fn build(self) -> Result<SetRecording, String> {
        Ok(SetRecording {
            method: SetRecordingMethod::SetRecording,
            params: SetRecordingParams {
                should_record: self.should_record.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(should_record))
                })?,
                service: self
                    .service
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(service)))?,
            },
        })
    }
}
impl ClearEvents {
    pub fn builder() -> ClearEventsBuilder {
        <ClearEventsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ClearEventsBuilder {
    service: Option<super::types::ServiceName>,
}
impl ClearEventsBuilder {
    pub fn service(mut self, service: impl Into<super::types::ServiceName>) -> Self {
        self.service = Some(service.into());
        self
    }
    pub fn build(self) -> Result<ClearEvents, String> {
        Ok(ClearEvents {
            method: ClearEventsMethod::ClearEvents,
            params: ClearEventsParams {
                service: self
                    .service
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(service)))?,
            },
        })
    }
}
