use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCategoriesReturns {
    #[doc = "A list of supported tracing categories."]
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTrackEventDescriptorReturns {
    #[doc = "Base64-encoded serialized perfetto.protos.TrackEventDescriptor protobuf message."]
    #[serde(rename = "descriptor")]
    pub descriptor: super::super::super::Binary,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestMemoryDumpReturns {
    #[doc = "GUID of the resulting global memory dump."]
    #[serde(rename = "dumpGuid")]
    pub dump_guid: String,
    #[doc = "True iff the global memory dump succeeded."]
    #[serde(rename = "success")]
    pub success: bool,
}
