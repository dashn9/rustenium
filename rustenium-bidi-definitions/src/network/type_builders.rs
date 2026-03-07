use super::types::*;
impl AuthCredentials {
    pub fn builder() -> AuthCredentialsBuilder {
        <AuthCredentialsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AuthCredentialsBuilder {
    r#type: Option<String>,
    username: Option<String>,
    password: Option<String>,
}
impl AuthCredentialsBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
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
    pub fn build(self) -> Result<AuthCredentials, String> {
        Ok(AuthCredentials {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            username: self
                .username
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(username)))?,
            password: self
                .password
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(password)))?,
        })
    }
}
impl StringValue {
    pub fn builder() -> StringValueBuilder {
        <StringValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StringValueBuilder {
    r#type: Option<String>,
    value: Option<String>,
}
impl StringValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<StringValue, String> {
        Ok(StringValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl Base64Value {
    pub fn builder() -> Base64ValueBuilder {
        <Base64ValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct Base64ValueBuilder {
    r#type: Option<String>,
    value: Option<String>,
}
impl Base64ValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<Base64Value, String> {
        Ok(Base64Value {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
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
    value: Option<BytesValue>,
    domain: Option<String>,
    path: Option<String>,
    size: Option<u64>,
    http_only: Option<bool>,
    secure: Option<bool>,
    same_site: Option<SameSite>,
    expiry: Option<u64>,
    extensible: Option<std::collections::HashMap<String, serde_json::Value>>,
}
impl CookieBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<BytesValue>) -> Self {
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
    pub fn size(mut self, size: impl Into<u64>) -> Self {
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
    pub fn same_site(mut self, same_site: impl Into<SameSite>) -> Self {
        self.same_site = Some(same_site.into());
        self
    }
    pub fn expiry(mut self, expiry: impl Into<u64>) -> Self {
        self.expiry = Some(expiry.into());
        self
    }
    pub fn extensible(
        mut self,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.extensible = Some(extensible.into());
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
            size: self
                .size
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(size)))?,
            http_only: self
                .http_only
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(http_only)))?,
            secure: self
                .secure
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(secure)))?,
            same_site: self
                .same_site
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(same_site)))?,
            expiry: self.expiry,
            extensible: self
                .extensible
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(extensible)))?,
        })
    }
}
impl CookieHeader {
    pub fn builder() -> CookieHeaderBuilder {
        <CookieHeaderBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CookieHeaderBuilder {
    name: Option<String>,
    value: Option<BytesValue>,
}
impl CookieHeaderBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<BytesValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<CookieHeader, String> {
        Ok(CookieHeader {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl Header {
    pub fn builder() -> HeaderBuilder {
        <HeaderBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct HeaderBuilder {
    name: Option<String>,
    value: Option<BytesValue>,
}
impl HeaderBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<BytesValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<Header, String> {
        Ok(Header {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl SetCookieHeader {
    pub fn builder() -> SetCookieHeaderBuilder {
        <SetCookieHeaderBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetCookieHeaderBuilder {
    name: Option<String>,
    value: Option<BytesValue>,
    domain: Option<String>,
    http_only: Option<bool>,
    expiry: Option<String>,
    max_age: Option<i64>,
    path: Option<String>,
    same_site: Option<SameSite>,
    secure: Option<bool>,
}
impl SetCookieHeaderBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<BytesValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }
    pub fn http_only(mut self, http_only: impl Into<bool>) -> Self {
        self.http_only = Some(http_only.into());
        self
    }
    pub fn expiry(mut self, expiry: impl Into<String>) -> Self {
        self.expiry = Some(expiry.into());
        self
    }
    pub fn max_age(mut self, max_age: impl Into<i64>) -> Self {
        self.max_age = Some(max_age.into());
        self
    }
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
    pub fn same_site(mut self, same_site: impl Into<SameSite>) -> Self {
        self.same_site = Some(same_site.into());
        self
    }
    pub fn secure(mut self, secure: impl Into<bool>) -> Self {
        self.secure = Some(secure.into());
        self
    }
    pub fn build(self) -> Result<SetCookieHeader, String> {
        Ok(SetCookieHeader {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            domain: self.domain,
            http_only: self.http_only,
            expiry: self.expiry,
            max_age: self.max_age,
            path: self.path,
            same_site: self.same_site,
            secure: self.secure,
        })
    }
}
impl UrlPatternPattern {
    pub fn builder() -> UrlPatternPatternBuilder {
        <UrlPatternPatternBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UrlPatternPatternBuilder {
    r#type: Option<String>,
    protocol: Option<String>,
    hostname: Option<String>,
    port: Option<String>,
    pathname: Option<String>,
    search: Option<String>,
}
impl UrlPatternPatternBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn protocol(mut self, protocol: impl Into<String>) -> Self {
        self.protocol = Some(protocol.into());
        self
    }
    pub fn hostname(mut self, hostname: impl Into<String>) -> Self {
        self.hostname = Some(hostname.into());
        self
    }
    pub fn port(mut self, port: impl Into<String>) -> Self {
        self.port = Some(port.into());
        self
    }
    pub fn pathname(mut self, pathname: impl Into<String>) -> Self {
        self.pathname = Some(pathname.into());
        self
    }
    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }
    pub fn build(self) -> Result<UrlPatternPattern, String> {
        Ok(UrlPatternPattern {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            protocol: self.protocol,
            hostname: self.hostname,
            port: self.port,
            pathname: self.pathname,
            search: self.search,
        })
    }
}
impl UrlPatternString {
    pub fn builder() -> UrlPatternStringBuilder {
        <UrlPatternStringBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UrlPatternStringBuilder {
    r#type: Option<String>,
    pattern: Option<String>,
}
impl UrlPatternStringBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }
    pub fn build(self) -> Result<UrlPatternString, String> {
        Ok(UrlPatternString {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            pattern: self
                .pattern
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(pattern)))?,
        })
    }
}
impl ContinueWithAuthCredentials {
    pub fn builder() -> ContinueWithAuthCredentialsBuilder {
        <ContinueWithAuthCredentialsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ContinueWithAuthCredentialsBuilder {
    action: Option<String>,
    credentials: Option<AuthCredentials>,
}
impl ContinueWithAuthCredentialsBuilder {
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }
    pub fn credentials(mut self, credentials: impl Into<AuthCredentials>) -> Self {
        self.credentials = Some(credentials.into());
        self
    }
    pub fn build(self) -> Result<ContinueWithAuthCredentials, String> {
        Ok(ContinueWithAuthCredentials {
            action: self
                .action
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(action)))?,
            credentials: self
                .credentials
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(credentials)))?,
        })
    }
}
impl ContinueWithAuthNoCredentials {
    pub fn builder() -> ContinueWithAuthNoCredentialsBuilder {
        <ContinueWithAuthNoCredentialsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ContinueWithAuthNoCredentialsBuilder {
    action: Option<ContinueWithAuthNoCredentialsAction>,
}
impl ContinueWithAuthNoCredentialsBuilder {
    pub fn action(mut self, action: impl Into<ContinueWithAuthNoCredentialsAction>) -> Self {
        self.action = Some(action.into());
        self
    }
    pub fn build(self) -> Result<ContinueWithAuthNoCredentials, String> {
        Ok(ContinueWithAuthNoCredentials {
            action: self
                .action
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(action)))?,
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
    scheme: Option<String>,
    realm: Option<String>,
}
impl AuthChallengeBuilder {
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
            scheme: self
                .scheme
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(scheme)))?,
            realm: self
                .realm
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(realm)))?,
        })
    }
}
impl BaseParameters {
    pub fn builder() -> BaseParametersBuilder {
        <BaseParametersBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BaseParametersBuilder {
    context: Option<crate::browsing_context::types::BrowsingContext>,
    is_blocked: Option<bool>,
    navigation: Option<crate::browsing_context::types::Navigation>,
    redirect_count: Option<u64>,
    request: Option<RequestData>,
    timestamp: Option<u64>,
    intercepts: Option<Vec<Intercept>>,
}
impl BaseParametersBuilder {
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn is_blocked(mut self, is_blocked: impl Into<bool>) -> Self {
        self.is_blocked = Some(is_blocked.into());
        self
    }
    pub fn navigation(
        mut self,
        navigation: impl Into<crate::browsing_context::types::Navigation>,
    ) -> Self {
        self.navigation = Some(navigation.into());
        self
    }
    pub fn redirect_count(mut self, redirect_count: impl Into<u64>) -> Self {
        self.redirect_count = Some(redirect_count.into());
        self
    }
    pub fn request(mut self, request: impl Into<RequestData>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn timestamp(mut self, timestamp: impl Into<u64>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
    pub fn intercept(mut self, intercept: impl Into<Intercept>) -> Self {
        let v = self.intercepts.get_or_insert(Vec::new());
        v.push(intercept.into());
        self
    }
    pub fn intercepts<I, S>(mut self, intercepts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Intercept>,
    {
        let v = self.intercepts.get_or_insert(Vec::new());
        for val in intercepts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<BaseParameters, String> {
        Ok(BaseParameters {
            context: self.context,
            is_blocked: self
                .is_blocked
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(is_blocked)))?,
            navigation: self.navigation,
            redirect_count: self.redirect_count.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(redirect_count))
            })?,
            request: self
                .request
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request)))?,
            timestamp: self
                .timestamp
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(timestamp)))?,
            intercepts: self.intercepts,
        })
    }
}
impl FetchTimingInfo {
    pub fn builder() -> FetchTimingInfoBuilder {
        <FetchTimingInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct FetchTimingInfoBuilder {
    time_origin: Option<f64>,
    request_time: Option<f64>,
    redirect_start: Option<f64>,
    redirect_end: Option<f64>,
    fetch_start: Option<f64>,
    dns_start: Option<f64>,
    dns_end: Option<f64>,
    connect_start: Option<f64>,
    connect_end: Option<f64>,
    tls_start: Option<f64>,
    request_start: Option<f64>,
    response_start: Option<f64>,
    response_end: Option<f64>,
}
impl FetchTimingInfoBuilder {
    pub fn time_origin(mut self, time_origin: impl Into<f64>) -> Self {
        self.time_origin = Some(time_origin.into());
        self
    }
    pub fn request_time(mut self, request_time: impl Into<f64>) -> Self {
        self.request_time = Some(request_time.into());
        self
    }
    pub fn redirect_start(mut self, redirect_start: impl Into<f64>) -> Self {
        self.redirect_start = Some(redirect_start.into());
        self
    }
    pub fn redirect_end(mut self, redirect_end: impl Into<f64>) -> Self {
        self.redirect_end = Some(redirect_end.into());
        self
    }
    pub fn fetch_start(mut self, fetch_start: impl Into<f64>) -> Self {
        self.fetch_start = Some(fetch_start.into());
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
    pub fn tls_start(mut self, tls_start: impl Into<f64>) -> Self {
        self.tls_start = Some(tls_start.into());
        self
    }
    pub fn request_start(mut self, request_start: impl Into<f64>) -> Self {
        self.request_start = Some(request_start.into());
        self
    }
    pub fn response_start(mut self, response_start: impl Into<f64>) -> Self {
        self.response_start = Some(response_start.into());
        self
    }
    pub fn response_end(mut self, response_end: impl Into<f64>) -> Self {
        self.response_end = Some(response_end.into());
        self
    }
    pub fn build(self) -> Result<FetchTimingInfo, String> {
        Ok(FetchTimingInfo {
            time_origin: self
                .time_origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(time_origin)))?,
            request_time: self.request_time.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(request_time))
            })?,
            redirect_start: self.redirect_start.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(redirect_start))
            })?,
            redirect_end: self.redirect_end.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(redirect_end))
            })?,
            fetch_start: self
                .fetch_start
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(fetch_start)))?,
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
            tls_start: self
                .tls_start
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(tls_start)))?,
            request_start: self.request_start.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(request_start))
            })?,
            response_start: self.response_start.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(response_start))
            })?,
            response_end: self.response_end.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(response_end))
            })?,
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
    column_number: Option<u64>,
    line_number: Option<u64>,
    request: Option<Request>,
    stack_trace: Option<crate::script::types::StackTrace>,
    r#type: Option<InitiatorType>,
}
impl InitiatorBuilder {
    pub fn column_number(mut self, column_number: impl Into<u64>) -> Self {
        self.column_number = Some(column_number.into());
        self
    }
    pub fn line_number(mut self, line_number: impl Into<u64>) -> Self {
        self.line_number = Some(line_number.into());
        self
    }
    pub fn request(mut self, request: impl Into<Request>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn stack_trace(mut self, stack_trace: impl Into<crate::script::types::StackTrace>) -> Self {
        self.stack_trace = Some(stack_trace.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<InitiatorType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Initiator {
        Initiator {
            column_number: self.column_number,
            line_number: self.line_number,
            request: self.request,
            stack_trace: self.stack_trace,
            r#type: self.r#type,
        }
    }
}
impl RequestData {
    pub fn builder() -> RequestDataBuilder {
        <RequestDataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RequestDataBuilder {
    request: Option<Request>,
    url: Option<String>,
    method: Option<String>,
    headers: Option<Vec<Header>>,
    cookies: Option<Vec<Cookie>>,
    headers_size: Option<u64>,
    body_size: Option<u64>,
    destination: Option<String>,
    initiator_type: Option<String>,
    timings: Option<FetchTimingInfo>,
}
impl RequestDataBuilder {
    pub fn request(mut self, request: impl Into<Request>) -> Self {
        self.request = Some(request.into());
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
    pub fn header(mut self, header: impl Into<Header>) -> Self {
        let v = self.headers.get_or_insert(Vec::new());
        v.push(header.into());
        self
    }
    pub fn headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Header>,
    {
        let v = self.headers.get_or_insert(Vec::new());
        for val in headers {
            v.push(val.into());
        }
        self
    }
    pub fn cookie(mut self, cookie: impl Into<Cookie>) -> Self {
        let v = self.cookies.get_or_insert(Vec::new());
        v.push(cookie.into());
        self
    }
    pub fn cookies<I, S>(mut self, cookies: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Cookie>,
    {
        let v = self.cookies.get_or_insert(Vec::new());
        for val in cookies {
            v.push(val.into());
        }
        self
    }
    pub fn headers_size(mut self, headers_size: impl Into<u64>) -> Self {
        self.headers_size = Some(headers_size.into());
        self
    }
    pub fn body_size(mut self, body_size: impl Into<u64>) -> Self {
        self.body_size = Some(body_size.into());
        self
    }
    pub fn destination(mut self, destination: impl Into<String>) -> Self {
        self.destination = Some(destination.into());
        self
    }
    pub fn initiator_type(mut self, initiator_type: impl Into<String>) -> Self {
        self.initiator_type = Some(initiator_type.into());
        self
    }
    pub fn timings(mut self, timings: impl Into<FetchTimingInfo>) -> Self {
        self.timings = Some(timings.into());
        self
    }
    pub fn build(self) -> Result<RequestData, String> {
        Ok(RequestData {
            request: self
                .request
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request)))?,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            method: self
                .method
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(method)))?,
            headers: self
                .headers
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(headers)))?,
            cookies: self
                .cookies
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cookies)))?,
            headers_size: self.headers_size.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(headers_size))
            })?,
            body_size: self.body_size,
            destination: self
                .destination
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(destination)))?,
            initiator_type: self.initiator_type,
            timings: self
                .timings
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(timings)))?,
        })
    }
}
impl ResponseContent {
    pub fn builder() -> ResponseContentBuilder {
        <ResponseContentBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ResponseContentBuilder {
    size: Option<u64>,
}
impl ResponseContentBuilder {
    pub fn size(mut self, size: impl Into<u64>) -> Self {
        self.size = Some(size.into());
        self
    }
    pub fn build(self) -> Result<ResponseContent, String> {
        Ok(ResponseContent {
            size: self
                .size
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(size)))?,
        })
    }
}
impl ResponseData {
    pub fn builder() -> ResponseDataBuilder {
        <ResponseDataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ResponseDataBuilder {
    url: Option<String>,
    protocol: Option<String>,
    status: Option<u64>,
    status_text: Option<String>,
    from_cache: Option<bool>,
    headers: Option<Vec<Header>>,
    mime_type: Option<String>,
    bytes_received: Option<u64>,
    headers_size: Option<u64>,
    body_size: Option<u64>,
    content: Option<ResponseContent>,
    auth_challenges: Option<Vec<AuthChallenge>>,
}
impl ResponseDataBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn protocol(mut self, protocol: impl Into<String>) -> Self {
        self.protocol = Some(protocol.into());
        self
    }
    pub fn status(mut self, status: impl Into<u64>) -> Self {
        self.status = Some(status.into());
        self
    }
    pub fn status_text(mut self, status_text: impl Into<String>) -> Self {
        self.status_text = Some(status_text.into());
        self
    }
    pub fn from_cache(mut self, from_cache: impl Into<bool>) -> Self {
        self.from_cache = Some(from_cache.into());
        self
    }
    pub fn header(mut self, header: impl Into<Header>) -> Self {
        let v = self.headers.get_or_insert(Vec::new());
        v.push(header.into());
        self
    }
    pub fn headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Header>,
    {
        let v = self.headers.get_or_insert(Vec::new());
        for val in headers {
            v.push(val.into());
        }
        self
    }
    pub fn mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.mime_type = Some(mime_type.into());
        self
    }
    pub fn bytes_received(mut self, bytes_received: impl Into<u64>) -> Self {
        self.bytes_received = Some(bytes_received.into());
        self
    }
    pub fn headers_size(mut self, headers_size: impl Into<u64>) -> Self {
        self.headers_size = Some(headers_size.into());
        self
    }
    pub fn body_size(mut self, body_size: impl Into<u64>) -> Self {
        self.body_size = Some(body_size.into());
        self
    }
    pub fn content(mut self, content: impl Into<ResponseContent>) -> Self {
        self.content = Some(content.into());
        self
    }
    pub fn auth_challenge(mut self, auth_challenge: impl Into<AuthChallenge>) -> Self {
        let v = self.auth_challenges.get_or_insert(Vec::new());
        v.push(auth_challenge.into());
        self
    }
    pub fn auth_challenges<I, S>(mut self, auth_challenges: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AuthChallenge>,
    {
        let v = self.auth_challenges.get_or_insert(Vec::new());
        for val in auth_challenges {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<ResponseData, String> {
        Ok(ResponseData {
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            protocol: self
                .protocol
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(protocol)))?,
            status: self
                .status
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(status)))?,
            status_text: self
                .status_text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(status_text)))?,
            from_cache: self
                .from_cache
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(from_cache)))?,
            headers: self
                .headers
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(headers)))?,
            mime_type: self
                .mime_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(mime_type)))?,
            bytes_received: self.bytes_received.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(bytes_received))
            })?,
            headers_size: self.headers_size,
            body_size: self.body_size,
            content: self
                .content
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(content)))?,
            auth_challenges: self.auth_challenges,
        })
    }
}
