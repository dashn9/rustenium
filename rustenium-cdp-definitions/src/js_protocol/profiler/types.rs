use serde::{Deserialize, Serialize};
#[doc = "Profile node. Holds callsite information, execution statistics and child nodes.\n[ProfileNode](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#type-ProfileNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileNode {
    #[doc = "Unique id of the node."]
    #[serde(rename = "id")]
    pub id: i64,
    #[doc = "Function location."]
    #[serde(rename = "callFrame")]
    pub call_frame: crate::js_protocol::runtime::types::CallFrame,
    #[doc = "Number of samples where this node was on top of the call stack."]
    #[serde(rename = "hitCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub hit_count: Option<i64>,
    #[doc = "Child node ids."]
    #[serde(rename = "children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub children: Option<Vec<i64>>,
    #[doc = "The reason of being not optimized. The function may be deoptimized or marked as don't\noptimize."]
    #[serde(rename = "deoptReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub deopt_reason: Option<String>,
    #[doc = "An array of source position ticks."]
    #[serde(rename = "positionTicks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub position_ticks: Option<Vec<PositionTickInfo>>,
}
impl ProfileNode {
    pub fn new(
        id: impl Into<i64>,
        call_frame: impl Into<crate::js_protocol::runtime::types::CallFrame>,
    ) -> Self {
        Self {
            id: id.into(),
            call_frame: call_frame.into(),
            hit_count: None,
            children: None,
            deopt_reason: None,
            position_ticks: None,
        }
    }
}
impl ProfileNode {
    pub const IDENTIFIER: &'static str = "Profiler.ProfileNode";
}
#[doc = "Profile.\n[Profile](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#type-Profile)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    #[doc = "The list of profile nodes. First item is the root node."]
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<ProfileNode>,
    #[doc = "Profiling start timestamp in microseconds."]
    #[serde(rename = "startTime")]
    pub start_time: f64,
    #[doc = "Profiling end timestamp in microseconds."]
    #[serde(rename = "endTime")]
    pub end_time: f64,
    #[doc = "Ids of samples top nodes."]
    #[serde(rename = "samples")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub samples: Option<Vec<i64>>,
    #[doc = "Time intervals between adjacent samples in microseconds. The first delta is relative to the\nprofile startTime."]
    #[serde(rename = "timeDeltas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub time_deltas: Option<Vec<i64>>,
}
impl Profile {
    pub fn new(
        nodes: Vec<ProfileNode>,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
    ) -> Self {
        Self {
            nodes,
            start_time: start_time.into(),
            end_time: end_time.into(),
            samples: None,
            time_deltas: None,
        }
    }
}
impl Profile {
    pub const IDENTIFIER: &'static str = "Profiler.Profile";
}
#[doc = "Specifies a number of samples attributed to a certain source position.\n[PositionTickInfo](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#type-PositionTickInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionTickInfo {
    #[doc = "Source line number (1-based)."]
    #[serde(rename = "line")]
    pub line: i64,
    #[doc = "Number of samples attributed to the source line."]
    #[serde(rename = "ticks")]
    pub ticks: i64,
}
impl PositionTickInfo {
    pub fn new(line: impl Into<i64>, ticks: impl Into<i64>) -> Self {
        Self {
            line: line.into(),
            ticks: ticks.into(),
        }
    }
}
impl PositionTickInfo {
    pub const IDENTIFIER: &'static str = "Profiler.PositionTickInfo";
}
#[doc = "Coverage data for a source range.\n[CoverageRange](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#type-CoverageRange)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoverageRange {
    #[doc = "JavaScript script source offset for the range start."]
    #[serde(rename = "startOffset")]
    pub start_offset: i64,
    #[doc = "JavaScript script source offset for the range end."]
    #[serde(rename = "endOffset")]
    pub end_offset: i64,
    #[doc = "Collected execution count of the source range."]
    #[serde(rename = "count")]
    pub count: i64,
}
impl CoverageRange {
    pub fn new(
        start_offset: impl Into<i64>,
        end_offset: impl Into<i64>,
        count: impl Into<i64>,
    ) -> Self {
        Self {
            start_offset: start_offset.into(),
            end_offset: end_offset.into(),
            count: count.into(),
        }
    }
}
impl CoverageRange {
    pub const IDENTIFIER: &'static str = "Profiler.CoverageRange";
}
#[doc = "Coverage data for a JavaScript function.\n[FunctionCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#type-FunctionCoverage)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCoverage {
    #[doc = "JavaScript function name."]
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[doc = "Source ranges inside the function with coverage data."]
    #[serde(rename = "ranges")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ranges: Vec<CoverageRange>,
    #[doc = "Whether coverage data for this function has block granularity."]
    #[serde(rename = "isBlockCoverage")]
    pub is_block_coverage: bool,
}
impl FunctionCoverage {
    pub fn new(
        function_name: impl Into<String>,
        ranges: Vec<CoverageRange>,
        is_block_coverage: impl Into<bool>,
    ) -> Self {
        Self {
            function_name: function_name.into(),
            ranges,
            is_block_coverage: is_block_coverage.into(),
        }
    }
}
impl FunctionCoverage {
    pub const IDENTIFIER: &'static str = "Profiler.FunctionCoverage";
}
#[doc = "Coverage data for a JavaScript script.\n[ScriptCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#type-ScriptCoverage)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptCoverage {
    #[doc = "JavaScript script id."]
    #[serde(rename = "scriptId")]
    pub script_id: crate::js_protocol::runtime::types::ScriptId,
    #[doc = "JavaScript script name or url."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Functions contained in the script that has coverage data."]
    #[serde(rename = "functions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<FunctionCoverage>,
}
impl ScriptCoverage {
    pub fn new(
        script_id: impl Into<crate::js_protocol::runtime::types::ScriptId>,
        url: impl Into<String>,
        functions: Vec<FunctionCoverage>,
    ) -> Self {
        Self {
            script_id: script_id.into(),
            url: url.into(),
            functions,
        }
    }
}
impl ScriptCoverage {
    pub const IDENTIFIER: &'static str = "Profiler.ScriptCoverage";
}
group_enum ! (ProfilerTypes { ProfileNode (ProfileNode) , Profile (Profile) , PositionTickInfo (PositionTickInfo) , CoverageRange (CoverageRange) , FunctionCoverage (FunctionCoverage) , ScriptCoverage (ScriptCoverage) });
