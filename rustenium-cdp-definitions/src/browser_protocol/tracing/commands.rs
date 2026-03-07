use serde::{Deserialize, Serialize};
#[doc = "Stop trace events collection.\n[end](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#method-end)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EndMethod {
    #[serde(rename = "Tracing.end")]
    End,
}
#[doc = "Stop trace events collection.\n[end](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#method-end)"]
#[derive(Debug, Clone, PartialEq)]
pub struct End {
    pub method: EndMethod,
    pub params: EndParams,
}
impl End {
    pub const IDENTIFIER: &'static str = "Tracing.end";
}
impl crate::CommandResult for End {
    type Result = super::results::EndResult;
}
#[doc = "Gets supported tracing categories.\n[getCategories](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#method-getCategories)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCategoriesParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetCategoriesMethod {
    #[serde(rename = "Tracing.getCategories")]
    GetCategories,
}
#[doc = "Gets supported tracing categories.\n[getCategories](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#method-getCategories)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetCategories {
    pub method: GetCategoriesMethod,
    pub params: GetCategoriesParams,
}
impl GetCategories {
    pub const IDENTIFIER: &'static str = "Tracing.getCategories";
}
impl crate::CommandResult for GetCategories {
    type Result = super::results::GetCategoriesResult;
}
#[doc = "Return a descriptor for all available tracing categories.\n[getTrackEventDescriptor](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#method-getTrackEventDescriptor)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTrackEventDescriptorParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetTrackEventDescriptorMethod {
    #[serde(rename = "Tracing.getTrackEventDescriptor")]
    GetTrackEventDescriptor,
}
#[doc = "Return a descriptor for all available tracing categories.\n[getTrackEventDescriptor](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#method-getTrackEventDescriptor)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetTrackEventDescriptor {
    pub method: GetTrackEventDescriptorMethod,
    pub params: GetTrackEventDescriptorParams,
}
impl GetTrackEventDescriptor {
    pub const IDENTIFIER: &'static str = "Tracing.getTrackEventDescriptor";
}
impl crate::CommandResult for GetTrackEventDescriptor {
    type Result = super::results::GetTrackEventDescriptorResult;
}
#[doc = "Record a clock sync marker in the trace.\n[recordClockSyncMarker](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#method-recordClockSyncMarker)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordClockSyncMarkerParams {
    #[doc = "The ID of this clock sync marker"]
    #[serde(rename = "syncId")]
    pub sync_id: String,
}
impl RecordClockSyncMarkerParams {
    pub fn new(sync_id: impl Into<String>) -> Self {
        Self {
            sync_id: sync_id.into(),
        }
    }
}
impl<T: Into<String>> From<T> for RecordClockSyncMarkerParams {
    fn from(url: T) -> Self {
        RecordClockSyncMarkerParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecordClockSyncMarkerMethod {
    #[serde(rename = "Tracing.recordClockSyncMarker")]
    RecordClockSyncMarker,
}
#[doc = "Record a clock sync marker in the trace.\n[recordClockSyncMarker](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#method-recordClockSyncMarker)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RecordClockSyncMarker {
    pub method: RecordClockSyncMarkerMethod,
    pub params: RecordClockSyncMarkerParams,
}
impl RecordClockSyncMarker {
    pub const IDENTIFIER: &'static str = "Tracing.recordClockSyncMarker";
}
impl crate::CommandResult for RecordClockSyncMarker {
    type Result = super::results::RecordClockSyncMarkerResult;
}
#[doc = "Request a global memory dump.\n[requestMemoryDump](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#method-requestMemoryDump)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestMemoryDumpParams {
    #[doc = "Enables more deterministic results by forcing garbage collection"]
    #[serde(rename = "deterministic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub deterministic: Option<bool>,
    #[doc = "Specifies level of details in memory dump. Defaults to \"detailed\"."]
    #[serde(rename = "levelOfDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub level_of_detail: Option<super::types::MemoryDumpLevelOfDetail>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RequestMemoryDumpMethod {
    #[serde(rename = "Tracing.requestMemoryDump")]
    RequestMemoryDump,
}
#[doc = "Request a global memory dump.\n[requestMemoryDump](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#method-requestMemoryDump)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestMemoryDump {
    pub method: RequestMemoryDumpMethod,
    pub params: RequestMemoryDumpParams,
}
impl RequestMemoryDump {
    pub const IDENTIFIER: &'static str = "Tracing.requestMemoryDump";
}
impl crate::CommandResult for RequestMemoryDump {
    type Result = super::results::RequestMemoryDumpResult;
}
#[doc = "Start trace events collection.\n[start](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#method-start)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartParams {
    #[doc = "If set, the agent will issue bufferUsage events at this interval, specified in milliseconds"]
    #[serde(rename = "bufferUsageReportingInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub buffer_usage_reporting_interval: Option<f64>,
    #[doc = "Whether to report trace events as series of dataCollected events or to save trace to a\nstream (defaults to `ReportEvents`)."]
    #[serde(rename = "transferMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub transfer_mode: Option<StartTransferMode>,
    #[doc = "Trace data format to use. This only applies when using `ReturnAsStream`\ntransfer mode (defaults to `json`)."]
    #[serde(rename = "streamFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stream_format: Option<super::types::StreamFormat>,
    #[doc = "Compression format to use. This only applies when using `ReturnAsStream`\ntransfer mode (defaults to `none`)"]
    #[serde(rename = "streamCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stream_compression: Option<super::types::StreamCompression>,
    #[serde(rename = "traceConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub trace_config: Option<super::types::TraceConfig>,
    #[doc = "Base64-encoded serialized perfetto.protos.TraceConfig protobuf message\nWhen specified, the parameters `categories`, `options`, `traceConfig`\nare ignored."]
    #[serde(rename = "perfettoConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub perfetto_config: Option<crate::Binary>,
    #[doc = "Backend type (defaults to `auto`)"]
    #[serde(rename = "tracingBackend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tracing_backend: Option<super::types::TracingBackend>,
}
#[doc = "Whether to report trace events as series of dataCollected events or to save trace to a\nstream (defaults to `ReportEvents`)."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StartTransferMode {
    #[serde(rename = "ReportEvents")]
    ReportEvents,
    #[serde(rename = "ReturnAsStream")]
    ReturnAsStream,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartMethod {
    #[serde(rename = "Tracing.start")]
    Start,
}
#[doc = "Start trace events collection.\n[start](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#method-start)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Start {
    pub method: StartMethod,
    pub params: StartParams,
}
impl Start {
    pub const IDENTIFIER: &'static str = "Tracing.start";
}
impl crate::CommandResult for Start {
    type Result = super::results::StartResult;
}
group_enum ! (TracingCommands { End (End) , GetCategories (GetCategories) , GetTrackEventDescriptor (GetTrackEventDescriptor) , RecordClockSyncMarker (RecordClockSyncMarker) , RequestMemoryDump (RequestMemoryDump) , Start (Start) });
