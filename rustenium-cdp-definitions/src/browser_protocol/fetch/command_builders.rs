use super::commands::*;
impl Enable {
    pub fn builder() -> EnableBuilder {
        EnableBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct EnableBuilder {
    patterns: Option<Vec<super::types::RequestPattern>>,
    handle_auth_requests: Option<bool>,
}
impl EnableBuilder {
    pub fn pattern(mut self, pattern: impl Into<super::types::RequestPattern>) -> Self {
        let v = self.patterns.get_or_insert(Vec::new());
        v.push(pattern.into());
        self
    }
    pub fn patterns<I, S>(mut self, patterns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::RequestPattern>,
    {
        let v = self.patterns.get_or_insert(Vec::new());
        for val in patterns {
            v.push(val.into());
        }
        self
    }
    pub fn handle_auth_requests(mut self, handle_auth_requests: impl Into<bool>) -> Self {
        self.handle_auth_requests = Some(handle_auth_requests.into());
        self
    }
    pub fn build(self) -> Enable {
        Enable {
            method: EnableMethod::Enable,
            params: EnableParams {
                patterns: self.patterns,
                handle_auth_requests: self.handle_auth_requests,
            },
        }
    }
}
impl FailRequest {
    pub fn builder() -> FailRequestBuilder {
        FailRequestBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FailRequestBuilder {
    request_id: Option<super::types::RequestId>,
    error_reason: Option<super::super::network::types::ErrorReason>,
}
impl FailRequestBuilder {
    pub fn request_id(mut self, request_id: impl Into<super::types::RequestId>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn error_reason(
        mut self,
        error_reason: impl Into<super::super::network::types::ErrorReason>,
    ) -> Self {
        self.error_reason = Some(error_reason.into());
        self
    }
    pub fn build(self) -> Result<FailRequest, String> {
        Ok(FailRequest {
            method: FailRequestMethod::FailRequest,
            params: FailRequestParams {
                request_id: self.request_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(request_id))
                })?,
                error_reason: self.error_reason.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(error_reason))
                })?,
            },
        })
    }
}
impl FulfillRequest {
    pub fn builder() -> FulfillRequestBuilder {
        FulfillRequestBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FulfillRequestBuilder {
    request_id: Option<super::types::RequestId>,
    response_code: Option<i64>,
    response_headers: Option<Vec<super::types::HeaderEntry>>,
    binary_response_headers: Option<super::super::super::Binary>,
    body: Option<super::super::super::Binary>,
    response_phrase: Option<String>,
}
impl FulfillRequestBuilder {
    pub fn request_id(mut self, request_id: impl Into<super::types::RequestId>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn response_code(mut self, response_code: impl Into<i64>) -> Self {
        self.response_code = Some(response_code.into());
        self
    }
    pub fn response_header(
        mut self,
        response_header: impl Into<super::types::HeaderEntry>,
    ) -> Self {
        let v = self.response_headers.get_or_insert(Vec::new());
        v.push(response_header.into());
        self
    }
    pub fn response_headers<I, S>(mut self, response_headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::HeaderEntry>,
    {
        let v = self.response_headers.get_or_insert(Vec::new());
        for val in response_headers {
            v.push(val.into());
        }
        self
    }
    pub fn binary_response_headers(
        mut self,
        binary_response_headers: impl Into<super::super::super::Binary>,
    ) -> Self {
        self.binary_response_headers = Some(binary_response_headers.into());
        self
    }
    pub fn body(mut self, body: impl Into<super::super::super::Binary>) -> Self {
        self.body = Some(body.into());
        self
    }
    pub fn response_phrase(mut self, response_phrase: impl Into<String>) -> Self {
        self.response_phrase = Some(response_phrase.into());
        self
    }
    pub fn build(self) -> Result<FulfillRequest, String> {
        Ok(FulfillRequest {
            method: FulfillRequestMethod::FulfillRequest,
            params: FulfillRequestParams {
                request_id: self.request_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(request_id))
                })?,
                response_code: self.response_code.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(response_code))
                })?,
                response_headers: self.response_headers,
                binary_response_headers: self.binary_response_headers,
                body: self.body,
                response_phrase: self.response_phrase,
            },
        })
    }
}
impl ContinueRequest {
    pub fn builder() -> ContinueRequestBuilder {
        ContinueRequestBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ContinueRequestBuilder {
    request_id: Option<super::types::RequestId>,
    url: Option<String>,
    method: Option<String>,
    post_data: Option<super::super::super::Binary>,
    headers: Option<Vec<super::types::HeaderEntry>>,
    intercept_response: Option<bool>,
}
impl ContinueRequestBuilder {
    pub fn request_id(mut self, request_id: impl Into<super::types::RequestId>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
    pub fn post_data(mut self, post_data: impl Into<super::super::super::Binary>) -> Self {
        self.post_data = Some(post_data.into());
        self
    }
    pub fn header(mut self, header: impl Into<super::types::HeaderEntry>) -> Self {
        let v = self.headers.get_or_insert(Vec::new());
        v.push(header.into());
        self
    }
    pub fn headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::HeaderEntry>,
    {
        let v = self.headers.get_or_insert(Vec::new());
        for val in headers {
            v.push(val.into());
        }
        self
    }
    pub fn intercept_response(mut self, intercept_response: impl Into<bool>) -> Self {
        self.intercept_response = Some(intercept_response.into());
        self
    }
    pub fn build(self) -> Result<ContinueRequest, String> {
        Ok(ContinueRequest {
            method: ContinueRequestMethod::ContinueRequest,
            params: ContinueRequestParams {
                request_id: self.request_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(request_id))
                })?,
                url: self.url,
                method: self.method,
                post_data: self.post_data,
                headers: self.headers,
                intercept_response: self.intercept_response,
            },
        })
    }
}
impl ContinueWithAuth {
    pub fn builder() -> ContinueWithAuthBuilder {
        ContinueWithAuthBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ContinueWithAuthBuilder {
    request_id: Option<super::types::RequestId>,
    auth_challenge_response: Option<super::types::AuthChallengeResponse>,
}
impl ContinueWithAuthBuilder {
    pub fn request_id(mut self, request_id: impl Into<super::types::RequestId>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn auth_challenge_response(
        mut self,
        auth_challenge_response: impl Into<super::types::AuthChallengeResponse>,
    ) -> Self {
        self.auth_challenge_response = Some(auth_challenge_response.into());
        self
    }
    pub fn build(self) -> Result<ContinueWithAuth, String> {
        Ok(ContinueWithAuth {
            method: ContinueWithAuthMethod::ContinueWithAuth,
            params: ContinueWithAuthParams {
                request_id: self.request_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(request_id))
                })?,
                auth_challenge_response: self.auth_challenge_response.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(auth_challenge_response)
                    )
                })?,
            },
        })
    }
}
impl ContinueResponse {
    pub fn builder() -> ContinueResponseBuilder {
        ContinueResponseBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ContinueResponseBuilder {
    request_id: Option<super::types::RequestId>,
    response_code: Option<i64>,
    response_phrase: Option<String>,
    response_headers: Option<Vec<super::types::HeaderEntry>>,
    binary_response_headers: Option<super::super::super::Binary>,
}
impl ContinueResponseBuilder {
    pub fn request_id(mut self, request_id: impl Into<super::types::RequestId>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn response_code(mut self, response_code: impl Into<i64>) -> Self {
        self.response_code = Some(response_code.into());
        self
    }
    pub fn response_phrase(mut self, response_phrase: impl Into<String>) -> Self {
        self.response_phrase = Some(response_phrase.into());
        self
    }
    pub fn response_header(
        mut self,
        response_header: impl Into<super::types::HeaderEntry>,
    ) -> Self {
        let v = self.response_headers.get_or_insert(Vec::new());
        v.push(response_header.into());
        self
    }
    pub fn response_headers<I, S>(mut self, response_headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::HeaderEntry>,
    {
        let v = self.response_headers.get_or_insert(Vec::new());
        for val in response_headers {
            v.push(val.into());
        }
        self
    }
    pub fn binary_response_headers(
        mut self,
        binary_response_headers: impl Into<super::super::super::Binary>,
    ) -> Self {
        self.binary_response_headers = Some(binary_response_headers.into());
        self
    }
    pub fn build(self) -> Result<ContinueResponse, String> {
        Ok(ContinueResponse {
            method: ContinueResponseMethod::ContinueResponse,
            params: ContinueResponseParams {
                request_id: self.request_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(request_id))
                })?,
                response_code: self.response_code,
                response_phrase: self.response_phrase,
                response_headers: self.response_headers,
                binary_response_headers: self.binary_response_headers,
            },
        })
    }
}
impl GetResponseBody {
    pub fn builder() -> GetResponseBodyBuilder {
        GetResponseBodyBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetResponseBodyBuilder {
    request_id: Option<super::types::RequestId>,
}
impl GetResponseBodyBuilder {
    pub fn request_id(mut self, request_id: impl Into<super::types::RequestId>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn build(self) -> Result<GetResponseBody, String> {
        Ok(GetResponseBody {
            method: GetResponseBodyMethod::GetResponseBody,
            params: GetResponseBodyParams {
                request_id: self.request_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(request_id))
                })?,
            },
        })
    }
}
impl TakeResponseBodyAsStream {
    pub fn builder() -> TakeResponseBodyAsStreamBuilder {
        TakeResponseBodyAsStreamBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct TakeResponseBodyAsStreamBuilder {
    request_id: Option<super::types::RequestId>,
}
impl TakeResponseBodyAsStreamBuilder {
    pub fn request_id(mut self, request_id: impl Into<super::types::RequestId>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn build(self) -> Result<TakeResponseBodyAsStream, String> {
        Ok(TakeResponseBodyAsStream {
            method: TakeResponseBodyAsStreamMethod::TakeResponseBodyAsStream,
            params: TakeResponseBodyAsStreamParams {
                request_id: self.request_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(request_id))
                })?,
            },
        })
    }
}
