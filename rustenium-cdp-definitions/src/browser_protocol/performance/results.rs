use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMetricsReturns {
    #[doc = "Current values for run-time metrics."]
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<super::types::Metric>,
}
