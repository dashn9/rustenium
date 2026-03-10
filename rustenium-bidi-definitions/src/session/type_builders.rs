use super::types::*;
impl CapabilitiesRequest {
    pub fn builder() -> CapabilitiesRequestBuilder {
        <CapabilitiesRequestBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CapabilitiesRequestBuilder {
    always_match: Option<CapabilityRequest>,
    first_match: Option<Vec<CapabilityRequest>>,
}
impl CapabilitiesRequestBuilder {
    pub fn always_match(mut self, always_match: impl Into<CapabilityRequest>) -> Self {
        self.always_match = Some(always_match.into());
        self
    }
    pub fn first_match(mut self, first_match: impl Into<CapabilityRequest>) -> Self {
        let v = self.first_match.get_or_insert(Vec::new());
        v.push(first_match.into());
        self
    }
    pub fn first_matchs<I, S>(mut self, first_matchs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CapabilityRequest>,
    {
        let v = self.first_match.get_or_insert(Vec::new());
        for val in first_matchs {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> CapabilitiesRequest {
        CapabilitiesRequest {
            always_match: self.always_match,
            first_match: self.first_match,
        }
    }
}
impl CapabilityRequest {
    pub fn builder() -> CapabilityRequestBuilder {
        <CapabilityRequestBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CapabilityRequestBuilder {
    accept_insecure_certs: Option<bool>,
    browser_name: Option<String>,
    browser_version: Option<String>,
    platform_name: Option<String>,
    proxy: Option<ProxyConfiguration>,
    unhandled_prompt_behavior: Option<UserPromptHandler>,
    extensible: Option<std::collections::HashMap<String, serde_json::Value>>,
}
impl CapabilityRequestBuilder {
    pub fn accept_insecure_certs(mut self, accept_insecure_certs: impl Into<bool>) -> Self {
        self.accept_insecure_certs = Some(accept_insecure_certs.into());
        self
    }
    pub fn browser_name(mut self, browser_name: impl Into<String>) -> Self {
        self.browser_name = Some(browser_name.into());
        self
    }
    pub fn browser_version(mut self, browser_version: impl Into<String>) -> Self {
        self.browser_version = Some(browser_version.into());
        self
    }
    pub fn platform_name(mut self, platform_name: impl Into<String>) -> Self {
        self.platform_name = Some(platform_name.into());
        self
    }
    pub fn proxy(mut self, proxy: impl Into<ProxyConfiguration>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }
    pub fn unhandled_prompt_behavior(
        mut self,
        unhandled_prompt_behavior: impl Into<UserPromptHandler>,
    ) -> Self {
        self.unhandled_prompt_behavior = Some(unhandled_prompt_behavior.into());
        self
    }
    pub fn extensible(
        mut self,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.extensible = Some(extensible.into());
        self
    }
    pub fn build(self) -> Result<CapabilityRequest, String> {
        Ok(CapabilityRequest {
            accept_insecure_certs: self.accept_insecure_certs,
            browser_name: self.browser_name,
            browser_version: self.browser_version,
            platform_name: self.platform_name,
            proxy: self.proxy,
            unhandled_prompt_behavior: self.unhandled_prompt_behavior,
            extensible: self
                .extensible
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(extensible)))?,
        })
    }
}
impl ProxyConfiguration {
    pub fn builder() -> ProxyConfigurationBuilder {
        <ProxyConfigurationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ProxyConfigurationBuilder {
    autodetect_proxy_configuration: Option<serde_json::Value>,
    direct_proxy_configuration: Option<serde_json::Value>,
    manual_proxy_configuration: Option<serde_json::Value>,
    pac_proxy_configuration: Option<serde_json::Value>,
    system_proxy_configuration: Option<SystemProxyConfiguration>,
}
impl ProxyConfigurationBuilder {
    pub fn autodetect_proxy_configuration(
        mut self,
        autodetect_proxy_configuration: impl Into<serde_json::Value>,
    ) -> Self {
        self.autodetect_proxy_configuration = Some(autodetect_proxy_configuration.into());
        self
    }
    pub fn direct_proxy_configuration(
        mut self,
        direct_proxy_configuration: impl Into<serde_json::Value>,
    ) -> Self {
        self.direct_proxy_configuration = Some(direct_proxy_configuration.into());
        self
    }
    pub fn manual_proxy_configuration(
        mut self,
        manual_proxy_configuration: impl Into<serde_json::Value>,
    ) -> Self {
        self.manual_proxy_configuration = Some(manual_proxy_configuration.into());
        self
    }
    pub fn pac_proxy_configuration(
        mut self,
        pac_proxy_configuration: impl Into<serde_json::Value>,
    ) -> Self {
        self.pac_proxy_configuration = Some(pac_proxy_configuration.into());
        self
    }
    pub fn system_proxy_configuration(
        mut self,
        system_proxy_configuration: impl Into<SystemProxyConfiguration>,
    ) -> Self {
        self.system_proxy_configuration = Some(system_proxy_configuration.into());
        self
    }
    pub fn build(self) -> Result<ProxyConfiguration, String> {
        Ok(ProxyConfiguration {
            autodetect_proxy_configuration: self.autodetect_proxy_configuration.ok_or_else(
                || {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(autodetect_proxy_configuration)
                    )
                },
            )?,
            direct_proxy_configuration: self.direct_proxy_configuration.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(direct_proxy_configuration)
                )
            })?,
            manual_proxy_configuration: self.manual_proxy_configuration.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(manual_proxy_configuration)
                )
            })?,
            pac_proxy_configuration: self.pac_proxy_configuration.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(pac_proxy_configuration)
                )
            })?,
            system_proxy_configuration: self.system_proxy_configuration.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(system_proxy_configuration)
                )
            })?,
        })
    }
}
impl AutodetectProxyConfiguration {
    pub fn builder() -> AutodetectProxyConfigurationBuilder {
        <AutodetectProxyConfigurationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AutodetectProxyConfigurationBuilder {
    proxy_type: Option<AutodetectProxyConfigurationProxyType>,
    extensible: Option<std::collections::HashMap<String, serde_json::Value>>,
}
impl AutodetectProxyConfigurationBuilder {
    pub fn proxy_type(
        mut self,
        proxy_type: impl Into<AutodetectProxyConfigurationProxyType>,
    ) -> Self {
        self.proxy_type = Some(proxy_type.into());
        self
    }
    pub fn extensible(
        mut self,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.extensible = Some(extensible.into());
        self
    }
    pub fn build(self) -> Result<AutodetectProxyConfiguration, String> {
        Ok(AutodetectProxyConfiguration {
            proxy_type: self
                .proxy_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(proxy_type)))?,
            extensible: self
                .extensible
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(extensible)))?,
        })
    }
}
impl DirectProxyConfiguration {
    pub fn builder() -> DirectProxyConfigurationBuilder {
        <DirectProxyConfigurationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DirectProxyConfigurationBuilder {
    proxy_type: Option<DirectProxyConfigurationProxyType>,
    extensible: Option<std::collections::HashMap<String, serde_json::Value>>,
}
impl DirectProxyConfigurationBuilder {
    pub fn proxy_type(mut self, proxy_type: impl Into<DirectProxyConfigurationProxyType>) -> Self {
        self.proxy_type = Some(proxy_type.into());
        self
    }
    pub fn extensible(
        mut self,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.extensible = Some(extensible.into());
        self
    }
    pub fn build(self) -> Result<DirectProxyConfiguration, String> {
        Ok(DirectProxyConfiguration {
            proxy_type: self
                .proxy_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(proxy_type)))?,
            extensible: self
                .extensible
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(extensible)))?,
        })
    }
}
impl ManualProxyConfiguration {
    pub fn builder() -> ManualProxyConfigurationBuilder {
        <ManualProxyConfigurationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ManualProxyConfigurationBuilder {
    proxy_type: Option<ManualProxyConfigurationProxyType>,
    http_proxy: Option<String>,
    ssl_proxy: Option<String>,
    no_proxy: Option<Vec<String>>,
    extensible: Option<std::collections::HashMap<String, serde_json::Value>>,
}
impl ManualProxyConfigurationBuilder {
    pub fn proxy_type(mut self, proxy_type: impl Into<ManualProxyConfigurationProxyType>) -> Self {
        self.proxy_type = Some(proxy_type.into());
        self
    }
    pub fn http_proxy(mut self, http_proxy: impl Into<String>) -> Self {
        self.http_proxy = Some(http_proxy.into());
        self
    }
    pub fn ssl_proxy(mut self, ssl_proxy: impl Into<String>) -> Self {
        self.ssl_proxy = Some(ssl_proxy.into());
        self
    }
    pub fn no_proxy(mut self, no_proxy: impl Into<String>) -> Self {
        let v = self.no_proxy.get_or_insert(Vec::new());
        v.push(no_proxy.into());
        self
    }
    pub fn no_proxys<I, S>(mut self, no_proxys: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.no_proxy.get_or_insert(Vec::new());
        for val in no_proxys {
            v.push(val.into());
        }
        self
    }
    pub fn extensible(
        mut self,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.extensible = Some(extensible.into());
        self
    }
    pub fn build(self) -> Result<ManualProxyConfiguration, String> {
        Ok(ManualProxyConfiguration {
            proxy_type: self
                .proxy_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(proxy_type)))?,
            http_proxy: self.http_proxy,
            ssl_proxy: self.ssl_proxy,
            no_proxy: self.no_proxy,
            extensible: self
                .extensible
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(extensible)))?,
        })
    }
}
impl SocksProxyConfiguration {
    pub fn builder() -> SocksProxyConfigurationBuilder {
        <SocksProxyConfigurationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SocksProxyConfigurationBuilder {
    socks_proxy: Option<String>,
    socks_version: Option<u64>,
}
impl SocksProxyConfigurationBuilder {
    pub fn socks_proxy(mut self, socks_proxy: impl Into<String>) -> Self {
        self.socks_proxy = Some(socks_proxy.into());
        self
    }
    pub fn socks_version(mut self, socks_version: impl Into<u64>) -> Self {
        self.socks_version = Some(socks_version.into());
        self
    }
    pub fn build(self) -> Result<SocksProxyConfiguration, String> {
        Ok(SocksProxyConfiguration {
            socks_proxy: self
                .socks_proxy
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(socks_proxy)))?,
            socks_version: self.socks_version.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(socks_version))
            })?,
        })
    }
}
impl PacProxyConfiguration {
    pub fn builder() -> PacProxyConfigurationBuilder {
        <PacProxyConfigurationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PacProxyConfigurationBuilder {
    proxy_type: Option<PacProxyConfigurationProxyType>,
    proxy_autoconfig_url: Option<String>,
    extensible: Option<std::collections::HashMap<String, serde_json::Value>>,
}
impl PacProxyConfigurationBuilder {
    pub fn proxy_type(mut self, proxy_type: impl Into<PacProxyConfigurationProxyType>) -> Self {
        self.proxy_type = Some(proxy_type.into());
        self
    }
    pub fn proxy_autoconfig_url(mut self, proxy_autoconfig_url: impl Into<String>) -> Self {
        self.proxy_autoconfig_url = Some(proxy_autoconfig_url.into());
        self
    }
    pub fn extensible(
        mut self,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.extensible = Some(extensible.into());
        self
    }
    pub fn build(self) -> Result<PacProxyConfiguration, String> {
        Ok(PacProxyConfiguration {
            proxy_type: self
                .proxy_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(proxy_type)))?,
            proxy_autoconfig_url: self.proxy_autoconfig_url.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(proxy_autoconfig_url)
                )
            })?,
            extensible: self
                .extensible
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(extensible)))?,
        })
    }
}
impl SystemProxyConfiguration {
    pub fn builder() -> SystemProxyConfigurationBuilder {
        <SystemProxyConfigurationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SystemProxyConfigurationBuilder {
    proxy_type: Option<SystemProxyConfigurationProxyType>,
    extensible: Option<std::collections::HashMap<String, serde_json::Value>>,
}
impl SystemProxyConfigurationBuilder {
    pub fn proxy_type(mut self, proxy_type: impl Into<SystemProxyConfigurationProxyType>) -> Self {
        self.proxy_type = Some(proxy_type.into());
        self
    }
    pub fn extensible(
        mut self,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.extensible = Some(extensible.into());
        self
    }
    pub fn build(self) -> Result<SystemProxyConfiguration, String> {
        Ok(SystemProxyConfiguration {
            proxy_type: self
                .proxy_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(proxy_type)))?,
            extensible: self
                .extensible
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(extensible)))?,
        })
    }
}
impl UserPromptHandler {
    pub fn builder() -> UserPromptHandlerBuilder {
        <UserPromptHandlerBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UserPromptHandlerBuilder {
    alert: Option<UserPromptHandlerType>,
    before_unload: Option<UserPromptHandlerType>,
    confirm: Option<UserPromptHandlerType>,
    r#default: Option<UserPromptHandlerType>,
    file: Option<UserPromptHandlerType>,
    prompt: Option<UserPromptHandlerType>,
}
impl UserPromptHandlerBuilder {
    pub fn alert(mut self, alert: impl Into<UserPromptHandlerType>) -> Self {
        self.alert = Some(alert.into());
        self
    }
    pub fn before_unload(mut self, before_unload: impl Into<UserPromptHandlerType>) -> Self {
        self.before_unload = Some(before_unload.into());
        self
    }
    pub fn confirm(mut self, confirm: impl Into<UserPromptHandlerType>) -> Self {
        self.confirm = Some(confirm.into());
        self
    }
    pub fn r#default(mut self, r#default: impl Into<UserPromptHandlerType>) -> Self {
        self.r#default = Some(r#default.into());
        self
    }
    pub fn file(mut self, file: impl Into<UserPromptHandlerType>) -> Self {
        self.file = Some(file.into());
        self
    }
    pub fn prompt(mut self, prompt: impl Into<UserPromptHandlerType>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }
    pub fn build(self) -> UserPromptHandler {
        UserPromptHandler {
            alert: self.alert,
            before_unload: self.before_unload,
            confirm: self.confirm,
            r#default: self.r#default,
            file: self.file,
            prompt: self.prompt,
        }
    }
}
impl UnsubscribeByIdRequest {
    pub fn builder() -> UnsubscribeByIdRequestBuilder {
        <UnsubscribeByIdRequestBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UnsubscribeByIdRequestBuilder {
    subscriptions: Option<Vec<Subscription>>,
}
impl UnsubscribeByIdRequestBuilder {
    pub fn subscription(mut self, subscription: impl Into<Subscription>) -> Self {
        let v = self.subscriptions.get_or_insert(Vec::new());
        v.push(subscription.into());
        self
    }
    pub fn subscriptions<I, S>(mut self, subscriptions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Subscription>,
    {
        let v = self.subscriptions.get_or_insert(Vec::new());
        for val in subscriptions {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<UnsubscribeByIdRequest, String> {
        Ok(UnsubscribeByIdRequest {
            subscriptions: self.subscriptions.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(subscriptions))
            })?,
        })
    }
}
impl UnsubscribeByAttributesRequest {
    pub fn builder() -> UnsubscribeByAttributesRequestBuilder {
        <UnsubscribeByAttributesRequestBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UnsubscribeByAttributesRequestBuilder {
    events: Option<Vec<String>>,
}
impl UnsubscribeByAttributesRequestBuilder {
    pub fn event(mut self, event: impl Into<String>) -> Self {
        let v = self.events.get_or_insert(Vec::new());
        v.push(event.into());
        self
    }
    pub fn events<I, S>(mut self, events: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.events.get_or_insert(Vec::new());
        for val in events {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<UnsubscribeByAttributesRequest, String> {
        Ok(UnsubscribeByAttributesRequest {
            events: self
                .events
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(events)))?,
        })
    }
}
