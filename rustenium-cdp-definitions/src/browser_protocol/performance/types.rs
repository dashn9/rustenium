use serde::{Deserialize, Serialize};
#[doc = "Run-time execution metric.\n[Metric](https://chromedevtools.github.io/devtools-protocol/tot/Performance/#type-Metric)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    #[doc = "Metric name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Metric value."]
    #[serde(rename = "value")]
    pub value: f64,
}
impl Metric {
    pub fn new(name: impl Into<String>, value: impl Into<f64>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
impl Metric {
    pub const IDENTIFIER: &'static str = "Performance.Metric";
}
group_enum ! (Type { Metric (Metric) });
