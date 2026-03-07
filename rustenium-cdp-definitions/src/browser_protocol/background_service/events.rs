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
impl RecordingStateChangedMethod {
    pub const IDENTIFIER: &'static str = "BackgroundService.recordingStateChanged";
}
#[doc = "Called when the recording state for the service has been updated.\n[recordingStateChanged](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#event-recordingStateChanged)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RecordingStateChanged {
    pub method: RecordingStateChangedMethod,
    pub params: RecordingStateChangedParams,
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
impl BackgroundServiceEventReceivedMethod {
    pub const IDENTIFIER: &'static str = "BackgroundService.backgroundServiceEventReceived";
}
#[doc = "Called with all existing backgroundServiceEvents when enabled, and all new\nevents afterwards if enabled and recording.\n[backgroundServiceEventReceived](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#event-backgroundServiceEventReceived)"]
#[derive(Debug, Clone, PartialEq)]
pub struct BackgroundServiceEventReceived {
    pub method: BackgroundServiceEventReceivedMethod,
    pub params: BackgroundServiceEventReceivedParams,
}
group_enum ! (BackgroundServiceEvents { RecordingStateChanged (RecordingStateChanged) , BackgroundServiceEventReceived (BackgroundServiceEventReceived) });
