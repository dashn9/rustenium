use super::commands::*;
impl Enable {
    pub fn builder() -> EnableBuilder {
        EnableBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct EnableBuilder {
    enable_ui: Option<bool>,
}
impl EnableBuilder {
    pub fn enable_ui(mut self, enable_ui: impl Into<bool>) -> Self {
        self.enable_ui = Some(enable_ui.into());
        self
    }
    pub fn build(self) -> Enable {
        Enable {
            method: EnableMethod::Enable,
            params: EnableParams {
                enable_ui: self.enable_ui,
            },
        }
    }
}
impl AddVirtualAuthenticator {
    pub fn builder() -> AddVirtualAuthenticatorBuilder {
        AddVirtualAuthenticatorBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AddVirtualAuthenticatorBuilder {
    options: Option<super::types::VirtualAuthenticatorOptions>,
}
impl AddVirtualAuthenticatorBuilder {
    pub fn options(
        mut self,
        options: impl Into<super::types::VirtualAuthenticatorOptions>,
    ) -> Self {
        self.options = Some(options.into());
        self
    }
    pub fn build(self) -> Result<AddVirtualAuthenticator, String> {
        Ok(AddVirtualAuthenticator {
            method: AddVirtualAuthenticatorMethod::AddVirtualAuthenticator,
            params: AddVirtualAuthenticatorParams {
                options: self
                    .options
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(options)))?,
            },
        })
    }
}
impl SetResponseOverrideBits {
    pub fn builder() -> SetResponseOverrideBitsBuilder {
        SetResponseOverrideBitsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetResponseOverrideBitsBuilder {
    authenticator_id: Option<super::types::AuthenticatorId>,
    is_bogus_signature: Option<bool>,
    is_bad_uv: Option<bool>,
    is_bad_up: Option<bool>,
}
impl SetResponseOverrideBitsBuilder {
    pub fn authenticator_id(
        mut self,
        authenticator_id: impl Into<super::types::AuthenticatorId>,
    ) -> Self {
        self.authenticator_id = Some(authenticator_id.into());
        self
    }
    pub fn is_bogus_signature(mut self, is_bogus_signature: impl Into<bool>) -> Self {
        self.is_bogus_signature = Some(is_bogus_signature.into());
        self
    }
    pub fn is_bad_uv(mut self, is_bad_uv: impl Into<bool>) -> Self {
        self.is_bad_uv = Some(is_bad_uv.into());
        self
    }
    pub fn is_bad_up(mut self, is_bad_up: impl Into<bool>) -> Self {
        self.is_bad_up = Some(is_bad_up.into());
        self
    }
    pub fn build(self) -> Result<SetResponseOverrideBits, String> {
        Ok(SetResponseOverrideBits {
            method: SetResponseOverrideBitsMethod::SetResponseOverrideBits,
            params: SetResponseOverrideBitsParams {
                authenticator_id: self.authenticator_id.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(authenticator_id)
                    )
                })?,
                is_bogus_signature: self.is_bogus_signature,
                is_bad_uv: self.is_bad_uv,
                is_bad_up: self.is_bad_up,
            },
        })
    }
}
impl RemoveVirtualAuthenticator {
    pub fn builder() -> RemoveVirtualAuthenticatorBuilder {
        RemoveVirtualAuthenticatorBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveVirtualAuthenticatorBuilder {
    authenticator_id: Option<super::types::AuthenticatorId>,
}
impl RemoveVirtualAuthenticatorBuilder {
    pub fn authenticator_id(
        mut self,
        authenticator_id: impl Into<super::types::AuthenticatorId>,
    ) -> Self {
        self.authenticator_id = Some(authenticator_id.into());
        self
    }
    pub fn build(self) -> Result<RemoveVirtualAuthenticator, String> {
        Ok(RemoveVirtualAuthenticator {
            method: RemoveVirtualAuthenticatorMethod::RemoveVirtualAuthenticator,
            params: RemoveVirtualAuthenticatorParams {
                authenticator_id: self.authenticator_id.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(authenticator_id)
                    )
                })?,
            },
        })
    }
}
impl AddCredential {
    pub fn builder() -> AddCredentialBuilder {
        AddCredentialBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AddCredentialBuilder {
    authenticator_id: Option<super::types::AuthenticatorId>,
    credential: Option<super::types::Credential>,
}
impl AddCredentialBuilder {
    pub fn authenticator_id(
        mut self,
        authenticator_id: impl Into<super::types::AuthenticatorId>,
    ) -> Self {
        self.authenticator_id = Some(authenticator_id.into());
        self
    }
    pub fn credential(mut self, credential: impl Into<super::types::Credential>) -> Self {
        self.credential = Some(credential.into());
        self
    }
    pub fn build(self) -> Result<AddCredential, String> {
        Ok(AddCredential {
            method: AddCredentialMethod::AddCredential,
            params: AddCredentialParams {
                authenticator_id: self.authenticator_id.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(authenticator_id)
                    )
                })?,
                credential: self.credential.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(credential))
                })?,
            },
        })
    }
}
impl GetCredential {
    pub fn builder() -> GetCredentialBuilder {
        GetCredentialBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetCredentialBuilder {
    authenticator_id: Option<super::types::AuthenticatorId>,
    credential_id: Option<super::super::super::Binary>,
}
impl GetCredentialBuilder {
    pub fn authenticator_id(
        mut self,
        authenticator_id: impl Into<super::types::AuthenticatorId>,
    ) -> Self {
        self.authenticator_id = Some(authenticator_id.into());
        self
    }
    pub fn credential_id(mut self, credential_id: impl Into<super::super::super::Binary>) -> Self {
        self.credential_id = Some(credential_id.into());
        self
    }
    pub fn build(self) -> Result<GetCredential, String> {
        Ok(GetCredential {
            method: GetCredentialMethod::GetCredential,
            params: GetCredentialParams {
                authenticator_id: self.authenticator_id.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(authenticator_id)
                    )
                })?,
                credential_id: self.credential_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(credential_id))
                })?,
            },
        })
    }
}
impl GetCredentials {
    pub fn builder() -> GetCredentialsBuilder {
        GetCredentialsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetCredentialsBuilder {
    authenticator_id: Option<super::types::AuthenticatorId>,
}
impl GetCredentialsBuilder {
    pub fn authenticator_id(
        mut self,
        authenticator_id: impl Into<super::types::AuthenticatorId>,
    ) -> Self {
        self.authenticator_id = Some(authenticator_id.into());
        self
    }
    pub fn build(self) -> Result<GetCredentials, String> {
        Ok(GetCredentials {
            method: GetCredentialsMethod::GetCredentials,
            params: GetCredentialsParams {
                authenticator_id: self.authenticator_id.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(authenticator_id)
                    )
                })?,
            },
        })
    }
}
impl RemoveCredential {
    pub fn builder() -> RemoveCredentialBuilder {
        RemoveCredentialBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveCredentialBuilder {
    authenticator_id: Option<super::types::AuthenticatorId>,
    credential_id: Option<super::super::super::Binary>,
}
impl RemoveCredentialBuilder {
    pub fn authenticator_id(
        mut self,
        authenticator_id: impl Into<super::types::AuthenticatorId>,
    ) -> Self {
        self.authenticator_id = Some(authenticator_id.into());
        self
    }
    pub fn credential_id(mut self, credential_id: impl Into<super::super::super::Binary>) -> Self {
        self.credential_id = Some(credential_id.into());
        self
    }
    pub fn build(self) -> Result<RemoveCredential, String> {
        Ok(RemoveCredential {
            method: RemoveCredentialMethod::RemoveCredential,
            params: RemoveCredentialParams {
                authenticator_id: self.authenticator_id.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(authenticator_id)
                    )
                })?,
                credential_id: self.credential_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(credential_id))
                })?,
            },
        })
    }
}
impl ClearCredentials {
    pub fn builder() -> ClearCredentialsBuilder {
        ClearCredentialsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ClearCredentialsBuilder {
    authenticator_id: Option<super::types::AuthenticatorId>,
}
impl ClearCredentialsBuilder {
    pub fn authenticator_id(
        mut self,
        authenticator_id: impl Into<super::types::AuthenticatorId>,
    ) -> Self {
        self.authenticator_id = Some(authenticator_id.into());
        self
    }
    pub fn build(self) -> Result<ClearCredentials, String> {
        Ok(ClearCredentials {
            method: ClearCredentialsMethod::ClearCredentials,
            params: ClearCredentialsParams {
                authenticator_id: self.authenticator_id.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(authenticator_id)
                    )
                })?,
            },
        })
    }
}
impl SetUserVerified {
    pub fn builder() -> SetUserVerifiedBuilder {
        SetUserVerifiedBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetUserVerifiedBuilder {
    authenticator_id: Option<super::types::AuthenticatorId>,
    is_user_verified: Option<bool>,
}
impl SetUserVerifiedBuilder {
    pub fn authenticator_id(
        mut self,
        authenticator_id: impl Into<super::types::AuthenticatorId>,
    ) -> Self {
        self.authenticator_id = Some(authenticator_id.into());
        self
    }
    pub fn is_user_verified(mut self, is_user_verified: impl Into<bool>) -> Self {
        self.is_user_verified = Some(is_user_verified.into());
        self
    }
    pub fn build(self) -> Result<SetUserVerified, String> {
        Ok(SetUserVerified {
            method: SetUserVerifiedMethod::SetUserVerified,
            params: SetUserVerifiedParams {
                authenticator_id: self.authenticator_id.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(authenticator_id)
                    )
                })?,
                is_user_verified: self.is_user_verified.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(is_user_verified)
                    )
                })?,
            },
        })
    }
}
impl SetAutomaticPresenceSimulation {
    pub fn builder() -> SetAutomaticPresenceSimulationBuilder {
        SetAutomaticPresenceSimulationBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetAutomaticPresenceSimulationBuilder {
    authenticator_id: Option<super::types::AuthenticatorId>,
    enabled: Option<bool>,
}
impl SetAutomaticPresenceSimulationBuilder {
    pub fn authenticator_id(
        mut self,
        authenticator_id: impl Into<super::types::AuthenticatorId>,
    ) -> Self {
        self.authenticator_id = Some(authenticator_id.into());
        self
    }
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn build(self) -> Result<SetAutomaticPresenceSimulation, String> {
        Ok(SetAutomaticPresenceSimulation {
            method: SetAutomaticPresenceSimulationMethod::SetAutomaticPresenceSimulation,
            params: SetAutomaticPresenceSimulationParams {
                authenticator_id: self.authenticator_id.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(authenticator_id)
                    )
                })?,
                enabled: self
                    .enabled
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
            },
        })
    }
}
impl SetCredentialProperties {
    pub fn builder() -> SetCredentialPropertiesBuilder {
        SetCredentialPropertiesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetCredentialPropertiesBuilder {
    authenticator_id: Option<super::types::AuthenticatorId>,
    credential_id: Option<super::super::super::Binary>,
    backup_eligibility: Option<bool>,
    backup_state: Option<bool>,
}
impl SetCredentialPropertiesBuilder {
    pub fn authenticator_id(
        mut self,
        authenticator_id: impl Into<super::types::AuthenticatorId>,
    ) -> Self {
        self.authenticator_id = Some(authenticator_id.into());
        self
    }
    pub fn credential_id(mut self, credential_id: impl Into<super::super::super::Binary>) -> Self {
        self.credential_id = Some(credential_id.into());
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
    pub fn build(self) -> Result<SetCredentialProperties, String> {
        Ok(SetCredentialProperties {
            method: SetCredentialPropertiesMethod::SetCredentialProperties,
            params: SetCredentialPropertiesParams {
                authenticator_id: self.authenticator_id.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(authenticator_id)
                    )
                })?,
                credential_id: self.credential_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(credential_id))
                })?,
                backup_eligibility: self.backup_eligibility,
                backup_state: self.backup_state,
            },
        })
    }
}
