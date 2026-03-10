use super::commands::*;
impl AddDataCollector {
    pub fn builder() -> AddDataCollectorBuilder {
        <AddDataCollectorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AddDataCollectorBuilder {
    data_types: Option<Vec<super::types::DataType>>,
    max_encoded_data_size: Option<u64>,
    collector_type: Option<super::types::CollectorType>,
    contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl AddDataCollectorBuilder {
    pub fn data_type(mut self, data_type: impl Into<super::types::DataType>) -> Self {
        let v = self.data_types.get_or_insert(Vec::new());
        v.push(data_type.into());
        self
    }
    pub fn data_types<I, S>(mut self, data_types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::DataType>,
    {
        let v = self.data_types.get_or_insert(Vec::new());
        for val in data_types {
            v.push(val.into());
        }
        self
    }
    pub fn max_encoded_data_size(mut self, max_encoded_data_size: impl Into<u64>) -> Self {
        self.max_encoded_data_size = Some(max_encoded_data_size.into());
        self
    }
    pub fn collector_type(
        mut self,
        collector_type: impl Into<super::types::CollectorType>,
    ) -> Self {
        self.collector_type = Some(collector_type.into());
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        let v = self.contexts.get_or_insert(Vec::new());
        v.push(context.into());
        self
    }
    pub fn contexts<I, S>(mut self, contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browsing_context::types::BrowsingContext>,
    {
        let v = self.contexts.get_or_insert(Vec::new());
        for val in contexts {
            v.push(val.into());
        }
        self
    }
    pub fn user_context(
        mut self,
        user_context: impl Into<crate::browser::types::UserContext>,
    ) -> Self {
        let v = self.user_contexts.get_or_insert(Vec::new());
        v.push(user_context.into());
        self
    }
    pub fn user_contexts<I, S>(mut self, user_contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser::types::UserContext>,
    {
        let v = self.user_contexts.get_or_insert(Vec::new());
        for val in user_contexts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<AddDataCollector, String> {
        Ok(AddDataCollector {
            method: AddDataCollectorMethod::AddDataCollector,
            params: AddDataCollectorParams {
                data_types: self.data_types.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(data_types))
                })?,
                max_encoded_data_size: self.max_encoded_data_size.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(max_encoded_data_size)
                    )
                })?,
                collector_type: self.collector_type,
                contexts: self.contexts,
                user_contexts: self.user_contexts,
            },
        })
    }
}
impl AddIntercept {
    pub fn builder() -> AddInterceptBuilder {
        <AddInterceptBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AddInterceptBuilder {
    phases: Option<Vec<super::types::InterceptPhase>>,
    contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    url_patterns: Option<Vec<super::types::UrlPattern>>,
}
impl AddInterceptBuilder {
    pub fn phase(mut self, phase: impl Into<super::types::InterceptPhase>) -> Self {
        let v = self.phases.get_or_insert(Vec::new());
        v.push(phase.into());
        self
    }
    pub fn phases<I, S>(mut self, phases: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::InterceptPhase>,
    {
        let v = self.phases.get_or_insert(Vec::new());
        for val in phases {
            v.push(val.into());
        }
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        let v = self.contexts.get_or_insert(Vec::new());
        v.push(context.into());
        self
    }
    pub fn contexts<I, S>(mut self, contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browsing_context::types::BrowsingContext>,
    {
        let v = self.contexts.get_or_insert(Vec::new());
        for val in contexts {
            v.push(val.into());
        }
        self
    }
    pub fn url_pattern(mut self, url_pattern: impl Into<super::types::UrlPattern>) -> Self {
        let v = self.url_patterns.get_or_insert(Vec::new());
        v.push(url_pattern.into());
        self
    }
    pub fn url_patterns<I, S>(mut self, url_patterns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::UrlPattern>,
    {
        let v = self.url_patterns.get_or_insert(Vec::new());
        for val in url_patterns {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<AddIntercept, String> {
        Ok(AddIntercept {
            method: AddInterceptMethod::AddIntercept,
            params: AddInterceptParams {
                phases: self
                    .phases
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(phases)))?,
                contexts: self.contexts,
                url_patterns: self.url_patterns,
            },
        })
    }
}
impl ContinueRequest {
    pub fn builder() -> ContinueRequestBuilder {
        <ContinueRequestBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ContinueRequestBuilder {
    request: Option<super::types::Request>,
    body: Option<super::types::BytesValue>,
    cookies: Option<Vec<super::types::CookieHeader>>,
    headers: Option<Vec<super::types::Header>>,
    method: Option<String>,
    url: Option<String>,
}
impl ContinueRequestBuilder {
    pub fn request(mut self, request: impl Into<super::types::Request>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn body(mut self, body: impl Into<super::types::BytesValue>) -> Self {
        self.body = Some(body.into());
        self
    }
    pub fn cookie(mut self, cookie: impl Into<super::types::CookieHeader>) -> Self {
        let v = self.cookies.get_or_insert(Vec::new());
        v.push(cookie.into());
        self
    }
    pub fn cookies<I, S>(mut self, cookies: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::CookieHeader>,
    {
        let v = self.cookies.get_or_insert(Vec::new());
        for val in cookies {
            v.push(val.into());
        }
        self
    }
    pub fn header(mut self, header: impl Into<super::types::Header>) -> Self {
        let v = self.headers.get_or_insert(Vec::new());
        v.push(header.into());
        self
    }
    pub fn headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::Header>,
    {
        let v = self.headers.get_or_insert(Vec::new());
        for val in headers {
            v.push(val.into());
        }
        self
    }
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn build(self) -> Result<ContinueRequest, String> {
        Ok(ContinueRequest {
            method: ContinueRequestMethod::ContinueRequest,
            params: ContinueRequestParams {
                request: self
                    .request
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request)))?,
                body: self.body,
                cookies: self.cookies,
                headers: self.headers,
                method: self.method,
                url: self.url,
            },
        })
    }
}
impl ContinueResponse {
    pub fn builder() -> ContinueResponseBuilder {
        <ContinueResponseBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ContinueResponseBuilder {
    request: Option<super::types::Request>,
    cookies: Option<Vec<super::types::SetCookieHeader>>,
    credentials: Option<super::types::AuthCredentials>,
    headers: Option<Vec<super::types::Header>>,
    reason_phrase: Option<String>,
    status_code: Option<u64>,
}
impl ContinueResponseBuilder {
    pub fn request(mut self, request: impl Into<super::types::Request>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn cookie(mut self, cookie: impl Into<super::types::SetCookieHeader>) -> Self {
        let v = self.cookies.get_or_insert(Vec::new());
        v.push(cookie.into());
        self
    }
    pub fn cookies<I, S>(mut self, cookies: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::SetCookieHeader>,
    {
        let v = self.cookies.get_or_insert(Vec::new());
        for val in cookies {
            v.push(val.into());
        }
        self
    }
    pub fn credentials(mut self, credentials: impl Into<super::types::AuthCredentials>) -> Self {
        self.credentials = Some(credentials.into());
        self
    }
    pub fn header(mut self, header: impl Into<super::types::Header>) -> Self {
        let v = self.headers.get_or_insert(Vec::new());
        v.push(header.into());
        self
    }
    pub fn headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::Header>,
    {
        let v = self.headers.get_or_insert(Vec::new());
        for val in headers {
            v.push(val.into());
        }
        self
    }
    pub fn reason_phrase(mut self, reason_phrase: impl Into<String>) -> Self {
        self.reason_phrase = Some(reason_phrase.into());
        self
    }
    pub fn status_code(mut self, status_code: impl Into<u64>) -> Self {
        self.status_code = Some(status_code.into());
        self
    }
    pub fn build(self) -> Result<ContinueResponse, String> {
        Ok(ContinueResponse {
            method: ContinueResponseMethod::ContinueResponse,
            params: ContinueResponseParams {
                request: self
                    .request
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request)))?,
                cookies: self.cookies,
                credentials: self.credentials,
                headers: self.headers,
                reason_phrase: self.reason_phrase,
                status_code: self.status_code,
            },
        })
    }
}
impl ContinueWithAuth {
    pub fn builder() -> ContinueWithAuthBuilder {
        <ContinueWithAuthBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ContinueWithAuthBuilder {
    request: Option<super::types::Request>,
    continue_with_auth_credentials_continue_with_auth_no_credentials_union:
        Option<super::types::ContinueWithAuthCredentialsContinueWithAuthNoCredentialsUnion>,
}
impl ContinueWithAuthBuilder {
    pub fn request(mut self, request: impl Into<super::types::Request>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn continue_with_auth_credentials_continue_with_auth_no_credentials_union(
        mut self,
        continue_with_auth_credentials_continue_with_auth_no_credentials_union: impl Into<
            super::types::ContinueWithAuthCredentialsContinueWithAuthNoCredentialsUnion,
        >,
    ) -> Self {
        self.continue_with_auth_credentials_continue_with_auth_no_credentials_union =
            Some(continue_with_auth_credentials_continue_with_auth_no_credentials_union.into());
        self
    }
    pub fn build(self) -> Result<ContinueWithAuth, String> {
        Ok (ContinueWithAuth { method : ContinueWithAuthMethod :: ContinueWithAuth , params : ContinueWithAuthParams { request : self . request . ok_or_else (|| format ! ("Field `{}` is mandatory." , std :: stringify ! (request))) ? , continue_with_auth_credentials_continue_with_auth_no_credentials_union : self . continue_with_auth_credentials_continue_with_auth_no_credentials_union . ok_or_else (|| format ! ("Field `{}` is mandatory." , std :: stringify ! (continue_with_auth_credentials_continue_with_auth_no_credentials_union))) ? , } , })
    }
}
impl DisownData {
    pub fn builder() -> DisownDataBuilder {
        <DisownDataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DisownDataBuilder {
    data_type: Option<super::types::DataType>,
    collector: Option<super::types::Collector>,
    request: Option<super::types::Request>,
}
impl DisownDataBuilder {
    pub fn data_type(mut self, data_type: impl Into<super::types::DataType>) -> Self {
        self.data_type = Some(data_type.into());
        self
    }
    pub fn collector(mut self, collector: impl Into<super::types::Collector>) -> Self {
        self.collector = Some(collector.into());
        self
    }
    pub fn request(mut self, request: impl Into<super::types::Request>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn build(self) -> Result<DisownData, String> {
        Ok(DisownData {
            method: DisownDataMethod::DisownData,
            params: DisownDataParams {
                data_type: self.data_type.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(data_type))
                })?,
                collector: self.collector.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(collector))
                })?,
                request: self
                    .request
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request)))?,
            },
        })
    }
}
impl FailRequest {
    pub fn builder() -> FailRequestBuilder {
        <FailRequestBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct FailRequestBuilder {
    request: Option<super::types::Request>,
}
impl FailRequestBuilder {
    pub fn request(mut self, request: impl Into<super::types::Request>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn build(self) -> Result<FailRequest, String> {
        Ok(FailRequest {
            method: FailRequestMethod::FailRequest,
            params: FailRequestParams {
                request: self
                    .request
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request)))?,
            },
        })
    }
}
impl GetData {
    pub fn builder() -> GetDataBuilder {
        <GetDataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetDataBuilder {
    data_type: Option<super::types::DataType>,
    collector: Option<super::types::Collector>,
    disown: Option<bool>,
    request: Option<super::types::Request>,
}
impl GetDataBuilder {
    pub fn data_type(mut self, data_type: impl Into<super::types::DataType>) -> Self {
        self.data_type = Some(data_type.into());
        self
    }
    pub fn collector(mut self, collector: impl Into<super::types::Collector>) -> Self {
        self.collector = Some(collector.into());
        self
    }
    pub fn disown(mut self, disown: impl Into<bool>) -> Self {
        self.disown = Some(disown.into());
        self
    }
    pub fn request(mut self, request: impl Into<super::types::Request>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn build(self) -> Result<GetData, String> {
        Ok(GetData {
            method: GetDataMethod::GetData,
            params: GetDataParams {
                data_type: self.data_type.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(data_type))
                })?,
                collector: self.collector,
                disown: self.disown,
                request: self
                    .request
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request)))?,
            },
        })
    }
}
impl ProvideResponse {
    pub fn builder() -> ProvideResponseBuilder {
        <ProvideResponseBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ProvideResponseBuilder {
    request: Option<super::types::Request>,
    body: Option<super::types::BytesValue>,
    cookies: Option<Vec<super::types::SetCookieHeader>>,
    headers: Option<Vec<super::types::Header>>,
    reason_phrase: Option<String>,
    status_code: Option<u64>,
}
impl ProvideResponseBuilder {
    pub fn request(mut self, request: impl Into<super::types::Request>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn body(mut self, body: impl Into<super::types::BytesValue>) -> Self {
        self.body = Some(body.into());
        self
    }
    pub fn cookie(mut self, cookie: impl Into<super::types::SetCookieHeader>) -> Self {
        let v = self.cookies.get_or_insert(Vec::new());
        v.push(cookie.into());
        self
    }
    pub fn cookies<I, S>(mut self, cookies: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::SetCookieHeader>,
    {
        let v = self.cookies.get_or_insert(Vec::new());
        for val in cookies {
            v.push(val.into());
        }
        self
    }
    pub fn header(mut self, header: impl Into<super::types::Header>) -> Self {
        let v = self.headers.get_or_insert(Vec::new());
        v.push(header.into());
        self
    }
    pub fn headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::Header>,
    {
        let v = self.headers.get_or_insert(Vec::new());
        for val in headers {
            v.push(val.into());
        }
        self
    }
    pub fn reason_phrase(mut self, reason_phrase: impl Into<String>) -> Self {
        self.reason_phrase = Some(reason_phrase.into());
        self
    }
    pub fn status_code(mut self, status_code: impl Into<u64>) -> Self {
        self.status_code = Some(status_code.into());
        self
    }
    pub fn build(self) -> Result<ProvideResponse, String> {
        Ok(ProvideResponse {
            method: ProvideResponseMethod::ProvideResponse,
            params: ProvideResponseParams {
                request: self
                    .request
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request)))?,
                body: self.body,
                cookies: self.cookies,
                headers: self.headers,
                reason_phrase: self.reason_phrase,
                status_code: self.status_code,
            },
        })
    }
}
impl RemoveDataCollector {
    pub fn builder() -> RemoveDataCollectorBuilder {
        <RemoveDataCollectorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveDataCollectorBuilder {
    collector: Option<super::types::Collector>,
}
impl RemoveDataCollectorBuilder {
    pub fn collector(mut self, collector: impl Into<super::types::Collector>) -> Self {
        self.collector = Some(collector.into());
        self
    }
    pub fn build(self) -> Result<RemoveDataCollector, String> {
        Ok(RemoveDataCollector {
            method: RemoveDataCollectorMethod::RemoveDataCollector,
            params: RemoveDataCollectorParams {
                collector: self.collector.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(collector))
                })?,
            },
        })
    }
}
impl RemoveIntercept {
    pub fn builder() -> RemoveInterceptBuilder {
        <RemoveInterceptBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveInterceptBuilder {
    intercept: Option<super::types::Intercept>,
}
impl RemoveInterceptBuilder {
    pub fn intercept(mut self, intercept: impl Into<super::types::Intercept>) -> Self {
        self.intercept = Some(intercept.into());
        self
    }
    pub fn build(self) -> Result<RemoveIntercept, String> {
        Ok(RemoveIntercept {
            method: RemoveInterceptMethod::RemoveIntercept,
            params: RemoveInterceptParams {
                intercept: self.intercept.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(intercept))
                })?,
            },
        })
    }
}
impl SetCacheBehavior {
    pub fn builder() -> SetCacheBehaviorBuilder {
        <SetCacheBehaviorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetCacheBehaviorBuilder {
    cache_behavior: Option<SetCacheBehaviorCacheBehavior>,
    contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
}
impl SetCacheBehaviorBuilder {
    pub fn cache_behavior(
        mut self,
        cache_behavior: impl Into<SetCacheBehaviorCacheBehavior>,
    ) -> Self {
        self.cache_behavior = Some(cache_behavior.into());
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        let v = self.contexts.get_or_insert(Vec::new());
        v.push(context.into());
        self
    }
    pub fn contexts<I, S>(mut self, contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browsing_context::types::BrowsingContext>,
    {
        let v = self.contexts.get_or_insert(Vec::new());
        for val in contexts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetCacheBehavior, String> {
        Ok(SetCacheBehavior {
            method: SetCacheBehaviorMethod::SetCacheBehavior,
            params: SetCacheBehaviorParams {
                cache_behavior: self.cache_behavior.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(cache_behavior))
                })?,
                contexts: self.contexts,
            },
        })
    }
}
impl SetExtraHeaders {
    pub fn builder() -> SetExtraHeadersBuilder {
        <SetExtraHeadersBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetExtraHeadersBuilder {
    headers: Option<Vec<super::types::Header>>,
    contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl SetExtraHeadersBuilder {
    pub fn header(mut self, header: impl Into<super::types::Header>) -> Self {
        let v = self.headers.get_or_insert(Vec::new());
        v.push(header.into());
        self
    }
    pub fn headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::Header>,
    {
        let v = self.headers.get_or_insert(Vec::new());
        for val in headers {
            v.push(val.into());
        }
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        let v = self.contexts.get_or_insert(Vec::new());
        v.push(context.into());
        self
    }
    pub fn contexts<I, S>(mut self, contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browsing_context::types::BrowsingContext>,
    {
        let v = self.contexts.get_or_insert(Vec::new());
        for val in contexts {
            v.push(val.into());
        }
        self
    }
    pub fn user_context(
        mut self,
        user_context: impl Into<crate::browser::types::UserContext>,
    ) -> Self {
        let v = self.user_contexts.get_or_insert(Vec::new());
        v.push(user_context.into());
        self
    }
    pub fn user_contexts<I, S>(mut self, user_contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser::types::UserContext>,
    {
        let v = self.user_contexts.get_or_insert(Vec::new());
        for val in user_contexts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetExtraHeaders, String> {
        Ok(SetExtraHeaders {
            method: SetExtraHeadersMethod::SetExtraHeaders,
            params: SetExtraHeadersParams {
                headers: self
                    .headers
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(headers)))?,
                contexts: self.contexts,
                user_contexts: self.user_contexts,
            },
        })
    }
}
