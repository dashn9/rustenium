use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BufferUsageParams {
    #[doc = "A number in range [0..1] that indicates the used size of event buffer as a fraction of its\ntotal size."]
    #[serde(rename = "percentFull")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub percent_full: Option<f64>,
    #[doc = "An approximate number of events in the trace log."]
    #[serde(rename = "eventCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub event_count: Option<f64>,
    #[doc = "A number in range [0..1] that indicates the used size of event buffer as a fraction of its\ntotal size."]
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BufferUsageMethod {
    #[serde(rename = "Tracing.bufferUsage")]
    BufferUsage,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BufferUsage {
    pub method: BufferUsageMethod,
    pub params: BufferUsageParams,
}
impl BufferUsage {
    pub const IDENTIFIER: &'static str = "Tracing.bufferUsage";
}
#[doc = "Contains a bucket of collected trace events. When tracing is stopped collected events will be\nsent as a sequence of dataCollected events followed by tracingComplete event.\n[dataCollected](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#event-dataCollected)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataCollectedParams {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataCollectedMethod {
    #[serde(rename = "Tracing.dataCollected")]
    DataCollected,
}
#[doc = "Contains a bucket of collected trace events. When tracing is stopped collected events will be\nsent as a sequence of dataCollected events followed by tracingComplete event.\n[dataCollected](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#event-dataCollected)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataCollected {
    pub method: DataCollectedMethod,
    pub params: DataCollectedParams,
}
impl DataCollected {
    pub const IDENTIFIER: &'static str = "Tracing.dataCollected";
}
#[doc = "Signals that tracing is stopped and there is no trace buffers pending flush, all data were\ndelivered via dataCollected events.\n[tracingComplete](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#event-tracingComplete)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TracingCompleteParams {
    #[doc = "Indicates whether some trace data is known to have been lost, e.g. because the trace ring\nbuffer wrapped around."]
    #[serde(rename = "dataLossOccurred")]
    pub data_loss_occurred: bool,
    #[doc = "A handle of the stream that holds resulting trace data."]
    #[serde(rename = "stream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stream: Option<crate::browser_protocol::io::types::StreamHandle>,
    #[doc = "Trace data format of returned stream."]
    #[serde(rename = "traceFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub trace_format: Option<super::types::StreamFormat>,
    #[doc = "Compression format of returned stream."]
    #[serde(rename = "streamCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stream_compression: Option<super::types::StreamCompression>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TracingCompleteMethod {
    #[serde(rename = "Tracing.tracingComplete")]
    TracingComplete,
}
#[doc = "Signals that tracing is stopped and there is no trace buffers pending flush, all data were\ndelivered via dataCollected events.\n[tracingComplete](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#event-tracingComplete)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TracingComplete {
    pub method: TracingCompleteMethod,
    pub params: TracingCompleteParams,
}
impl TracingComplete {
    pub const IDENTIFIER: &'static str = "Tracing.tracingComplete";
}
group_enum ! (TracingEvents { BufferUsage (BufferUsage) , DataCollected (DataCollected) , TracingComplete (TracingComplete) });
