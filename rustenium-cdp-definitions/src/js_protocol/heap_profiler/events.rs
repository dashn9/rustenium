use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddHeapSnapshotChunkParams {
    #[serde(rename = "chunk")]
    pub chunk: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddHeapSnapshotChunkMethod {
    #[serde(rename = "HeapProfiler.addHeapSnapshotChunk")]
    AddHeapSnapshotChunk,
}
impl AddHeapSnapshotChunkMethod {
    pub const IDENTIFIER: &'static str = "HeapProfiler.addHeapSnapshotChunk";
}
#[derive(Debug, Clone, PartialEq)]
pub struct AddHeapSnapshotChunk {
    pub method: AddHeapSnapshotChunkMethod,
    pub params: AddHeapSnapshotChunkParams,
}
#[doc = "If heap objects tracking has been started then backend may send update for one or more fragments\n[heapStatsUpdate](https://chromedevtools.github.io/devtools-protocol/tot/HeapProfiler/#event-heapStatsUpdate)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeapStatsUpdateParams {
    #[doc = "An array of triplets. Each triplet describes a fragment. The first integer is the fragment\nindex, the second integer is a total count of objects for the fragment, the third integer is\na total size of the objects for the fragment."]
    #[serde(rename = "statsUpdate")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stats_update: Vec<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HeapStatsUpdateMethod {
    #[serde(rename = "HeapProfiler.heapStatsUpdate")]
    HeapStatsUpdate,
}
impl HeapStatsUpdateMethod {
    pub const IDENTIFIER: &'static str = "HeapProfiler.heapStatsUpdate";
}
#[doc = "If heap objects tracking has been started then backend may send update for one or more fragments\n[heapStatsUpdate](https://chromedevtools.github.io/devtools-protocol/tot/HeapProfiler/#event-heapStatsUpdate)"]
#[derive(Debug, Clone, PartialEq)]
pub struct HeapStatsUpdate {
    pub method: HeapStatsUpdateMethod,
    pub params: HeapStatsUpdateParams,
}
#[doc = "If heap objects tracking has been started then backend regularly sends a current value for last\nseen object id and corresponding timestamp. If the were changes in the heap since last event\nthen one or more heapStatsUpdate events will be sent before a new lastSeenObjectId event.\n[lastSeenObjectId](https://chromedevtools.github.io/devtools-protocol/tot/HeapProfiler/#event-lastSeenObjectId)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastSeenObjectIdParams {
    #[serde(rename = "lastSeenObjectId")]
    pub last_seen_object_id: i64,
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LastSeenObjectIdMethod {
    #[serde(rename = "HeapProfiler.lastSeenObjectId")]
    LastSeenObjectId,
}
impl LastSeenObjectIdMethod {
    pub const IDENTIFIER: &'static str = "HeapProfiler.lastSeenObjectId";
}
#[doc = "If heap objects tracking has been started then backend regularly sends a current value for last\nseen object id and corresponding timestamp. If the were changes in the heap since last event\nthen one or more heapStatsUpdate events will be sent before a new lastSeenObjectId event.\n[lastSeenObjectId](https://chromedevtools.github.io/devtools-protocol/tot/HeapProfiler/#event-lastSeenObjectId)"]
#[derive(Debug, Clone, PartialEq)]
pub struct LastSeenObjectId {
    pub method: LastSeenObjectIdMethod,
    pub params: LastSeenObjectIdParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportHeapSnapshotProgressParams {
    #[serde(rename = "done")]
    pub done: i64,
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(rename = "finished")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub finished: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReportHeapSnapshotProgressMethod {
    #[serde(rename = "HeapProfiler.reportHeapSnapshotProgress")]
    ReportHeapSnapshotProgress,
}
impl ReportHeapSnapshotProgressMethod {
    pub const IDENTIFIER: &'static str = "HeapProfiler.reportHeapSnapshotProgress";
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReportHeapSnapshotProgress {
    pub method: ReportHeapSnapshotProgressMethod,
    pub params: ReportHeapSnapshotProgressParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetProfilesParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResetProfilesMethod {
    #[serde(rename = "HeapProfiler.resetProfiles")]
    ResetProfiles,
}
impl ResetProfilesMethod {
    pub const IDENTIFIER: &'static str = "HeapProfiler.resetProfiles";
}
#[derive(Debug, Clone, PartialEq)]
pub struct ResetProfiles {
    pub method: ResetProfilesMethod,
    pub params: ResetProfilesParams,
}
group_enum ! (HeapProfilerEvents { AddHeapSnapshotChunk (AddHeapSnapshotChunk) , HeapStatsUpdate (HeapStatsUpdate) , LastSeenObjectId (LastSeenObjectId) , ReportHeapSnapshotProgress (ReportHeapSnapshotProgress) , ResetProfiles (ResetProfiles) });
