use serde::{Deserialize, Serialize};
#[doc = "Issued when new message was logged.\n[entryAdded](https://chromedevtools.github.io/devtools-protocol/tot/Log/#event-entryAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryAdded {
    #[doc = "The entry."]
    #[serde(rename = "entry")]
    pub entry: super::types::LogEntry,
}
impl EntryAdded {
    pub const IDENTIFIER: &'static str = "Log.entryAdded";
}
group_enum ! (LogEvents { EntryAdded (EntryAdded) });
