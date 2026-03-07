use serde::{Deserialize, Serialize};
#[doc = "The security state of the page changed.\n[visibleSecurityStateChanged](https://chromedevtools.github.io/devtools-protocol/tot/Security/#event-visibleSecurityStateChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisibleSecurityStateChangedParams {
    #[doc = "Security state information about the page."]
    #[serde(rename = "visibleSecurityState")]
    pub visible_security_state: super::types::VisibleSecurityState,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VisibleSecurityStateChangedMethod {
    #[serde(rename = "Security.visibleSecurityStateChanged")]
    VisibleSecurityStateChanged,
}
#[doc = "The security state of the page changed.\n[visibleSecurityStateChanged](https://chromedevtools.github.io/devtools-protocol/tot/Security/#event-visibleSecurityStateChanged)"]
#[derive(Debug, Clone, PartialEq)]
pub struct VisibleSecurityStateChanged {
    pub method: VisibleSecurityStateChangedMethod,
    pub params: VisibleSecurityStateChangedParams,
}
impl VisibleSecurityStateChanged {
    pub const IDENTIFIER: &'static str = "Security.visibleSecurityStateChanged";
}
group_enum ! (SecurityEvents { VisibleSecurityStateChanged (VisibleSecurityStateChanged) });
