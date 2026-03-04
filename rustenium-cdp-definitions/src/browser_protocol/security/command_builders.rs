use super::commands::*;
impl SetIgnoreCertificateErrors {
    pub fn builder() -> SetIgnoreCertificateErrorsBuilder {
        SetIgnoreCertificateErrorsBuilder::default()
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
