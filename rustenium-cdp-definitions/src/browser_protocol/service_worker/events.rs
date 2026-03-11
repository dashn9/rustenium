use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerErrorReportedParams {
    #[serde(rename = "errorMessage")]
    pub error_message: super::types::ServiceWorkerErrorMessage,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkerErrorReportedMethod {
    #[serde(rename = "ServiceWorker.workerErrorReported")]
    WorkerErrorReported,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerErrorReported {
    pub method: WorkerErrorReportedMethod,
    pub params: WorkerErrorReportedParams,
}
impl WorkerErrorReported {
    pub const IDENTIFIER: &'static str = "ServiceWorker.workerErrorReported";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerRegistrationUpdatedParams {
    #[serde(rename = "registrations")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub registrations: Vec<super::types::ServiceWorkerRegistration>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkerRegistrationUpdatedMethod {
    #[serde(rename = "ServiceWorker.workerRegistrationUpdated")]
    WorkerRegistrationUpdated,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerRegistrationUpdated {
    pub method: WorkerRegistrationUpdatedMethod,
    pub params: WorkerRegistrationUpdatedParams,
}
impl WorkerRegistrationUpdated {
    pub const IDENTIFIER: &'static str = "ServiceWorker.workerRegistrationUpdated";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerVersionUpdatedParams {
    #[serde(rename = "versions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub versions: Vec<super::types::ServiceWorkerVersion>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkerVersionUpdatedMethod {
    #[serde(rename = "ServiceWorker.workerVersionUpdated")]
    WorkerVersionUpdated,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerVersionUpdated {
    pub method: WorkerVersionUpdatedMethod,
    pub params: WorkerVersionUpdatedParams,
}
impl WorkerVersionUpdated {
    pub const IDENTIFIER: &'static str = "ServiceWorker.workerVersionUpdated";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (ServiceWorkerEvents { WorkerErrorReported (WorkerErrorReported) , WorkerRegistrationUpdated (WorkerRegistrationUpdated) , WorkerVersionUpdated (WorkerVersionUpdated) });
