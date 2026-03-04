use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHeapObjectIdReturns {
    #[doc = "Id of the heap snapshot object corresponding to the passed remote object id."]
    #[serde(rename = "heapSnapshotObjectId")]
    pub heap_snapshot_object_id: super::types::HeapSnapshotObjectId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetObjectByHeapObjectIdReturns {
    #[doc = "Evaluation result."]
    #[serde(rename = "result")]
    pub result: super::super::runtime::types::RemoteObject,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSamplingProfileReturns {
    #[doc = "Return the sampling profile being collected."]
    #[serde(rename = "profile")]
    pub profile: super::types::SamplingHeapProfile,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopSamplingReturns {
    #[doc = "Recorded sampling heap profile."]
    #[serde(rename = "profile")]
    pub profile: super::types::SamplingHeapProfile,
}
