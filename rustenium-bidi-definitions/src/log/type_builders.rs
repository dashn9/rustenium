use super::types::*;
impl BaseLogEntry {
    pub fn builder() -> BaseLogEntryBuilder {
        <BaseLogEntryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BaseLogEntryBuilder {
    level: Option<Level>,
    source: Option<crate::script::types::Source>,
    text: Option<String>,
    timestamp: Option<u64>,
    stack_trace: Option<crate::script::types::StackTrace>,
}
impl BaseLogEntryBuilder {
    pub fn level(mut self, level: impl Into<Level>) -> Self {
        self.level = Some(level.into());
        self
    }
    pub fn source(mut self, source: impl Into<crate::script::types::Source>) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn timestamp(mut self, timestamp: impl Into<u64>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
    pub fn stack_trace(mut self, stack_trace: impl Into<crate::script::types::StackTrace>) -> Self {
        self.stack_trace = Some(stack_trace.into());
        self
    }
    pub fn build(self) -> Result<BaseLogEntry, String> {
        Ok(BaseLogEntry {
            level: self
                .level
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(level)))?,
            source: self
                .source
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(source)))?,
            text: self.text,
            timestamp: self
                .timestamp
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(timestamp)))?,
            stack_trace: self.stack_trace,
        })
    }
}
impl GenericLogEntry {
    pub fn builder() -> GenericLogEntryBuilder {
        <GenericLogEntryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GenericLogEntryBuilder {
    base_log_entry: Option<BaseLogEntry>,
    r#type: Option<String>,
}
impl GenericLogEntryBuilder {
    pub fn base_log_entry(mut self, base_log_entry: impl Into<BaseLogEntry>) -> Self {
        self.base_log_entry = Some(base_log_entry.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<GenericLogEntry, String> {
        Ok(GenericLogEntry {
            base_log_entry: self.base_log_entry.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(base_log_entry))
            })?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl ConsoleLogEntry {
    pub fn builder() -> ConsoleLogEntryBuilder {
        <ConsoleLogEntryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ConsoleLogEntryBuilder {
    base_log_entry: Option<BaseLogEntry>,
    r#type: Option<ConsoleLogEntryType>,
    method: Option<String>,
    args: Option<Vec<crate::script::types::RemoteValue>>,
}
impl ConsoleLogEntryBuilder {
    pub fn base_log_entry(mut self, base_log_entry: impl Into<BaseLogEntry>) -> Self {
        self.base_log_entry = Some(base_log_entry.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<ConsoleLogEntryType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
    pub fn arg(mut self, arg: impl Into<crate::script::types::RemoteValue>) -> Self {
        let v = self.args.get_or_insert(Vec::new());
        v.push(arg.into());
        self
    }
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::script::types::RemoteValue>,
    {
        let v = self.args.get_or_insert(Vec::new());
        for val in args {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<ConsoleLogEntry, String> {
        Ok(ConsoleLogEntry {
            base_log_entry: self.base_log_entry.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(base_log_entry))
            })?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            method: self
                .method
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(method)))?,
            args: self
                .args
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(args)))?,
        })
    }
}
impl JavascriptLogEntry {
    pub fn builder() -> JavascriptLogEntryBuilder {
        <JavascriptLogEntryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct JavascriptLogEntryBuilder {
    base_log_entry: Option<BaseLogEntry>,
    r#type: Option<JavascriptLogEntryType>,
}
impl JavascriptLogEntryBuilder {
    pub fn base_log_entry(mut self, base_log_entry: impl Into<BaseLogEntry>) -> Self {
        self.base_log_entry = Some(base_log_entry.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<JavascriptLogEntryType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<JavascriptLogEntry, String> {
        Ok(JavascriptLogEntry {
            base_log_entry: self.base_log_entry.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(base_log_entry))
            })?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
