use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EndResult {}
impl TryFrom<serde_json::Value> for EndResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCategoriesResult {
    #[doc = "A list of supported tracing categories."]
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
}
impl TryFrom<serde_json::Value> for GetCategoriesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTrackEventDescriptorResult {
    #[doc = "Base64-encoded serialized perfetto.protos.TrackEventDescriptor protobuf message."]
    #[serde(rename = "descriptor")]
    pub descriptor: crate::Binary,
}
impl TryFrom<serde_json::Value> for GetTrackEventDescriptorResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RecordClockSyncMarkerResult {}
impl TryFrom<serde_json::Value> for RecordClockSyncMarkerResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestMemoryDumpResult {
    #[doc = "GUID of the resulting global memory dump."]
    #[serde(rename = "dumpGuid")]
    pub dump_guid: String,
    #[doc = "True iff the global memory dump succeeded."]
    #[serde(rename = "success")]
    pub success: bool,
}
impl TryFrom<serde_json::Value> for RequestMemoryDumpResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartResult {}
impl TryFrom<serde_json::Value> for StartResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
