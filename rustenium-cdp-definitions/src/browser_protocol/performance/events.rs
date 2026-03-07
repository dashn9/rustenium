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
impl MetricsMethod {
    pub const IDENTIFIER: &'static str = "Performance.metrics";
}
#[doc = "Current values of the metrics.\n[metrics](https://chromedevtools.github.io/devtools-protocol/tot/Performance/#event-metrics)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Metrics {
    pub method: MetricsMethod,
    pub params: MetricsParams,
}
group_enum ! (PerformanceEvents { Metrics (Metrics) });
