use serde::{Deserialize, Serialize};
#[doc = "Current values of the metrics.\n[metrics](https://chromedevtools.github.io/devtools-protocol/tot/Performance/#event-metrics)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricsParams {
    #[doc = "Current values of the metrics."]
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<super::types::Metric>,
    #[doc = "Timestamp title."]
    #[serde(rename = "title")]
    pub title: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetricsMethod {
    #[serde(rename = "Performance.metrics")]
    Metrics,
}
#[doc = "Current values of the metrics.\n[metrics](https://chromedevtools.github.io/devtools-protocol/tot/Performance/#event-metrics)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    pub method: MetricsMethod,
    pub params: MetricsParams,
}
impl Metrics {
    pub const IDENTIFIER: &'static str = "Performance.metrics";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (PerformanceEvents { Metrics (Metrics) } + identifiable);
