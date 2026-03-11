use serde::{Deserialize, Serialize};
#[doc = "Called when the recording state for the service has been updated.\n[recordingStateChanged](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#event-recordingStateChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordingStateChangedParams {
    #[serde(rename = "isRecording")]
    pub is_recording: bool,
    #[serde(rename = "service")]
    pub service: super::types::ServiceName,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecordingStateChangedMethod {
    #[serde(rename = "BackgroundService.recordingStateChanged")]
    RecordingStateChanged,
}
#[doc = "Called when the recording state for the service has been updated.\n[recordingStateChanged](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#event-recordingStateChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordingStateChanged {
    pub method: RecordingStateChangedMethod,
    pub params: RecordingStateChangedParams,
}
impl RecordingStateChanged {
    pub const IDENTIFIER: &'static str = "BackgroundService.recordingStateChanged";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Called with all existing backgroundServiceEvents when enabled, and all new\nevents afterwards if enabled and recording.\n[backgroundServiceEventReceived](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#event-backgroundServiceEventReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundServiceEventReceivedParams {
    #[serde(rename = "backgroundServiceEvent")]
    pub background_service_event: super::types::BackgroundServiceEvent,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BackgroundServiceEventReceivedMethod {
    #[serde(rename = "BackgroundService.backgroundServiceEventReceived")]
    BackgroundServiceEventReceived,
}
#[doc = "Called with all existing backgroundServiceEvents when enabled, and all new\nevents afterwards if enabled and recording.\n[backgroundServiceEventReceived](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#event-backgroundServiceEventReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundServiceEventReceived {
    pub method: BackgroundServiceEventReceivedMethod,
    pub params: BackgroundServiceEventReceivedParams,
}
impl BackgroundServiceEventReceived {
    pub const IDENTIFIER: &'static str = "BackgroundService.backgroundServiceEventReceived";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (BackgroundServiceEvents { RecordingStateChanged (RecordingStateChanged) , BackgroundServiceEventReceived (BackgroundServiceEventReceived) } + identifiable);
