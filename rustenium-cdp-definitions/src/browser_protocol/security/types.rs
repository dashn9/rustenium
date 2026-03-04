use serde::{Deserialize, Serialize};
#[doc = "An internal certificate ID value.\n[CertificateId](https://chromedevtools.github.io/devtools-protocol/tot/Security/#type-CertificateId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Copy, Hash)]
pub struct CertificateId(i64);
impl CertificateId {
    pub fn new(val: impl Into<i64>) -> Self {
        CertificateId(val.into())
    }
    pub fn inner(&self) -> &i64 {
        &self.0
    }
}
impl CertificateId {
    pub const IDENTIFIER: &'static str = "Security.CertificateId";
}
#[doc = "A description of mixed content (HTTP resources on HTTPS pages), as defined by\nhttps://www.w3.org/TR/mixed-content/#categories"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MixedContentType {
    #[serde(rename = "blockable")]
    Blockable,
    #[serde(rename = "optionally-blockable")]
    OptionallyBlockable,
    #[serde(rename = "none")]
    None,
}
#[doc = "The security level of a page or resource."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecurityState {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "insecure")]
    Insecure,
    #[serde(rename = "secure")]
    Secure,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "insecure-broken")]
    InsecureBroken,
}
#[doc = "Details about the security state of the page certificate.\n[CertificateSecurityState](https://chromedevtools.github.io/devtools-protocol/tot/Security/#type-CertificateSecurityState)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateSecurityState {
    #[doc = "Protocol name (e.g. \"TLS 1.2\" or \"QUIC\")."]
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[doc = "Key Exchange used by the connection, or the empty string if not applicable."]
    #[serde(rename = "keyExchange")]
    pub key_exchange: String,
    #[doc = "(EC)DH group used by the connection, if applicable."]
    #[serde(rename = "keyExchangeGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub key_exchange_group: Option<String>,
    #[doc = "Cipher name."]
    #[serde(rename = "cipher")]
    pub cipher: String,
    #[doc = "TLS MAC. Note that AEAD ciphers do not have separate MACs."]
    #[serde(rename = "mac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mac: Option<String>,
    #[doc = "Page certificate."]
    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub certificate: Vec<String>,
    #[doc = "Certificate subject name."]
    #[serde(rename = "subjectName")]
    pub subject_name: String,
    #[doc = "Name of the issuing CA."]
    #[serde(rename = "issuer")]
    pub issuer: String,
    #[doc = "Certificate valid from date."]
    #[serde(rename = "validFrom")]
    pub valid_from: super::super::network::types::TimeSinceEpoch,
    #[doc = "Certificate valid to (expiration) date"]
    #[serde(rename = "validTo")]
    pub valid_to: super::super::network::types::TimeSinceEpoch,
    #[doc = "The highest priority network error code, if the certificate has an error."]
    #[serde(rename = "certificateNetworkError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub certificate_network_error: Option<String>,
    #[doc = "True if the certificate uses a weak signature algorithm."]
    #[serde(rename = "certificateHasWeakSignature")]
    pub certificate_has_weak_signature: bool,
    #[doc = "True if the certificate has a SHA1 signature in the chain."]
    #[serde(rename = "certificateHasSha1Signature")]
    pub certificate_has_sha1_signature: bool,
    #[doc = "True if modern SSL"]
    #[serde(rename = "modernSSL")]
    pub modern_ssl: bool,
    #[doc = "True if the connection is using an obsolete SSL protocol."]
    #[serde(rename = "obsoleteSslProtocol")]
    pub obsolete_ssl_protocol: bool,
    #[doc = "True if the connection is using an obsolete SSL key exchange."]
    #[serde(rename = "obsoleteSslKeyExchange")]
    pub obsolete_ssl_key_exchange: bool,
    #[doc = "True if the connection is using an obsolete SSL cipher."]
    #[serde(rename = "obsoleteSslCipher")]
    pub obsolete_ssl_cipher: bool,
    #[doc = "True if the connection is using an obsolete SSL signature."]
    #[serde(rename = "obsoleteSslSignature")]
    pub obsolete_ssl_signature: bool,
}
impl CertificateSecurityState {
    pub const IDENTIFIER: &'static str = "Security.CertificateSecurityState";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SafetyTipStatus {
    #[serde(rename = "badReputation")]
    BadReputation,
    #[serde(rename = "lookalike")]
    Lookalike,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyTipInfo {
    #[doc = "Describes whether the page triggers any safety tips or reputation warnings. Default is unknown."]
    #[serde(rename = "safetyTipStatus")]
    pub safety_tip_status: SafetyTipStatus,
    #[doc = "The URL the safety tip suggested (\"Did you mean?\"). Only filled in for lookalike matches."]
    #[serde(rename = "safeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub safe_url: Option<String>,
}
impl SafetyTipInfo {
    pub fn new(safety_tip_status: impl Into<SafetyTipStatus>) -> Self {
        Self {
            safety_tip_status: safety_tip_status.into(),
            safe_url: None,
        }
    }
}
impl SafetyTipInfo {
    pub const IDENTIFIER: &'static str = "Security.SafetyTipInfo";
}
#[doc = "Security state information about the page.\n[VisibleSecurityState](https://chromedevtools.github.io/devtools-protocol/tot/Security/#type-VisibleSecurityState)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisibleSecurityState {
    #[doc = "The security level of the page."]
    #[serde(rename = "securityState")]
    pub security_state: SecurityState,
    #[doc = "Security state details about the page certificate."]
    #[serde(rename = "certificateSecurityState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub certificate_security_state: Option<CertificateSecurityState>,
    #[doc = "The type of Safety Tip triggered on the page. Note that this field will be set even if the Safety Tip UI was not actually shown."]
    #[serde(rename = "safetyTipInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub safety_tip_info: Option<SafetyTipInfo>,
    #[doc = "Array of security state issues ids."]
    #[serde(rename = "securityStateIssueIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security_state_issue_ids: Vec<String>,
}
impl VisibleSecurityState {
    pub fn new(
        security_state: impl Into<SecurityState>,
        security_state_issue_ids: Vec<String>,
    ) -> Self {
        Self {
            security_state: security_state.into(),
            security_state_issue_ids,
            certificate_security_state: None,
            safety_tip_info: None,
        }
    }
}
impl VisibleSecurityState {
    pub const IDENTIFIER: &'static str = "Security.VisibleSecurityState";
}
#[doc = "An explanation of an factor contributing to the security state.\n[SecurityStateExplanation](https://chromedevtools.github.io/devtools-protocol/tot/Security/#type-SecurityStateExplanation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityStateExplanation {
    #[doc = "Security state representing the severity of the factor being explained."]
    #[serde(rename = "securityState")]
    pub security_state: SecurityState,
    #[doc = "Title describing the type of factor."]
    #[serde(rename = "title")]
    pub title: String,
    #[doc = "Short phrase describing the type of factor."]
    #[serde(rename = "summary")]
    pub summary: String,
    #[doc = "Full text explanation of the factor."]
    #[serde(rename = "description")]
    pub description: String,
    #[doc = "The type of mixed content described by the explanation."]
    #[serde(rename = "mixedContentType")]
    pub mixed_content_type: MixedContentType,
    #[doc = "Page certificate."]
    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub certificate: Vec<String>,
    #[doc = "Recommendations to fix any issues."]
    #[serde(rename = "recommendations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub recommendations: Option<Vec<String>>,
}
impl SecurityStateExplanation {
    pub const IDENTIFIER: &'static str = "Security.SecurityStateExplanation";
}
#[doc = "The action to take when a certificate error occurs. continue will continue processing the\nrequest and cancel will cancel the request."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CertificateErrorAction {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "cancel")]
    Cancel,
}
group_enum ! (Type { CertificateId (CertificateId) , MixedContentType (MixedContentType) , SecurityState (SecurityState) , CertificateSecurityState (CertificateSecurityState) , SafetyTipStatus (SafetyTipStatus) , SafetyTipInfo (SafetyTipInfo) , VisibleSecurityState (VisibleSecurityState) , SecurityStateExplanation (SecurityStateExplanation) , CertificateErrorAction (CertificateErrorAction) });
