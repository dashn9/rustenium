use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddHeapSnapshotChunk {
    #[serde(rename = "chunk")]
    pub chunk: String,
}
impl AddHeapSnapshotChunk {
    pub const IDENTIFIER: &'static str = "HeapProfiler.addHeapSnapshotChunk";
}
#[doc = "If heap objects tracking has been started then backend may send update for one or more fragments\n[heapStatsUpdate](https://chromedevtools.github.io/devtools-protocol/tot/HeapProfiler/#event-heapStatsUpdate)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeapStatsUpdate {
    #[doc = "An array of triplets. Each triplet describes a fragment. The first integer is the fragment\nindex, the second integer is a total count of objects for the fragment, the third integer is\na total size of the objects for the fragment."]
    #[serde(rename = "statsUpdate")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stats_update: Vec<i64>,
}
impl HeapStatsUpdate {
    pub const IDENTIFIER: &'static str = "HeapProfiler.heapStatsUpdate";
}
#[doc = "If heap objects tracking has been started then backend regularly sends a current value for last\nseen object id and corresponding timestamp. If the were changes in the heap since last event\nthen one or more heapStatsUpdate events will be sent before a new lastSeenObjectId event.\n[lastSeenObjectId](https://chromedevtools.github.io/devtools-protocol/tot/HeapProfiler/#event-lastSeenObjectId)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastSeenObjectId {
    #[serde(rename = "lastSeenObjectId")]
    pub last_seen_object_id: i64,
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
}
impl LastSeenObjectId {
    pub const IDENTIFIER: &'static str = "HeapProfiler.lastSeenObjectId";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportHeapSnapshotProgress {
    #[serde(rename = "done")]
    pub done: i64,
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(rename = "finished")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub finished: Option<bool>,
}
impl ReportHeapSnapshotProgress {
    pub const IDENTIFIER: &'static str = "HeapProfiler.reportHeapSnapshotProgress";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResetProfiles {}
impl ResetProfiles {
    pub const IDENTIFIER: &'static str = "HeapProfiler.resetProfiles";
}
group_enum ! (HeapProfilerEvents { AddHeapSnapshotChunk (AddHeapSnapshotChunk) , HeapStatsUpdate (HeapStatsUpdate) , LastSeenObjectId (LastSeenObjectId) , ReportHeapSnapshotProgress (ReportHeapSnapshotProgress) , ResetProfiles (ResetProfiles) });
