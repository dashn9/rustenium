use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddInspectedHeapObjectResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CollectGarbageResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartSamplingResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartTrackingHeapObjectsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopTrackingHeapObjectsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TakeHeapSnapshotResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHeapObjectIdResult {
    #[doc = "Id of the heap snapshot object corresponding to the passed remote object id."]
    #[serde(rename = "heapSnapshotObjectId")]
    pub heap_snapshot_object_id: super::types::HeapSnapshotObjectId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetObjectByHeapObjectIdResult {
    #[doc = "Evaluation result."]
    #[serde(rename = "result")]
    pub result: super::super::runtime::types::RemoteObject,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSamplingProfileResult {
    #[doc = "Return the sampling profile being collected."]
    #[serde(rename = "profile")]
    pub profile: super::types::SamplingHeapProfile,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopSamplingResult {
    #[doc = "Recorded sampling heap profile."]
    #[serde(rename = "profile")]
    pub profile: super::types::SamplingHeapProfile,
}
