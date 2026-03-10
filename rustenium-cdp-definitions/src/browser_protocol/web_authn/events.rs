use serde::{Deserialize, Serialize};
#[doc = "Triggered when a credential is added to an authenticator.\n[credentialAdded](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#event-credentialAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialAddedParams {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
    #[serde(rename = "credential")]
    pub credential: super::types::Credential,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CredentialAddedMethod {
    #[serde(rename = "WebAuthn.credentialAdded")]
    CredentialAdded,
}
#[doc = "Triggered when a credential is added to an authenticator.\n[credentialAdded](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#event-credentialAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialAdded {
    pub method: CredentialAddedMethod,
    pub params: CredentialAddedParams,
}
impl CredentialAdded {
    pub const IDENTIFIER: &'static str = "WebAuthn.credentialAdded";
}
#[doc = "Triggered when a credential is deleted, e.g. through\nPublicKeyCredential.signalUnknownCredential().\n[credentialDeleted](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#event-credentialDeleted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialDeletedParams {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
    #[serde(rename = "credentialId")]
    pub credential_id: crate::Binary,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CredentialDeletedMethod {
    #[serde(rename = "WebAuthn.credentialDeleted")]
    CredentialDeleted,
}
#[doc = "Triggered when a credential is deleted, e.g. through\nPublicKeyCredential.signalUnknownCredential().\n[credentialDeleted](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#event-credentialDeleted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialDeleted {
    pub method: CredentialDeletedMethod,
    pub params: CredentialDeletedParams,
}
impl CredentialDeleted {
    pub const IDENTIFIER: &'static str = "WebAuthn.credentialDeleted";
}
#[doc = "Triggered when a credential is updated, e.g. through\nPublicKeyCredential.signalCurrentUserDetails().\n[credentialUpdated](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#event-credentialUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialUpdatedParams {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
    #[serde(rename = "credential")]
    pub credential: super::types::Credential,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CredentialUpdatedMethod {
    #[serde(rename = "WebAuthn.credentialUpdated")]
    CredentialUpdated,
}
#[doc = "Triggered when a credential is updated, e.g. through\nPublicKeyCredential.signalCurrentUserDetails().\n[credentialUpdated](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#event-credentialUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialUpdated {
    pub method: CredentialUpdatedMethod,
    pub params: CredentialUpdatedParams,
}
impl CredentialUpdated {
    pub const IDENTIFIER: &'static str = "WebAuthn.credentialUpdated";
}
#[doc = "Triggered when a credential is used in a webauthn assertion.\n[credentialAsserted](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#event-credentialAsserted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialAssertedParams {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
    #[serde(rename = "credential")]
    pub credential: super::types::Credential,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CredentialAssertedMethod {
    #[serde(rename = "WebAuthn.credentialAsserted")]
    CredentialAsserted,
}
#[doc = "Triggered when a credential is used in a webauthn assertion.\n[credentialAsserted](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#event-credentialAsserted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialAsserted {
    pub method: CredentialAssertedMethod,
    pub params: CredentialAssertedParams,
}
impl CredentialAsserted {
    pub const IDENTIFIER: &'static str = "WebAuthn.credentialAsserted";
}
group_enum ! (WebAuthnEvents { CredentialAdded (CredentialAdded) , CredentialDeleted (CredentialDeleted) , CredentialUpdated (CredentialUpdated) , CredentialAsserted (CredentialAsserted) });
