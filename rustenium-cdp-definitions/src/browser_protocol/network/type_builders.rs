use super::types::*;
impl ResourceTiming {
    pub fn builder() -> ResourceTimingBuilder {
        <ResourceTimingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ResourceTimingBuilder {
    request_time: Option<f64>,
    proxy_start: Option<f64>,
    proxy_end: Option<f64>,
    dns_start: Option<f64>,
    dns_end: Option<f64>,
    connect_start: Option<f64>,
    connect_end: Option<f64>,
    ssl_start: Option<f64>,
    ssl_end: Option<f64>,
    worker_start: Option<f64>,
    worker_ready: Option<f64>,
    worker_fetch_start: Option<f64>,
    worker_respond_with_settled: Option<f64>,
    worker_router_evaluation_start: Option<f64>,
    worker_cache_lookup_start: Option<f64>,
    send_start: Option<f64>,
    send_end: Option<f64>,
    push_start: Option<f64>,
    push_end: Option<f64>,
    receive_headers_start: Option<f64>,
    receive_headers_end: Option<f64>,
}
impl ResourceTimingBuilder {
    pub fn request_time(mut self, request_time: impl Into<f64>) -> Self {
        self.request_time = Some(request_time.into());
        self
    }
    pub fn proxy_start(mut self, proxy_start: impl Into<f64>) -> Self {
        self.proxy_start = Some(proxy_start.into());
        self
    }
    pub fn proxy_end(mut self, proxy_end: impl Into<f64>) -> Self {
        self.proxy_end = Some(proxy_end.into());
        self
    }
    pub fn dns_start(mut self, dns_start: impl Into<f64>) -> Self {
        self.dns_start = Some(dns_start.into());
        self
    }
    pub fn dns_end(mut self, dns_end: impl Into<f64>) -> Self {
        self.dns_end = Some(dns_end.into());
        self
    }
    pub fn connect_start(mut self, connect_start: impl Into<f64>) -> Self {
        self.connect_start = Some(connect_start.into());
        self
    }
    pub fn connect_end(mut self, connect_end: impl Into<f64>) -> Self {
        self.connect_end = Some(connect_end.into());
        self
    }
    pub fn ssl_start(mut self, ssl_start: impl Into<f64>) -> Self {
        self.ssl_start = Some(ssl_start.into());
        self
    }
    pub fn ssl_end(mut self, ssl_end: impl Into<f64>) -> Self {
        self.ssl_end = Some(ssl_end.into());
        self
    }
    pub fn worker_start(mut self, worker_start: impl Into<f64>) -> Self {
        self.worker_start = Some(worker_start.into());
        self
    }
    pub fn worker_ready(mut self, worker_ready: impl Into<f64>) -> Self {
        self.worker_ready = Some(worker_ready.into());
        self
    }
    pub fn worker_fetch_start(mut self, worker_fetch_start: impl Into<f64>) -> Self {
        self.worker_fetch_start = Some(worker_fetch_start.into());
        self
    }
    pub fn worker_respond_with_settled(
        mut self,
        worker_respond_with_settled: impl Into<f64>,
    ) -> Self {
        self.worker_respond_with_settled = Some(worker_respond_with_settled.into());
        self
    }
    pub fn worker_router_evaluation_start(
        mut self,
        worker_router_evaluation_start: impl Into<f64>,
    ) -> Self {
        self.worker_router_evaluation_start = Some(worker_router_evaluation_start.into());
        self
    }
    pub fn worker_cache_lookup_start(mut self, worker_cache_lookup_start: impl Into<f64>) -> Self {
        self.worker_cache_lookup_start = Some(worker_cache_lookup_start.into());
        self
    }
    pub fn send_start(mut self, send_start: impl Into<f64>) -> Self {
        self.send_start = Some(send_start.into());
        self
    }
    pub fn send_end(mut self, send_end: impl Into<f64>) -> Self {
        self.send_end = Some(send_end.into());
        self
    }
    pub fn push_start(mut self, push_start: impl Into<f64>) -> Self {
        self.push_start = Some(push_start.into());
        self
    }
    pub fn push_end(mut self, push_end: impl Into<f64>) -> Self {
        self.push_end = Some(push_end.into());
        self
    }
    pub fn receive_headers_start(mut self, receive_headers_start: impl Into<f64>) -> Self {
        self.receive_headers_start = Some(receive_headers_start.into());
        self
    }
    pub fn receive_headers_end(mut self, receive_headers_end: impl Into<f64>) -> Self {
        self.receive_headers_end = Some(receive_headers_end.into());
        self
    }
    pub fn build(self) -> Result<ResourceTiming, String> {
        Ok(ResourceTiming {
            request_time: self.request_time.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(request_time))
            })?,
            proxy_start: self
                .proxy_start
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(proxy_start)))?,
            proxy_end: self
                .proxy_end
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(proxy_end)))?,
            dns_start: self
                .dns_start
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(dns_start)))?,
            dns_end: self
                .dns_end
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(dns_end)))?,
            connect_start: self.connect_start.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(connect_start))
            })?,
            connect_end: self
                .connect_end
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(connect_end)))?,
            ssl_start: self
                .ssl_start
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(ssl_start)))?,
            ssl_end: self
                .ssl_end
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(ssl_end)))?,
            worker_start: self.worker_start.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(worker_start))
            })?,
            worker_ready: self.worker_ready.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(worker_ready))
            })?,
            worker_fetch_start: self.worker_fetch_start.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(worker_fetch_start)
                )
            })?,
            worker_respond_with_settled: self.worker_respond_with_settled.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(worker_respond_with_settled)
                )
            })?,
            worker_router_evaluation_start: self.worker_router_evaluation_start,
            worker_cache_lookup_start: self.worker_cache_lookup_start,
            send_start: self
                .send_start
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(send_start)))?,
            send_end: self
                .send_end
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(send_end)))?,
            push_start: self
                .push_start
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(push_start)))?,
            push_end: self
                .push_end
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(push_end)))?,
            receive_headers_start: self.receive_headers_start.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(receive_headers_start)
                )
            })?,
            receive_headers_end: self.receive_headers_end.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(receive_headers_end)
                )
            })?,
        })
    }
}
impl PostDataEntry {
    pub fn builder() -> PostDataEntryBuilder {
        <PostDataEntryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PostDataEntryBuilder {
    bytes: Option<crate::Binary>,
}
impl PostDataEntryBuilder {
    pub fn bytes(mut self, bytes: impl Into<crate::Binary>) -> Self {
        self.bytes = Some(bytes.into());
        self
    }
    pub fn build(self) -> PostDataEntry {
        PostDataEntry { bytes: self.bytes }
    }
}
impl Request {
    pub fn builder() -> RequestBuilder {
        <RequestBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RequestBuilder {
    url: Option<String>,
    url_fragment: Option<String>,
    method: Option<String>,
    headers: Option<Headers>,
    has_post_data: Option<bool>,
    post_data_entries: Option<Vec<PostDataEntry>>,
    mixed_content_type: Option<crate::browser_protocol::security::types::MixedContentType>,
    initial_priority: Option<ResourcePriority>,
    referrer_policy: Option<RequestReferrerPolicy>,
    is_link_preload: Option<bool>,
    trust_token_params: Option<TrustTokenParams>,
    is_same_site: Option<bool>,
    is_ad_related: Option<bool>,
}
impl RequestBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn url_fragment(mut self, url_fragment: impl Into<String>) -> Self {
        self.url_fragment = Some(url_fragment.into());
        self
    }
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
    pub fn headers(mut self, headers: impl Into<Headers>) -> Self {
        self.headers = Some(headers.into());
        self
    }
    pub fn has_post_data(mut self, has_post_data: impl Into<bool>) -> Self {
        self.has_post_data = Some(has_post_data.into());
        self
    }
    pub fn post_data_entrie(mut self, post_data_entrie: impl Into<PostDataEntry>) -> Self {
        let v = self.post_data_entries.get_or_insert(Vec::new());
        v.push(post_data_entrie.into());
        self
    }
    pub fn post_data_entries<I, S>(mut self, post_data_entries: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<PostDataEntry>,
    {
        let v = self.post_data_entries.get_or_insert(Vec::new());
        for val in post_data_entries {
            v.push(val.into());
        }
        self
    }
    pub fn mixed_content_type(
        mut self,
        mixed_content_type: impl Into<crate::browser_protocol::security::types::MixedContentType>,
    ) -> Self {
        self.mixed_content_type = Some(mixed_content_type.into());
        self
    }
    pub fn initial_priority(mut self, initial_priority: impl Into<ResourcePriority>) -> Self {
        self.initial_priority = Some(initial_priority.into());
        self
    }
    pub fn referrer_policy(mut self, referrer_policy: impl Into<RequestReferrerPolicy>) -> Self {
        self.referrer_policy = Some(referrer_policy.into());
        self
    }
    pub fn is_link_preload(mut self, is_link_preload: impl Into<bool>) -> Self {
        self.is_link_preload = Some(is_link_preload.into());
        self
    }
    pub fn trust_token_params(mut self, trust_token_params: impl Into<TrustTokenParams>) -> Self {
        self.trust_token_params = Some(trust_token_params.into());
        self
    }
    pub fn is_same_site(mut self, is_same_site: impl Into<bool>) -> Self {
        self.is_same_site = Some(is_same_site.into());
        self
    }
    pub fn is_ad_related(mut self, is_ad_related: impl Into<bool>) -> Self {
        self.is_ad_related = Some(is_ad_related.into());
        self
    }
    pub fn build(self) -> Result<Request, String> {
        Ok(Request {
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            url_fragment: self.url_fragment,
            method: self
                .method
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(method)))?,
            headers: self
                .headers
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(headers)))?,
            has_post_data: self.has_post_data,
            post_data_entries: self.post_data_entries,
            mixed_content_type: self.mixed_content_type,
            initial_priority: self.initial_priority.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(initial_priority)
                )
            })?,
            referrer_policy: self.referrer_policy.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(referrer_policy))
            })?,
            is_link_preload: self.is_link_preload,
            trust_token_params: self.trust_token_params,
            is_same_site: self.is_same_site,
            is_ad_related: self.is_ad_related,
        })
    }
}
impl SignedCertificateTimestamp {
    pub fn builder() -> SignedCertificateTimestampBuilder {
        <SignedCertificateTimestampBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SignedCertificateTimestampBuilder {
    status: Option<String>,
    origin: Option<String>,
    log_description: Option<String>,
    log_id: Option<String>,
    timestamp: Option<f64>,
    hash_algorithm: Option<String>,
    signature_algorithm: Option<String>,
    signature_data: Option<String>,
}
impl SignedCertificateTimestampBuilder {
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn log_description(mut self, log_description: impl Into<String>) -> Self {
        self.log_description = Some(log_description.into());
        self
    }
    pub fn log_id(mut self, log_id: impl Into<String>) -> Self {
        self.log_id = Some(log_id.into());
        self
    }
    pub fn timestamp(mut self, timestamp: impl Into<f64>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
    pub fn hash_algorithm(mut self, hash_algorithm: impl Into<String>) -> Self {
        self.hash_algorithm = Some(hash_algorithm.into());
        self
    }
    pub fn signature_algorithm(mut self, signature_algorithm: impl Into<String>) -> Self {
        self.signature_algorithm = Some(signature_algorithm.into());
        self
    }
    pub fn signature_data(mut self, signature_data: impl Into<String>) -> Self {
        self.signature_data = Some(signature_data.into());
        self
    }
    pub fn build(self) -> Result<SignedCertificateTimestamp, String> {
        Ok(SignedCertificateTimestamp {
            status: self
                .status
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(status)))?,
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            log_description: self.log_description.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(log_description))
            })?,
            log_id: self
                .log_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(log_id)))?,
            timestamp: self
                .timestamp
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(timestamp)))?,
            hash_algorithm: self.hash_algorithm.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(hash_algorithm))
            })?,
            signature_algorithm: self.signature_algorithm.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(signature_algorithm)
                )
            })?,
            signature_data: self.signature_data.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(signature_data))
            })?,
        })
    }
}
impl SecurityDetails {
    pub fn builder() -> SecurityDetailsBuilder {
        <SecurityDetailsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SecurityDetailsBuilder {
    protocol: Option<String>,
    key_exchange: Option<String>,
    key_exchange_group: Option<String>,
    cipher: Option<String>,
    mac: Option<String>,
    certificate_id: Option<crate::browser_protocol::security::types::CertificateId>,
    subject_name: Option<String>,
    san_list: Option<Vec<String>>,
    issuer: Option<String>,
    valid_from: Option<TimeSinceEpoch>,
    valid_to: Option<TimeSinceEpoch>,
    signed_certificate_timestamp_list: Option<Vec<SignedCertificateTimestamp>>,
    certificate_transparency_compliance: Option<CertificateTransparencyCompliance>,
    server_signature_algorithm: Option<i64>,
    encrypted_client_hello: Option<bool>,
}
impl SecurityDetailsBuilder {
    pub fn protocol(mut self, protocol: impl Into<String>) -> Self {
        self.protocol = Some(protocol.into());
        self
    }
    pub fn key_exchange(mut self, key_exchange: impl Into<String>) -> Self {
        self.key_exchange = Some(key_exchange.into());
        self
    }
    pub fn key_exchange_group(mut self, key_exchange_group: impl Into<String>) -> Self {
        self.key_exchange_group = Some(key_exchange_group.into());
        self
    }
    pub fn cipher(mut self, cipher: impl Into<String>) -> Self {
        self.cipher = Some(cipher.into());
        self
    }
    pub fn mac(mut self, mac: impl Into<String>) -> Self {
        self.mac = Some(mac.into());
        self
    }
    pub fn certificate_id(
        mut self,
        certificate_id: impl Into<crate::browser_protocol::security::types::CertificateId>,
    ) -> Self {
        self.certificate_id = Some(certificate_id.into());
        self
    }
    pub fn subject_name(mut self, subject_name: impl Into<String>) -> Self {
        self.subject_name = Some(subject_name.into());
        self
    }
    pub fn san_list(mut self, san_list: impl Into<String>) -> Self {
        let v = self.san_list.get_or_insert(Vec::new());
        v.push(san_list.into());
        self
    }
    pub fn san_lists<I, S>(mut self, san_lists: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.san_list.get_or_insert(Vec::new());
        for val in san_lists {
            v.push(val.into());
        }
        self
    }
    pub fn issuer(mut self, issuer: impl Into<String>) -> Self {
        self.issuer = Some(issuer.into());
        self
    }
    pub fn valid_from(mut self, valid_from: impl Into<TimeSinceEpoch>) -> Self {
        self.valid_from = Some(valid_from.into());
        self
    }
    pub fn valid_to(mut self, valid_to: impl Into<TimeSinceEpoch>) -> Self {
        self.valid_to = Some(valid_to.into());
        self
    }
    pub fn signed_certificate_timestamp_list(
        mut self,
        signed_certificate_timestamp_list: impl Into<SignedCertificateTimestamp>,
    ) -> Self {
        let v = self
            .signed_certificate_timestamp_list
            .get_or_insert(Vec::new());
        v.push(signed_certificate_timestamp_list.into());
        self
    }
    pub fn signed_certificate_timestamp_lists<I, S>(
        mut self,
        signed_certificate_timestamp_lists: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<SignedCertificateTimestamp>,
    {
        let v = self
            .signed_certificate_timestamp_list
            .get_or_insert(Vec::new());
        for val in signed_certificate_timestamp_lists {
            v.push(val.into());
        }
        self
    }
    pub fn certificate_transparency_compliance(
        mut self,
        certificate_transparency_compliance: impl Into<CertificateTransparencyCompliance>,
    ) -> Self {
        self.certificate_transparency_compliance = Some(certificate_transparency_compliance.into());
        self
    }
    pub fn server_signature_algorithm(
        mut self,
        server_signature_algorithm: impl Into<i64>,
    ) -> Self {
        self.server_signature_algorithm = Some(server_signature_algorithm.into());
        self
    }
    pub fn encrypted_client_hello(mut self, encrypted_client_hello: impl Into<bool>) -> Self {
        self.encrypted_client_hello = Some(encrypted_client_hello.into());
        self
    }
    pub fn build(self) -> Result<SecurityDetails, String> {
        Ok(SecurityDetails {
            protocol: self
                .protocol
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(protocol)))?,
            key_exchange: self.key_exchange.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(key_exchange))
            })?,
            key_exchange_group: self.key_exchange_group,
            cipher: self
                .cipher
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cipher)))?,
            mac: self.mac,
            certificate_id: self.certificate_id.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(certificate_id))
            })?,
            subject_name: self.subject_name.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(subject_name))
            })?,
            san_list: self
                .san_list
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(san_list)))?,
            issuer: self
                .issuer
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(issuer)))?,
            valid_from: self
                .valid_from
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(valid_from)))?,
            valid_to: self
                .valid_to
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(valid_to)))?,
            signed_certificate_timestamp_list: self.signed_certificate_timestamp_list.ok_or_else(
                || {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(signed_certificate_timestamp_list)
                    )
                },
            )?,
            certificate_transparency_compliance: self
                .certificate_transparency_compliance
                .ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(certificate_transparency_compliance)
                    )
                })?,
            server_signature_algorithm: self.server_signature_algorithm,
            encrypted_client_hello: self.encrypted_client_hello.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(encrypted_client_hello)
                )
            })?,
        })
    }
}
impl CorsErrorStatus {
    pub fn builder() -> CorsErrorStatusBuilder {
        <CorsErrorStatusBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CorsErrorStatusBuilder {
    cors_error: Option<CorsError>,
    failed_parameter: Option<String>,
}
impl CorsErrorStatusBuilder {
    pub fn cors_error(mut self, cors_error: impl Into<CorsError>) -> Self {
        self.cors_error = Some(cors_error.into());
        self
    }
    pub fn failed_parameter(mut self, failed_parameter: impl Into<String>) -> Self {
        self.failed_parameter = Some(failed_parameter.into());
        self
    }
    pub fn build(self) -> Result<CorsErrorStatus, String> {
        Ok(CorsErrorStatus {
            cors_error: self
                .cors_error
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cors_error)))?,
            failed_parameter: self.failed_parameter.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(failed_parameter)
                )
            })?,
        })
    }
}
impl TrustTokenParams {
    pub fn builder() -> TrustTokenParamsBuilder {
        <TrustTokenParamsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct TrustTokenParamsBuilder {
    operation: Option<TrustTokenOperationType>,
    refresh_policy: Option<TrustTokenParamsRefreshPolicy>,
    issuers: Option<Vec<String>>,
}
impl TrustTokenParamsBuilder {
    pub fn operation(mut self, operation: impl Into<TrustTokenOperationType>) -> Self {
        self.operation = Some(operation.into());
        self
    }
    pub fn refresh_policy(
        mut self,
        refresh_policy: impl Into<TrustTokenParamsRefreshPolicy>,
    ) -> Self {
        self.refresh_policy = Some(refresh_policy.into());
        self
    }
    pub fn issuer(mut self, issuer: impl Into<String>) -> Self {
        let v = self.issuers.get_or_insert(Vec::new());
        v.push(issuer.into());
        self
    }
    pub fn issuers<I, S>(mut self, issuers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.issuers.get_or_insert(Vec::new());
        for val in issuers {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<TrustTokenParams, String> {
        Ok(TrustTokenParams {
            operation: self
                .operation
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(operation)))?,
            refresh_policy: self.refresh_policy.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(refresh_policy))
            })?,
            issuers: self.issuers,
        })
    }
}
impl ServiceWorkerRouterInfo {
    pub fn builder() -> ServiceWorkerRouterInfoBuilder {
        <ServiceWorkerRouterInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ServiceWorkerRouterInfoBuilder {
    rule_id_matched: Option<i64>,
    matched_source_type: Option<ServiceWorkerRouterSource>,
    actual_source_type: Option<ServiceWorkerRouterSource>,
}
impl ServiceWorkerRouterInfoBuilder {
    pub fn rule_id_matched(mut self, rule_id_matched: impl Into<i64>) -> Self {
        self.rule_id_matched = Some(rule_id_matched.into());
        self
    }
    pub fn matched_source_type(
        mut self,
        matched_source_type: impl Into<ServiceWorkerRouterSource>,
    ) -> Self {
        self.matched_source_type = Some(matched_source_type.into());
        self
    }
    pub fn actual_source_type(
        mut self,
        actual_source_type: impl Into<ServiceWorkerRouterSource>,
    ) -> Self {
        self.actual_source_type = Some(actual_source_type.into());
        self
    }
    pub fn build(self) -> ServiceWorkerRouterInfo {
        ServiceWorkerRouterInfo {
            rule_id_matched: self.rule_id_matched,
            matched_source_type: self.matched_source_type,
            actual_source_type: self.actual_source_type,
        }
    }
}
impl Response {
    pub fn builder() -> ResponseBuilder {
        <ResponseBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ResponseBuilder {
    url: Option<String>,
    status: Option<i64>,
    status_text: Option<String>,
    headers: Option<Headers>,
    mime_type: Option<String>,
    charset: Option<String>,
    request_headers: Option<Headers>,
    connection_reused: Option<bool>,
    connection_id: Option<f64>,
    remote_ip_address: Option<String>,
    remote_port: Option<i64>,
    from_disk_cache: Option<bool>,
    from_service_worker: Option<bool>,
    from_prefetch_cache: Option<bool>,
    from_early_hints: Option<bool>,
    service_worker_router_info: Option<ServiceWorkerRouterInfo>,
    encoded_data_length: Option<f64>,
    timing: Option<ResourceTiming>,
    service_worker_response_source: Option<ServiceWorkerResponseSource>,
    response_time: Option<TimeSinceEpoch>,
    cache_storage_cache_name: Option<String>,
    protocol: Option<String>,
    alternate_protocol_usage: Option<AlternateProtocolUsage>,
    security_state: Option<crate::browser_protocol::security::types::SecurityState>,
    security_details: Option<SecurityDetails>,
}
impl ResponseBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn status(mut self, status: impl Into<i64>) -> Self {
        self.status = Some(status.into());
        self
    }
    pub fn status_text(mut self, status_text: impl Into<String>) -> Self {
        self.status_text = Some(status_text.into());
        self
    }
    pub fn headers(mut self, headers: impl Into<Headers>) -> Self {
        self.headers = Some(headers.into());
        self
    }
    pub fn mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.mime_type = Some(mime_type.into());
        self
    }
    pub fn charset(mut self, charset: impl Into<String>) -> Self {
        self.charset = Some(charset.into());
        self
    }
    pub fn request_headers(mut self, request_headers: impl Into<Headers>) -> Self {
        self.request_headers = Some(request_headers.into());
        self
    }
    pub fn connection_reused(mut self, connection_reused: impl Into<bool>) -> Self {
        self.connection_reused = Some(connection_reused.into());
        self
    }
    pub fn connection_id(mut self, connection_id: impl Into<f64>) -> Self {
        self.connection_id = Some(connection_id.into());
        self
    }
    pub fn remote_ip_address(mut self, remote_ip_address: impl Into<String>) -> Self {
        self.remote_ip_address = Some(remote_ip_address.into());
        self
    }
    pub fn remote_port(mut self, remote_port: impl Into<i64>) -> Self {
        self.remote_port = Some(remote_port.into());
        self
    }
    pub fn from_disk_cache(mut self, from_disk_cache: impl Into<bool>) -> Self {
        self.from_disk_cache = Some(from_disk_cache.into());
        self
    }
    pub fn from_service_worker(mut self, from_service_worker: impl Into<bool>) -> Self {
        self.from_service_worker = Some(from_service_worker.into());
        self
    }
    pub fn from_prefetch_cache(mut self, from_prefetch_cache: impl Into<bool>) -> Self {
        self.from_prefetch_cache = Some(from_prefetch_cache.into());
        self
    }
    pub fn from_early_hints(mut self, from_early_hints: impl Into<bool>) -> Self {
        self.from_early_hints = Some(from_early_hints.into());
        self
    }
    pub fn service_worker_router_info(
        mut self,
        service_worker_router_info: impl Into<ServiceWorkerRouterInfo>,
    ) -> Self {
        self.service_worker_router_info = Some(service_worker_router_info.into());
        self
    }
    pub fn encoded_data_length(mut self, encoded_data_length: impl Into<f64>) -> Self {
        self.encoded_data_length = Some(encoded_data_length.into());
        self
    }
    pub fn timing(mut self, timing: impl Into<ResourceTiming>) -> Self {
        self.timing = Some(timing.into());
        self
    }
    pub fn service_worker_response_source(
        mut self,
        service_worker_response_source: impl Into<ServiceWorkerResponseSource>,
    ) -> Self {
        self.service_worker_response_source = Some(service_worker_response_source.into());
        self
    }
    pub fn response_time(mut self, response_time: impl Into<TimeSinceEpoch>) -> Self {
        self.response_time = Some(response_time.into());
        self
    }
    pub fn cache_storage_cache_name(mut self, cache_storage_cache_name: impl Into<String>) -> Self {
        self.cache_storage_cache_name = Some(cache_storage_cache_name.into());
        self
    }
    pub fn protocol(mut self, protocol: impl Into<String>) -> Self {
        self.protocol = Some(protocol.into());
        self
    }
    pub fn alternate_protocol_usage(
        mut self,
        alternate_protocol_usage: impl Into<AlternateProtocolUsage>,
    ) -> Self {
        self.alternate_protocol_usage = Some(alternate_protocol_usage.into());
        self
    }
    pub fn security_state(
        mut self,
        security_state: impl Into<crate::browser_protocol::security::types::SecurityState>,
    ) -> Self {
        self.security_state = Some(security_state.into());
        self
    }
    pub fn security_details(mut self, security_details: impl Into<SecurityDetails>) -> Self {
        self.security_details = Some(security_details.into());
        self
    }
    pub fn build(self) -> Result<Response, String> {
        Ok(Response {
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            status: self
                .status
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(status)))?,
            status_text: self
                .status_text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(status_text)))?,
            headers: self
                .headers
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(headers)))?,
            mime_type: self
                .mime_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(mime_type)))?,
            charset: self
                .charset
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(charset)))?,
            request_headers: self.request_headers,
            connection_reused: self.connection_reused.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(connection_reused)
                )
            })?,
            connection_id: self.connection_id.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(connection_id))
            })?,
            remote_ip_address: self.remote_ip_address,
            remote_port: self.remote_port,
            from_disk_cache: self.from_disk_cache,
            from_service_worker: self.from_service_worker,
            from_prefetch_cache: self.from_prefetch_cache,
            from_early_hints: self.from_early_hints,
            service_worker_router_info: self.service_worker_router_info,
            encoded_data_length: self.encoded_data_length.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(encoded_data_length)
                )
            })?,
            timing: self.timing,
            service_worker_response_source: self.service_worker_response_source,
            response_time: self.response_time,
            cache_storage_cache_name: self.cache_storage_cache_name,
            protocol: self.protocol,
            alternate_protocol_usage: self.alternate_protocol_usage,
            security_state: self.security_state.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(security_state))
            })?,
            security_details: self.security_details,
        })
    }
}
impl WebSocketRequest {
    pub fn builder() -> WebSocketRequestBuilder {
        <WebSocketRequestBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct WebSocketRequestBuilder {
    headers: Option<Headers>,
}
impl WebSocketRequestBuilder {
    pub fn headers(mut self, headers: impl Into<Headers>) -> Self {
        self.headers = Some(headers.into());
        self
    }
    pub fn build(self) -> Result<WebSocketRequest, String> {
        Ok(WebSocketRequest {
            headers: self
                .headers
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(headers)))?,
        })
    }
}
impl WebSocketResponse {
    pub fn builder() -> WebSocketResponseBuilder {
        <WebSocketResponseBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct WebSocketResponseBuilder {
    status: Option<i64>,
    status_text: Option<String>,
    headers: Option<Headers>,
    headers_text: Option<String>,
    request_headers: Option<Headers>,
    request_headers_text: Option<String>,
}
impl WebSocketResponseBuilder {
    pub fn status(mut self, status: impl Into<i64>) -> Self {
        self.status = Some(status.into());
        self
    }
    pub fn status_text(mut self, status_text: impl Into<String>) -> Self {
        self.status_text = Some(status_text.into());
        self
    }
    pub fn headers(mut self, headers: impl Into<Headers>) -> Self {
        self.headers = Some(headers.into());
        self
    }
    pub fn headers_text(mut self, headers_text: impl Into<String>) -> Self {
        self.headers_text = Some(headers_text.into());
        self
    }
    pub fn request_headers(mut self, request_headers: impl Into<Headers>) -> Self {
        self.request_headers = Some(request_headers.into());
        self
    }
    pub fn request_headers_text(mut self, request_headers_text: impl Into<String>) -> Self {
        self.request_headers_text = Some(request_headers_text.into());
        self
    }
    pub fn build(self) -> Result<WebSocketResponse, String> {
        Ok(WebSocketResponse {
            status: self
                .status
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(status)))?,
            status_text: self
                .status_text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(status_text)))?,
            headers: self
                .headers
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(headers)))?,
            headers_text: self.headers_text,
            request_headers: self.request_headers,
            request_headers_text: self.request_headers_text,
        })
    }
}
impl WebSocketFrame {
    pub fn builder() -> WebSocketFrameBuilder {
        <WebSocketFrameBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct WebSocketFrameBuilder {
    opcode: Option<f64>,
    mask: Option<bool>,
    payload_data: Option<String>,
}
impl WebSocketFrameBuilder {
    pub fn opcode(mut self, opcode: impl Into<f64>) -> Self {
        self.opcode = Some(opcode.into());
        self
    }
    pub fn mask(mut self, mask: impl Into<bool>) -> Self {
        self.mask = Some(mask.into());
        self
    }
    pub fn payload_data(mut self, payload_data: impl Into<String>) -> Self {
        self.payload_data = Some(payload_data.into());
        self
    }
    pub fn build(self) -> Result<WebSocketFrame, String> {
        Ok(WebSocketFrame {
            opcode: self
                .opcode
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(opcode)))?,
            mask: self
                .mask
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(mask)))?,
            payload_data: self.payload_data.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(payload_data))
            })?,
        })
    }
}
impl CachedResource {
    pub fn builder() -> CachedResourceBuilder {
        <CachedResourceBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CachedResourceBuilder {
    url: Option<String>,
    r#type: Option<ResourceType>,
    response: Option<Response>,
    body_size: Option<f64>,
}
impl CachedResourceBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<ResourceType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn response(mut self, response: impl Into<Response>) -> Self {
        self.response = Some(response.into());
        self
    }
    pub fn body_size(mut self, body_size: impl Into<f64>) -> Self {
        self.body_size = Some(body_size.into());
        self
    }
    pub fn build(self) -> Result<CachedResource, String> {
        Ok(CachedResource {
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            response: self.response,
            body_size: self
                .body_size
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(body_size)))?,
        })
    }
}
impl Initiator {
    pub fn builder() -> InitiatorBuilder {
        <InitiatorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct InitiatorBuilder {
    r#type: Option<InitiatorType>,
    stack: Option<crate::js_protocol::runtime::types::StackTrace>,
    url: Option<String>,
    line_number: Option<f64>,
    column_number: Option<f64>,
    request_id: Option<RequestId>,
}
impl InitiatorBuilder {
    pub fn r#type(mut self, r#type: impl Into<InitiatorType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn stack(
        mut self,
        stack: impl Into<crate::js_protocol::runtime::types::StackTrace>,
    ) -> Self {
        self.stack = Some(stack.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn line_number(mut self, line_number: impl Into<f64>) -> Self {
        self.line_number = Some(line_number.into());
        self
    }
    pub fn column_number(mut self, column_number: impl Into<f64>) -> Self {
        self.column_number = Some(column_number.into());
        self
    }
    pub fn request_id(mut self, request_id: impl Into<RequestId>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn build(self) -> Result<Initiator, String> {
        Ok(Initiator {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            stack: self.stack,
            url: self.url,
            line_number: self.line_number,
            column_number: self.column_number,
            request_id: self.request_id,
        })
    }
}
impl CookiePartitionKey {
    pub fn builder() -> CookiePartitionKeyBuilder {
        <CookiePartitionKeyBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CookiePartitionKeyBuilder {
    top_level_site: Option<String>,
    has_cross_site_ancestor: Option<bool>,
}
impl CookiePartitionKeyBuilder {
    pub fn top_level_site(mut self, top_level_site: impl Into<String>) -> Self {
        self.top_level_site = Some(top_level_site.into());
        self
    }
    pub fn has_cross_site_ancestor(mut self, has_cross_site_ancestor: impl Into<bool>) -> Self {
        self.has_cross_site_ancestor = Some(has_cross_site_ancestor.into());
        self
    }
    pub fn build(self) -> Result<CookiePartitionKey, String> {
        Ok(CookiePartitionKey {
            top_level_site: self.top_level_site.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(top_level_site))
            })?,
            has_cross_site_ancestor: self.has_cross_site_ancestor.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(has_cross_site_ancestor)
                )
            })?,
        })
    }
}
impl Cookie {
    pub fn builder() -> CookieBuilder {
        <CookieBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CookieBuilder {
    name: Option<String>,
    value: Option<String>,
    domain: Option<String>,
    path: Option<String>,
    expires: Option<f64>,
    size: Option<i64>,
    http_only: Option<bool>,
    secure: Option<bool>,
    session: Option<bool>,
    same_site: Option<CookieSameSite>,
    priority: Option<CookiePriority>,
    source_scheme: Option<CookieSourceScheme>,
    source_port: Option<i64>,
    partition_key: Option<CookiePartitionKey>,
    partition_key_opaque: Option<bool>,
}
impl CookieBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
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
    pub fn expires(mut self, expires: impl Into<f64>) -> Self {
        self.expires = Some(expires.into());
        self
    }
    pub fn size(mut self, size: impl Into<i64>) -> Self {
        self.size = Some(size.into());
        self
    }
    pub fn http_only(mut self, http_only: impl Into<bool>) -> Self {
        self.http_only = Some(http_only.into());
        self
    }
    pub fn secure(mut self, secure: impl Into<bool>) -> Self {
        self.secure = Some(secure.into());
        self
    }
    pub fn session(mut self, session: impl Into<bool>) -> Self {
        self.session = Some(session.into());
        self
    }
    pub fn same_site(mut self, same_site: impl Into<CookieSameSite>) -> Self {
        self.same_site = Some(same_site.into());
        self
    }
    pub fn priority(mut self, priority: impl Into<CookiePriority>) -> Self {
        self.priority = Some(priority.into());
        self
    }
    pub fn source_scheme(mut self, source_scheme: impl Into<CookieSourceScheme>) -> Self {
        self.source_scheme = Some(source_scheme.into());
        self
    }
    pub fn source_port(mut self, source_port: impl Into<i64>) -> Self {
        self.source_port = Some(source_port.into());
        self
    }
    pub fn partition_key(mut self, partition_key: impl Into<CookiePartitionKey>) -> Self {
        self.partition_key = Some(partition_key.into());
        self
    }
    pub fn partition_key_opaque(mut self, partition_key_opaque: impl Into<bool>) -> Self {
        self.partition_key_opaque = Some(partition_key_opaque.into());
        self
    }
    pub fn build(self) -> Result<Cookie, String> {
        Ok(Cookie {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            domain: self
                .domain
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(domain)))?,
            path: self
                .path
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(path)))?,
            expires: self
                .expires
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(expires)))?,
            size: self
                .size
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(size)))?,
            http_only: self
                .http_only
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(http_only)))?,
            secure: self
                .secure
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(secure)))?,
            session: self
                .session
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(session)))?,
            same_site: self.same_site,
            priority: self
                .priority
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(priority)))?,
            source_scheme: self.source_scheme.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(source_scheme))
            })?,
            source_port: self
                .source_port
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(source_port)))?,
            partition_key: self.partition_key,
            partition_key_opaque: self.partition_key_opaque,
        })
    }
}
impl BlockedSetCookieWithReason {
    pub fn builder() -> BlockedSetCookieWithReasonBuilder {
        <BlockedSetCookieWithReasonBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BlockedSetCookieWithReasonBuilder {
    blocked_reasons: Option<Vec<SetCookieBlockedReason>>,
    cookie_line: Option<String>,
    cookie: Option<Cookie>,
}
impl BlockedSetCookieWithReasonBuilder {
    pub fn blocked_reason(mut self, blocked_reason: impl Into<SetCookieBlockedReason>) -> Self {
        let v = self.blocked_reasons.get_or_insert(Vec::new());
        v.push(blocked_reason.into());
        self
    }
    pub fn blocked_reasons<I, S>(mut self, blocked_reasons: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<SetCookieBlockedReason>,
    {
        let v = self.blocked_reasons.get_or_insert(Vec::new());
        for val in blocked_reasons {
            v.push(val.into());
        }
        self
    }
    pub fn cookie_line(mut self, cookie_line: impl Into<String>) -> Self {
        self.cookie_line = Some(cookie_line.into());
        self
    }
    pub fn cookie(mut self, cookie: impl Into<Cookie>) -> Self {
        self.cookie = Some(cookie.into());
        self
    }
    pub fn build(self) -> Result<BlockedSetCookieWithReason, String> {
        Ok(BlockedSetCookieWithReason {
            blocked_reasons: self.blocked_reasons.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(blocked_reasons))
            })?,
            cookie_line: self
                .cookie_line
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cookie_line)))?,
            cookie: self.cookie,
        })
    }
}
impl ExemptedSetCookieWithReason {
    pub fn builder() -> ExemptedSetCookieWithReasonBuilder {
        <ExemptedSetCookieWithReasonBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ExemptedSetCookieWithReasonBuilder {
    exemption_reason: Option<CookieExemptionReason>,
    cookie_line: Option<String>,
    cookie: Option<Cookie>,
}
impl ExemptedSetCookieWithReasonBuilder {
    pub fn exemption_reason(mut self, exemption_reason: impl Into<CookieExemptionReason>) -> Self {
        self.exemption_reason = Some(exemption_reason.into());
        self
    }
    pub fn cookie_line(mut self, cookie_line: impl Into<String>) -> Self {
        self.cookie_line = Some(cookie_line.into());
        self
    }
    pub fn cookie(mut self, cookie: impl Into<Cookie>) -> Self {
        self.cookie = Some(cookie.into());
        self
    }
    pub fn build(self) -> Result<ExemptedSetCookieWithReason, String> {
        Ok(ExemptedSetCookieWithReason {
            exemption_reason: self.exemption_reason.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(exemption_reason)
                )
            })?,
            cookie_line: self
                .cookie_line
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cookie_line)))?,
            cookie: self
                .cookie
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cookie)))?,
        })
    }
}
impl AssociatedCookie {
    pub fn builder() -> AssociatedCookieBuilder {
        <AssociatedCookieBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AssociatedCookieBuilder {
    cookie: Option<Cookie>,
    blocked_reasons: Option<Vec<CookieBlockedReason>>,
    exemption_reason: Option<CookieExemptionReason>,
}
impl AssociatedCookieBuilder {
    pub fn cookie(mut self, cookie: impl Into<Cookie>) -> Self {
        self.cookie = Some(cookie.into());
        self
    }
    pub fn blocked_reason(mut self, blocked_reason: impl Into<CookieBlockedReason>) -> Self {
        let v = self.blocked_reasons.get_or_insert(Vec::new());
        v.push(blocked_reason.into());
        self
    }
    pub fn blocked_reasons<I, S>(mut self, blocked_reasons: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CookieBlockedReason>,
    {
        let v = self.blocked_reasons.get_or_insert(Vec::new());
        for val in blocked_reasons {
            v.push(val.into());
        }
        self
    }
    pub fn exemption_reason(mut self, exemption_reason: impl Into<CookieExemptionReason>) -> Self {
        self.exemption_reason = Some(exemption_reason.into());
        self
    }
    pub fn build(self) -> Result<AssociatedCookie, String> {
        Ok(AssociatedCookie {
            cookie: self
                .cookie
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cookie)))?,
            blocked_reasons: self.blocked_reasons.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(blocked_reasons))
            })?,
            exemption_reason: self.exemption_reason,
        })
    }
}
impl CookieParam {
    pub fn builder() -> CookieParamBuilder {
        <CookieParamBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CookieParamBuilder {
    name: Option<String>,
    value: Option<String>,
    url: Option<String>,
    domain: Option<String>,
    path: Option<String>,
    secure: Option<bool>,
    http_only: Option<bool>,
    same_site: Option<CookieSameSite>,
    expires: Option<TimeSinceEpoch>,
    priority: Option<CookiePriority>,
    source_scheme: Option<CookieSourceScheme>,
    source_port: Option<i64>,
    partition_key: Option<CookiePartitionKey>,
}
impl CookieParamBuilder {
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
    pub fn same_site(mut self, same_site: impl Into<CookieSameSite>) -> Self {
        self.same_site = Some(same_site.into());
        self
    }
    pub fn expires(mut self, expires: impl Into<TimeSinceEpoch>) -> Self {
        self.expires = Some(expires.into());
        self
    }
    pub fn priority(mut self, priority: impl Into<CookiePriority>) -> Self {
        self.priority = Some(priority.into());
        self
    }
    pub fn source_scheme(mut self, source_scheme: impl Into<CookieSourceScheme>) -> Self {
        self.source_scheme = Some(source_scheme.into());
        self
    }
    pub fn source_port(mut self, source_port: impl Into<i64>) -> Self {
        self.source_port = Some(source_port.into());
        self
    }
    pub fn partition_key(mut self, partition_key: impl Into<CookiePartitionKey>) -> Self {
        self.partition_key = Some(partition_key.into());
        self
    }
    pub fn build(self) -> Result<CookieParam, String> {
        Ok(CookieParam {
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
        })
    }
}
impl AuthChallenge {
    pub fn builder() -> AuthChallengeBuilder {
        <AuthChallengeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AuthChallengeBuilder {
    source: Option<AuthChallengeSource>,
    origin: Option<String>,
    scheme: Option<String>,
    realm: Option<String>,
}
impl AuthChallengeBuilder {
    pub fn source(mut self, source: impl Into<AuthChallengeSource>) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn scheme(mut self, scheme: impl Into<String>) -> Self {
        self.scheme = Some(scheme.into());
        self
    }
    pub fn realm(mut self, realm: impl Into<String>) -> Self {
        self.realm = Some(realm.into());
        self
    }
    pub fn build(self) -> Result<AuthChallenge, String> {
        Ok(AuthChallenge {
            source: self.source,
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            scheme: self
                .scheme
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(scheme)))?,
            realm: self
                .realm
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(realm)))?,
        })
    }
}
impl AuthChallengeResponse {
    pub fn builder() -> AuthChallengeResponseBuilder {
        <AuthChallengeResponseBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AuthChallengeResponseBuilder {
    response: Option<AuthChallengeResponseResponse>,
    username: Option<String>,
    password: Option<String>,
}
impl AuthChallengeResponseBuilder {
    pub fn response(mut self, response: impl Into<AuthChallengeResponseResponse>) -> Self {
        self.response = Some(response.into());
        self
    }
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }
    pub fn build(self) -> Result<AuthChallengeResponse, String> {
        Ok(AuthChallengeResponse {
            response: self
                .response
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(response)))?,
            username: self.username,
            password: self.password,
        })
    }
}
impl RequestPattern {
    pub fn builder() -> RequestPatternBuilder {
        <RequestPatternBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RequestPatternBuilder {
    url_pattern: Option<String>,
    resource_type: Option<ResourceType>,
    interception_stage: Option<InterceptionStage>,
}
impl RequestPatternBuilder {
    pub fn url_pattern(mut self, url_pattern: impl Into<String>) -> Self {
        self.url_pattern = Some(url_pattern.into());
        self
    }
    pub fn resource_type(mut self, resource_type: impl Into<ResourceType>) -> Self {
        self.resource_type = Some(resource_type.into());
        self
    }
    pub fn interception_stage(mut self, interception_stage: impl Into<InterceptionStage>) -> Self {
        self.interception_stage = Some(interception_stage.into());
        self
    }
    pub fn build(self) -> RequestPattern {
        RequestPattern {
            url_pattern: self.url_pattern,
            resource_type: self.resource_type,
            interception_stage: self.interception_stage,
        }
    }
}
impl SignedExchangeSignature {
    pub fn builder() -> SignedExchangeSignatureBuilder {
        <SignedExchangeSignatureBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SignedExchangeSignatureBuilder {
    label: Option<String>,
    signature: Option<String>,
    integrity: Option<String>,
    cert_url: Option<String>,
    cert_sha256: Option<String>,
    validity_url: Option<String>,
    date: Option<i64>,
    expires: Option<i64>,
    certificates: Option<Vec<String>>,
}
impl SignedExchangeSignatureBuilder {
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn signature(mut self, signature: impl Into<String>) -> Self {
        self.signature = Some(signature.into());
        self
    }
    pub fn integrity(mut self, integrity: impl Into<String>) -> Self {
        self.integrity = Some(integrity.into());
        self
    }
    pub fn cert_url(mut self, cert_url: impl Into<String>) -> Self {
        self.cert_url = Some(cert_url.into());
        self
    }
    pub fn cert_sha256(mut self, cert_sha256: impl Into<String>) -> Self {
        self.cert_sha256 = Some(cert_sha256.into());
        self
    }
    pub fn validity_url(mut self, validity_url: impl Into<String>) -> Self {
        self.validity_url = Some(validity_url.into());
        self
    }
    pub fn date(mut self, date: impl Into<i64>) -> Self {
        self.date = Some(date.into());
        self
    }
    pub fn expires(mut self, expires: impl Into<i64>) -> Self {
        self.expires = Some(expires.into());
        self
    }
    pub fn certificate(mut self, certificate: impl Into<String>) -> Self {
        let v = self.certificates.get_or_insert(Vec::new());
        v.push(certificate.into());
        self
    }
    pub fn certificates<I, S>(mut self, certificates: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.certificates.get_or_insert(Vec::new());
        for val in certificates {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SignedExchangeSignature, String> {
        Ok(SignedExchangeSignature {
            label: self
                .label
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(label)))?,
            signature: self
                .signature
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(signature)))?,
            integrity: self
                .integrity
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(integrity)))?,
            cert_url: self.cert_url,
            cert_sha256: self.cert_sha256,
            validity_url: self.validity_url.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(validity_url))
            })?,
            date: self
                .date
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(date)))?,
            expires: self
                .expires
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(expires)))?,
            certificates: self.certificates,
        })
    }
}
impl SignedExchangeHeader {
    pub fn builder() -> SignedExchangeHeaderBuilder {
        <SignedExchangeHeaderBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SignedExchangeHeaderBuilder {
    request_url: Option<String>,
    response_code: Option<i64>,
    response_headers: Option<Headers>,
    signatures: Option<Vec<SignedExchangeSignature>>,
    header_integrity: Option<String>,
}
impl SignedExchangeHeaderBuilder {
    pub fn request_url(mut self, request_url: impl Into<String>) -> Self {
        self.request_url = Some(request_url.into());
        self
    }
    pub fn response_code(mut self, response_code: impl Into<i64>) -> Self {
        self.response_code = Some(response_code.into());
        self
    }
    pub fn response_headers(mut self, response_headers: impl Into<Headers>) -> Self {
        self.response_headers = Some(response_headers.into());
        self
    }
    pub fn signature(mut self, signature: impl Into<SignedExchangeSignature>) -> Self {
        let v = self.signatures.get_or_insert(Vec::new());
        v.push(signature.into());
        self
    }
    pub fn signatures<I, S>(mut self, signatures: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<SignedExchangeSignature>,
    {
        let v = self.signatures.get_or_insert(Vec::new());
        for val in signatures {
            v.push(val.into());
        }
        self
    }
    pub fn header_integrity(mut self, header_integrity: impl Into<String>) -> Self {
        self.header_integrity = Some(header_integrity.into());
        self
    }
    pub fn build(self) -> Result<SignedExchangeHeader, String> {
        Ok(SignedExchangeHeader {
            request_url: self
                .request_url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request_url)))?,
            response_code: self.response_code.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(response_code))
            })?,
            response_headers: self.response_headers.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(response_headers)
                )
            })?,
            signatures: self
                .signatures
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(signatures)))?,
            header_integrity: self.header_integrity.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(header_integrity)
                )
            })?,
        })
    }
}
impl SignedExchangeError {
    pub fn builder() -> SignedExchangeErrorBuilder {
        <SignedExchangeErrorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SignedExchangeErrorBuilder {
    message: Option<String>,
    signature_index: Option<i64>,
    error_field: Option<SignedExchangeErrorField>,
}
impl SignedExchangeErrorBuilder {
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
    pub fn signature_index(mut self, signature_index: impl Into<i64>) -> Self {
        self.signature_index = Some(signature_index.into());
        self
    }
    pub fn error_field(mut self, error_field: impl Into<SignedExchangeErrorField>) -> Self {
        self.error_field = Some(error_field.into());
        self
    }
    pub fn build(self) -> Result<SignedExchangeError, String> {
        Ok(SignedExchangeError {
            message: self
                .message
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(message)))?,
            signature_index: self.signature_index,
            error_field: self.error_field,
        })
    }
}
impl SignedExchangeInfo {
    pub fn builder() -> SignedExchangeInfoBuilder {
        <SignedExchangeInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SignedExchangeInfoBuilder {
    outer_response: Option<Response>,
    has_extra_info: Option<bool>,
    header: Option<SignedExchangeHeader>,
    security_details: Option<SecurityDetails>,
    errors: Option<Vec<SignedExchangeError>>,
}
impl SignedExchangeInfoBuilder {
    pub fn outer_response(mut self, outer_response: impl Into<Response>) -> Self {
        self.outer_response = Some(outer_response.into());
        self
    }
    pub fn has_extra_info(mut self, has_extra_info: impl Into<bool>) -> Self {
        self.has_extra_info = Some(has_extra_info.into());
        self
    }
    pub fn header(mut self, header: impl Into<SignedExchangeHeader>) -> Self {
        self.header = Some(header.into());
        self
    }
    pub fn security_details(mut self, security_details: impl Into<SecurityDetails>) -> Self {
        self.security_details = Some(security_details.into());
        self
    }
    pub fn error(mut self, error: impl Into<SignedExchangeError>) -> Self {
        let v = self.errors.get_or_insert(Vec::new());
        v.push(error.into());
        self
    }
    pub fn errors<I, S>(mut self, errors: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<SignedExchangeError>,
    {
        let v = self.errors.get_or_insert(Vec::new());
        for val in errors {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SignedExchangeInfo, String> {
        Ok(SignedExchangeInfo {
            outer_response: self.outer_response.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(outer_response))
            })?,
            has_extra_info: self.has_extra_info.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(has_extra_info))
            })?,
            header: self.header,
            security_details: self.security_details,
            errors: self.errors,
        })
    }
}
impl NetworkConditions {
    pub fn builder() -> NetworkConditionsBuilder {
        <NetworkConditionsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct NetworkConditionsBuilder {
    url_pattern: Option<String>,
    latency: Option<f64>,
    download_throughput: Option<f64>,
    upload_throughput: Option<f64>,
    connection_type: Option<ConnectionType>,
    packet_loss: Option<f64>,
    packet_queue_length: Option<i64>,
    packet_reordering: Option<bool>,
}
impl NetworkConditionsBuilder {
    pub fn url_pattern(mut self, url_pattern: impl Into<String>) -> Self {
        self.url_pattern = Some(url_pattern.into());
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
    pub fn connection_type(mut self, connection_type: impl Into<ConnectionType>) -> Self {
        self.connection_type = Some(connection_type.into());
        self
    }
    pub fn packet_loss(mut self, packet_loss: impl Into<f64>) -> Self {
        self.packet_loss = Some(packet_loss.into());
        self
    }
    pub fn packet_queue_length(mut self, packet_queue_length: impl Into<i64>) -> Self {
        self.packet_queue_length = Some(packet_queue_length.into());
        self
    }
    pub fn packet_reordering(mut self, packet_reordering: impl Into<bool>) -> Self {
        self.packet_reordering = Some(packet_reordering.into());
        self
    }
    pub fn build(self) -> Result<NetworkConditions, String> {
        Ok(NetworkConditions {
            url_pattern: self
                .url_pattern
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url_pattern)))?,
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
            packet_loss: self.packet_loss,
            packet_queue_length: self.packet_queue_length,
            packet_reordering: self.packet_reordering,
        })
    }
}
impl BlockPattern {
    pub fn builder() -> BlockPatternBuilder {
        <BlockPatternBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BlockPatternBuilder {
    url_pattern: Option<String>,
    block: Option<bool>,
}
impl BlockPatternBuilder {
    pub fn url_pattern(mut self, url_pattern: impl Into<String>) -> Self {
        self.url_pattern = Some(url_pattern.into());
        self
    }
    pub fn block(mut self, block: impl Into<bool>) -> Self {
        self.block = Some(block.into());
        self
    }
    pub fn build(self) -> Result<BlockPattern, String> {
        Ok(BlockPattern {
            url_pattern: self
                .url_pattern
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url_pattern)))?,
            block: self
                .block
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(block)))?,
        })
    }
}
impl DirectTcpSocketOptions {
    pub fn builder() -> DirectTcpSocketOptionsBuilder {
        <DirectTcpSocketOptionsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DirectTcpSocketOptionsBuilder {
    no_delay: Option<bool>,
    keep_alive_delay: Option<f64>,
    send_buffer_size: Option<f64>,
    receive_buffer_size: Option<f64>,
    dns_query_type: Option<DirectSocketDnsQueryType>,
}
impl DirectTcpSocketOptionsBuilder {
    pub fn no_delay(mut self, no_delay: impl Into<bool>) -> Self {
        self.no_delay = Some(no_delay.into());
        self
    }
    pub fn keep_alive_delay(mut self, keep_alive_delay: impl Into<f64>) -> Self {
        self.keep_alive_delay = Some(keep_alive_delay.into());
        self
    }
    pub fn send_buffer_size(mut self, send_buffer_size: impl Into<f64>) -> Self {
        self.send_buffer_size = Some(send_buffer_size.into());
        self
    }
    pub fn receive_buffer_size(mut self, receive_buffer_size: impl Into<f64>) -> Self {
        self.receive_buffer_size = Some(receive_buffer_size.into());
        self
    }
    pub fn dns_query_type(mut self, dns_query_type: impl Into<DirectSocketDnsQueryType>) -> Self {
        self.dns_query_type = Some(dns_query_type.into());
        self
    }
    pub fn build(self) -> Result<DirectTcpSocketOptions, String> {
        Ok(DirectTcpSocketOptions {
            no_delay: self
                .no_delay
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(no_delay)))?,
            keep_alive_delay: self.keep_alive_delay,
            send_buffer_size: self.send_buffer_size,
            receive_buffer_size: self.receive_buffer_size,
            dns_query_type: self.dns_query_type,
        })
    }
}
impl DirectUdpSocketOptions {
    pub fn builder() -> DirectUdpSocketOptionsBuilder {
        <DirectUdpSocketOptionsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DirectUdpSocketOptionsBuilder {
    remote_addr: Option<String>,
    remote_port: Option<i64>,
    local_addr: Option<String>,
    local_port: Option<i64>,
    dns_query_type: Option<DirectSocketDnsQueryType>,
    send_buffer_size: Option<f64>,
    receive_buffer_size: Option<f64>,
    multicast_loopback: Option<bool>,
    multicast_time_to_live: Option<i64>,
    multicast_allow_address_sharing: Option<bool>,
}
impl DirectUdpSocketOptionsBuilder {
    pub fn remote_addr(mut self, remote_addr: impl Into<String>) -> Self {
        self.remote_addr = Some(remote_addr.into());
        self
    }
    pub fn remote_port(mut self, remote_port: impl Into<i64>) -> Self {
        self.remote_port = Some(remote_port.into());
        self
    }
    pub fn local_addr(mut self, local_addr: impl Into<String>) -> Self {
        self.local_addr = Some(local_addr.into());
        self
    }
    pub fn local_port(mut self, local_port: impl Into<i64>) -> Self {
        self.local_port = Some(local_port.into());
        self
    }
    pub fn dns_query_type(mut self, dns_query_type: impl Into<DirectSocketDnsQueryType>) -> Self {
        self.dns_query_type = Some(dns_query_type.into());
        self
    }
    pub fn send_buffer_size(mut self, send_buffer_size: impl Into<f64>) -> Self {
        self.send_buffer_size = Some(send_buffer_size.into());
        self
    }
    pub fn receive_buffer_size(mut self, receive_buffer_size: impl Into<f64>) -> Self {
        self.receive_buffer_size = Some(receive_buffer_size.into());
        self
    }
    pub fn multicast_loopback(mut self, multicast_loopback: impl Into<bool>) -> Self {
        self.multicast_loopback = Some(multicast_loopback.into());
        self
    }
    pub fn multicast_time_to_live(mut self, multicast_time_to_live: impl Into<i64>) -> Self {
        self.multicast_time_to_live = Some(multicast_time_to_live.into());
        self
    }
    pub fn multicast_allow_address_sharing(
        mut self,
        multicast_allow_address_sharing: impl Into<bool>,
    ) -> Self {
        self.multicast_allow_address_sharing = Some(multicast_allow_address_sharing.into());
        self
    }
    pub fn build(self) -> DirectUdpSocketOptions {
        DirectUdpSocketOptions {
            remote_addr: self.remote_addr,
            remote_port: self.remote_port,
            local_addr: self.local_addr,
            local_port: self.local_port,
            dns_query_type: self.dns_query_type,
            send_buffer_size: self.send_buffer_size,
            receive_buffer_size: self.receive_buffer_size,
            multicast_loopback: self.multicast_loopback,
            multicast_time_to_live: self.multicast_time_to_live,
            multicast_allow_address_sharing: self.multicast_allow_address_sharing,
        }
    }
}
impl DirectUdpMessage {
    pub fn builder() -> DirectUdpMessageBuilder {
        <DirectUdpMessageBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DirectUdpMessageBuilder {
    data: Option<crate::Binary>,
    remote_addr: Option<String>,
    remote_port: Option<i64>,
}
impl DirectUdpMessageBuilder {
    pub fn data(mut self, data: impl Into<crate::Binary>) -> Self {
        self.data = Some(data.into());
        self
    }
    pub fn remote_addr(mut self, remote_addr: impl Into<String>) -> Self {
        self.remote_addr = Some(remote_addr.into());
        self
    }
    pub fn remote_port(mut self, remote_port: impl Into<i64>) -> Self {
        self.remote_port = Some(remote_port.into());
        self
    }
    pub fn build(self) -> Result<DirectUdpMessage, String> {
        Ok(DirectUdpMessage {
            data: self
                .data
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(data)))?,
            remote_addr: self.remote_addr,
            remote_port: self.remote_port,
        })
    }
}
impl ConnectTiming {
    pub fn builder() -> ConnectTimingBuilder {
        <ConnectTimingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ConnectTimingBuilder {
    request_time: Option<f64>,
}
impl ConnectTimingBuilder {
    pub fn request_time(mut self, request_time: impl Into<f64>) -> Self {
        self.request_time = Some(request_time.into());
        self
    }
    pub fn build(self) -> Result<ConnectTiming, String> {
        Ok(ConnectTiming {
            request_time: self.request_time.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(request_time))
            })?,
        })
    }
}
impl ClientSecurityState {
    pub fn builder() -> ClientSecurityStateBuilder {
        <ClientSecurityStateBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ClientSecurityStateBuilder {
    initiator_is_secure_context: Option<bool>,
    initiator_ip_address_space: Option<IpAddressSpace>,
    local_network_access_request_policy: Option<LocalNetworkAccessRequestPolicy>,
}
impl ClientSecurityStateBuilder {
    pub fn initiator_is_secure_context(
        mut self,
        initiator_is_secure_context: impl Into<bool>,
    ) -> Self {
        self.initiator_is_secure_context = Some(initiator_is_secure_context.into());
        self
    }
    pub fn initiator_ip_address_space(
        mut self,
        initiator_ip_address_space: impl Into<IpAddressSpace>,
    ) -> Self {
        self.initiator_ip_address_space = Some(initiator_ip_address_space.into());
        self
    }
    pub fn local_network_access_request_policy(
        mut self,
        local_network_access_request_policy: impl Into<LocalNetworkAccessRequestPolicy>,
    ) -> Self {
        self.local_network_access_request_policy = Some(local_network_access_request_policy.into());
        self
    }
    pub fn build(self) -> Result<ClientSecurityState, String> {
        Ok(ClientSecurityState {
            initiator_is_secure_context: self.initiator_is_secure_context.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(initiator_is_secure_context)
                )
            })?,
            initiator_ip_address_space: self.initiator_ip_address_space.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(initiator_ip_address_space)
                )
            })?,
            local_network_access_request_policy: self
                .local_network_access_request_policy
                .ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(local_network_access_request_policy)
                    )
                })?,
        })
    }
}
impl CrossOriginOpenerPolicyStatus {
    pub fn builder() -> CrossOriginOpenerPolicyStatusBuilder {
        <CrossOriginOpenerPolicyStatusBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CrossOriginOpenerPolicyStatusBuilder {
    value: Option<CrossOriginOpenerPolicyValue>,
    report_only_value: Option<CrossOriginOpenerPolicyValue>,
    reporting_endpoint: Option<String>,
    report_only_reporting_endpoint: Option<String>,
}
impl CrossOriginOpenerPolicyStatusBuilder {
    pub fn value(mut self, value: impl Into<CrossOriginOpenerPolicyValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn report_only_value(
        mut self,
        report_only_value: impl Into<CrossOriginOpenerPolicyValue>,
    ) -> Self {
        self.report_only_value = Some(report_only_value.into());
        self
    }
    pub fn reporting_endpoint(mut self, reporting_endpoint: impl Into<String>) -> Self {
        self.reporting_endpoint = Some(reporting_endpoint.into());
        self
    }
    pub fn report_only_reporting_endpoint(
        mut self,
        report_only_reporting_endpoint: impl Into<String>,
    ) -> Self {
        self.report_only_reporting_endpoint = Some(report_only_reporting_endpoint.into());
        self
    }
    pub fn build(self) -> Result<CrossOriginOpenerPolicyStatus, String> {
        Ok(CrossOriginOpenerPolicyStatus {
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            report_only_value: self.report_only_value.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(report_only_value)
                )
            })?,
            reporting_endpoint: self.reporting_endpoint,
            report_only_reporting_endpoint: self.report_only_reporting_endpoint,
        })
    }
}
impl CrossOriginEmbedderPolicyStatus {
    pub fn builder() -> CrossOriginEmbedderPolicyStatusBuilder {
        <CrossOriginEmbedderPolicyStatusBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CrossOriginEmbedderPolicyStatusBuilder {
    value: Option<CrossOriginEmbedderPolicyValue>,
    report_only_value: Option<CrossOriginEmbedderPolicyValue>,
    reporting_endpoint: Option<String>,
    report_only_reporting_endpoint: Option<String>,
}
impl CrossOriginEmbedderPolicyStatusBuilder {
    pub fn value(mut self, value: impl Into<CrossOriginEmbedderPolicyValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn report_only_value(
        mut self,
        report_only_value: impl Into<CrossOriginEmbedderPolicyValue>,
    ) -> Self {
        self.report_only_value = Some(report_only_value.into());
        self
    }
    pub fn reporting_endpoint(mut self, reporting_endpoint: impl Into<String>) -> Self {
        self.reporting_endpoint = Some(reporting_endpoint.into());
        self
    }
    pub fn report_only_reporting_endpoint(
        mut self,
        report_only_reporting_endpoint: impl Into<String>,
    ) -> Self {
        self.report_only_reporting_endpoint = Some(report_only_reporting_endpoint.into());
        self
    }
    pub fn build(self) -> Result<CrossOriginEmbedderPolicyStatus, String> {
        Ok(CrossOriginEmbedderPolicyStatus {
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            report_only_value: self.report_only_value.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(report_only_value)
                )
            })?,
            reporting_endpoint: self.reporting_endpoint,
            report_only_reporting_endpoint: self.report_only_reporting_endpoint,
        })
    }
}
impl ContentSecurityPolicyStatus {
    pub fn builder() -> ContentSecurityPolicyStatusBuilder {
        <ContentSecurityPolicyStatusBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ContentSecurityPolicyStatusBuilder {
    effective_directives: Option<String>,
    is_enforced: Option<bool>,
    source: Option<ContentSecurityPolicySource>,
}
impl ContentSecurityPolicyStatusBuilder {
    pub fn effective_directives(mut self, effective_directives: impl Into<String>) -> Self {
        self.effective_directives = Some(effective_directives.into());
        self
    }
    pub fn is_enforced(mut self, is_enforced: impl Into<bool>) -> Self {
        self.is_enforced = Some(is_enforced.into());
        self
    }
    pub fn source(mut self, source: impl Into<ContentSecurityPolicySource>) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn build(self) -> Result<ContentSecurityPolicyStatus, String> {
        Ok(ContentSecurityPolicyStatus {
            effective_directives: self.effective_directives.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(effective_directives)
                )
            })?,
            is_enforced: self
                .is_enforced
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(is_enforced)))?,
            source: self
                .source
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(source)))?,
        })
    }
}
impl SecurityIsolationStatus {
    pub fn builder() -> SecurityIsolationStatusBuilder {
        <SecurityIsolationStatusBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SecurityIsolationStatusBuilder {
    coop: Option<CrossOriginOpenerPolicyStatus>,
    coep: Option<CrossOriginEmbedderPolicyStatus>,
    csp: Option<Vec<ContentSecurityPolicyStatus>>,
}
impl SecurityIsolationStatusBuilder {
    pub fn coop(mut self, coop: impl Into<CrossOriginOpenerPolicyStatus>) -> Self {
        self.coop = Some(coop.into());
        self
    }
    pub fn coep(mut self, coep: impl Into<CrossOriginEmbedderPolicyStatus>) -> Self {
        self.coep = Some(coep.into());
        self
    }
    pub fn csp(mut self, csp: impl Into<ContentSecurityPolicyStatus>) -> Self {
        let v = self.csp.get_or_insert(Vec::new());
        v.push(csp.into());
        self
    }
    pub fn csps<I, S>(mut self, csps: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<ContentSecurityPolicyStatus>,
    {
        let v = self.csp.get_or_insert(Vec::new());
        for val in csps {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> SecurityIsolationStatus {
        SecurityIsolationStatus {
            coop: self.coop,
            coep: self.coep,
            csp: self.csp,
        }
    }
}
impl ReportingApiReport {
    pub fn builder() -> ReportingApiReportBuilder {
        <ReportingApiReportBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ReportingApiReportBuilder {
    id: Option<ReportId>,
    initiator_url: Option<String>,
    destination: Option<String>,
    r#type: Option<String>,
    timestamp: Option<TimeSinceEpoch>,
    depth: Option<i64>,
    completed_attempts: Option<i64>,
    body: Option<serde_json::Value>,
    status: Option<ReportStatus>,
}
impl ReportingApiReportBuilder {
    pub fn id(mut self, id: impl Into<ReportId>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn initiator_url(mut self, initiator_url: impl Into<String>) -> Self {
        self.initiator_url = Some(initiator_url.into());
        self
    }
    pub fn destination(mut self, destination: impl Into<String>) -> Self {
        self.destination = Some(destination.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn timestamp(mut self, timestamp: impl Into<TimeSinceEpoch>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
    pub fn depth(mut self, depth: impl Into<i64>) -> Self {
        self.depth = Some(depth.into());
        self
    }
    pub fn completed_attempts(mut self, completed_attempts: impl Into<i64>) -> Self {
        self.completed_attempts = Some(completed_attempts.into());
        self
    }
    pub fn body(mut self, body: impl Into<serde_json::Value>) -> Self {
        self.body = Some(body.into());
        self
    }
    pub fn status(mut self, status: impl Into<ReportStatus>) -> Self {
        self.status = Some(status.into());
        self
    }
    pub fn build(self) -> Result<ReportingApiReport, String> {
        Ok(ReportingApiReport {
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            initiator_url: self.initiator_url.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(initiator_url))
            })?,
            destination: self
                .destination
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(destination)))?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            timestamp: self
                .timestamp
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(timestamp)))?,
            depth: self
                .depth
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(depth)))?,
            completed_attempts: self.completed_attempts.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(completed_attempts)
                )
            })?,
            body: self
                .body
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(body)))?,
            status: self
                .status
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(status)))?,
        })
    }
}
impl ReportingApiEndpoint {
    pub fn builder() -> ReportingApiEndpointBuilder {
        <ReportingApiEndpointBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ReportingApiEndpointBuilder {
    url: Option<String>,
    group_name: Option<String>,
}
impl ReportingApiEndpointBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn group_name(mut self, group_name: impl Into<String>) -> Self {
        self.group_name = Some(group_name.into());
        self
    }
    pub fn build(self) -> Result<ReportingApiEndpoint, String> {
        Ok(ReportingApiEndpoint {
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            group_name: self
                .group_name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(group_name)))?,
        })
    }
}
impl DeviceBoundSessionKey {
    pub fn builder() -> DeviceBoundSessionKeyBuilder {
        <DeviceBoundSessionKeyBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeviceBoundSessionKeyBuilder {
    site: Option<String>,
    id: Option<String>,
}
impl DeviceBoundSessionKeyBuilder {
    pub fn site(mut self, site: impl Into<String>) -> Self {
        self.site = Some(site.into());
        self
    }
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn build(self) -> Result<DeviceBoundSessionKey, String> {
        Ok(DeviceBoundSessionKey {
            site: self
                .site
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(site)))?,
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
        })
    }
}
impl DeviceBoundSessionWithUsage {
    pub fn builder() -> DeviceBoundSessionWithUsageBuilder {
        <DeviceBoundSessionWithUsageBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeviceBoundSessionWithUsageBuilder {
    session_key: Option<DeviceBoundSessionKey>,
    usage: Option<DeviceBoundSessionWithUsageUsage>,
}
impl DeviceBoundSessionWithUsageBuilder {
    pub fn session_key(mut self, session_key: impl Into<DeviceBoundSessionKey>) -> Self {
        self.session_key = Some(session_key.into());
        self
    }
    pub fn usage(mut self, usage: impl Into<DeviceBoundSessionWithUsageUsage>) -> Self {
        self.usage = Some(usage.into());
        self
    }
    pub fn build(self) -> Result<DeviceBoundSessionWithUsage, String> {
        Ok(DeviceBoundSessionWithUsage {
            session_key: self
                .session_key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(session_key)))?,
            usage: self
                .usage
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(usage)))?,
        })
    }
}
impl DeviceBoundSessionCookieCraving {
    pub fn builder() -> DeviceBoundSessionCookieCravingBuilder {
        <DeviceBoundSessionCookieCravingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeviceBoundSessionCookieCravingBuilder {
    name: Option<String>,
    domain: Option<String>,
    path: Option<String>,
    secure: Option<bool>,
    http_only: Option<bool>,
    same_site: Option<CookieSameSite>,
}
impl DeviceBoundSessionCookieCravingBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
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
    pub fn same_site(mut self, same_site: impl Into<CookieSameSite>) -> Self {
        self.same_site = Some(same_site.into());
        self
    }
    pub fn build(self) -> Result<DeviceBoundSessionCookieCraving, String> {
        Ok(DeviceBoundSessionCookieCraving {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            domain: self
                .domain
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(domain)))?,
            path: self
                .path
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(path)))?,
            secure: self
                .secure
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(secure)))?,
            http_only: self
                .http_only
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(http_only)))?,
            same_site: self.same_site,
        })
    }
}
impl DeviceBoundSessionUrlRule {
    pub fn builder() -> DeviceBoundSessionUrlRuleBuilder {
        <DeviceBoundSessionUrlRuleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeviceBoundSessionUrlRuleBuilder {
    rule_type: Option<DeviceBoundSessionUrlRuleRuleType>,
    host_pattern: Option<String>,
    path_prefix: Option<String>,
}
impl DeviceBoundSessionUrlRuleBuilder {
    pub fn rule_type(mut self, rule_type: impl Into<DeviceBoundSessionUrlRuleRuleType>) -> Self {
        self.rule_type = Some(rule_type.into());
        self
    }
    pub fn host_pattern(mut self, host_pattern: impl Into<String>) -> Self {
        self.host_pattern = Some(host_pattern.into());
        self
    }
    pub fn path_prefix(mut self, path_prefix: impl Into<String>) -> Self {
        self.path_prefix = Some(path_prefix.into());
        self
    }
    pub fn build(self) -> Result<DeviceBoundSessionUrlRule, String> {
        Ok(DeviceBoundSessionUrlRule {
            rule_type: self
                .rule_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(rule_type)))?,
            host_pattern: self.host_pattern.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(host_pattern))
            })?,
            path_prefix: self
                .path_prefix
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(path_prefix)))?,
        })
    }
}
impl DeviceBoundSessionInclusionRules {
    pub fn builder() -> DeviceBoundSessionInclusionRulesBuilder {
        <DeviceBoundSessionInclusionRulesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeviceBoundSessionInclusionRulesBuilder {
    origin: Option<String>,
    include_site: Option<bool>,
    url_rules: Option<Vec<DeviceBoundSessionUrlRule>>,
}
impl DeviceBoundSessionInclusionRulesBuilder {
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn include_site(mut self, include_site: impl Into<bool>) -> Self {
        self.include_site = Some(include_site.into());
        self
    }
    pub fn url_rule(mut self, url_rule: impl Into<DeviceBoundSessionUrlRule>) -> Self {
        let v = self.url_rules.get_or_insert(Vec::new());
        v.push(url_rule.into());
        self
    }
    pub fn url_rules<I, S>(mut self, url_rules: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<DeviceBoundSessionUrlRule>,
    {
        let v = self.url_rules.get_or_insert(Vec::new());
        for val in url_rules {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<DeviceBoundSessionInclusionRules, String> {
        Ok(DeviceBoundSessionInclusionRules {
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            include_site: self.include_site.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(include_site))
            })?,
            url_rules: self
                .url_rules
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url_rules)))?,
        })
    }
}
impl DeviceBoundSession {
    pub fn builder() -> DeviceBoundSessionBuilder {
        <DeviceBoundSessionBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeviceBoundSessionBuilder {
    key: Option<DeviceBoundSessionKey>,
    refresh_url: Option<String>,
    inclusion_rules: Option<DeviceBoundSessionInclusionRules>,
    cookie_cravings: Option<Vec<DeviceBoundSessionCookieCraving>>,
    expiry_date: Option<TimeSinceEpoch>,
    cached_challenge: Option<String>,
    allowed_refresh_initiators: Option<Vec<String>>,
}
impl DeviceBoundSessionBuilder {
    pub fn key(mut self, key: impl Into<DeviceBoundSessionKey>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn refresh_url(mut self, refresh_url: impl Into<String>) -> Self {
        self.refresh_url = Some(refresh_url.into());
        self
    }
    pub fn inclusion_rules(
        mut self,
        inclusion_rules: impl Into<DeviceBoundSessionInclusionRules>,
    ) -> Self {
        self.inclusion_rules = Some(inclusion_rules.into());
        self
    }
    pub fn cookie_craving(
        mut self,
        cookie_craving: impl Into<DeviceBoundSessionCookieCraving>,
    ) -> Self {
        let v = self.cookie_cravings.get_or_insert(Vec::new());
        v.push(cookie_craving.into());
        self
    }
    pub fn cookie_cravings<I, S>(mut self, cookie_cravings: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<DeviceBoundSessionCookieCraving>,
    {
        let v = self.cookie_cravings.get_or_insert(Vec::new());
        for val in cookie_cravings {
            v.push(val.into());
        }
        self
    }
    pub fn expiry_date(mut self, expiry_date: impl Into<TimeSinceEpoch>) -> Self {
        self.expiry_date = Some(expiry_date.into());
        self
    }
    pub fn cached_challenge(mut self, cached_challenge: impl Into<String>) -> Self {
        self.cached_challenge = Some(cached_challenge.into());
        self
    }
    pub fn allowed_refresh_initiator(
        mut self,
        allowed_refresh_initiator: impl Into<String>,
    ) -> Self {
        let v = self.allowed_refresh_initiators.get_or_insert(Vec::new());
        v.push(allowed_refresh_initiator.into());
        self
    }
    pub fn allowed_refresh_initiators<I, S>(mut self, allowed_refresh_initiators: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.allowed_refresh_initiators.get_or_insert(Vec::new());
        for val in allowed_refresh_initiators {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<DeviceBoundSession, String> {
        Ok(DeviceBoundSession {
            key: self
                .key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key)))?,
            refresh_url: self
                .refresh_url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(refresh_url)))?,
            inclusion_rules: self.inclusion_rules.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(inclusion_rules))
            })?,
            cookie_cravings: self.cookie_cravings.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(cookie_cravings))
            })?,
            expiry_date: self
                .expiry_date
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(expiry_date)))?,
            cached_challenge: self.cached_challenge,
            allowed_refresh_initiators: self.allowed_refresh_initiators.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(allowed_refresh_initiators)
                )
            })?,
        })
    }
}
impl DeviceBoundSessionFailedRequest {
    pub fn builder() -> DeviceBoundSessionFailedRequestBuilder {
        <DeviceBoundSessionFailedRequestBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeviceBoundSessionFailedRequestBuilder {
    request_url: Option<String>,
    net_error: Option<String>,
    response_error: Option<i64>,
    response_error_body: Option<String>,
}
impl DeviceBoundSessionFailedRequestBuilder {
    pub fn request_url(mut self, request_url: impl Into<String>) -> Self {
        self.request_url = Some(request_url.into());
        self
    }
    pub fn net_error(mut self, net_error: impl Into<String>) -> Self {
        self.net_error = Some(net_error.into());
        self
    }
    pub fn response_error(mut self, response_error: impl Into<i64>) -> Self {
        self.response_error = Some(response_error.into());
        self
    }
    pub fn response_error_body(mut self, response_error_body: impl Into<String>) -> Self {
        self.response_error_body = Some(response_error_body.into());
        self
    }
    pub fn build(self) -> Result<DeviceBoundSessionFailedRequest, String> {
        Ok(DeviceBoundSessionFailedRequest {
            request_url: self
                .request_url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request_url)))?,
            net_error: self.net_error,
            response_error: self.response_error,
            response_error_body: self.response_error_body,
        })
    }
}
impl CreationEventDetails {
    pub fn builder() -> CreationEventDetailsBuilder {
        <CreationEventDetailsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CreationEventDetailsBuilder {
    fetch_result: Option<DeviceBoundSessionFetchResult>,
    new_session: Option<DeviceBoundSession>,
    failed_request: Option<DeviceBoundSessionFailedRequest>,
}
impl CreationEventDetailsBuilder {
    pub fn fetch_result(mut self, fetch_result: impl Into<DeviceBoundSessionFetchResult>) -> Self {
        self.fetch_result = Some(fetch_result.into());
        self
    }
    pub fn new_session(mut self, new_session: impl Into<DeviceBoundSession>) -> Self {
        self.new_session = Some(new_session.into());
        self
    }
    pub fn failed_request(
        mut self,
        failed_request: impl Into<DeviceBoundSessionFailedRequest>,
    ) -> Self {
        self.failed_request = Some(failed_request.into());
        self
    }
    pub fn build(self) -> Result<CreationEventDetails, String> {
        Ok(CreationEventDetails {
            fetch_result: self.fetch_result.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(fetch_result))
            })?,
            new_session: self.new_session,
            failed_request: self.failed_request,
        })
    }
}
impl RefreshEventDetails {
    pub fn builder() -> RefreshEventDetailsBuilder {
        <RefreshEventDetailsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RefreshEventDetailsBuilder {
    refresh_result: Option<RefreshEventDetailsRefreshResult>,
    fetch_result: Option<DeviceBoundSessionFetchResult>,
    new_session: Option<DeviceBoundSession>,
    was_fully_proactive_refresh: Option<bool>,
    failed_request: Option<DeviceBoundSessionFailedRequest>,
}
impl RefreshEventDetailsBuilder {
    pub fn refresh_result(
        mut self,
        refresh_result: impl Into<RefreshEventDetailsRefreshResult>,
    ) -> Self {
        self.refresh_result = Some(refresh_result.into());
        self
    }
    pub fn fetch_result(mut self, fetch_result: impl Into<DeviceBoundSessionFetchResult>) -> Self {
        self.fetch_result = Some(fetch_result.into());
        self
    }
    pub fn new_session(mut self, new_session: impl Into<DeviceBoundSession>) -> Self {
        self.new_session = Some(new_session.into());
        self
    }
    pub fn was_fully_proactive_refresh(
        mut self,
        was_fully_proactive_refresh: impl Into<bool>,
    ) -> Self {
        self.was_fully_proactive_refresh = Some(was_fully_proactive_refresh.into());
        self
    }
    pub fn failed_request(
        mut self,
        failed_request: impl Into<DeviceBoundSessionFailedRequest>,
    ) -> Self {
        self.failed_request = Some(failed_request.into());
        self
    }
    pub fn build(self) -> Result<RefreshEventDetails, String> {
        Ok(RefreshEventDetails {
            refresh_result: self.refresh_result.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(refresh_result))
            })?,
            fetch_result: self.fetch_result,
            new_session: self.new_session,
            was_fully_proactive_refresh: self.was_fully_proactive_refresh.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(was_fully_proactive_refresh)
                )
            })?,
            failed_request: self.failed_request,
        })
    }
}
impl TerminationEventDetails {
    pub fn builder() -> TerminationEventDetailsBuilder {
        <TerminationEventDetailsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct TerminationEventDetailsBuilder {
    deletion_reason: Option<TerminationEventDetailsDeletionReason>,
}
impl TerminationEventDetailsBuilder {
    pub fn deletion_reason(
        mut self,
        deletion_reason: impl Into<TerminationEventDetailsDeletionReason>,
    ) -> Self {
        self.deletion_reason = Some(deletion_reason.into());
        self
    }
    pub fn build(self) -> Result<TerminationEventDetails, String> {
        Ok(TerminationEventDetails {
            deletion_reason: self.deletion_reason.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(deletion_reason))
            })?,
        })
    }
}
impl ChallengeEventDetails {
    pub fn builder() -> ChallengeEventDetailsBuilder {
        <ChallengeEventDetailsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ChallengeEventDetailsBuilder {
    challenge_result: Option<ChallengeEventDetailsChallengeResult>,
    challenge: Option<String>,
}
impl ChallengeEventDetailsBuilder {
    pub fn challenge_result(
        mut self,
        challenge_result: impl Into<ChallengeEventDetailsChallengeResult>,
    ) -> Self {
        self.challenge_result = Some(challenge_result.into());
        self
    }
    pub fn challenge(mut self, challenge: impl Into<String>) -> Self {
        self.challenge = Some(challenge.into());
        self
    }
    pub fn build(self) -> Result<ChallengeEventDetails, String> {
        Ok(ChallengeEventDetails {
            challenge_result: self.challenge_result.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(challenge_result)
                )
            })?,
            challenge: self
                .challenge
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(challenge)))?,
        })
    }
}
impl LoadNetworkResourcePageResult {
    pub fn builder() -> LoadNetworkResourcePageResultBuilder {
        <LoadNetworkResourcePageResultBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct LoadNetworkResourcePageResultBuilder {
    success: Option<bool>,
    net_error: Option<f64>,
    net_error_name: Option<String>,
    http_status_code: Option<f64>,
    stream: Option<crate::browser_protocol::io::types::StreamHandle>,
    headers: Option<Headers>,
}
impl LoadNetworkResourcePageResultBuilder {
    pub fn success(mut self, success: impl Into<bool>) -> Self {
        self.success = Some(success.into());
        self
    }
    pub fn net_error(mut self, net_error: impl Into<f64>) -> Self {
        self.net_error = Some(net_error.into());
        self
    }
    pub fn net_error_name(mut self, net_error_name: impl Into<String>) -> Self {
        self.net_error_name = Some(net_error_name.into());
        self
    }
    pub fn http_status_code(mut self, http_status_code: impl Into<f64>) -> Self {
        self.http_status_code = Some(http_status_code.into());
        self
    }
    pub fn stream(
        mut self,
        stream: impl Into<crate::browser_protocol::io::types::StreamHandle>,
    ) -> Self {
        self.stream = Some(stream.into());
        self
    }
    pub fn headers(mut self, headers: impl Into<Headers>) -> Self {
        self.headers = Some(headers.into());
        self
    }
    pub fn build(self) -> Result<LoadNetworkResourcePageResult, String> {
        Ok(LoadNetworkResourcePageResult {
            success: self
                .success
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(success)))?,
            net_error: self.net_error,
            net_error_name: self.net_error_name,
            http_status_code: self.http_status_code,
            stream: self.stream,
            headers: self.headers,
        })
    }
}
impl LoadNetworkResourceOptions {
    pub fn builder() -> LoadNetworkResourceOptionsBuilder {
        <LoadNetworkResourceOptionsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct LoadNetworkResourceOptionsBuilder {
    disable_cache: Option<bool>,
    include_credentials: Option<bool>,
}
impl LoadNetworkResourceOptionsBuilder {
    pub fn disable_cache(mut self, disable_cache: impl Into<bool>) -> Self {
        self.disable_cache = Some(disable_cache.into());
        self
    }
    pub fn include_credentials(mut self, include_credentials: impl Into<bool>) -> Self {
        self.include_credentials = Some(include_credentials.into());
        self
    }
    pub fn build(self) -> Result<LoadNetworkResourceOptions, String> {
        Ok(LoadNetworkResourceOptions {
            disable_cache: self.disable_cache.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(disable_cache))
            })?,
            include_credentials: self.include_credentials.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(include_credentials)
                )
            })?,
        })
    }
}
