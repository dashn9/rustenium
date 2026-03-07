use super::types::*;
impl ServiceWorkerRegistration {
    pub fn builder() -> ServiceWorkerRegistrationBuilder {
        <ServiceWorkerRegistrationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ServiceWorkerRegistrationBuilder {
    registration_id: Option<RegistrationId>,
    scope_url: Option<String>,
    is_deleted: Option<bool>,
}
impl ServiceWorkerRegistrationBuilder {
    pub fn registration_id(mut self, registration_id: impl Into<RegistrationId>) -> Self {
        self.registration_id = Some(registration_id.into());
        self
    }
    pub fn scope_url(mut self, scope_url: impl Into<String>) -> Self {
        self.scope_url = Some(scope_url.into());
        self
    }
    pub fn is_deleted(mut self, is_deleted: impl Into<bool>) -> Self {
        self.is_deleted = Some(is_deleted.into());
        self
    }
    pub fn build(self) -> Result<ServiceWorkerRegistration, String> {
        Ok(ServiceWorkerRegistration {
            registration_id: self.registration_id.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(registration_id))
            })?,
            scope_url: self
                .scope_url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(scope_url)))?,
            is_deleted: self
                .is_deleted
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(is_deleted)))?,
        })
    }
}
impl ServiceWorkerVersion {
    pub fn builder() -> ServiceWorkerVersionBuilder {
        <ServiceWorkerVersionBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ServiceWorkerVersionBuilder {
    version_id: Option<String>,
    registration_id: Option<RegistrationId>,
    script_url: Option<String>,
    running_status: Option<ServiceWorkerVersionRunningStatus>,
    status: Option<ServiceWorkerVersionStatus>,
    script_last_modified: Option<f64>,
    script_response_time: Option<f64>,
    controlled_clients: Option<Vec<crate::browser_protocol::target::types::TargetId>>,
    target_id: Option<crate::browser_protocol::target::types::TargetId>,
    router_rules: Option<String>,
}
impl ServiceWorkerVersionBuilder {
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }
    pub fn registration_id(mut self, registration_id: impl Into<RegistrationId>) -> Self {
        self.registration_id = Some(registration_id.into());
        self
    }
    pub fn script_url(mut self, script_url: impl Into<String>) -> Self {
        self.script_url = Some(script_url.into());
        self
    }
    pub fn running_status(
        mut self,
        running_status: impl Into<ServiceWorkerVersionRunningStatus>,
    ) -> Self {
        self.running_status = Some(running_status.into());
        self
    }
    pub fn status(mut self, status: impl Into<ServiceWorkerVersionStatus>) -> Self {
        self.status = Some(status.into());
        self
    }
    pub fn script_last_modified(mut self, script_last_modified: impl Into<f64>) -> Self {
        self.script_last_modified = Some(script_last_modified.into());
        self
    }
    pub fn script_response_time(mut self, script_response_time: impl Into<f64>) -> Self {
        self.script_response_time = Some(script_response_time.into());
        self
    }
    pub fn controlled_client(
        mut self,
        controlled_client: impl Into<crate::browser_protocol::target::types::TargetId>,
    ) -> Self {
        let v = self.controlled_clients.get_or_insert(Vec::new());
        v.push(controlled_client.into());
        self
    }
    pub fn controlled_clients<I, S>(mut self, controlled_clients: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser_protocol::target::types::TargetId>,
    {
        let v = self.controlled_clients.get_or_insert(Vec::new());
        for val in controlled_clients {
            v.push(val.into());
        }
        self
    }
    pub fn target_id(
        mut self,
        target_id: impl Into<crate::browser_protocol::target::types::TargetId>,
    ) -> Self {
        self.target_id = Some(target_id.into());
        self
    }
    pub fn router_rules(mut self, router_rules: impl Into<String>) -> Self {
        self.router_rules = Some(router_rules.into());
        self
    }
    pub fn build(self) -> Result<ServiceWorkerVersion, String> {
        Ok(ServiceWorkerVersion {
            version_id: self
                .version_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(version_id)))?,
            registration_id: self.registration_id.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(registration_id))
            })?,
            script_url: self
                .script_url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(script_url)))?,
            running_status: self.running_status.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(running_status))
            })?,
            status: self
                .status
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(status)))?,
            script_last_modified: self.script_last_modified,
            script_response_time: self.script_response_time,
            controlled_clients: self.controlled_clients,
            target_id: self.target_id,
            router_rules: self.router_rules,
        })
    }
}
impl ServiceWorkerErrorMessage {
    pub fn builder() -> ServiceWorkerErrorMessageBuilder {
        <ServiceWorkerErrorMessageBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ServiceWorkerErrorMessageBuilder {
    error_message: Option<String>,
    registration_id: Option<RegistrationId>,
    version_id: Option<String>,
    source_url: Option<String>,
    line_number: Option<i64>,
    column_number: Option<i64>,
}
impl ServiceWorkerErrorMessageBuilder {
    pub fn error_message(mut self, error_message: impl Into<String>) -> Self {
        self.error_message = Some(error_message.into());
        self
    }
    pub fn registration_id(mut self, registration_id: impl Into<RegistrationId>) -> Self {
        self.registration_id = Some(registration_id.into());
        self
    }
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }
    pub fn source_url(mut self, source_url: impl Into<String>) -> Self {
        self.source_url = Some(source_url.into());
        self
    }
    pub fn line_number(mut self, line_number: impl Into<i64>) -> Self {
        self.line_number = Some(line_number.into());
        self
    }
    pub fn column_number(mut self, column_number: impl Into<i64>) -> Self {
        self.column_number = Some(column_number.into());
        self
    }
    pub fn build(self) -> Result<ServiceWorkerErrorMessage, String> {
        Ok(ServiceWorkerErrorMessage {
            error_message: self.error_message.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(error_message))
            })?,
            registration_id: self.registration_id.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(registration_id))
            })?,
            version_id: self
                .version_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(version_id)))?,
            source_url: self
                .source_url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(source_url)))?,
            line_number: self
                .line_number
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(line_number)))?,
            column_number: self.column_number.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(column_number))
            })?,
        })
    }
}
