use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetTimeDomainResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMetricsResult {
    #[doc = "Current values for run-time metrics."]
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<super::types::Metric>,
}
