use super::commands::*;
impl Bind {
    pub fn builder() -> BindBuilder {
        <BindBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BindBuilder {
    port: Option<i64>,
}
impl BindBuilder {
    pub fn port(mut self, port: impl Into<i64>) -> Self {
        self.port = Some(port.into());
        self
    }
    pub fn build(self) -> Result<Bind, String> {
        Ok(Bind {
            method: BindMethod::Bind,
            params: BindParams {
                port: self
                    .port
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(port)))?,
            },
        })
    }
}
impl Unbind {
    pub fn builder() -> UnbindBuilder {
        <UnbindBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UnbindBuilder {
    port: Option<i64>,
}
impl UnbindBuilder {
    pub fn port(mut self, port: impl Into<i64>) -> Self {
        self.port = Some(port.into());
        self
    }
    pub fn build(self) -> Result<Unbind, String> {
        Ok(Unbind {
            method: UnbindMethod::Unbind,
            params: UnbindParams {
                port: self
                    .port
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(port)))?,
            },
        })
    }
}
