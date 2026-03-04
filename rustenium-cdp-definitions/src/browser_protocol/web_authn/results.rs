use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddVirtualAuthenticatorReturns {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCredentialReturns {
    #[serde(rename = "credential")]
    pub credential: super::types::Credential,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCredentialsReturns {
    #[serde(rename = "credentials")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub credentials: Vec<super::types::Credential>,
}
