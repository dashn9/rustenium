use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompositingReasonsResult {
    #[doc = "A list of strings specifying reasons for the given layer to become composited."]
    #[serde(rename = "compositingReasons")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub compositing_reasons: Vec<String>,
    #[doc = "A list of strings specifying reason IDs for the given layer to become composited."]
    #[serde(rename = "compositingReasonIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub compositing_reason_ids: Vec<String>,
}
impl TryFrom<serde_json::Value> for CompositingReasonsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
impl TryFrom<serde_json::Value> for DisableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
impl TryFrom<serde_json::Value> for EnableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadSnapshotResult {
    #[doc = "The id of the snapshot."]
    #[serde(rename = "snapshotId")]
    pub snapshot_id: super::types::SnapshotId,
}
impl TryFrom<serde_json::Value> for LoadSnapshotResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakeSnapshotResult {
    #[doc = "The id of the layer snapshot."]
    #[serde(rename = "snapshotId")]
    pub snapshot_id: super::types::SnapshotId,
}
impl TryFrom<serde_json::Value> for MakeSnapshotResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileSnapshotResult {
    #[doc = "The array of paint profiles, one per run."]
    #[serde(rename = "timings")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub timings: Vec<super::types::PaintProfile>,
}
impl TryFrom<serde_json::Value> for ProfileSnapshotResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ReleaseSnapshotResult {}
impl TryFrom<serde_json::Value> for ReleaseSnapshotResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplaySnapshotResult {
    #[doc = "A data: URL for resulting image."]
    #[serde(rename = "dataURL")]
    pub data_url: String,
}
impl TryFrom<serde_json::Value> for ReplaySnapshotResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotCommandLogResult {
    #[doc = "The array of canvas function calls."]
    #[serde(rename = "commandLog")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub command_log: Vec<serde_json::Value>,
}
impl TryFrom<serde_json::Value> for SnapshotCommandLogResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
