use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRealtimeDataReturns {
    #[serde(rename = "realtimeData")]
    pub realtime_data: super::types::ContextRealtimeData,
}
