use super::types::*;
impl CertificateSecurityState {
    pub fn builder() -> CertificateSecurityStateBuilder {
        <CertificateSecurityStateBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CertificateSecurityStateBuilder {
    protocol: Option<String>,
    key_exchange: Option<String>,
    key_exchange_group: Option<String>,
    cipher: Option<String>,
    mac: Option<String>,
    certificate: Option<Vec<String>>,
    subject_name: Option<String>,
    issuer: Option<String>,
    valid_from: Option<crate::browser_protocol::network::types::TimeSinceEpoch>,
    valid_to: Option<crate::browser_protocol::network::types::TimeSinceEpoch>,
    certificate_network_error: Option<String>,
    certificate_has_weak_signature: Option<bool>,
    certificate_has_sha1_signature: Option<bool>,
    modern_ssl: Option<bool>,
    obsolete_ssl_protocol: Option<bool>,
    obsolete_ssl_key_exchange: Option<bool>,
    obsolete_ssl_cipher: Option<bool>,
    obsolete_ssl_signature: Option<bool>,
}
impl CertificateSecurityStateBuilder {
    pub fn protocol(mut self, protocol: impl Into<String>) -> Self {
        self.protocol = Some(protocol.into());
        self
    }
    pub fn key_exchange(mut self, key_exchange: impl Into<String>) -> Self {
        self.key_exchange = Some(key_exchange.into());
        self
    }
    pub fn key_exchange_group(mut self, key_exchange_group: impl Into<String>) -> Self {
        self.key_exchange_group = Some(key_exchange_group.into());
        self
    }
    pub fn cipher(mut self, cipher: impl Into<String>) -> Self {
        self.cipher = Some(cipher.into());
        self
    }
    pub fn mac(mut self, mac: impl Into<String>) -> Self {
        self.mac = Some(mac.into());
        self
    }
    pub fn certificate(mut self, certificate: impl Into<String>) -> Self {
        let v = self.certificate.get_or_insert(Vec::new());
        v.push(certificate.into());
        self
    }
    pub fn certificates<I, S>(mut self, certificates: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.certificate.get_or_insert(Vec::new());
        for val in certificates {
            v.push(val.into());
        }
        self
    }
    pub fn subject_name(mut self, subject_name: impl Into<String>) -> Self {
        self.subject_name = Some(subject_name.into());
        self
    }
    pub fn issuer(mut self, issuer: impl Into<String>) -> Self {
        self.issuer = Some(issuer.into());
        self
    }
    pub fn valid_from(
        mut self,
        valid_from: impl Into<crate::browser_protocol::network::types::TimeSinceEpoch>,
    ) -> Self {
        self.valid_from = Some(valid_from.into());
        self
    }
    pub fn valid_to(
        mut self,
        valid_to: impl Into<crate::browser_protocol::network::types::TimeSinceEpoch>,
    ) -> Self {
        self.valid_to = Some(valid_to.into());
        self
    }
    pub fn certificate_network_error(
        mut self,
        certificate_network_error: impl Into<String>,
    ) -> Self {
        self.certificate_network_error = Some(certificate_network_error.into());
        self
    }
    pub fn certificate_has_weak_signature(
        mut self,
        certificate_has_weak_signature: impl Into<bool>,
    ) -> Self {
        self.certificate_has_weak_signature = Some(certificate_has_weak_signature.into());
        self
    }
    pub fn certificate_has_sha1_signature(
        mut self,
        certificate_has_sha1_signature: impl Into<bool>,
    ) -> Self {
        self.certificate_has_sha1_signature = Some(certificate_has_sha1_signature.into());
        self
    }
    pub fn modern_ssl(mut self, modern_ssl: impl Into<bool>) -> Self {
        self.modern_ssl = Some(modern_ssl.into());
        self
    }
    pub fn obsolete_ssl_protocol(mut self, obsolete_ssl_protocol: impl Into<bool>) -> Self {
        self.obsolete_ssl_protocol = Some(obsolete_ssl_protocol.into());
        self
    }
    pub fn obsolete_ssl_key_exchange(mut self, obsolete_ssl_key_exchange: impl Into<bool>) -> Self {
        self.obsolete_ssl_key_exchange = Some(obsolete_ssl_key_exchange.into());
        self
    }
    pub fn obsolete_ssl_cipher(mut self, obsolete_ssl_cipher: impl Into<bool>) -> Self {
        self.obsolete_ssl_cipher = Some(obsolete_ssl_cipher.into());
        self
    }
    pub fn obsolete_ssl_signature(mut self, obsolete_ssl_signature: impl Into<bool>) -> Self {
        self.obsolete_ssl_signature = Some(obsolete_ssl_signature.into());
        self
    }
    pub fn build(self) -> Result<CertificateSecurityState, String> {
        Ok(CertificateSecurityState {
            protocol: self
                .protocol
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(protocol)))?,
            key_exchange: self.key_exchange.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(key_exchange))
            })?,
            key_exchange_group: self.key_exchange_group,
            cipher: self
                .cipher
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cipher)))?,
            mac: self.mac,
            certificate: self
                .certificate
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(certificate)))?,
            subject_name: self.subject_name.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(subject_name))
            })?,
            issuer: self
                .issuer
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(issuer)))?,
            valid_from: self
                .valid_from
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(valid_from)))?,
            valid_to: self
                .valid_to
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(valid_to)))?,
            certificate_network_error: self.certificate_network_error,
            certificate_has_weak_signature: self.certificate_has_weak_signature.ok_or_else(
                || {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(certificate_has_weak_signature)
                    )
                },
            )?,
            certificate_has_sha1_signature: self.certificate_has_sha1_signature.ok_or_else(
                || {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(certificate_has_sha1_signature)
                    )
                },
            )?,
            modern_ssl: self
                .modern_ssl
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(modern_ssl)))?,
            obsolete_ssl_protocol: self.obsolete_ssl_protocol.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(obsolete_ssl_protocol)
                )
            })?,
            obsolete_ssl_key_exchange: self.obsolete_ssl_key_exchange.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(obsolete_ssl_key_exchange)
                )
            })?,
            obsolete_ssl_cipher: self.obsolete_ssl_cipher.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(obsolete_ssl_cipher)
                )
            })?,
            obsolete_ssl_signature: self.obsolete_ssl_signature.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(obsolete_ssl_signature)
                )
            })?,
        })
    }
}
impl SafetyTipInfo {
    pub fn builder() -> SafetyTipInfoBuilder {
        <SafetyTipInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SafetyTipInfoBuilder {
    safety_tip_status: Option<SafetyTipStatus>,
    safe_url: Option<String>,
}
impl SafetyTipInfoBuilder {
    pub fn safety_tip_status(mut self, safety_tip_status: impl Into<SafetyTipStatus>) -> Self {
        self.safety_tip_status = Some(safety_tip_status.into());
        self
    }
    pub fn safe_url(mut self, safe_url: impl Into<String>) -> Self {
        self.safe_url = Some(safe_url.into());
        self
    }
    pub fn build(self) -> Result<SafetyTipInfo, String> {
        Ok(SafetyTipInfo {
            safety_tip_status: self.safety_tip_status.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(safety_tip_status)
                )
            })?,
            safe_url: self.safe_url,
        })
    }
}
impl VisibleSecurityState {
    pub fn builder() -> VisibleSecurityStateBuilder {
        <VisibleSecurityStateBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct VisibleSecurityStateBuilder {
    security_state: Option<SecurityState>,
    certificate_security_state: Option<CertificateSecurityState>,
    safety_tip_info: Option<SafetyTipInfo>,
    security_state_issue_ids: Option<Vec<String>>,
}
impl VisibleSecurityStateBuilder {
    pub fn security_state(mut self, security_state: impl Into<SecurityState>) -> Self {
        self.security_state = Some(security_state.into());
        self
    }
    pub fn certificate_security_state(
        mut self,
        certificate_security_state: impl Into<CertificateSecurityState>,
    ) -> Self {
        self.certificate_security_state = Some(certificate_security_state.into());
        self
    }
    pub fn safety_tip_info(mut self, safety_tip_info: impl Into<SafetyTipInfo>) -> Self {
        self.safety_tip_info = Some(safety_tip_info.into());
        self
    }
    pub fn security_state_issue_id(mut self, security_state_issue_id: impl Into<String>) -> Self {
        let v = self.security_state_issue_ids.get_or_insert(Vec::new());
        v.push(security_state_issue_id.into());
        self
    }
    pub fn security_state_issue_ids<I, S>(mut self, security_state_issue_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.security_state_issue_ids.get_or_insert(Vec::new());
        for val in security_state_issue_ids {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<VisibleSecurityState, String> {
        Ok(VisibleSecurityState {
            security_state: self.security_state.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(security_state))
            })?,
            certificate_security_state: self.certificate_security_state,
            safety_tip_info: self.safety_tip_info,
            security_state_issue_ids: self.security_state_issue_ids.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(security_state_issue_ids)
                )
            })?,
        })
    }
}
impl SecurityStateExplanation {
    pub fn builder() -> SecurityStateExplanationBuilder {
        <SecurityStateExplanationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SecurityStateExplanationBuilder {
    security_state: Option<SecurityState>,
    title: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    mixed_content_type: Option<MixedContentType>,
    certificate: Option<Vec<String>>,
    recommendations: Option<Vec<String>>,
}
impl SecurityStateExplanationBuilder {
    pub fn security_state(mut self, security_state: impl Into<SecurityState>) -> Self {
        self.security_state = Some(security_state.into());
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn mixed_content_type(mut self, mixed_content_type: impl Into<MixedContentType>) -> Self {
        self.mixed_content_type = Some(mixed_content_type.into());
        self
    }
    pub fn certificate(mut self, certificate: impl Into<String>) -> Self {
        let v = self.certificate.get_or_insert(Vec::new());
        v.push(certificate.into());
        self
    }
    pub fn certificates<I, S>(mut self, certificates: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.certificate.get_or_insert(Vec::new());
        for val in certificates {
            v.push(val.into());
        }
        self
    }
    pub fn recommendation(mut self, recommendation: impl Into<String>) -> Self {
        let v = self.recommendations.get_or_insert(Vec::new());
        v.push(recommendation.into());
        self
    }
    pub fn recommendations<I, S>(mut self, recommendations: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.recommendations.get_or_insert(Vec::new());
        for val in recommendations {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SecurityStateExplanation, String> {
        Ok(SecurityStateExplanation {
            security_state: self.security_state.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(security_state))
            })?,
            title: self
                .title
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(title)))?,
            summary: self
                .summary
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(summary)))?,
            description: self
                .description
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(description)))?,
            mixed_content_type: self.mixed_content_type.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(mixed_content_type)
                )
            })?,
            certificate: self
                .certificate
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(certificate)))?,
            recommendations: self.recommendations,
        })
    }
}
