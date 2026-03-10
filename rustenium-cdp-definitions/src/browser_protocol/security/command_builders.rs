use super::commands::*;
#[derive(Debug, Clone, Default)]
pub struct DisableBuilder;
impl DisableBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Disable {
        Disable {
            method: DisableMethod::Disable,
            params: DisableParams {},
        }
    }
}
impl Disable {
    pub fn builder() -> DisableBuilder {
        DisableBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct EnableBuilder;
impl EnableBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Enable {
        Enable {
            method: EnableMethod::Enable,
            params: EnableParams {},
        }
    }
}
impl Enable {
    pub fn builder() -> EnableBuilder {
        EnableBuilder
    }
}
impl SetIgnoreCertificateErrors {
    pub fn builder() -> SetIgnoreCertificateErrorsBuilder {
        <SetIgnoreCertificateErrorsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetIgnoreCertificateErrorsBuilder {
    ignore: Option<bool>,
}
impl SetIgnoreCertificateErrorsBuilder {
    pub fn ignore(mut self, ignore: impl Into<bool>) -> Self {
        self.ignore = Some(ignore.into());
        self
    }
    pub fn build(self) -> Result<SetIgnoreCertificateErrors, String> {
        Ok(SetIgnoreCertificateErrors {
            method: SetIgnoreCertificateErrorsMethod::SetIgnoreCertificateErrors,
            params: SetIgnoreCertificateErrorsParams {
                ignore: self
                    .ignore
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(ignore)))?,
            },
        })
    }
}
