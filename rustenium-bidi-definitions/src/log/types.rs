use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Level {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Entry {
    GenericLogEntry(GenericLogEntry),
    ConsoleLogEntry(ConsoleLogEntry),
    JavascriptLogEntry(JavascriptLogEntry),
}
impl From<GenericLogEntry> for Entry {
    fn from(v: GenericLogEntry) -> Self {
        Entry::GenericLogEntry(v)
    }
}
impl TryFrom<Entry> for GenericLogEntry {
    type Error = Entry;
    fn try_from(e: Entry) -> Result<Self, Self::Error> {
        match e {
            Entry::GenericLogEntry(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<ConsoleLogEntry> for Entry {
    fn from(v: ConsoleLogEntry) -> Self {
        Entry::ConsoleLogEntry(v)
    }
}
impl TryFrom<Entry> for ConsoleLogEntry {
    type Error = Entry;
    fn try_from(e: Entry) -> Result<Self, Self::Error> {
        match e {
            Entry::ConsoleLogEntry(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<JavascriptLogEntry> for Entry {
    fn from(v: JavascriptLogEntry) -> Self {
        Entry::JavascriptLogEntry(v)
    }
}
impl TryFrom<Entry> for JavascriptLogEntry {
    type Error = Entry;
    fn try_from(e: Entry) -> Result<Self, Self::Error> {
        match e {
            Entry::JavascriptLogEntry(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseLogEntry {
    #[serde(rename = "level")]
    pub level: Level,
    #[serde(rename = "source")]
    pub source: crate::script::types::Source,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub text: Option<String>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "stackTrace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stack_trace: Option<crate::script::types::StackTrace>,
}
impl BaseLogEntry {
    pub fn new(
        level: impl Into<Level>,
        source: impl Into<crate::script::types::Source>,
        timestamp: impl Into<u64>,
    ) -> Self {
        Self {
            level: level.into(),
            source: source.into(),
            timestamp: timestamp.into(),
            text: None,
            stack_trace: None,
        }
    }
}
impl BaseLogEntry {
    pub const IDENTIFIER: &'static str = "log.BaseLogEntry";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericLogEntry {
    #[serde(flatten)]
    #[serde(default)]
    pub base_log_entry: BaseLogEntry,
    #[serde(rename = "type")]
    pub r#type: String,
}
impl GenericLogEntry {
    pub fn new(base_log_entry: impl Into<BaseLogEntry>, r#type: impl Into<String>) -> Self {
        Self {
            base_log_entry: base_log_entry.into(),
            r#type: r#type.into(),
        }
    }
}
impl GenericLogEntry {
    pub const IDENTIFIER: &'static str = "log.GenericLogEntry";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsoleLogEntry {
    #[serde(flatten)]
    #[serde(default)]
    pub base_log_entry: BaseLogEntry,
    #[serde(rename = "type")]
    pub r#type: ConsoleLogEntryType,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<crate::script::types::RemoteValue>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsoleLogEntryType {
    #[serde(rename = "console")]
    Console,
}
impl ConsoleLogEntry {
    pub fn new(
        base_log_entry: impl Into<BaseLogEntry>,
        r#type: impl Into<ConsoleLogEntryType>,
        method: impl Into<String>,
        args: Vec<crate::script::types::RemoteValue>,
    ) -> Self {
        Self {
            base_log_entry: base_log_entry.into(),
            r#type: r#type.into(),
            method: method.into(),
            args,
        }
    }
}
impl ConsoleLogEntry {
    pub const IDENTIFIER: &'static str = "log.ConsoleLogEntry";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JavascriptLogEntry {
    #[serde(flatten)]
    #[serde(default)]
    pub base_log_entry: BaseLogEntry,
    #[serde(rename = "type")]
    pub r#type: JavascriptLogEntryType,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JavascriptLogEntryType {
    #[serde(rename = "javascript")]
    Javascript,
}
impl JavascriptLogEntry {
    pub fn new(
        base_log_entry: impl Into<BaseLogEntry>,
        r#type: impl Into<JavascriptLogEntryType>,
    ) -> Self {
        Self {
            base_log_entry: base_log_entry.into(),
            r#type: r#type.into(),
        }
    }
}
impl JavascriptLogEntry {
    pub const IDENTIFIER: &'static str = "log.JavascriptLogEntry";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
