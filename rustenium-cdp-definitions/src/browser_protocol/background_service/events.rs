use serde::{Deserialize, Serialize};
#[doc = "Called when the recording state for the service has been updated.\n[recordingStateChanged](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#event-recordingStateChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordingStateChanged {
    #[serde(rename = "isRecording")]
    pub is_recording: bool,
    #[serde(rename = "service")]
    pub service: super::types::ServiceName,
}
impl RecordingStateChanged {
    pub const IDENTIFIER: &'static str = "BackgroundService.recordingStateChanged";
}
#[doc = "Called with all existing backgroundServiceEvents when enabled, and all new\nevents afterwards if enabled and recording.\n[backgroundServiceEventReceived](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#event-backgroundServiceEventReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundServiceEventReceived {
    #[serde(rename = "backgroundServiceEvent")]
    pub background_service_event: super::types::BackgroundServiceEvent,
}
impl BackgroundServiceEventReceived {
    pub const IDENTIFIER: &'static str = "BackgroundService.backgroundServiceEventReceived";
}
group_enum ! (BackgroundServiceEvents { RecordingStateChanged (RecordingStateChanged) , BackgroundServiceEventReceived (BackgroundServiceEventReceived) });
