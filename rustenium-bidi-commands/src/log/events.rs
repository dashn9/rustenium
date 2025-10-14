// Generated events for module

use serde::{Serialize, Deserialize};
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogEvent {
    EntryAdded(EntryAdded),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogEntryAddedMethod {
    #[serde(rename = "log.EntryAdded")]
    LogEntryAdded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryAdded {
    #[serde(rename = "method")]
    pub method: LogEntryAddedMethod,
    #[serde(rename = "params")]
    pub params: Entry,
}

