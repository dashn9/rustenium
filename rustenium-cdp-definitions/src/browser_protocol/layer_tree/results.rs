use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompositingReasonsReturns {
    #[doc = "A list of strings specifying reasons for the given layer to become composited."]
    #[serde(rename = "compositingReasons")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub compositing_reasons: Vec<String>,
    #[doc = "A list of strings specifying reason IDs for the given layer to become composited."]
    #[serde(rename = "compositingReasonIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub compositing_reason_ids: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadSnapshotReturns {
    #[doc = "The id of the snapshot."]
    #[serde(rename = "snapshotId")]
    pub snapshot_id: super::types::SnapshotId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakeSnapshotReturns {
    #[doc = "The id of the layer snapshot."]
    #[serde(rename = "snapshotId")]
    pub snapshot_id: super::types::SnapshotId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileSnapshotReturns {
    #[doc = "The array of paint profiles, one per run."]
    #[serde(rename = "timings")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub timings: Vec<super::types::PaintProfile>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplaySnapshotReturns {
    #[doc = "A data: URL for resulting image."]
    #[serde(rename = "dataURL")]
    pub data_url: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotCommandLogReturns {
    #[doc = "The array of canvas function calls."]
    #[serde(rename = "commandLog")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub command_log: Vec<serde_json::Value>,
}
