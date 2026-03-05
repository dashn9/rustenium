use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRealtimeDataResult {
    #[serde(rename = "realtimeData")]
    pub realtime_data: super::types::ContextRealtimeData,
}
