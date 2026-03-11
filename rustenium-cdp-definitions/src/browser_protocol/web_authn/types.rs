use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct AuthenticatorId(String);
impl AuthenticatorId {
    pub fn new(val: impl Into<String>) -> Self {
        AuthenticatorId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for AuthenticatorId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<AuthenticatorId> for String {
    fn from(el: AuthenticatorId) -> String {
        el.0
    }
}
impl From<String> for AuthenticatorId {
    fn from(expr: String) -> Self {
        AuthenticatorId(expr)
    }
}
impl std::borrow::Borrow<str> for AuthenticatorId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl AuthenticatorId {
    pub const IDENTIFIER: &'static str = "WebAuthn.AuthenticatorId";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthenticatorProtocol {
    #[doc = "Universal 2nd Factor."]
    #[serde(rename = "u2f")]
    U2f,
    #[doc = "Client To Authenticator Protocol 2."]
    #[serde(rename = "ctap2")]
    Ctap2,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ctap2Version {
    #[serde(rename = "ctap2_0")]
    Ctap20,
    #[serde(rename = "ctap2_1")]
    Ctap21,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthenticatorTransport {
    #[doc = "Cross-Platform authenticator attachments:"]
    #[serde(rename = "usb")]
    Usb,
    #[serde(rename = "nfc")]
    Nfc,
    #[serde(rename = "ble")]
    Ble,
    #[serde(rename = "cable")]
    Cable,
    #[doc = "Platform authenticator attachment:"]
    #[serde(rename = "internal")]
    Internal,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualAuthenticatorOptions {
    #[serde(rename = "protocol")]
    pub protocol: AuthenticatorProtocol,
    #[doc = "Defaults to ctap2_0. Ignored if |protocol| == u2f."]
    #[serde(rename = "ctap2Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ctap2_version: Option<Ctap2Version>,
    #[serde(rename = "transport")]
    pub transport: AuthenticatorTransport,
    #[doc = "Defaults to false."]
    #[serde(rename = "hasResidentKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub has_resident_key: Option<bool>,
    #[doc = "Defaults to false."]
    #[serde(rename = "hasUserVerification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub has_user_verification: Option<bool>,
    #[doc = "If set to true, the authenticator will support the largeBlob extension.\nhttps://w3c.github.io/webauthn#largeBlob\nDefaults to false."]
    #[serde(rename = "hasLargeBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub has_large_blob: Option<bool>,
    #[doc = "If set to true, the authenticator will support the credBlob extension.\nhttps://fidoalliance.org/specs/fido-v2.1-rd-20201208/fido-client-to-authenticator-protocol-v2.1-rd-20201208.html#sctn-credBlob-extension\nDefaults to false."]
    #[serde(rename = "hasCredBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub has_cred_blob: Option<bool>,
    #[doc = "If set to true, the authenticator will support the minPinLength extension.\nhttps://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-minpinlength-extension\nDefaults to false."]
    #[serde(rename = "hasMinPinLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub has_min_pin_length: Option<bool>,
    #[doc = "If set to true, the authenticator will support the prf extension.\nhttps://w3c.github.io/webauthn/#prf-extension\nDefaults to false."]
    #[serde(rename = "hasPrf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub has_prf: Option<bool>,
    #[doc = "If set to true, tests of user presence will succeed immediately.\nOtherwise, they will not be resolved. Defaults to true."]
    #[serde(rename = "automaticPresenceSimulation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub automatic_presence_simulation: Option<bool>,
    #[doc = "Sets whether User Verification succeeds or fails for an authenticator.\nDefaults to false."]
    #[serde(rename = "isUserVerified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_user_verified: Option<bool>,
    #[doc = "Credentials created by this authenticator will have the backup\neligibility (BE) flag set to this value. Defaults to false.\nhttps://w3c.github.io/webauthn/#sctn-credential-backup"]
    #[serde(rename = "defaultBackupEligibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub default_backup_eligibility: Option<bool>,
    #[doc = "Credentials created by this authenticator will have the backup state\n(BS) flag set to this value. Defaults to false.\nhttps://w3c.github.io/webauthn/#sctn-credential-backup"]
    #[serde(rename = "defaultBackupState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub default_backup_state: Option<bool>,
}
impl VirtualAuthenticatorOptions {
    pub fn new(
        protocol: impl Into<AuthenticatorProtocol>,
        transport: impl Into<AuthenticatorTransport>,
    ) -> Self {
        Self {
            protocol: protocol.into(),
            transport: transport.into(),
            ctap2_version: None,
            has_resident_key: None,
            has_user_verification: None,
            has_large_blob: None,
            has_cred_blob: None,
            has_min_pin_length: None,
            has_prf: None,
            automatic_presence_simulation: None,
            is_user_verified: None,
            default_backup_eligibility: None,
            default_backup_state: None,
        }
    }
}
impl VirtualAuthenticatorOptions {
    pub const IDENTIFIER: &'static str = "WebAuthn.VirtualAuthenticatorOptions";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Credential {
    #[serde(rename = "credentialId")]
    pub credential_id: crate::Binary,
    #[serde(rename = "isResidentCredential")]
    pub is_resident_credential: bool,
    #[doc = "Relying Party ID the credential is scoped to. Must be set when adding a\ncredential."]
    #[serde(rename = "rpId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub rp_id: Option<String>,
    #[doc = "The ECDSA P-256 private key in PKCS#8 format."]
    #[serde(rename = "privateKey")]
    pub private_key: crate::Binary,
    #[doc = "An opaque byte sequence with a maximum size of 64 bytes mapping the\ncredential to a specific user."]
    #[serde(rename = "userHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_handle: Option<crate::Binary>,
    #[doc = "Signature counter. This is incremented by one for each successful\nassertion.\nSee https://w3c.github.io/webauthn/#signature-counter"]
    #[serde(rename = "signCount")]
    pub sign_count: i64,
    #[doc = "The large blob associated with the credential.\nSee https://w3c.github.io/webauthn/#sctn-large-blob-extension"]
    #[serde(rename = "largeBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub large_blob: Option<crate::Binary>,
    #[doc = "Assertions returned by this credential will have the backup eligibility\n(BE) flag set to this value. Defaults to the authenticator's\ndefaultBackupEligibility value."]
    #[serde(rename = "backupEligibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backup_eligibility: Option<bool>,
    #[doc = "Assertions returned by this credential will have the backup state (BS)\nflag set to this value. Defaults to the authenticator's\ndefaultBackupState value."]
    #[serde(rename = "backupState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backup_state: Option<bool>,
    #[doc = "The credential's user.name property. Equivalent to empty if not set.\nhttps://w3c.github.io/webauthn/#dom-publickeycredentialentity-name"]
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_name: Option<String>,
    #[doc = "The credential's user.displayName property. Equivalent to empty if\nnot set.\nhttps://w3c.github.io/webauthn/#dom-publickeycredentialuserentity-displayname"]
    #[serde(rename = "userDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_display_name: Option<String>,
}
impl Credential {
    pub fn new(
        credential_id: impl Into<crate::Binary>,
        is_resident_credential: impl Into<bool>,
        private_key: impl Into<crate::Binary>,
        sign_count: impl Into<i64>,
    ) -> Self {
        Self {
            credential_id: credential_id.into(),
            is_resident_credential: is_resident_credential.into(),
            private_key: private_key.into(),
            sign_count: sign_count.into(),
            rp_id: None,
            user_handle: None,
            large_blob: None,
            backup_eligibility: None,
            backup_state: None,
            user_name: None,
            user_display_name: None,
        }
    }
}
impl Credential {
    pub const IDENTIFIER: &'static str = "WebAuthn.Credential";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (WebAuthnTypes { AuthenticatorId (AuthenticatorId) , AuthenticatorProtocol (AuthenticatorProtocol) , Ctap2Version (Ctap2Version) , AuthenticatorTransport (AuthenticatorTransport) , VirtualAuthenticatorOptions (VirtualAuthenticatorOptions) , Credential (Credential) });
