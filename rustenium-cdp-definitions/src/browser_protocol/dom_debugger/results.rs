use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEventListenersReturns {
    #[doc = "Array of relevant listeners."]
    #[serde(rename = "listeners")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub listeners: Vec<super::types::EventListener>,
}
