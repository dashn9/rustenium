use serde::{Deserialize, Serialize};
#[doc = "Configuration for memory dump. Used only when \"memory-infra\" category is enabled.\n[MemoryDumpConfig](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#type-MemoryDumpConfig)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MemoryDumpConfig(serde_json::Value);
impl MemoryDumpConfig {
    pub fn new(val: impl Into<serde_json::Value>) -> Self {
        MemoryDumpConfig(val.into())
    }
    pub fn inner(&self) -> &serde_json::Value {
        &self.0
    }
}
impl MemoryDumpConfig {
    pub const IDENTIFIER: &'static str = "Tracing.MemoryDumpConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TraceConfig {
    #[doc = "Controls how the trace buffer stores data. The default is `recordUntilFull`."]
    #[serde(rename = "recordMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub record_mode: Option<TraceConfigRecordMode>,
    #[doc = "Size of the trace buffer in kilobytes. If not specified or zero is passed, a default value\nof 200 MB would be used."]
    #[serde(rename = "traceBufferSizeInKb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub trace_buffer_size_in_kb: Option<f64>,
    #[doc = "Turns on JavaScript stack sampling."]
    #[serde(rename = "enableSampling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enable_sampling: Option<bool>,
    #[doc = "Turns on system tracing."]
    #[serde(rename = "enableSystrace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enable_systrace: Option<bool>,
    #[doc = "Turns on argument filter."]
    #[serde(rename = "enableArgumentFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enable_argument_filter: Option<bool>,
    #[doc = "Included category filters."]
    #[serde(rename = "includedCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub included_categories: Option<Vec<String>>,
    #[doc = "Excluded category filters."]
    #[serde(rename = "excludedCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub excluded_categories: Option<Vec<String>>,
    #[doc = "Configuration to synthesize the delays in tracing."]
    #[serde(rename = "syntheticDelays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub synthetic_delays: Option<Vec<String>>,
    #[doc = "Configuration for memory dump triggers. Used only when \"memory-infra\" category is enabled."]
    #[serde(rename = "memoryDumpConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub memory_dump_config: Option<MemoryDumpConfig>,
}
#[doc = "Controls how the trace buffer stores data. The default is `recordUntilFull`."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraceConfigRecordMode {
    #[serde(rename = "recordUntilFull")]
    RecordUntilFull,
    #[serde(rename = "recordContinuously")]
    RecordContinuously,
    #[serde(rename = "recordAsMuchAsPossible")]
    RecordAsMuchAsPossible,
    #[serde(rename = "echoToConsole")]
    EchoToConsole,
}
impl TraceConfig {
    pub const IDENTIFIER: &'static str = "Tracing.TraceConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Data format of a trace. Can be either the legacy JSON format or the\nprotocol buffer format. Note that the JSON format will be deprecated soon."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StreamFormat {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "proto")]
    Proto,
}
#[doc = "Compression type to use for traces returned via streams."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StreamCompression {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "gzip")]
    Gzip,
}
#[doc = "Details exposed when memory request explicitly declared.\nKeep consistent with memory_dump_request_args.h and\nmemory_instrumentation.mojom"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryDumpLevelOfDetail {
    #[serde(rename = "background")]
    Background,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "detailed")]
    Detailed,
}
#[doc = "Backend type to use for tracing. `chrome` uses the Chrome-integrated\ntracing service and is supported on all platforms. `system` is only\nsupported on Chrome OS and uses the Perfetto system tracing service.\n`auto` chooses `system` when the perfettoConfig provided to Tracing.start\nspecifies at least one non-Chrome data source; otherwise uses `chrome`."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TracingBackend {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "chrome")]
    Chrome,
    #[serde(rename = "system")]
    System,
}
group_enum ! (TracingTypes { MemoryDumpConfig (MemoryDumpConfig) , TraceConfig (TraceConfig) , StreamFormat (StreamFormat) , StreamCompression (StreamCompression) , MemoryDumpLevelOfDetail (MemoryDumpLevelOfDetail) , TracingBackend (TracingBackend) });
