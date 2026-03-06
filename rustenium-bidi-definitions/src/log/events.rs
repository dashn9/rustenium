use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EntryAddedParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EntryAddedMethod {
    #[serde(rename = "log.entryAdded")]
    EntryAdded,
}
impl EntryAddedMethod {
    pub const IDENTIFIER: &'static str = "log.entryAdded";
}
#[derive(Debug, Clone, PartialEq)]
pub struct EntryAdded {
    pub method: EntryAddedMethod,
    pub params: EntryAddedParams,
}
group_enum ! (LogEvents { EntryAdded (EntryAdded) });
