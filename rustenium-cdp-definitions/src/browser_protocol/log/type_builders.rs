use super::types::*;
impl LogEntry {
    pub fn builder() -> LogEntryBuilder {
        <LogEntryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct LogEntryBuilder {
    source: Option<LogEntrySource>,
    level: Option<LogEntryLevel>,
    text: Option<String>,
    category: Option<LogEntryCategory>,
    timestamp: Option<crate::js_protocol::runtime::types::Timestamp>,
    url: Option<String>,
    line_number: Option<i64>,
    stack_trace: Option<crate::js_protocol::runtime::types::StackTrace>,
    network_request_id: Option<crate::browser_protocol::network::types::RequestId>,
    worker_id: Option<String>,
    args: Option<Vec<crate::js_protocol::runtime::types::RemoteObject>>,
}
impl LogEntryBuilder {
    pub fn source(mut self, source: impl Into<LogEntrySource>) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn level(mut self, level: impl Into<LogEntryLevel>) -> Self {
        self.level = Some(level.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn category(mut self, category: impl Into<LogEntryCategory>) -> Self {
        self.category = Some(category.into());
        self
    }
    pub fn timestamp(
        mut self,
        timestamp: impl Into<crate::js_protocol::runtime::types::Timestamp>,
    ) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn line_number(mut self, line_number: impl Into<i64>) -> Self {
        self.line_number = Some(line_number.into());
        self
    }
    pub fn stack_trace(
        mut self,
        stack_trace: impl Into<crate::js_protocol::runtime::types::StackTrace>,
    ) -> Self {
        self.stack_trace = Some(stack_trace.into());
        self
    }
    pub fn network_request_id(
        mut self,
        network_request_id: impl Into<crate::browser_protocol::network::types::RequestId>,
    ) -> Self {
        self.network_request_id = Some(network_request_id.into());
        self
    }
    pub fn worker_id(mut self, worker_id: impl Into<String>) -> Self {
        self.worker_id = Some(worker_id.into());
        self
    }
    pub fn arg(mut self, arg: impl Into<crate::js_protocol::runtime::types::RemoteObject>) -> Self {
        let v = self.args.get_or_insert(Vec::new());
        v.push(arg.into());
        self
    }
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::js_protocol::runtime::types::RemoteObject>,
    {
        let v = self.args.get_or_insert(Vec::new());
        for val in args {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<LogEntry, String> {
        Ok(LogEntry {
            source: self
                .source
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(source)))?,
            level: self
                .level
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(level)))?,
            text: self
                .text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
            category: self.category,
            timestamp: self
                .timestamp
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(timestamp)))?,
            url: self.url,
            line_number: self.line_number,
            stack_trace: self.stack_trace,
            network_request_id: self.network_request_id,
            worker_id: self.worker_id,
            args: self.args,
        })
    }
}
impl ViolationSetting {
    pub fn builder() -> ViolationSettingBuilder {
        <ViolationSettingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ViolationSettingBuilder {
    name: Option<ViolationSettingName>,
    threshold: Option<f64>,
}
impl ViolationSettingBuilder {
    pub fn name(mut self, name: impl Into<ViolationSettingName>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn threshold(mut self, threshold: impl Into<f64>) -> Self {
        self.threshold = Some(threshold.into());
        self
    }
    pub fn build(self) -> Result<ViolationSetting, String> {
        Ok(ViolationSetting {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            threshold: self
                .threshold
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(threshold)))?,
        })
    }
}
