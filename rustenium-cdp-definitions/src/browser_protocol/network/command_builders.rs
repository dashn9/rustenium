use super::commands::*;
impl SetAcceptedEncodings {
    pub fn builder() -> SetAcceptedEncodingsBuilder {
        SetAcceptedEncodingsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetAcceptedEncodingsBuilder {
    encodings: Option<Vec<super::types::ContentEncoding>>,
}
impl SetAcceptedEncodingsBuilder {
    pub fn encoding(mut self, encoding: impl Into<super::types::ContentEncoding>) -> Self {
        let v = self.encodings.get_or_insert(Vec::new());
        v.push(encoding.into());
        self
    }
    pub fn encodings<I, S>(mut self, encodings: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::ContentEncoding>,
    {
        let v = self.encodings.get_or_insert(Vec::new());
        for val in encodings {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetAcceptedEncodings, String> {
        Ok(SetAcceptedEncodings {
            method: SetAcceptedEncodingsMethod::SetAcceptedEncodings,
            params: SetAcceptedEncodingsParams {
                encodings: self.encodings.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(encodings))
                })?,
            },
        })
    }
}
impl DeleteCookies {
    pub fn builder() -> DeleteCookiesBuilder {
        DeleteCookiesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DeleteCookiesBuilder {
    name: Option<String>,
    url: Option<String>,
    domain: Option<String>,
    path: Option<String>,
    partition_key: Option<super::types::CookiePartitionKey>,
}
impl DeleteCookiesBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
    pub fn partition_key(
        mut self,
        partition_key: impl Into<super::types::CookiePartitionKey>,
    ) -> Self {
        self.partition_key = Some(partition_key.into());
        self
    }
    pub fn build(self) -> Result<DeleteCookies, String> {
        Ok(DeleteCookies {
            method: DeleteCookiesMethod::DeleteCookies,
            params: DeleteCookiesParams {
                name: self
                    .name
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
                url: self.url,
                domain: self.domain,
                path: self.path,
                partition_key: self.partition_key,
            },
        })
    }
}
impl EmulateNetworkConditionsByRule {
    pub fn builder() -> EmulateNetworkConditionsByRuleBuilder {
        EmulateNetworkConditionsByRuleBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct EmulateNetworkConditionsByRuleBuilder {
    offline: Option<bool>,
    matched_network_conditions: Option<Vec<super::types::NetworkConditions>>,
}
impl EmulateNetworkConditionsByRuleBuilder {
    pub fn offline(mut self, offline: impl Into<bool>) -> Self {
        self.offline = Some(offline.into());
        self
    }
    pub fn matched_network_condition(
        mut self,
        matched_network_condition: impl Into<super::types::NetworkConditions>,
    ) -> Self {
        let v = self.matched_network_conditions.get_or_insert(Vec::new());
        v.push(matched_network_condition.into());
        self
    }
    pub fn matched_network_conditions<I, S>(mut self, matched_network_conditions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::NetworkConditions>,
    {
        let v = self.matched_network_conditions.get_or_insert(Vec::new());
        for val in matched_network_conditions {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<EmulateNetworkConditionsByRule, String> {
        Ok(EmulateNetworkConditionsByRule {
            method: EmulateNetworkConditionsByRuleMethod::EmulateNetworkConditionsByRule,
            params: EmulateNetworkConditionsByRuleParams {
                offline: self
                    .offline
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(offline)))?,
                matched_network_conditions: self.matched_network_conditions.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(matched_network_conditions)
                    )
                })?,
            },
        })
    }
}
impl OverrideNetworkState {
    pub fn builder() -> OverrideNetworkStateBuilder {
        OverrideNetworkStateBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct OverrideNetworkStateBuilder {
    offline: Option<bool>,
    latency: Option<f64>,
    download_throughput: Option<f64>,
    upload_throughput: Option<f64>,
    connection_type: Option<super::types::ConnectionType>,
}
impl OverrideNetworkStateBuilder {
    pub fn offline(mut self, offline: impl Into<bool>) -> Self {
        self.offline = Some(offline.into());
        self
    }
    pub fn latency(mut self, latency: impl Into<f64>) -> Self {
        self.latency = Some(latency.into());
        self
    }
    pub fn download_throughput(mut self, download_throughput: impl Into<f64>) -> Self {
        self.download_throughput = Some(download_throughput.into());
        self
    }
    pub fn upload_throughput(mut self, upload_throughput: impl Into<f64>) -> Self {
        self.upload_throughput = Some(upload_throughput.into());
        self
    }
    pub fn connection_type(
        mut self,
        connection_type: impl Into<super::types::ConnectionType>,
    ) -> Self {
        self.connection_type = Some(connection_type.into());
        self
    }
    pub fn build(self) -> Result<OverrideNetworkState, String> {
        Ok(OverrideNetworkState {
            method: OverrideNetworkStateMethod::OverrideNetworkState,
            params: OverrideNetworkStateParams {
                offline: self
                    .offline
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(offline)))?,
                latency: self
                    .latency
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(latency)))?,
                download_throughput: self.download_throughput.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(download_throughput)
                    )
                })?,
                upload_throughput: self.upload_throughput.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(upload_throughput)
                    )
                })?,
                connection_type: self.connection_type,
            },
        })
    }
}
impl Enable {
    pub fn builder() -> EnableBuilder {
        EnableBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct EnableBuilder {
    max_total_buffer_size: Option<i64>,
    max_resource_buffer_size: Option<i64>,
    max_post_data_size: Option<i64>,
    report_direct_socket_traffic: Option<bool>,
    enable_durable_messages: Option<bool>,
}
impl EnableBuilder {
    pub fn max_total_buffer_size(mut self, max_total_buffer_size: impl Into<i64>) -> Self {
        self.max_total_buffer_size = Some(max_total_buffer_size.into());
        self
    }
    pub fn max_resource_buffer_size(mut self, max_resource_buffer_size: impl Into<i64>) -> Self {
        self.max_resource_buffer_size = Some(max_resource_buffer_size.into());
        self
    }
    pub fn max_post_data_size(mut self, max_post_data_size: impl Into<i64>) -> Self {
        self.max_post_data_size = Some(max_post_data_size.into());
        self
    }
    pub fn report_direct_socket_traffic(
        mut self,
        report_direct_socket_traffic: impl Into<bool>,
    ) -> Self {
        self.report_direct_socket_traffic = Some(report_direct_socket_traffic.into());
        self
    }
    pub fn enable_durable_messages(mut self, enable_durable_messages: impl Into<bool>) -> Self {
        self.enable_durable_messages = Some(enable_durable_messages.into());
        self
    }
    pub fn build(self) -> Enable {
        Enable {
            method: EnableMethod::Enable,
            params: EnableParams {
                max_total_buffer_size: self.max_total_buffer_size,
                max_resource_buffer_size: self.max_resource_buffer_size,
                max_post_data_size: self.max_post_data_size,
                report_direct_socket_traffic: self.report_direct_socket_traffic,
                enable_durable_messages: self.enable_durable_messages,
            },
        }
    }
}
impl ConfigureDurableMessages {
    pub fn builder() -> ConfigureDurableMessagesBuilder {
        ConfigureDurableMessagesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ConfigureDurableMessagesBuilder {
    max_total_buffer_size: Option<i64>,
    max_resource_buffer_size: Option<i64>,
}
impl ConfigureDurableMessagesBuilder {
    pub fn max_total_buffer_size(mut self, max_total_buffer_size: impl Into<i64>) -> Self {
        self.max_total_buffer_size = Some(max_total_buffer_size.into());
        self
    }
    pub fn max_resource_buffer_size(mut self, max_resource_buffer_size: impl Into<i64>) -> Self {
        self.max_resource_buffer_size = Some(max_resource_buffer_size.into());
        self
    }
    pub fn build(self) -> ConfigureDurableMessages {
        ConfigureDurableMessages {
            method: ConfigureDurableMessagesMethod::ConfigureDurableMessages,
            params: ConfigureDurableMessagesParams {
                max_total_buffer_size: self.max_total_buffer_size,
                max_resource_buffer_size: self.max_resource_buffer_size,
            },
        }
    }
}
impl GetCertificate {
    pub fn builder() -> GetCertificateBuilder {
        GetCertificateBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetCertificateBuilder {
    origin: Option<String>,
}
impl GetCertificateBuilder {
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn build(self) -> Result<GetCertificate, String> {
        Ok(GetCertificate {
            method: GetCertificateMethod::GetCertificate,
            params: GetCertificateParams {
                origin: self
                    .origin
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            },
        })
    }
}
impl GetCookies {
    pub fn builder() -> GetCookiesBuilder {
        GetCookiesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetCookiesBuilder {
    urls: Option<Vec<String>>,
}
impl GetCookiesBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        let v = self.urls.get_or_insert(Vec::new());
        v.push(url.into());
        self
    }
    pub fn urls<I, S>(mut self, urls: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.urls.get_or_insert(Vec::new());
        for val in urls {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> GetCookies {
        GetCookies {
            method: GetCookiesMethod::GetCookies,
            params: GetCookiesParams { urls: self.urls },
        }
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
impl GetRequestPostData {
    pub fn builder() -> GetRequestPostDataBuilder {
        GetRequestPostDataBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetRequestPostDataBuilder {
    request_id: Option<super::types::RequestId>,
}
impl GetRequestPostDataBuilder {
    pub fn request_id(mut self, request_id: impl Into<super::types::RequestId>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn build(self) -> Result<GetRequestPostData, String> {
        Ok(GetRequestPostData {
            method: GetRequestPostDataMethod::GetRequestPostData,
            params: GetRequestPostDataParams {
                request_id: self.request_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(request_id))
                })?,
            },
        })
    }
}
impl GetResponseBodyForInterception {
    pub fn builder() -> GetResponseBodyForInterceptionBuilder {
        GetResponseBodyForInterceptionBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetResponseBodyForInterceptionBuilder {
    interception_id: Option<super::types::InterceptionId>,
}
impl GetResponseBodyForInterceptionBuilder {
    pub fn interception_id(
        mut self,
        interception_id: impl Into<super::types::InterceptionId>,
    ) -> Self {
        self.interception_id = Some(interception_id.into());
        self
    }
    pub fn build(self) -> Result<GetResponseBodyForInterception, String> {
        Ok(GetResponseBodyForInterception {
            method: GetResponseBodyForInterceptionMethod::GetResponseBodyForInterception,
            params: GetResponseBodyForInterceptionParams {
                interception_id: self.interception_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(interception_id))
                })?,
            },
        })
    }
}
impl TakeResponseBodyForInterceptionAsStream {
    pub fn builder() -> TakeResponseBodyForInterceptionAsStreamBuilder {
        TakeResponseBodyForInterceptionAsStreamBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct TakeResponseBodyForInterceptionAsStreamBuilder {
    interception_id: Option<super::types::InterceptionId>,
}
impl TakeResponseBodyForInterceptionAsStreamBuilder {
    pub fn interception_id(
        mut self,
        interception_id: impl Into<super::types::InterceptionId>,
    ) -> Self {
        self.interception_id = Some(interception_id.into());
        self
    }
    pub fn build(self) -> Result<TakeResponseBodyForInterceptionAsStream, String> {
        Ok (TakeResponseBodyForInterceptionAsStream { method : TakeResponseBodyForInterceptionAsStreamMethod :: TakeResponseBodyForInterceptionAsStream , params : TakeResponseBodyForInterceptionAsStreamParams { interception_id : self . interception_id . ok_or_else (|| format ! ("Field `{}` is mandatory." , std :: stringify ! (interception_id))) ? , } , })
    }
}
impl ReplayXhr {
    pub fn builder() -> ReplayXhrBuilder {
        ReplayXhrBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ReplayXhrBuilder {
    request_id: Option<super::types::RequestId>,
}
impl ReplayXhrBuilder {
    pub fn request_id(mut self, request_id: impl Into<super::types::RequestId>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn build(self) -> Result<ReplayXhr, String> {
        Ok(ReplayXhr {
            method: ReplayXhrMethod::ReplayXhr,
            params: ReplayXhrParams {
                request_id: self.request_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(request_id))
                })?,
            },
        })
    }
}
impl SearchInResponseBody {
    pub fn builder() -> SearchInResponseBodyBuilder {
        SearchInResponseBodyBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SearchInResponseBodyBuilder {
    request_id: Option<super::types::RequestId>,
    query: Option<String>,
    case_sensitive: Option<bool>,
    is_regex: Option<bool>,
}
impl SearchInResponseBodyBuilder {
    pub fn request_id(mut self, request_id: impl Into<super::types::RequestId>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }
    pub fn case_sensitive(mut self, case_sensitive: impl Into<bool>) -> Self {
        self.case_sensitive = Some(case_sensitive.into());
        self
    }
    pub fn is_regex(mut self, is_regex: impl Into<bool>) -> Self {
        self.is_regex = Some(is_regex.into());
        self
    }
    pub fn build(self) -> Result<SearchInResponseBody, String> {
        Ok(SearchInResponseBody {
            method: SearchInResponseBodyMethod::SearchInResponseBody,
            params: SearchInResponseBodyParams {
                request_id: self.request_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(request_id))
                })?,
                query: self
                    .query
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(query)))?,
                case_sensitive: self.case_sensitive,
                is_regex: self.is_regex,
            },
        })
    }
}
impl SetBlockedUrLs {
    pub fn builder() -> SetBlockedUrLsBuilder {
        SetBlockedUrLsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetBlockedUrLsBuilder {
    url_patterns: Option<Vec<super::types::BlockPattern>>,
}
impl SetBlockedUrLsBuilder {
    pub fn url_pattern(mut self, url_pattern: impl Into<super::types::BlockPattern>) -> Self {
        let v = self.url_patterns.get_or_insert(Vec::new());
        v.push(url_pattern.into());
        self
    }
    pub fn url_patterns<I, S>(mut self, url_patterns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::BlockPattern>,
    {
        let v = self.url_patterns.get_or_insert(Vec::new());
        for val in url_patterns {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> SetBlockedUrLs {
        SetBlockedUrLs {
            method: SetBlockedUrLsMethod::SetBlockedUrLs,
            params: SetBlockedUrLsParams {
                url_patterns: self.url_patterns,
            },
        }
    }
}
impl SetBypassServiceWorker {
    pub fn builder() -> SetBypassServiceWorkerBuilder {
        SetBypassServiceWorkerBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetBypassServiceWorkerBuilder {
    bypass: Option<bool>,
}
impl SetBypassServiceWorkerBuilder {
    pub fn bypass(mut self, bypass: impl Into<bool>) -> Self {
        self.bypass = Some(bypass.into());
        self
    }
    pub fn build(self) -> Result<SetBypassServiceWorker, String> {
        Ok(SetBypassServiceWorker {
            method: SetBypassServiceWorkerMethod::SetBypassServiceWorker,
            params: SetBypassServiceWorkerParams {
                bypass: self
                    .bypass
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(bypass)))?,
            },
        })
    }
}
impl SetCacheDisabled {
    pub fn builder() -> SetCacheDisabledBuilder {
        SetCacheDisabledBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetCacheDisabledBuilder {
    cache_disabled: Option<bool>,
}
impl SetCacheDisabledBuilder {
    pub fn cache_disabled(mut self, cache_disabled: impl Into<bool>) -> Self {
        self.cache_disabled = Some(cache_disabled.into());
        self
    }
    pub fn build(self) -> Result<SetCacheDisabled, String> {
        Ok(SetCacheDisabled {
            method: SetCacheDisabledMethod::SetCacheDisabled,
            params: SetCacheDisabledParams {
                cache_disabled: self.cache_disabled.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(cache_disabled))
                })?,
            },
        })
    }
}
impl SetCookie {
    pub fn builder() -> SetCookieBuilder {
        SetCookieBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetCookieBuilder {
    name: Option<String>,
    value: Option<String>,
    url: Option<String>,
    domain: Option<String>,
    path: Option<String>,
    secure: Option<bool>,
    http_only: Option<bool>,
    same_site: Option<super::types::CookieSameSite>,
    expires: Option<super::types::TimeSinceEpoch>,
    priority: Option<super::types::CookiePriority>,
    source_scheme: Option<super::types::CookieSourceScheme>,
    source_port: Option<i64>,
    partition_key: Option<super::types::CookiePartitionKey>,
}
impl SetCookieBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
    pub fn secure(mut self, secure: impl Into<bool>) -> Self {
        self.secure = Some(secure.into());
        self
    }
    pub fn http_only(mut self, http_only: impl Into<bool>) -> Self {
        self.http_only = Some(http_only.into());
        self
    }
    pub fn same_site(mut self, same_site: impl Into<super::types::CookieSameSite>) -> Self {
        self.same_site = Some(same_site.into());
        self
    }
    pub fn expires(mut self, expires: impl Into<super::types::TimeSinceEpoch>) -> Self {
        self.expires = Some(expires.into());
        self
    }
    pub fn priority(mut self, priority: impl Into<super::types::CookiePriority>) -> Self {
        self.priority = Some(priority.into());
        self
    }
    pub fn source_scheme(
        mut self,
        source_scheme: impl Into<super::types::CookieSourceScheme>,
    ) -> Self {
        self.source_scheme = Some(source_scheme.into());
        self
    }
    pub fn source_port(mut self, source_port: impl Into<i64>) -> Self {
        self.source_port = Some(source_port.into());
        self
    }
    pub fn partition_key(
        mut self,
        partition_key: impl Into<super::types::CookiePartitionKey>,
    ) -> Self {
        self.partition_key = Some(partition_key.into());
        self
    }
    pub fn build(self) -> Result<SetCookie, String> {
        Ok(SetCookie {
            method: SetCookieMethod::SetCookie,
            params: SetCookieParams {
                name: self
                    .name
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
                value: self
                    .value
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
                url: self.url,
                domain: self.domain,
                path: self.path,
                secure: self.secure,
                http_only: self.http_only,
                same_site: self.same_site,
                expires: self.expires,
                priority: self.priority,
                source_scheme: self.source_scheme,
                source_port: self.source_port,
                partition_key: self.partition_key,
            },
        })
    }
}
impl SetCookies {
    pub fn builder() -> SetCookiesBuilder {
        SetCookiesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetCookiesBuilder {
    cookies: Option<Vec<super::types::CookieParam>>,
}
impl SetCookiesBuilder {
    pub fn cookie(mut self, cookie: impl Into<super::types::CookieParam>) -> Self {
        let v = self.cookies.get_or_insert(Vec::new());
        v.push(cookie.into());
        self
    }
    pub fn cookies<I, S>(mut self, cookies: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::CookieParam>,
    {
        let v = self.cookies.get_or_insert(Vec::new());
        for val in cookies {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetCookies, String> {
        Ok(SetCookies {
            method: SetCookiesMethod::SetCookies,
            params: SetCookiesParams {
                cookies: self
                    .cookies
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cookies)))?,
            },
        })
    }
}
impl SetExtraHttpHeaders {
    pub fn builder() -> SetExtraHttpHeadersBuilder {
        SetExtraHttpHeadersBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetExtraHttpHeadersBuilder {
    headers: Option<super::types::Headers>,
}
impl SetExtraHttpHeadersBuilder {
    pub fn headers(mut self, headers: impl Into<super::types::Headers>) -> Self {
        self.headers = Some(headers.into());
        self
    }
    pub fn build(self) -> Result<SetExtraHttpHeaders, String> {
        Ok(SetExtraHttpHeaders {
            method: SetExtraHttpHeadersMethod::SetExtraHttpHeaders,
            params: SetExtraHttpHeadersParams {
                headers: self
                    .headers
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(headers)))?,
            },
        })
    }
}
impl SetAttachDebugStack {
    pub fn builder() -> SetAttachDebugStackBuilder {
        SetAttachDebugStackBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetAttachDebugStackBuilder {
    enabled: Option<bool>,
}
impl SetAttachDebugStackBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn build(self) -> Result<SetAttachDebugStack, String> {
        Ok(SetAttachDebugStack {
            method: SetAttachDebugStackMethod::SetAttachDebugStack,
            params: SetAttachDebugStackParams {
                enabled: self
                    .enabled
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
            },
        })
    }
}
impl SetUserAgentOverride {
    pub fn builder() -> SetUserAgentOverrideBuilder {
        SetUserAgentOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetUserAgentOverrideBuilder {
    user_agent: Option<String>,
    accept_language: Option<String>,
    platform: Option<String>,
    user_agent_metadata: Option<super::super::emulation::types::UserAgentMetadata>,
}
impl SetUserAgentOverrideBuilder {
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
    pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
        self.accept_language = Some(accept_language.into());
        self
    }
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
    pub fn user_agent_metadata(
        mut self,
        user_agent_metadata: impl Into<super::super::emulation::types::UserAgentMetadata>,
    ) -> Self {
        self.user_agent_metadata = Some(user_agent_metadata.into());
        self
    }
    pub fn build(self) -> Result<SetUserAgentOverride, String> {
        Ok(SetUserAgentOverride {
            method: SetUserAgentOverrideMethod::SetUserAgentOverride,
            params: SetUserAgentOverrideParams {
                user_agent: self.user_agent.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(user_agent))
                })?,
                accept_language: self.accept_language,
                platform: self.platform,
                user_agent_metadata: self.user_agent_metadata,
            },
        })
    }
}
impl StreamResourceContent {
    pub fn builder() -> StreamResourceContentBuilder {
        StreamResourceContentBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct StreamResourceContentBuilder {
    request_id: Option<super::types::RequestId>,
}
impl StreamResourceContentBuilder {
    pub fn request_id(mut self, request_id: impl Into<super::types::RequestId>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn build(self) -> Result<StreamResourceContent, String> {
        Ok(StreamResourceContent {
            method: StreamResourceContentMethod::StreamResourceContent,
            params: StreamResourceContentParams {
                request_id: self.request_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(request_id))
                })?,
            },
        })
    }
}
impl GetSecurityIsolationStatus {
    pub fn builder() -> GetSecurityIsolationStatusBuilder {
        GetSecurityIsolationStatusBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetSecurityIsolationStatusBuilder {
    frame_id: Option<super::super::page::types::FrameId>,
}
impl GetSecurityIsolationStatusBuilder {
    pub fn frame_id(mut self, frame_id: impl Into<super::super::page::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn build(self) -> GetSecurityIsolationStatus {
        GetSecurityIsolationStatus {
            method: GetSecurityIsolationStatusMethod::GetSecurityIsolationStatus,
            params: GetSecurityIsolationStatusParams {
                frame_id: self.frame_id,
            },
        }
    }
}
impl EnableReportingApi {
    pub fn builder() -> EnableReportingApiBuilder {
        EnableReportingApiBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct EnableReportingApiBuilder {
    enable: Option<bool>,
}
impl EnableReportingApiBuilder {
    pub fn enable(mut self, enable: impl Into<bool>) -> Self {
        self.enable = Some(enable.into());
        self
    }
    pub fn build(self) -> Result<EnableReportingApi, String> {
        Ok(EnableReportingApi {
            method: EnableReportingApiMethod::EnableReportingApi,
            params: EnableReportingApiParams {
                enable: self
                    .enable
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enable)))?,
            },
        })
    }
}
impl EnableDeviceBoundSessions {
    pub fn builder() -> EnableDeviceBoundSessionsBuilder {
        EnableDeviceBoundSessionsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct EnableDeviceBoundSessionsBuilder {
    enable: Option<bool>,
}
impl EnableDeviceBoundSessionsBuilder {
    pub fn enable(mut self, enable: impl Into<bool>) -> Self {
        self.enable = Some(enable.into());
        self
    }
    pub fn build(self) -> Result<EnableDeviceBoundSessions, String> {
        Ok(EnableDeviceBoundSessions {
            method: EnableDeviceBoundSessionsMethod::EnableDeviceBoundSessions,
            params: EnableDeviceBoundSessionsParams {
                enable: self
                    .enable
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enable)))?,
            },
        })
    }
}
impl FetchSchemefulSite {
    pub fn builder() -> FetchSchemefulSiteBuilder {
        FetchSchemefulSiteBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FetchSchemefulSiteBuilder {
    origin: Option<String>,
}
impl FetchSchemefulSiteBuilder {
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn build(self) -> Result<FetchSchemefulSite, String> {
        Ok(FetchSchemefulSite {
            method: FetchSchemefulSiteMethod::FetchSchemefulSite,
            params: FetchSchemefulSiteParams {
                origin: self
                    .origin
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            },
        })
    }
}
impl LoadNetworkResource {
    pub fn builder() -> LoadNetworkResourceBuilder {
        LoadNetworkResourceBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct LoadNetworkResourceBuilder {
    frame_id: Option<super::super::page::types::FrameId>,
    url: Option<String>,
    options: Option<super::types::LoadNetworkResourceOptions>,
}
impl LoadNetworkResourceBuilder {
    pub fn frame_id(mut self, frame_id: impl Into<super::super::page::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn options(mut self, options: impl Into<super::types::LoadNetworkResourceOptions>) -> Self {
        self.options = Some(options.into());
        self
    }
    pub fn build(self) -> Result<LoadNetworkResource, String> {
        Ok(LoadNetworkResource {
            method: LoadNetworkResourceMethod::LoadNetworkResource,
            params: LoadNetworkResourceParams {
                frame_id: self.frame_id,
                url: self
                    .url
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
                options: self
                    .options
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(options)))?,
            },
        })
    }
}
impl SetCookieControls {
    pub fn builder() -> SetCookieControlsBuilder {
        SetCookieControlsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetCookieControlsBuilder {
    enable_third_party_cookie_restriction: Option<bool>,
    disable_third_party_cookie_metadata: Option<bool>,
    disable_third_party_cookie_heuristics: Option<bool>,
}
impl SetCookieControlsBuilder {
    pub fn enable_third_party_cookie_restriction(
        mut self,
        enable_third_party_cookie_restriction: impl Into<bool>,
    ) -> Self {
        self.enable_third_party_cookie_restriction =
            Some(enable_third_party_cookie_restriction.into());
        self
    }
    pub fn disable_third_party_cookie_metadata(
        mut self,
        disable_third_party_cookie_metadata: impl Into<bool>,
    ) -> Self {
        self.disable_third_party_cookie_metadata = Some(disable_third_party_cookie_metadata.into());
        self
    }
    pub fn disable_third_party_cookie_heuristics(
        mut self,
        disable_third_party_cookie_heuristics: impl Into<bool>,
    ) -> Self {
        self.disable_third_party_cookie_heuristics =
            Some(disable_third_party_cookie_heuristics.into());
        self
    }
    pub fn build(self) -> Result<SetCookieControls, String> {
        Ok(SetCookieControls {
            method: SetCookieControlsMethod::SetCookieControls,
            params: SetCookieControlsParams {
                enable_third_party_cookie_restriction: self
                    .enable_third_party_cookie_restriction
                    .ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(enable_third_party_cookie_restriction)
                    )
                })?,
                disable_third_party_cookie_metadata: self
                    .disable_third_party_cookie_metadata
                    .ok_or_else(|| {
                        format!(
                            "Field `{}` is mandatory.",
                            std::stringify!(disable_third_party_cookie_metadata)
                        )
                    })?,
                disable_third_party_cookie_heuristics: self
                    .disable_third_party_cookie_heuristics
                    .ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(disable_third_party_cookie_heuristics)
                    )
                })?,
            },
        })
    }
}
