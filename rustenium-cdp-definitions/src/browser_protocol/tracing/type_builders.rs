use super::types::*;
impl TraceConfig {
    pub fn builder() -> TraceConfigBuilder {
        TraceConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct TraceConfigBuilder {
    record_mode: Option<TraceConfigRecordMode>,
    trace_buffer_size_in_kb: Option<f64>,
    enable_sampling: Option<bool>,
    enable_systrace: Option<bool>,
    enable_argument_filter: Option<bool>,
    included_categories: Option<Vec<String>>,
    excluded_categories: Option<Vec<String>>,
    synthetic_delays: Option<Vec<String>>,
    memory_dump_config: Option<MemoryDumpConfig>,
}
impl TraceConfigBuilder {
    pub fn record_mode(mut self, record_mode: impl Into<TraceConfigRecordMode>) -> Self {
        self.record_mode = Some(record_mode.into());
        self
    }
    pub fn trace_buffer_size_in_kb(mut self, trace_buffer_size_in_kb: impl Into<f64>) -> Self {
        self.trace_buffer_size_in_kb = Some(trace_buffer_size_in_kb.into());
        self
    }
    pub fn enable_sampling(mut self, enable_sampling: impl Into<bool>) -> Self {
        self.enable_sampling = Some(enable_sampling.into());
        self
    }
    pub fn enable_systrace(mut self, enable_systrace: impl Into<bool>) -> Self {
        self.enable_systrace = Some(enable_systrace.into());
        self
    }
    pub fn enable_argument_filter(mut self, enable_argument_filter: impl Into<bool>) -> Self {
        self.enable_argument_filter = Some(enable_argument_filter.into());
        self
    }
    pub fn included_categorie(mut self, included_categorie: impl Into<String>) -> Self {
        let v = self.included_categories.get_or_insert(Vec::new());
        v.push(included_categorie.into());
        self
    }
    pub fn included_categories<I, S>(mut self, included_categories: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.included_categories.get_or_insert(Vec::new());
        for val in included_categories {
            v.push(val.into());
        }
        self
    }
    pub fn excluded_categorie(mut self, excluded_categorie: impl Into<String>) -> Self {
        let v = self.excluded_categories.get_or_insert(Vec::new());
        v.push(excluded_categorie.into());
        self
    }
    pub fn excluded_categories<I, S>(mut self, excluded_categories: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.excluded_categories.get_or_insert(Vec::new());
        for val in excluded_categories {
            v.push(val.into());
        }
        self
    }
    pub fn synthetic_delay(mut self, synthetic_delay: impl Into<String>) -> Self {
        let v = self.synthetic_delays.get_or_insert(Vec::new());
        v.push(synthetic_delay.into());
        self
    }
    pub fn synthetic_delays<I, S>(mut self, synthetic_delays: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.synthetic_delays.get_or_insert(Vec::new());
        for val in synthetic_delays {
            v.push(val.into());
        }
        self
    }
    pub fn memory_dump_config(mut self, memory_dump_config: impl Into<MemoryDumpConfig>) -> Self {
        self.memory_dump_config = Some(memory_dump_config.into());
        self
    }
    pub fn build(self) -> TraceConfig {
        TraceConfig {
            record_mode: self.record_mode,
            trace_buffer_size_in_kb: self.trace_buffer_size_in_kb,
            enable_sampling: self.enable_sampling,
            enable_systrace: self.enable_systrace,
            enable_argument_filter: self.enable_argument_filter,
            included_categories: self.included_categories,
            excluded_categories: self.excluded_categories,
            synthetic_delays: self.synthetic_delays,
            memory_dump_config: self.memory_dump_config,
        }
    }
}
