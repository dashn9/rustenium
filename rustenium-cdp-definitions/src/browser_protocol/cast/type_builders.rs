use super::types::*;
impl Sink {
    pub fn builder() -> SinkBuilder {
        <SinkBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SinkBuilder {
    name: Option<String>,
    id: Option<String>,
    session: Option<String>,
}
impl SinkBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn session(mut self, session: impl Into<String>) -> Self {
        self.session = Some(session.into());
        self
    }
    pub fn build(self) -> Result<Sink, String> {
        Ok(Sink {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            session: self.session,
        })
    }
}
