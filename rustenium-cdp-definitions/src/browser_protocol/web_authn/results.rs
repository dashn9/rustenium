use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddVirtualAuthenticatorResult {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetResponseOverrideBitsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveVirtualAuthenticatorResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddCredentialResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCredentialResult {
    #[serde(rename = "credential")]
    pub credential: super::types::Credential,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCredentialsResult {
    #[serde(rename = "credentials")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub credentials: Vec<super::types::Credential>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveCredentialResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearCredentialsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetUserVerifiedResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAutomaticPresenceSimulationResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetCredentialPropertiesResult {}
