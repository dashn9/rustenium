use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddInspectedHeapObjectResult {}
impl TryFrom<serde_json::Value> for AddInspectedHeapObjectResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CollectGarbageResult {}
impl TryFrom<serde_json::Value> for CollectGarbageResult {
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
pub struct GetHeapObjectIdResult {
    #[doc = "Id of the heap snapshot object corresponding to the passed remote object id."]
    #[serde(rename = "heapSnapshotObjectId")]
    pub heap_snapshot_object_id: super::types::HeapSnapshotObjectId,
}
impl TryFrom<serde_json::Value> for GetHeapObjectIdResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetObjectByHeapObjectIdResult {
    #[doc = "Evaluation result."]
    #[serde(rename = "result")]
    pub result: crate::js_protocol::runtime::types::RemoteObject,
}
impl TryFrom<serde_json::Value> for GetObjectByHeapObjectIdResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSamplingProfileResult {
    #[doc = "Return the sampling profile being collected."]
    #[serde(rename = "profile")]
    pub profile: super::types::SamplingHeapProfile,
}
impl TryFrom<serde_json::Value> for GetSamplingProfileResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartSamplingResult {}
impl TryFrom<serde_json::Value> for StartSamplingResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartTrackingHeapObjectsResult {}
impl TryFrom<serde_json::Value> for StartTrackingHeapObjectsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopSamplingResult {
    #[doc = "Recorded sampling heap profile."]
    #[serde(rename = "profile")]
    pub profile: super::types::SamplingHeapProfile,
}
impl TryFrom<serde_json::Value> for StopSamplingResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopTrackingHeapObjectsResult {}
impl TryFrom<serde_json::Value> for StopTrackingHeapObjectsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TakeHeapSnapshotResult {}
impl TryFrom<serde_json::Value> for TakeHeapSnapshotResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
