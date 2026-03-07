use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct RegistrationId(String);
impl RegistrationId {
    pub fn new(val: impl Into<String>) -> Self {
        RegistrationId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for RegistrationId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<RegistrationId> for String {
    fn from(el: RegistrationId) -> String {
        el.0
    }
}
impl From<String> for RegistrationId {
    fn from(expr: String) -> Self {
        RegistrationId(expr)
    }
}
impl std::borrow::Borrow<str> for RegistrationId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl RegistrationId {
    pub const IDENTIFIER: &'static str = "ServiceWorker.RegistrationID";
}
#[doc = "ServiceWorker registration.\n[ServiceWorkerRegistration](https://chromedevtools.github.io/devtools-protocol/tot/ServiceWorker/#type-ServiceWorkerRegistration)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceWorkerRegistration {
    #[serde(rename = "registrationId")]
    pub registration_id: RegistrationId,
    #[serde(rename = "scopeURL")]
    pub scope_url: String,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}
impl ServiceWorkerRegistration {
    pub fn new(
        registration_id: impl Into<RegistrationId>,
        scope_url: impl Into<String>,
        is_deleted: impl Into<bool>,
    ) -> Self {
        Self {
            registration_id: registration_id.into(),
            scope_url: scope_url.into(),
            is_deleted: is_deleted.into(),
        }
    }
}
impl ServiceWorkerRegistration {
    pub const IDENTIFIER: &'static str = "ServiceWorker.ServiceWorkerRegistration";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceWorkerVersionRunningStatus {
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopping")]
    Stopping,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceWorkerVersionStatus {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "installing")]
    Installing,
    #[serde(rename = "installed")]
    Installed,
    #[serde(rename = "activating")]
    Activating,
    #[serde(rename = "activated")]
    Activated,
    #[serde(rename = "redundant")]
    Redundant,
}
#[doc = "ServiceWorker version.\n[ServiceWorkerVersion](https://chromedevtools.github.io/devtools-protocol/tot/ServiceWorker/#type-ServiceWorkerVersion)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceWorkerVersion {
    #[serde(rename = "versionId")]
    pub version_id: String,
    #[serde(rename = "registrationId")]
    pub registration_id: RegistrationId,
    #[serde(rename = "scriptURL")]
    pub script_url: String,
    #[serde(rename = "runningStatus")]
    pub running_status: ServiceWorkerVersionRunningStatus,
    #[serde(rename = "status")]
    pub status: ServiceWorkerVersionStatus,
    #[doc = "The Last-Modified header value of the main script."]
    #[serde(rename = "scriptLastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub script_last_modified: Option<f64>,
    #[doc = "The time at which the response headers of the main script were received from the server.\nFor cached script it is the last time the cache entry was validated."]
    #[serde(rename = "scriptResponseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub script_response_time: Option<f64>,
    #[serde(rename = "controlledClients")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub controlled_clients: Option<Vec<crate::browser_protocol::target::types::TargetId>>,
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub target_id: Option<crate::browser_protocol::target::types::TargetId>,
    #[serde(rename = "routerRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub router_rules: Option<String>,
}
impl ServiceWorkerVersion {
    pub const IDENTIFIER: &'static str = "ServiceWorker.ServiceWorkerVersion";
}
#[doc = "ServiceWorker error message.\n[ServiceWorkerErrorMessage](https://chromedevtools.github.io/devtools-protocol/tot/ServiceWorker/#type-ServiceWorkerErrorMessage)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceWorkerErrorMessage {
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(rename = "registrationId")]
    pub registration_id: RegistrationId,
    #[serde(rename = "versionId")]
    pub version_id: String,
    #[serde(rename = "sourceURL")]
    pub source_url: String,
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    #[serde(rename = "columnNumber")]
    pub column_number: i64,
}
impl ServiceWorkerErrorMessage {
    pub const IDENTIFIER: &'static str = "ServiceWorker.ServiceWorkerErrorMessage";
}
group_enum ! (ServiceWorkerTypes { RegistrationId (RegistrationId) , ServiceWorkerRegistration (ServiceWorkerRegistration) , ServiceWorkerVersionRunningStatus (ServiceWorkerVersionRunningStatus) , ServiceWorkerVersionStatus (ServiceWorkerVersionStatus) , ServiceWorkerVersion (ServiceWorkerVersion) , ServiceWorkerErrorMessage (ServiceWorkerErrorMessage) });
