use serde::{Deserialize, Serialize};
#[doc = "Enables event updates for the service.\n[startObserving](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#method-startObserving)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartObservingParams {
    #[serde(rename = "service")]
    pub service: super::types::ServiceName,
}
impl StartObservingParams {
    pub fn new(service: impl Into<super::types::ServiceName>) -> Self {
        Self {
            service: service.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartObservingMethod {
    #[serde(rename = "BackgroundService.startObserving")]
    StartObserving,
}
#[doc = "Enables event updates for the service.\n[startObserving](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#method-startObserving)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartObserving {
    pub method: StartObservingMethod,
    pub params: StartObservingParams,
}
impl StartObserving {
    pub const IDENTIFIER: &'static str = "BackgroundService.startObserving";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StartObserving {
    type Result = super::results::StartObservingResult;
}
#[doc = "Disables event updates for the service.\n[stopObserving](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#method-stopObserving)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopObservingParams {
    #[serde(rename = "service")]
    pub service: super::types::ServiceName,
}
impl StopObservingParams {
    pub fn new(service: impl Into<super::types::ServiceName>) -> Self {
        Self {
            service: service.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopObservingMethod {
    #[serde(rename = "BackgroundService.stopObserving")]
    StopObserving,
}
#[doc = "Disables event updates for the service.\n[stopObserving](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#method-stopObserving)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopObserving {
    pub method: StopObservingMethod,
    pub params: StopObservingParams,
}
impl StopObserving {
    pub const IDENTIFIER: &'static str = "BackgroundService.stopObserving";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StopObserving {
    type Result = super::results::StopObservingResult;
}
#[doc = "Set the recording state for the service.\n[setRecording](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#method-setRecording)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRecordingParams {
    #[serde(rename = "shouldRecord")]
    pub should_record: bool,
    #[serde(rename = "service")]
    pub service: super::types::ServiceName,
}
impl SetRecordingParams {
    pub fn new(
        should_record: impl Into<bool>,
        service: impl Into<super::types::ServiceName>,
    ) -> Self {
        Self {
            should_record: should_record.into(),
            service: service.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetRecordingMethod {
    #[serde(rename = "BackgroundService.setRecording")]
    SetRecording,
}
#[doc = "Set the recording state for the service.\n[setRecording](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#method-setRecording)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRecording {
    pub method: SetRecordingMethod,
    pub params: SetRecordingParams,
}
impl SetRecording {
    pub const IDENTIFIER: &'static str = "BackgroundService.setRecording";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetRecording {
    type Result = super::results::SetRecordingResult;
}
#[doc = "Clears all stored data for the service.\n[clearEvents](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#method-clearEvents)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearEventsParams {
    #[serde(rename = "service")]
    pub service: super::types::ServiceName,
}
impl ClearEventsParams {
    pub fn new(service: impl Into<super::types::ServiceName>) -> Self {
        Self {
            service: service.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearEventsMethod {
    #[serde(rename = "BackgroundService.clearEvents")]
    ClearEvents,
}
#[doc = "Clears all stored data for the service.\n[clearEvents](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#method-clearEvents)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearEvents {
    pub method: ClearEventsMethod,
    pub params: ClearEventsParams,
}
impl ClearEvents {
    pub const IDENTIFIER: &'static str = "BackgroundService.clearEvents";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ClearEvents {
    type Result = super::results::ClearEventsResult;
}
group_enum ! (BackgroundServiceCommands { StartObserving (StartObserving) , StopObserving (StopObserving) , SetRecording (SetRecording) , ClearEvents (ClearEvents) } + identifiable);
