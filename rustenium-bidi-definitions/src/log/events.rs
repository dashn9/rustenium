use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryAddedParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EntryAddedMethod {
    #[serde(rename = "log.entryAdded")]
    EntryAdded,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryAdded {
    pub method: EntryAddedMethod,
    pub params: EntryAddedParams,
}
impl EntryAdded {
    pub const IDENTIFIER: &'static str = "log.entryAdded";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
group_enum ! (LogEvent { EntryAdded (EntryAdded) });
