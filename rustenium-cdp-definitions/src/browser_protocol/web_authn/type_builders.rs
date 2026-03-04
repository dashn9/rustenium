use super::types::*;
impl VirtualAuthenticatorOptions {
    pub fn builder() -> VirtualAuthenticatorOptionsBuilder {
        VirtualAuthenticatorOptionsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct VirtualAuthenticatorOptionsBuilder {
    protocol: Option<AuthenticatorProtocol>,
    ctap2_version: Option<Ctap2Version>,
    transport: Option<AuthenticatorTransport>,
    has_resident_key: Option<bool>,
    has_user_verification: Option<bool>,
    has_large_blob: Option<bool>,
    has_cred_blob: Option<bool>,
    has_min_pin_length: Option<bool>,
    has_prf: Option<bool>,
    automatic_presence_simulation: Option<bool>,
    is_user_verified: Option<bool>,
    default_backup_eligibility: Option<bool>,
    default_backup_state: Option<bool>,
}
impl VirtualAuthenticatorOptionsBuilder {
    pub fn protocol(mut self, protocol: impl Into<AuthenticatorProtocol>) -> Self {
        self.protocol = Some(protocol.into());
        self
    }
    pub fn ctap2_version(mut self, ctap2_version: impl Into<Ctap2Version>) -> Self {
        self.ctap2_version = Some(ctap2_version.into());
        self
    }
    pub fn transport(mut self, transport: impl Into<AuthenticatorTransport>) -> Self {
        self.transport = Some(transport.into());
        self
    }
    pub fn has_resident_key(mut self, has_resident_key: impl Into<bool>) -> Self {
        self.has_resident_key = Some(has_resident_key.into());
        self
    }
    pub fn has_user_verification(mut self, has_user_verification: impl Into<bool>) -> Self {
        self.has_user_verification = Some(has_user_verification.into());
        self
    }
    pub fn has_large_blob(mut self, has_large_blob: impl Into<bool>) -> Self {
        self.has_large_blob = Some(has_large_blob.into());
        self
    }
    pub fn has_cred_blob(mut self, has_cred_blob: impl Into<bool>) -> Self {
        self.has_cred_blob = Some(has_cred_blob.into());
        self
    }
    pub fn has_min_pin_length(mut self, has_min_pin_length: impl Into<bool>) -> Self {
        self.has_min_pin_length = Some(has_min_pin_length.into());
        self
    }
    pub fn has_prf(mut self, has_prf: impl Into<bool>) -> Self {
        self.has_prf = Some(has_prf.into());
        self
    }
    pub fn automatic_presence_simulation(
        mut self,
        automatic_presence_simulation: impl Into<bool>,
    ) -> Self {
        self.automatic_presence_simulation = Some(automatic_presence_simulation.into());
        self
    }
    pub fn is_user_verified(mut self, is_user_verified: impl Into<bool>) -> Self {
        self.is_user_verified = Some(is_user_verified.into());
        self
    }
    pub fn default_backup_eligibility(
        mut self,
        default_backup_eligibility: impl Into<bool>,
    ) -> Self {
        self.default_backup_eligibility = Some(default_backup_eligibility.into());
        self
    }
    pub fn default_backup_state(mut self, default_backup_state: impl Into<bool>) -> Self {
        self.default_backup_state = Some(default_backup_state.into());
        self
    }
    pub fn build(self) -> Result<VirtualAuthenticatorOptions, String> {
        Ok(VirtualAuthenticatorOptions {
            protocol: self
                .protocol
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(protocol)))?,
            ctap2_version: self.ctap2_version,
            transport: self
                .transport
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(transport)))?,
            has_resident_key: self.has_resident_key,
            has_user_verification: self.has_user_verification,
            has_large_blob: self.has_large_blob,
            has_cred_blob: self.has_cred_blob,
            has_min_pin_length: self.has_min_pin_length,
            has_prf: self.has_prf,
            automatic_presence_simulation: self.automatic_presence_simulation,
            is_user_verified: self.is_user_verified,
            default_backup_eligibility: self.default_backup_eligibility,
            default_backup_state: self.default_backup_state,
        })
    }
}
impl Credential {
    pub fn builder() -> CredentialBuilder {
        CredentialBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CredentialBuilder {
    credential_id: Option<super::super::super::Binary>,
    is_resident_credential: Option<bool>,
    rp_id: Option<String>,
    private_key: Option<super::super::super::Binary>,
    user_handle: Option<super::super::super::Binary>,
    sign_count: Option<i64>,
    large_blob: Option<super::super::super::Binary>,
    backup_eligibility: Option<bool>,
    backup_state: Option<bool>,
    user_name: Option<String>,
    user_display_name: Option<String>,
}
impl CredentialBuilder {
    pub fn credential_id(mut self, credential_id: impl Into<super::super::super::Binary>) -> Self {
        self.credential_id = Some(credential_id.into());
        self
    }
    pub fn is_resident_credential(mut self, is_resident_credential: impl Into<bool>) -> Self {
        self.is_resident_credential = Some(is_resident_credential.into());
        self
    }
    pub fn rp_id(mut self, rp_id: impl Into<String>) -> Self {
        self.rp_id = Some(rp_id.into());
        self
    }
    pub fn private_key(mut self, private_key: impl Into<super::super::super::Binary>) -> Self {
        self.private_key = Some(private_key.into());
        self
    }
    pub fn user_handle(mut self, user_handle: impl Into<super::super::super::Binary>) -> Self {
        self.user_handle = Some(user_handle.into());
        self
    }
    pub fn sign_count(mut self, sign_count: impl Into<i64>) -> Self {
        self.sign_count = Some(sign_count.into());
        self
    }
    pub fn large_blob(mut self, large_blob: impl Into<super::super::super::Binary>) -> Self {
        self.large_blob = Some(large_blob.into());
        self
    }
    pub fn backup_eligibility(mut self, backup_eligibility: impl Into<bool>) -> Self {
        self.backup_eligibility = Some(backup_eligibility.into());
        self
    }
    pub fn backup_state(mut self, backup_state: impl Into<bool>) -> Self {
        self.backup_state = Some(backup_state.into());
        self
    }
    pub fn user_name(mut self, user_name: impl Into<String>) -> Self {
        self.user_name = Some(user_name.into());
        self
    }
    pub fn user_display_name(mut self, user_display_name: impl Into<String>) -> Self {
        self.user_display_name = Some(user_display_name.into());
        self
    }
    pub fn build(self) -> Result<Credential, String> {
        Ok(Credential {
            credential_id: self.credential_id.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(credential_id))
            })?,
            is_resident_credential: self.is_resident_credential.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(is_resident_credential)
                )
            })?,
            rp_id: self.rp_id,
            private_key: self
                .private_key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(private_key)))?,
            user_handle: self.user_handle,
            sign_count: self
                .sign_count
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(sign_count)))?,
            large_blob: self.large_blob,
            backup_eligibility: self.backup_eligibility,
            backup_state: self.backup_state,
            user_name: self.user_name,
            user_display_name: self.user_display_name,
        })
    }
}
