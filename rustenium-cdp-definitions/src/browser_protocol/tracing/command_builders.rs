use super::commands::*;
impl RecordClockSyncMarker {
    pub fn builder() -> RecordClockSyncMarkerBuilder {
        <RecordClockSyncMarkerBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RecordClockSyncMarkerBuilder {
    sync_id: Option<String>,
}
impl RecordClockSyncMarkerBuilder {
    pub fn sync_id(mut self, sync_id: impl Into<String>) -> Self {
        self.sync_id = Some(sync_id.into());
        self
    }
    pub fn build(self) -> Result<RecordClockSyncMarker, String> {
        Ok(RecordClockSyncMarker {
            method: RecordClockSyncMarkerMethod::RecordClockSyncMarker,
            params: RecordClockSyncMarkerParams {
                sync_id: self
                    .sync_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(sync_id)))?,
            },
        })
    }
}
impl RequestMemoryDump {
    pub fn builder() -> RequestMemoryDumpBuilder {
        <RequestMemoryDumpBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RequestMemoryDumpBuilder {
    deterministic: Option<bool>,
    level_of_detail: Option<super::types::MemoryDumpLevelOfDetail>,
}
impl RequestMemoryDumpBuilder {
    pub fn deterministic(mut self, deterministic: impl Into<bool>) -> Self {
        self.deterministic = Some(deterministic.into());
        self
    }
    pub fn level_of_detail(
        mut self,
        level_of_detail: impl Into<super::types::MemoryDumpLevelOfDetail>,
    ) -> Self {
        self.level_of_detail = Some(level_of_detail.into());
        self
    }
    pub fn build(self) -> RequestMemoryDump {
        RequestMemoryDump {
            method: RequestMemoryDumpMethod::RequestMemoryDump,
            params: RequestMemoryDumpParams {
                deterministic: self.deterministic,
                level_of_detail: self.level_of_detail,
            },
        }
    }
}
impl Start {
    pub fn builder() -> StartBuilder {
        <StartBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StartBuilder {
    buffer_usage_reporting_interval: Option<f64>,
    transfer_mode: Option<StartTransferMode>,
    stream_format: Option<super::types::StreamFormat>,
    stream_compression: Option<super::types::StreamCompression>,
    trace_config: Option<super::types::TraceConfig>,
    perfetto_config: Option<crate::Binary>,
    tracing_backend: Option<super::types::TracingBackend>,
}
impl StartBuilder {
    pub fn buffer_usage_reporting_interval(
        mut self,
        buffer_usage_reporting_interval: impl Into<f64>,
    ) -> Self {
        self.buffer_usage_reporting_interval = Some(buffer_usage_reporting_interval.into());
        self
    }
    pub fn transfer_mode(mut self, transfer_mode: impl Into<StartTransferMode>) -> Self {
        self.transfer_mode = Some(transfer_mode.into());
        self
    }
    pub fn stream_format(mut self, stream_format: impl Into<super::types::StreamFormat>) -> Self {
        self.stream_format = Some(stream_format.into());
        self
    }
    pub fn stream_compression(
        mut self,
        stream_compression: impl Into<super::types::StreamCompression>,
    ) -> Self {
        self.stream_compression = Some(stream_compression.into());
        self
    }
    pub fn trace_config(mut self, trace_config: impl Into<super::types::TraceConfig>) -> Self {
        self.trace_config = Some(trace_config.into());
        self
    }
    pub fn perfetto_config(mut self, perfetto_config: impl Into<crate::Binary>) -> Self {
        self.perfetto_config = Some(perfetto_config.into());
        self
    }
    pub fn tracing_backend(
        mut self,
        tracing_backend: impl Into<super::types::TracingBackend>,
    ) -> Self {
        self.tracing_backend = Some(tracing_backend.into());
        self
    }
    pub fn build(self) -> Start {
        Start {
            method: StartMethod::Start,
            params: StartParams {
                buffer_usage_reporting_interval: self.buffer_usage_reporting_interval,
                transfer_mode: self.transfer_mode,
                stream_format: self.stream_format,
                stream_compression: self.stream_compression,
                trace_config: self.trace_config,
                perfetto_config: self.perfetto_config,
                tracing_backend: self.tracing_backend,
            },
        }
    }
}
