use super::types::*;
impl Metric {
    pub fn builder() -> MetricBuilder {
        MetricBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct MetricBuilder {
    name: Option<String>,
    value: Option<f64>,
}
impl MetricBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<f64>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<Metric, String> {
        Ok(Metric {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
