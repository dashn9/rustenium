use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EndResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCategoriesResult {
    #[doc = "A list of supported tracing categories."]
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTrackEventDescriptorResult {
    #[doc = "Base64-encoded serialized perfetto.protos.TrackEventDescriptor protobuf message."]
    #[serde(rename = "descriptor")]
    pub descriptor: crate::Binary,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RecordClockSyncMarkerResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestMemoryDumpResult {
    #[doc = "GUID of the resulting global memory dump."]
    #[serde(rename = "dumpGuid")]
    pub dump_guid: String,
    #[doc = "True iff the global memory dump succeeded."]
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartResult {}
