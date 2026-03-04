use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditCard {
    #[doc = "16-digit credit card number."]
    #[serde(rename = "number")]
    pub number: String,
    #[doc = "Name of the credit card owner."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "2-digit expiry month."]
    #[serde(rename = "expiryMonth")]
    pub expiry_month: String,
    #[doc = "4-digit expiry year."]
    #[serde(rename = "expiryYear")]
    pub expiry_year: String,
    #[doc = "3-digit card verification code."]
    #[serde(rename = "cvc")]
    pub cvc: String,
}
impl CreditCard {
    pub const IDENTIFIER: &'static str = "Autofill.CreditCard";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressField {
    #[doc = "address field name, for example GIVEN_NAME.\nThe full list of supported field names:\nhttps://source.chromium.org/chromium/chromium/src/+/main:components/autofill/core/browser/field_types.cc;l=38"]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "address field value, for example Jon Doe."]
    #[serde(rename = "value")]
    pub value: String,
}
impl AddressField {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
impl AddressField {
    pub const IDENTIFIER: &'static str = "Autofill.AddressField";
}
#[doc = "A list of address fields.\n[AddressFields](https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#type-AddressFields)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressFields {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<AddressField>,
}
impl AddressFields {
    pub fn new(fields: Vec<AddressField>) -> Self {
        Self { fields }
    }
}
impl AddressFields {
    pub const IDENTIFIER: &'static str = "Autofill.AddressFields";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    #[doc = "fields and values defining an address."]
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<AddressField>,
}
impl Address {
    pub fn new(fields: Vec<AddressField>) -> Self {
        Self { fields }
    }
}
impl Address {
    pub const IDENTIFIER: &'static str = "Autofill.Address";
}
#[doc = "Defines how an address can be displayed like in chrome://settings/addresses.\nAddress UI is a two dimensional array, each inner array is an \"address information line\", and when rendered in a UI surface should be displayed as such.\nThe following address UI for instance:\n[[{name: \"GIVE_NAME\", value: \"Jon\"}, {name: \"FAMILY_NAME\", value: \"Doe\"}], [{name: \"CITY\", value: \"Munich\"}, {name: \"ZIP\", value: \"81456\"}]]\nshould allow the receiver to render:\nJon Doe\nMunich 81456\n[AddressUI](https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#type-AddressUI)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressUi {
    #[doc = "A two dimension array containing the representation of values from an address profile."]
    #[serde(rename = "addressFields")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub address_fields: Vec<AddressFields>,
}
impl AddressUi {
    pub fn new(address_fields: Vec<AddressFields>) -> Self {
        Self { address_fields }
    }
}
impl AddressUi {
    pub const IDENTIFIER: &'static str = "Autofill.AddressUI";
}
#[doc = "Specified whether a filled field was done so by using the html autocomplete attribute or autofill heuristics."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FillingStrategy {
    #[serde(rename = "autocompleteAttribute")]
    AutocompleteAttribute,
    #[serde(rename = "autofillInferred")]
    AutofillInferred,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilledField {
    #[doc = "The type of the field, e.g text, password etc."]
    #[serde(rename = "htmlType")]
    pub html_type: String,
    #[doc = "the html id"]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "the html name"]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "the field value"]
    #[serde(rename = "value")]
    pub value: String,
    #[doc = "The actual field type, e.g FAMILY_NAME"]
    #[serde(rename = "autofillType")]
    pub autofill_type: String,
    #[doc = "The filling strategy"]
    #[serde(rename = "fillingStrategy")]
    pub filling_strategy: FillingStrategy,
    #[doc = "The frame the field belongs to"]
    #[serde(rename = "frameId")]
    pub frame_id: super::super::page::types::FrameId,
    #[doc = "The form field's DOM node"]
    #[serde(rename = "fieldId")]
    pub field_id: super::super::dom::types::BackendNodeId,
}
impl FilledField {
    pub const IDENTIFIER: &'static str = "Autofill.FilledField";
}
group_enum ! (Type { CreditCard (CreditCard) , AddressField (AddressField) , AddressFields (AddressFields) , Address (Address) , AddressUi (AddressUi) , FillingStrategy (FillingStrategy) , FilledField (FilledField) });
