use serde::{Deserialize, Serialize};
#[doc = "The security state of the page changed.\n[visibleSecurityStateChanged](https://chromedevtools.github.io/devtools-protocol/tot/Security/#event-visibleSecurityStateChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisibleSecurityStateChanged {
    #[doc = "Security state information about the page."]
    #[serde(rename = "visibleSecurityState")]
    pub visible_security_state: super::types::VisibleSecurityState,
}
impl VisibleSecurityStateChanged {
    pub const IDENTIFIER: &'static str = "Security.visibleSecurityStateChanged";
}
group_enum ! (SecurityEvents { VisibleSecurityStateChanged (VisibleSecurityStateChanged) });
