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
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericLogEntry {
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
    #[serde(rename = "type")]
    pub r#type: String,
}
impl GenericLogEntry {
    pub fn new(
        level: impl Into<Level>,
        source: impl Into<crate::script::types::Source>,
        timestamp: impl Into<u64>,
        r#type: impl Into<String>,
    ) -> Self {
        Self {
            level: level.into(),
            source: source.into(),
            timestamp: timestamp.into(),
            r#type: r#type.into(),
            text: None,
            stack_trace: None,
        }
    }
}
impl GenericLogEntry {
    pub const IDENTIFIER: &'static str = "log.GenericLogEntry";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsoleLogEntry {
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
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<crate::script::types::RemoteValue>,
}
impl ConsoleLogEntry {
    pub const IDENTIFIER: &'static str = "log.ConsoleLogEntry";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JavascriptLogEntry {
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
    #[serde(rename = "type")]
    pub r#type: String,
}
impl JavascriptLogEntry {
    pub fn new(
        level: impl Into<Level>,
        source: impl Into<crate::script::types::Source>,
        timestamp: impl Into<u64>,
        r#type: impl Into<String>,
    ) -> Self {
        Self {
            level: level.into(),
            source: source.into(),
            timestamp: timestamp.into(),
            r#type: r#type.into(),
            text: None,
            stack_trace: None,
        }
    }
}
impl JavascriptLogEntry {
    pub const IDENTIFIER: &'static str = "log.JavascriptLogEntry";
}
group_enum ! (LogTypes { Level (Level) , Entry (Entry) , BaseLogEntry (BaseLogEntry) , GenericLogEntry (GenericLogEntry) , ConsoleLogEntry (ConsoleLogEntry) , JavascriptLogEntry (JavascriptLogEntry) });
