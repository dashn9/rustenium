use serde::{Deserialize, Serialize};
#[doc = "Trigger autofill on a form identified by the fieldId.\nIf the field and related form cannot be autofilled, returns an error.\n[trigger](https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#method-trigger)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerParams {
    #[doc = "Identifies a field that serves as an anchor for autofill."]
    #[serde(rename = "fieldId")]
    pub field_id: super::super::dom::types::BackendNodeId,
    #[doc = "Identifies the frame that field belongs to."]
    #[serde(rename = "frameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_id: Option<super::super::page::types::FrameId>,
    #[doc = "Credit card information to fill out the form. Credit card data is not saved.  Mutually exclusive with `address`."]
    #[serde(rename = "card")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub card: Option<super::types::CreditCard>,
    #[doc = "Address to fill out the form. Address data is not saved. Mutually exclusive with `card`."]
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub address: Option<super::types::Address>,
}
impl TriggerParams {
    pub fn new(field_id: impl Into<super::super::dom::types::BackendNodeId>) -> Self {
        Self {
            field_id: field_id.into(),
            frame_id: None,
            card: None,
            address: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriggerMethod {
    #[serde(rename = "Autofill.trigger")]
    Trigger,
}
impl TriggerMethod {
    pub const IDENTIFIER: &'static str = "Autofill.trigger";
}
#[doc = "Trigger autofill on a form identified by the fieldId.\nIf the field and related form cannot be autofilled, returns an error.\n[trigger](https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#method-trigger)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Trigger {
    pub method: TriggerMethod,
    pub params: TriggerParams,
}
#[doc = "Set addresses so that developers can verify their forms implementation.\n[setAddresses](https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#method-setAddresses)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAddressesParams {
    #[doc = "Test addresses for the available countries."]
    #[serde(rename = "addresses")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<super::types::Address>,
}
impl SetAddressesParams {
    pub fn new(addresses: Vec<super::types::Address>) -> Self {
        Self { addresses }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetAddressesMethod {
    #[serde(rename = "Autofill.setAddresses")]
    SetAddresses,
}
impl SetAddressesMethod {
    pub const IDENTIFIER: &'static str = "Autofill.setAddresses";
}
#[doc = "Set addresses so that developers can verify their forms implementation.\n[setAddresses](https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#method-setAddresses)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetAddresses {
    pub method: SetAddressesMethod,
    pub params: SetAddressesParams,
}
#[doc = "Disables autofill domain notifications.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Autofill.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "Autofill.disable";
}
#[doc = "Disables autofill domain notifications.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
#[doc = "Enables autofill domain notifications.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Autofill.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "Autofill.enable";
}
#[doc = "Enables autofill domain notifications.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
group_enum ! (AutofillCommands { Trigger (Trigger) , SetAddresses (SetAddresses) , Disable (Disable) , Enable (Enable) });
