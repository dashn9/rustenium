// Generated types for module

use serde::{Serialize, Deserialize};
use crate::script::types::RemoteValue;
use crate::script::types::Source;
use crate::script::types::StackTrace;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseLogEntry {
    #[serde(rename = "level")]
    pub level: Level,
    #[serde(rename = "source")]
    pub source: Source,
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "stackTrace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<StackTrace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericLogEntry {
    #[serde(flatten)]
    pub base_log_entry: BaseLogEntry,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsoleEnum {
    #[serde(rename = "console")]
    Console,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleLogEntry {
    #[serde(flatten)]
    pub base_log_entry: BaseLogEntry,
    #[serde(rename = "type")]
    pub r#type: ConsoleEnum,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "args")]
    pub args: Vec<RemoteValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JavascriptEnum {
    #[serde(rename = "javascript")]
    Javascript,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavascriptLogEntry {
    #[serde(flatten)]
    pub base_log_entry: BaseLogEntry,
    #[serde(rename = "type")]
    pub r#type: JavascriptEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Entry {
    GenericLogEntry(GenericLogEntry),
    ConsoleLogEntry(ConsoleLogEntry),
    JavascriptLogEntry(JavascriptLogEntry),
}

