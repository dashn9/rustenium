use serde::{Deserialize, Serialize};
#[doc = "Enables console to refer to the node with given id via $x (see Command Line API for more details\n$x functions).\n[addInspectedHeapObject](https://chromedevtools.github.io/devtools-protocol/tot/HeapProfiler/#method-addInspectedHeapObject)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddInspectedHeapObjectParams {
    #[doc = "Heap snapshot object id to be accessible by means of $x command line API."]
    #[serde(rename = "heapObjectId")]
    pub heap_object_id: super::types::HeapSnapshotObjectId,
}
impl AddInspectedHeapObjectParams {
    pub fn new(heap_object_id: impl Into<super::types::HeapSnapshotObjectId>) -> Self {
        Self {
            heap_object_id: heap_object_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddInspectedHeapObjectMethod {
    #[serde(rename = "HeapProfiler.addInspectedHeapObject")]
    AddInspectedHeapObject,
}
#[doc = "Enables console to refer to the node with given id via $x (see Command Line API for more details\n$x functions).\n[addInspectedHeapObject](https://chromedevtools.github.io/devtools-protocol/tot/HeapProfiler/#method-addInspectedHeapObject)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddInspectedHeapObject {
    pub method: AddInspectedHeapObjectMethod,
    pub params: AddInspectedHeapObjectParams,
}
impl AddInspectedHeapObject {
    pub const IDENTIFIER: &'static str = "HeapProfiler.addInspectedHeapObject";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for AddInspectedHeapObject {
    type Result = super::results::AddInspectedHeapObjectResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectGarbageParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CollectGarbageMethod {
    #[serde(rename = "HeapProfiler.collectGarbage")]
    CollectGarbage,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectGarbage {
    pub method: CollectGarbageMethod,
    pub params: CollectGarbageParams,
}
impl CollectGarbage {
    pub const IDENTIFIER: &'static str = "HeapProfiler.collectGarbage";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for CollectGarbage {
    type Result = super::results::CollectGarbageResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "HeapProfiler.disable")]
    Disable,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "HeapProfiler.disable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "HeapProfiler.enable")]
    Enable,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "HeapProfiler.enable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHeapObjectIdParams {
    #[doc = "Identifier of the object to get heap object id for."]
    #[serde(rename = "objectId")]
    pub object_id: crate::js_protocol::runtime::types::RemoteObjectId,
}
impl GetHeapObjectIdParams {
    pub fn new(object_id: impl Into<crate::js_protocol::runtime::types::RemoteObjectId>) -> Self {
        Self {
            object_id: object_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetHeapObjectIdMethod {
    #[serde(rename = "HeapProfiler.getHeapObjectId")]
    GetHeapObjectId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHeapObjectId {
    pub method: GetHeapObjectIdMethod,
    pub params: GetHeapObjectIdParams,
}
impl GetHeapObjectId {
    pub const IDENTIFIER: &'static str = "HeapProfiler.getHeapObjectId";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetHeapObjectId {
    type Result = super::results::GetHeapObjectIdResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetObjectByHeapObjectIdParams {
    #[serde(rename = "objectId")]
    pub object_id: super::types::HeapSnapshotObjectId,
    #[doc = "Symbolic group name that can be used to release multiple objects."]
    #[serde(rename = "objectGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_group: Option<String>,
}
impl GetObjectByHeapObjectIdParams {
    pub fn new(object_id: impl Into<super::types::HeapSnapshotObjectId>) -> Self {
        Self {
            object_id: object_id.into(),
            object_group: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetObjectByHeapObjectIdMethod {
    #[serde(rename = "HeapProfiler.getObjectByHeapObjectId")]
    GetObjectByHeapObjectId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetObjectByHeapObjectId {
    pub method: GetObjectByHeapObjectIdMethod,
    pub params: GetObjectByHeapObjectIdParams,
}
impl GetObjectByHeapObjectId {
    pub const IDENTIFIER: &'static str = "HeapProfiler.getObjectByHeapObjectId";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetObjectByHeapObjectId {
    type Result = super::results::GetObjectByHeapObjectIdResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSamplingProfileParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetSamplingProfileMethod {
    #[serde(rename = "HeapProfiler.getSamplingProfile")]
    GetSamplingProfile,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSamplingProfile {
    pub method: GetSamplingProfileMethod,
    pub params: GetSamplingProfileParams,
}
impl GetSamplingProfile {
    pub const IDENTIFIER: &'static str = "HeapProfiler.getSamplingProfile";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetSamplingProfile {
    type Result = super::results::GetSamplingProfileResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartSamplingParams {
    #[doc = "Average sample interval in bytes. Poisson distribution is used for the intervals. The\ndefault value is 32768 bytes."]
    #[serde(rename = "samplingInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sampling_interval: Option<f64>,
    #[doc = "Maximum stack depth. The default value is 128."]
    #[serde(rename = "stackDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stack_depth: Option<f64>,
    #[doc = "By default, the sampling heap profiler reports only objects which are\nstill alive when the profile is returned via getSamplingProfile or\nstopSampling, which is useful for determining what functions contribute\nthe most to steady-state memory usage. This flag instructs the sampling\nheap profiler to also include information about objects discarded by\nmajor GC, which will show which functions cause large temporary memory\nusage or long GC pauses."]
    #[serde(rename = "includeObjectsCollectedByMajorGC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_objects_collected_by_major_gc: Option<bool>,
    #[doc = "By default, the sampling heap profiler reports only objects which are\nstill alive when the profile is returned via getSamplingProfile or\nstopSampling, which is useful for determining what functions contribute\nthe most to steady-state memory usage. This flag instructs the sampling\nheap profiler to also include information about objects discarded by\nminor GC, which is useful when tuning a latency-sensitive application\nfor minimal GC activity."]
    #[serde(rename = "includeObjectsCollectedByMinorGC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_objects_collected_by_minor_gc: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartSamplingMethod {
    #[serde(rename = "HeapProfiler.startSampling")]
    StartSampling,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartSampling {
    pub method: StartSamplingMethod,
    pub params: StartSamplingParams,
}
impl StartSampling {
    pub const IDENTIFIER: &'static str = "HeapProfiler.startSampling";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StartSampling {
    type Result = super::results::StartSamplingResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartTrackingHeapObjectsParams {
    #[serde(rename = "trackAllocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub track_allocations: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartTrackingHeapObjectsMethod {
    #[serde(rename = "HeapProfiler.startTrackingHeapObjects")]
    StartTrackingHeapObjects,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartTrackingHeapObjects {
    pub method: StartTrackingHeapObjectsMethod,
    pub params: StartTrackingHeapObjectsParams,
}
impl StartTrackingHeapObjects {
    pub const IDENTIFIER: &'static str = "HeapProfiler.startTrackingHeapObjects";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StartTrackingHeapObjects {
    type Result = super::results::StartTrackingHeapObjectsResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopSamplingParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopSamplingMethod {
    #[serde(rename = "HeapProfiler.stopSampling")]
    StopSampling,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopSampling {
    pub method: StopSamplingMethod,
    pub params: StopSamplingParams,
}
impl StopSampling {
    pub const IDENTIFIER: &'static str = "HeapProfiler.stopSampling";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StopSampling {
    type Result = super::results::StopSamplingResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopTrackingHeapObjectsParams {
    #[doc = "If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken\nwhen the tracking is stopped."]
    #[serde(rename = "reportProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub report_progress: Option<bool>,
    #[doc = "If true, numerical values are included in the snapshot"]
    #[serde(rename = "captureNumericValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub capture_numeric_value: Option<bool>,
    #[doc = "If true, exposes internals of the snapshot."]
    #[serde(rename = "exposeInternals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub expose_internals: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopTrackingHeapObjectsMethod {
    #[serde(rename = "HeapProfiler.stopTrackingHeapObjects")]
    StopTrackingHeapObjects,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopTrackingHeapObjects {
    pub method: StopTrackingHeapObjectsMethod,
    pub params: StopTrackingHeapObjectsParams,
}
impl StopTrackingHeapObjects {
    pub const IDENTIFIER: &'static str = "HeapProfiler.stopTrackingHeapObjects";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StopTrackingHeapObjects {
    type Result = super::results::StopTrackingHeapObjectsResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TakeHeapSnapshotParams {
    #[doc = "If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken."]
    #[serde(rename = "reportProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub report_progress: Option<bool>,
    #[doc = "If true, numerical values are included in the snapshot"]
    #[serde(rename = "captureNumericValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub capture_numeric_value: Option<bool>,
    #[doc = "If true, exposes internals of the snapshot."]
    #[serde(rename = "exposeInternals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub expose_internals: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TakeHeapSnapshotMethod {
    #[serde(rename = "HeapProfiler.takeHeapSnapshot")]
    TakeHeapSnapshot,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeHeapSnapshot {
    pub method: TakeHeapSnapshotMethod,
    pub params: TakeHeapSnapshotParams,
}
impl TakeHeapSnapshot {
    pub const IDENTIFIER: &'static str = "HeapProfiler.takeHeapSnapshot";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for TakeHeapSnapshot {
    type Result = super::results::TakeHeapSnapshotResult;
}
group_enum ! (HeapProfilerCommands { AddInspectedHeapObject (AddInspectedHeapObject) , CollectGarbage (CollectGarbage) , Disable (Disable) , Enable (Enable) , GetHeapObjectId (GetHeapObjectId) , GetObjectByHeapObjectId (GetObjectByHeapObjectId) , GetSamplingProfile (GetSamplingProfile) , StartSampling (StartSampling) , StartTrackingHeapObjects (StartTrackingHeapObjects) , StopSampling (StopSampling) , StopTrackingHeapObjects (StopTrackingHeapObjects) , TakeHeapSnapshot (TakeHeapSnapshot) } + identifiable);
