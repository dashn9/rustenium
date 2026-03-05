use serde::{Deserialize, Serialize};
#[doc = "Triggered when a credential is added to an authenticator.\n[credentialAdded](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#event-credentialAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialAdded {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
    #[serde(rename = "credential")]
    pub credential: super::types::Credential,
}
impl CredentialAdded {
    pub const IDENTIFIER: &'static str = "WebAuthn.credentialAdded";
}
#[doc = "Triggered when a credential is deleted, e.g. through\nPublicKeyCredential.signalUnknownCredential().\n[credentialDeleted](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#event-credentialDeleted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialDeleted {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
    #[serde(rename = "credentialId")]
    pub credential_id: super::super::super::Binary,
}
impl CredentialDeleted {
    pub const IDENTIFIER: &'static str = "WebAuthn.credentialDeleted";
}
#[doc = "Triggered when a credential is updated, e.g. through\nPublicKeyCredential.signalCurrentUserDetails().\n[credentialUpdated](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#event-credentialUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialUpdated {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
    #[serde(rename = "credential")]
    pub credential: super::types::Credential,
}
impl CredentialUpdated {
    pub const IDENTIFIER: &'static str = "WebAuthn.credentialUpdated";
}
#[doc = "Triggered when a credential is used in a webauthn assertion.\n[credentialAsserted](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#event-credentialAsserted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialAsserted {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
    #[serde(rename = "credential")]
    pub credential: super::types::Credential,
}
impl CredentialAsserted {
    pub const IDENTIFIER: &'static str = "WebAuthn.credentialAsserted";
}
group_enum ! (WebAuthnEvents { CredentialAdded (CredentialAdded) , CredentialDeleted (CredentialDeleted) , CredentialUpdated (CredentialUpdated) , CredentialAsserted (CredentialAsserted) });
