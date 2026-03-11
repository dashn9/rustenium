use serde::{Deserialize, Serialize};
#[doc = "Returns the response body and size if it were re-encoded with the specified settings. Only\napplies to images.\n[getEncodedResponse](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#method-getEncodedResponse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEncodedResponseParams {
    #[doc = "Identifier of the network request to get content for."]
    #[serde(rename = "requestId")]
    pub request_id: crate::browser_protocol::network::types::RequestId,
    #[doc = "The encoding to use."]
    #[serde(rename = "encoding")]
    pub encoding: GetEncodedResponseEncoding,
    #[doc = "The quality of the encoding (0-1). (defaults to 1)"]
    #[serde(rename = "quality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub quality: Option<f64>,
    #[doc = "Whether to only return the size information (defaults to false)."]
    #[serde(rename = "sizeOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub size_only: Option<bool>,
}
#[doc = "The encoding to use."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GetEncodedResponseEncoding {
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
}
impl GetEncodedResponseParams {
    pub fn new(
        request_id: impl Into<crate::browser_protocol::network::types::RequestId>,
        encoding: impl Into<GetEncodedResponseEncoding>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            encoding: encoding.into(),
            quality: None,
            size_only: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetEncodedResponseMethod {
    #[serde(rename = "Audits.getEncodedResponse")]
    GetEncodedResponse,
}
#[doc = "Returns the response body and size if it were re-encoded with the specified settings. Only\napplies to images.\n[getEncodedResponse](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#method-getEncodedResponse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEncodedResponse {
    pub method: GetEncodedResponseMethod,
    pub params: GetEncodedResponseParams,
}
impl GetEncodedResponse {
    pub const IDENTIFIER: &'static str = "Audits.getEncodedResponse";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetEncodedResponse {
    type Result = super::results::GetEncodedResponseResult;
}
#[doc = "Disables issues domain, prevents further issues from being reported to the client.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Audits.disable")]
    Disable,
}
#[doc = "Disables issues domain, prevents further issues from being reported to the client.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "Audits.disable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enables issues domain, sends the issues collected so far to the client by means of the\n`issueAdded` event.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Audits.enable")]
    Enable,
}
#[doc = "Enables issues domain, sends the issues collected so far to the client by means of the\n`issueAdded` event.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "Audits.enable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Runs the contrast check for the target page. Found issues are reported\nusing Audits.issueAdded event.\n[checkContrast](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#method-checkContrast)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CheckContrastParams {
    #[doc = "Whether to report WCAG AAA level issues. Default is false."]
    #[serde(rename = "reportAAA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub report_aaa: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CheckContrastMethod {
    #[serde(rename = "Audits.checkContrast")]
    CheckContrast,
}
#[doc = "Runs the contrast check for the target page. Found issues are reported\nusing Audits.issueAdded event.\n[checkContrast](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#method-checkContrast)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckContrast {
    pub method: CheckContrastMethod,
    pub params: CheckContrastParams,
}
impl CheckContrast {
    pub const IDENTIFIER: &'static str = "Audits.checkContrast";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for CheckContrast {
    type Result = super::results::CheckContrastResult;
}
#[doc = "Runs the form issues check for the target page. Found issues are reported\nusing Audits.issueAdded event.\n[checkFormsIssues](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#method-checkFormsIssues)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckFormsIssuesParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CheckFormsIssuesMethod {
    #[serde(rename = "Audits.checkFormsIssues")]
    CheckFormsIssues,
}
#[doc = "Runs the form issues check for the target page. Found issues are reported\nusing Audits.issueAdded event.\n[checkFormsIssues](https://chromedevtools.github.io/devtools-protocol/tot/Audits/#method-checkFormsIssues)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckFormsIssues {
    pub method: CheckFormsIssuesMethod,
    pub params: CheckFormsIssuesParams,
}
impl CheckFormsIssues {
    pub const IDENTIFIER: &'static str = "Audits.checkFormsIssues";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for CheckFormsIssues {
    type Result = super::results::CheckFormsIssuesResult;
}
group_enum ! (AuditsCommands { GetEncodedResponse (GetEncodedResponse) , Disable (Disable) , Enable (Enable) , CheckContrast (CheckContrast) , CheckFormsIssues (CheckFormsIssues) } + identifiable);
