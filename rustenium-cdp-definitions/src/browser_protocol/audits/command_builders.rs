use super::commands::*;
impl GetEncodedResponse {
    pub fn builder() -> GetEncodedResponseBuilder {
        <GetEncodedResponseBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetEncodedResponseBuilder {
    request_id: Option<crate::browser_protocol::network::types::RequestId>,
    encoding: Option<GetEncodedResponseEncoding>,
    quality: Option<f64>,
    size_only: Option<bool>,
}
impl GetEncodedResponseBuilder {
    pub fn request_id(
        mut self,
        request_id: impl Into<crate::browser_protocol::network::types::RequestId>,
    ) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn encoding(mut self, encoding: impl Into<GetEncodedResponseEncoding>) -> Self {
        self.encoding = Some(encoding.into());
        self
    }
    pub fn quality(mut self, quality: impl Into<f64>) -> Self {
        self.quality = Some(quality.into());
        self
    }
    pub fn size_only(mut self, size_only: impl Into<bool>) -> Self {
        self.size_only = Some(size_only.into());
        self
    }
    pub fn build(self) -> Result<GetEncodedResponse, String> {
        Ok(GetEncodedResponse {
            method: GetEncodedResponseMethod::GetEncodedResponse,
            params: GetEncodedResponseParams {
                request_id: self.request_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(request_id))
                })?,
                encoding: self.encoding.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(encoding))
                })?,
                quality: self.quality,
                size_only: self.size_only,
            },
        })
    }
}
impl CheckContrast {
    pub fn builder() -> CheckContrastBuilder {
        <CheckContrastBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CheckContrastBuilder {
    report_aaa: Option<bool>,
}
impl CheckContrastBuilder {
    pub fn report_aaa(mut self, report_aaa: impl Into<bool>) -> Self {
        self.report_aaa = Some(report_aaa.into());
        self
    }
    pub fn build(self) -> CheckContrast {
        CheckContrast {
            method: CheckContrastMethod::CheckContrast,
            params: CheckContrastParams {
                report_aaa: self.report_aaa,
            },
        }
    }
}
