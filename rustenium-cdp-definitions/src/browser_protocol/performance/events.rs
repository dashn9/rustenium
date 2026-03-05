use serde::{Deserialize, Serialize};
#[doc = "Current values of the metrics.\n[metrics](https://chromedevtools.github.io/devtools-protocol/tot/Performance/#event-metrics)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    #[doc = "Current values of the metrics."]
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<super::types::Metric>,
    #[doc = "Timestamp title."]
    #[serde(rename = "title")]
    pub title: String,
}
impl Metrics {
    pub const IDENTIFIER: &'static str = "Performance.metrics";
}
group_enum ! (PerformanceEvents { Metrics (Metrics) });
