use serde::{Deserialize, Serialize};
#[doc = "Heap snapshot object id.\n[HeapSnapshotObjectId](https://chromedevtools.github.io/devtools-protocol/tot/HeapProfiler/#type-HeapSnapshotObjectId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct HeapSnapshotObjectId(String);
impl HeapSnapshotObjectId {
    pub fn new(val: impl Into<String>) -> Self {
        HeapSnapshotObjectId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for HeapSnapshotObjectId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<HeapSnapshotObjectId> for String {
    fn from(el: HeapSnapshotObjectId) -> String {
        el.0
    }
}
impl From<String> for HeapSnapshotObjectId {
    fn from(expr: String) -> Self {
        HeapSnapshotObjectId(expr)
    }
}
impl std::borrow::Borrow<str> for HeapSnapshotObjectId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl HeapSnapshotObjectId {
    pub const IDENTIFIER: &'static str = "HeapProfiler.HeapSnapshotObjectId";
}
#[doc = "Sampling Heap Profile node. Holds callsite information, allocation statistics and child nodes.\n[SamplingHeapProfileNode](https://chromedevtools.github.io/devtools-protocol/tot/HeapProfiler/#type-SamplingHeapProfileNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SamplingHeapProfileNode {
    #[doc = "Function location."]
    #[serde(rename = "callFrame")]
    pub call_frame: super::super::runtime::types::CallFrame,
    #[doc = "Allocations size in bytes for the node excluding children."]
    #[serde(rename = "selfSize")]
    pub self_size: f64,
    #[doc = "Node id. Ids are unique across all profiles collected between startSampling and stopSampling."]
    #[serde(rename = "id")]
    pub id: i64,
    #[doc = "Child nodes."]
    #[serde(rename = "children")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<SamplingHeapProfileNode>,
}
impl SamplingHeapProfileNode {
    pub fn new(
        call_frame: impl Into<super::super::runtime::types::CallFrame>,
        self_size: impl Into<f64>,
        id: impl Into<i64>,
        children: Vec<SamplingHeapProfileNode>,
    ) -> Self {
        Self {
            call_frame: call_frame.into(),
            self_size: self_size.into(),
            id: id.into(),
            children,
        }
    }
}
impl SamplingHeapProfileNode {
    pub const IDENTIFIER: &'static str = "HeapProfiler.SamplingHeapProfileNode";
}
#[doc = "A single sample from a sampling profile.\n[SamplingHeapProfileSample](https://chromedevtools.github.io/devtools-protocol/tot/HeapProfiler/#type-SamplingHeapProfileSample)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SamplingHeapProfileSample {
    #[doc = "Allocation size in bytes attributed to the sample."]
    #[serde(rename = "size")]
    pub size: f64,
    #[doc = "Id of the corresponding profile tree node."]
    #[serde(rename = "nodeId")]
    pub node_id: i64,
    #[doc = "Time-ordered sample ordinal number. It is unique across all profiles retrieved\nbetween startSampling and stopSampling."]
    #[serde(rename = "ordinal")]
    pub ordinal: f64,
}
impl SamplingHeapProfileSample {
    pub fn new(size: impl Into<f64>, node_id: impl Into<i64>, ordinal: impl Into<f64>) -> Self {
        Self {
            size: size.into(),
            node_id: node_id.into(),
            ordinal: ordinal.into(),
        }
    }
}
impl SamplingHeapProfileSample {
    pub const IDENTIFIER: &'static str = "HeapProfiler.SamplingHeapProfileSample";
}
#[doc = "Sampling profile.\n[SamplingHeapProfile](https://chromedevtools.github.io/devtools-protocol/tot/HeapProfiler/#type-SamplingHeapProfile)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SamplingHeapProfile {
    #[serde(rename = "head")]
    pub head: SamplingHeapProfileNode,
    #[serde(rename = "samples")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub samples: Vec<SamplingHeapProfileSample>,
}
impl SamplingHeapProfile {
    pub fn new(
        head: impl Into<SamplingHeapProfileNode>,
        samples: Vec<SamplingHeapProfileSample>,
    ) -> Self {
        Self {
            head: head.into(),
            samples,
        }
    }
}
impl SamplingHeapProfile {
    pub const IDENTIFIER: &'static str = "HeapProfiler.SamplingHeapProfile";
}
group_enum ! (Type { HeapSnapshotObjectId (HeapSnapshotObjectId) , SamplingHeapProfileNode (SamplingHeapProfileNode) , SamplingHeapProfileSample (SamplingHeapProfileSample) , SamplingHeapProfile (SamplingHeapProfile) });
