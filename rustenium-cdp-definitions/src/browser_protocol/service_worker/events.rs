use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerErrorReported {
    #[serde(rename = "errorMessage")]
    pub error_message: super::types::ServiceWorkerErrorMessage,
}
impl WorkerErrorReported {
    pub const IDENTIFIER: &'static str = "ServiceWorker.workerErrorReported";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerRegistrationUpdated {
    #[serde(rename = "registrations")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub registrations: Vec<super::types::ServiceWorkerRegistration>,
}
impl WorkerRegistrationUpdated {
    pub const IDENTIFIER: &'static str = "ServiceWorker.workerRegistrationUpdated";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerVersionUpdated {
    #[serde(rename = "versions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub versions: Vec<super::types::ServiceWorkerVersion>,
}
impl WorkerVersionUpdated {
    pub const IDENTIFIER: &'static str = "ServiceWorker.workerVersionUpdated";
}
group_enum ! (ServiceWorkerEvents { WorkerErrorReported (WorkerErrorReported) , WorkerRegistrationUpdated (WorkerRegistrationUpdated) , WorkerVersionUpdated (WorkerVersionUpdated) });
