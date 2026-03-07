use serde::{Deserialize, Serialize};
#[doc = "Emitted when an address form is filled.\n[addressFormFilled](https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#event-addressFormFilled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressFormFilledParams {
    #[doc = "Information about the fields that were filled"]
    #[serde(rename = "filledFields")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub filled_fields: Vec<super::types::FilledField>,
    #[doc = "An UI representation of the address used to fill the form.\nConsists of a 2D array where each child represents an address/profile line."]
    #[serde(rename = "addressUi")]
    pub address_ui: super::types::AddressUi,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddressFormFilledMethod {
    #[serde(rename = "Autofill.addressFormFilled")]
    AddressFormFilled,
}
impl AddressFormFilledMethod {
    pub const IDENTIFIER: &'static str = "Autofill.addressFormFilled";
}
#[doc = "Emitted when an address form is filled.\n[addressFormFilled](https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#event-addressFormFilled)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AddressFormFilled {
    pub method: AddressFormFilledMethod,
    pub params: AddressFormFilledParams,
}
group_enum ! (AutofillEvents { AddressFormFilled (AddressFormFilled) });
