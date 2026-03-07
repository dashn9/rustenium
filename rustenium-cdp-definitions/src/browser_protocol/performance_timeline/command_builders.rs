use super::commands::*;
impl Enable {
    pub fn builder() -> EnableBuilder {
        <EnableBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct EnableBuilder {
    event_types: Option<Vec<String>>,
}
impl EnableBuilder {
    pub fn event_type(mut self, event_type: impl Into<String>) -> Self {
        let v = self.event_types.get_or_insert(Vec::new());
        v.push(event_type.into());
        self
    }
    pub fn event_types<I, S>(mut self, event_types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.event_types.get_or_insert(Vec::new());
        for val in event_types {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<Enable, String> {
        Ok(Enable {
            method: EnableMethod::Enable,
            params: EnableParams {
                event_types: self.event_types.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(event_types))
                })?,
            },
        })
    }
}
