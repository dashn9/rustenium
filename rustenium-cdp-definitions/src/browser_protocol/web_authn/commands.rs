use serde::{Deserialize, Serialize};
#[doc = "Enable the WebAuthn domain and start intercepting credential storage and\nretrieval with a virtual authenticator.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {
    #[doc = "Whether to enable the WebAuthn user interface. Enabling the UI is\nrecommended for debugging and demo purposes, as it is closer to the real\nexperience. Disabling the UI is recommended for automated testing.\nSupported at the embedder's discretion if UI is available.\nDefaults to false."]
    #[serde(rename = "enableUI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enable_ui: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "WebAuthn.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "WebAuthn.enable";
}
#[doc = "Enable the WebAuthn domain and start intercepting credential storage and\nretrieval with a virtual authenticator.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl super::super::super::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Disable the WebAuthn domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "WebAuthn.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "WebAuthn.disable";
}
#[doc = "Disable the WebAuthn domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl super::super::super::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Creates and adds a virtual authenticator.\n[addVirtualAuthenticator](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-addVirtualAuthenticator)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddVirtualAuthenticatorParams {
    #[serde(rename = "options")]
    pub options: super::types::VirtualAuthenticatorOptions,
}
impl AddVirtualAuthenticatorParams {
    pub fn new(options: impl Into<super::types::VirtualAuthenticatorOptions>) -> Self {
        Self {
            options: options.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddVirtualAuthenticatorMethod {
    #[serde(rename = "WebAuthn.addVirtualAuthenticator")]
    AddVirtualAuthenticator,
}
impl AddVirtualAuthenticatorMethod {
    pub const IDENTIFIER: &'static str = "WebAuthn.addVirtualAuthenticator";
}
#[doc = "Creates and adds a virtual authenticator.\n[addVirtualAuthenticator](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-addVirtualAuthenticator)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AddVirtualAuthenticator {
    pub method: AddVirtualAuthenticatorMethod,
    pub params: AddVirtualAuthenticatorParams,
}
impl super::super::super::CommandResult for AddVirtualAuthenticator {
    type Result = super::results::AddVirtualAuthenticatorResult;
}
#[doc = "Resets parameters isBogusSignature, isBadUV, isBadUP to false if they are not present.\n[setResponseOverrideBits](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-setResponseOverrideBits)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetResponseOverrideBitsParams {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
    #[doc = "If isBogusSignature is set, overrides the signature in the authenticator response to be zero.\nDefaults to false."]
    #[serde(rename = "isBogusSignature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_bogus_signature: Option<bool>,
    #[doc = "If isBadUV is set, overrides the UV bit in the flags in the authenticator response to\nbe zero. Defaults to false."]
    #[serde(rename = "isBadUV")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_bad_uv: Option<bool>,
    #[doc = "If isBadUP is set, overrides the UP bit in the flags in the authenticator response to\nbe zero. Defaults to false."]
    #[serde(rename = "isBadUP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_bad_up: Option<bool>,
}
impl SetResponseOverrideBitsParams {
    pub fn new(authenticator_id: impl Into<super::types::AuthenticatorId>) -> Self {
        Self {
            authenticator_id: authenticator_id.into(),
            is_bogus_signature: None,
            is_bad_uv: None,
            is_bad_up: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetResponseOverrideBitsMethod {
    #[serde(rename = "WebAuthn.setResponseOverrideBits")]
    SetResponseOverrideBits,
}
impl SetResponseOverrideBitsMethod {
    pub const IDENTIFIER: &'static str = "WebAuthn.setResponseOverrideBits";
}
#[doc = "Resets parameters isBogusSignature, isBadUV, isBadUP to false if they are not present.\n[setResponseOverrideBits](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-setResponseOverrideBits)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetResponseOverrideBits {
    pub method: SetResponseOverrideBitsMethod,
    pub params: SetResponseOverrideBitsParams,
}
impl super::super::super::CommandResult for SetResponseOverrideBits {
    type Result = super::results::SetResponseOverrideBitsResult;
}
#[doc = "Removes the given authenticator.\n[removeVirtualAuthenticator](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-removeVirtualAuthenticator)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveVirtualAuthenticatorParams {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
}
impl RemoveVirtualAuthenticatorParams {
    pub fn new(authenticator_id: impl Into<super::types::AuthenticatorId>) -> Self {
        Self {
            authenticator_id: authenticator_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveVirtualAuthenticatorMethod {
    #[serde(rename = "WebAuthn.removeVirtualAuthenticator")]
    RemoveVirtualAuthenticator,
}
impl RemoveVirtualAuthenticatorMethod {
    pub const IDENTIFIER: &'static str = "WebAuthn.removeVirtualAuthenticator";
}
#[doc = "Removes the given authenticator.\n[removeVirtualAuthenticator](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-removeVirtualAuthenticator)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RemoveVirtualAuthenticator {
    pub method: RemoveVirtualAuthenticatorMethod,
    pub params: RemoveVirtualAuthenticatorParams,
}
impl super::super::super::CommandResult for RemoveVirtualAuthenticator {
    type Result = super::results::RemoveVirtualAuthenticatorResult;
}
#[doc = "Adds the credential to the specified authenticator.\n[addCredential](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-addCredential)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddCredentialParams {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
    #[serde(rename = "credential")]
    pub credential: super::types::Credential,
}
impl AddCredentialParams {
    pub fn new(
        authenticator_id: impl Into<super::types::AuthenticatorId>,
        credential: impl Into<super::types::Credential>,
    ) -> Self {
        Self {
            authenticator_id: authenticator_id.into(),
            credential: credential.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddCredentialMethod {
    #[serde(rename = "WebAuthn.addCredential")]
    AddCredential,
}
impl AddCredentialMethod {
    pub const IDENTIFIER: &'static str = "WebAuthn.addCredential";
}
#[doc = "Adds the credential to the specified authenticator.\n[addCredential](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-addCredential)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AddCredential {
    pub method: AddCredentialMethod,
    pub params: AddCredentialParams,
}
impl super::super::super::CommandResult for AddCredential {
    type Result = super::results::AddCredentialResult;
}
#[doc = "Returns a single credential stored in the given virtual authenticator that\nmatches the credential ID.\n[getCredential](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-getCredential)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCredentialParams {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
    #[serde(rename = "credentialId")]
    pub credential_id: super::super::super::Binary,
}
impl GetCredentialParams {
    pub fn new(
        authenticator_id: impl Into<super::types::AuthenticatorId>,
        credential_id: impl Into<super::super::super::Binary>,
    ) -> Self {
        Self {
            authenticator_id: authenticator_id.into(),
            credential_id: credential_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetCredentialMethod {
    #[serde(rename = "WebAuthn.getCredential")]
    GetCredential,
}
impl GetCredentialMethod {
    pub const IDENTIFIER: &'static str = "WebAuthn.getCredential";
}
#[doc = "Returns a single credential stored in the given virtual authenticator that\nmatches the credential ID.\n[getCredential](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-getCredential)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetCredential {
    pub method: GetCredentialMethod,
    pub params: GetCredentialParams,
}
impl super::super::super::CommandResult for GetCredential {
    type Result = super::results::GetCredentialResult;
}
#[doc = "Returns all the credentials stored in the given virtual authenticator.\n[getCredentials](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-getCredentials)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCredentialsParams {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
}
impl GetCredentialsParams {
    pub fn new(authenticator_id: impl Into<super::types::AuthenticatorId>) -> Self {
        Self {
            authenticator_id: authenticator_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetCredentialsMethod {
    #[serde(rename = "WebAuthn.getCredentials")]
    GetCredentials,
}
impl GetCredentialsMethod {
    pub const IDENTIFIER: &'static str = "WebAuthn.getCredentials";
}
#[doc = "Returns all the credentials stored in the given virtual authenticator.\n[getCredentials](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-getCredentials)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetCredentials {
    pub method: GetCredentialsMethod,
    pub params: GetCredentialsParams,
}
impl super::super::super::CommandResult for GetCredentials {
    type Result = super::results::GetCredentialsResult;
}
#[doc = "Removes a credential from the authenticator.\n[removeCredential](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-removeCredential)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveCredentialParams {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
    #[serde(rename = "credentialId")]
    pub credential_id: super::super::super::Binary,
}
impl RemoveCredentialParams {
    pub fn new(
        authenticator_id: impl Into<super::types::AuthenticatorId>,
        credential_id: impl Into<super::super::super::Binary>,
    ) -> Self {
        Self {
            authenticator_id: authenticator_id.into(),
            credential_id: credential_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveCredentialMethod {
    #[serde(rename = "WebAuthn.removeCredential")]
    RemoveCredential,
}
impl RemoveCredentialMethod {
    pub const IDENTIFIER: &'static str = "WebAuthn.removeCredential";
}
#[doc = "Removes a credential from the authenticator.\n[removeCredential](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-removeCredential)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RemoveCredential {
    pub method: RemoveCredentialMethod,
    pub params: RemoveCredentialParams,
}
impl super::super::super::CommandResult for RemoveCredential {
    type Result = super::results::RemoveCredentialResult;
}
#[doc = "Clears all the credentials from the specified device.\n[clearCredentials](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-clearCredentials)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearCredentialsParams {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
}
impl ClearCredentialsParams {
    pub fn new(authenticator_id: impl Into<super::types::AuthenticatorId>) -> Self {
        Self {
            authenticator_id: authenticator_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearCredentialsMethod {
    #[serde(rename = "WebAuthn.clearCredentials")]
    ClearCredentials,
}
impl ClearCredentialsMethod {
    pub const IDENTIFIER: &'static str = "WebAuthn.clearCredentials";
}
#[doc = "Clears all the credentials from the specified device.\n[clearCredentials](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-clearCredentials)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ClearCredentials {
    pub method: ClearCredentialsMethod,
    pub params: ClearCredentialsParams,
}
impl super::super::super::CommandResult for ClearCredentials {
    type Result = super::results::ClearCredentialsResult;
}
#[doc = "Sets whether User Verification succeeds or fails for an authenticator.\nThe default is true.\n[setUserVerified](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-setUserVerified)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetUserVerifiedParams {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
    #[serde(rename = "isUserVerified")]
    pub is_user_verified: bool,
}
impl SetUserVerifiedParams {
    pub fn new(
        authenticator_id: impl Into<super::types::AuthenticatorId>,
        is_user_verified: impl Into<bool>,
    ) -> Self {
        Self {
            authenticator_id: authenticator_id.into(),
            is_user_verified: is_user_verified.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetUserVerifiedMethod {
    #[serde(rename = "WebAuthn.setUserVerified")]
    SetUserVerified,
}
impl SetUserVerifiedMethod {
    pub const IDENTIFIER: &'static str = "WebAuthn.setUserVerified";
}
#[doc = "Sets whether User Verification succeeds or fails for an authenticator.\nThe default is true.\n[setUserVerified](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-setUserVerified)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetUserVerified {
    pub method: SetUserVerifiedMethod,
    pub params: SetUserVerifiedParams,
}
impl super::super::super::CommandResult for SetUserVerified {
    type Result = super::results::SetUserVerifiedResult;
}
#[doc = "Sets whether tests of user presence will succeed immediately (if true) or fail to resolve (if false) for an authenticator.\nThe default is true.\n[setAutomaticPresenceSimulation](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-setAutomaticPresenceSimulation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAutomaticPresenceSimulationParams {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
impl SetAutomaticPresenceSimulationParams {
    pub fn new(
        authenticator_id: impl Into<super::types::AuthenticatorId>,
        enabled: impl Into<bool>,
    ) -> Self {
        Self {
            authenticator_id: authenticator_id.into(),
            enabled: enabled.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetAutomaticPresenceSimulationMethod {
    #[serde(rename = "WebAuthn.setAutomaticPresenceSimulation")]
    SetAutomaticPresenceSimulation,
}
impl SetAutomaticPresenceSimulationMethod {
    pub const IDENTIFIER: &'static str = "WebAuthn.setAutomaticPresenceSimulation";
}
#[doc = "Sets whether tests of user presence will succeed immediately (if true) or fail to resolve (if false) for an authenticator.\nThe default is true.\n[setAutomaticPresenceSimulation](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-setAutomaticPresenceSimulation)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetAutomaticPresenceSimulation {
    pub method: SetAutomaticPresenceSimulationMethod,
    pub params: SetAutomaticPresenceSimulationParams,
}
impl super::super::super::CommandResult for SetAutomaticPresenceSimulation {
    type Result = super::results::SetAutomaticPresenceSimulationResult;
}
#[doc = "Allows setting credential properties.\nhttps://w3c.github.io/webauthn/#sctn-automation-set-credential-properties\n[setCredentialProperties](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-setCredentialProperties)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCredentialPropertiesParams {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: super::types::AuthenticatorId,
    #[serde(rename = "credentialId")]
    pub credential_id: super::super::super::Binary,
    #[serde(rename = "backupEligibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backup_eligibility: Option<bool>,
    #[serde(rename = "backupState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backup_state: Option<bool>,
}
impl SetCredentialPropertiesParams {
    pub fn new(
        authenticator_id: impl Into<super::types::AuthenticatorId>,
        credential_id: impl Into<super::super::super::Binary>,
    ) -> Self {
        Self {
            authenticator_id: authenticator_id.into(),
            credential_id: credential_id.into(),
            backup_eligibility: None,
            backup_state: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetCredentialPropertiesMethod {
    #[serde(rename = "WebAuthn.setCredentialProperties")]
    SetCredentialProperties,
}
impl SetCredentialPropertiesMethod {
    pub const IDENTIFIER: &'static str = "WebAuthn.setCredentialProperties";
}
#[doc = "Allows setting credential properties.\nhttps://w3c.github.io/webauthn/#sctn-automation-set-credential-properties\n[setCredentialProperties](https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#method-setCredentialProperties)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetCredentialProperties {
    pub method: SetCredentialPropertiesMethod,
    pub params: SetCredentialPropertiesParams,
}
impl super::super::super::CommandResult for SetCredentialProperties {
    type Result = super::results::SetCredentialPropertiesResult;
}
group_enum ! (WebAuthnCommands { Enable (Enable) , Disable (Disable) , AddVirtualAuthenticator (AddVirtualAuthenticator) , SetResponseOverrideBits (SetResponseOverrideBits) , RemoveVirtualAuthenticator (RemoveVirtualAuthenticator) , AddCredential (AddCredential) , GetCredential (GetCredential) , GetCredentials (GetCredentials) , RemoveCredential (RemoveCredential) , ClearCredentials (ClearCredentials) , SetUserVerified (SetUserVerified) , SetAutomaticPresenceSimulation (SetAutomaticPresenceSimulation) , SetCredentialProperties (SetCredentialProperties) });
