use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sink {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "Text describing the current session. Present only if there is an active\nsession on the sink."]
    #[serde(rename = "session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub session: Option<String>,
}
impl Sink {
    pub fn new(name: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            id: id.into(),
            session: None,
        }
    }
}
impl Sink {
    pub const IDENTIFIER: &'static str = "Cast.Sink";
}
group_enum ! (Type { Sink (Sink) });
