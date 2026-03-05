use serde::{Deserialize, Serialize};
#[doc = "Log entry.\n[LogEntry](https://chromedevtools.github.io/devtools-protocol/tot/Log/#type-LogEntry)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    #[doc = "Log entry source."]
    #[serde(rename = "source")]
    pub source: LogEntrySource,
    #[doc = "Log entry severity."]
    #[serde(rename = "level")]
    pub level: LogEntryLevel,
    #[doc = "Logged text."]
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub category: Option<LogEntryCategory>,
    #[doc = "Timestamp when this entry was added."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::super::super::js_protocol::runtime::types::Timestamp,
    #[doc = "URL of the resource if known."]
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
    #[doc = "Line number in the resource."]
    #[serde(rename = "lineNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub line_number: Option<i64>,
    #[doc = "JavaScript stack trace."]
    #[serde(rename = "stackTrace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stack_trace: Option<super::super::super::js_protocol::runtime::types::StackTrace>,
    #[doc = "Identifier of the network request associated with this entry."]
    #[serde(rename = "networkRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub network_request_id: Option<super::super::network::types::RequestId>,
    #[doc = "Identifier of the worker associated with this entry."]
    #[serde(rename = "workerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub worker_id: Option<String>,
    #[doc = "Call arguments."]
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub args: Option<Vec<super::super::super::js_protocol::runtime::types::RemoteObject>>,
}
#[doc = "Log entry source."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogEntrySource {
    #[serde(rename = "xml")]
    Xml,
    #[serde(rename = "javascript")]
    Javascript,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "storage")]
    Storage,
    #[serde(rename = "appcache")]
    Appcache,
    #[serde(rename = "rendering")]
    Rendering,
    #[serde(rename = "security")]
    Security,
    #[serde(rename = "deprecation")]
    Deprecation,
    #[serde(rename = "worker")]
    Worker,
    #[serde(rename = "violation")]
    Violation,
    #[serde(rename = "intervention")]
    Intervention,
    #[serde(rename = "recommendation")]
    Recommendation,
    #[serde(rename = "other")]
    Other,
}
#[doc = "Log entry severity."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogEntryLevel {
    #[serde(rename = "verbose")]
    Verbose,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogEntryCategory {
    #[serde(rename = "cors")]
    Cors,
}
impl LogEntry {
    pub fn new(
        source: impl Into<LogEntrySource>,
        level: impl Into<LogEntryLevel>,
        text: impl Into<String>,
        timestamp: impl Into<super::super::super::js_protocol::runtime::types::Timestamp>,
    ) -> Self {
        Self {
            source: source.into(),
            level: level.into(),
            text: text.into(),
            timestamp: timestamp.into(),
            category: None,
            url: None,
            line_number: None,
            stack_trace: None,
            network_request_id: None,
            worker_id: None,
            args: None,
        }
    }
}
impl LogEntry {
    pub const IDENTIFIER: &'static str = "Log.LogEntry";
}
#[doc = "Violation configuration setting.\n[ViolationSetting](https://chromedevtools.github.io/devtools-protocol/tot/Log/#type-ViolationSetting)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViolationSetting {
    #[doc = "Violation type."]
    #[serde(rename = "name")]
    pub name: ViolationSettingName,
    #[doc = "Time threshold to trigger upon."]
    #[serde(rename = "threshold")]
    pub threshold: f64,
}
#[doc = "Violation type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ViolationSettingName {
    #[serde(rename = "longTask")]
    LongTask,
    #[serde(rename = "longLayout")]
    LongLayout,
    #[serde(rename = "blockedEvent")]
    BlockedEvent,
    #[serde(rename = "blockedParser")]
    BlockedParser,
    #[serde(rename = "discouragedAPIUse")]
    DiscouragedApiUse,
    #[serde(rename = "handler")]
    Handler,
    #[serde(rename = "recurringHandler")]
    RecurringHandler,
}
impl ViolationSetting {
    pub fn new(name: impl Into<ViolationSettingName>, threshold: impl Into<f64>) -> Self {
        Self {
            name: name.into(),
            threshold: threshold.into(),
        }
    }
}
impl ViolationSetting {
    pub const IDENTIFIER: &'static str = "Log.ViolationSetting";
}
group_enum ! (LogTypes { LogEntry (LogEntry) , ViolationSetting (ViolationSetting) });
