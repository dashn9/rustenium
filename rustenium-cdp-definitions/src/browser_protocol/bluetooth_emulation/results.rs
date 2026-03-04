use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddServiceReturns {
    #[doc = "An identifier that uniquely represents this service."]
    #[serde(rename = "serviceId")]
    pub service_id: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddCharacteristicReturns {
    #[doc = "An identifier that uniquely represents this characteristic."]
    #[serde(rename = "characteristicId")]
    pub characteristic_id: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddDescriptorReturns {
    #[doc = "An identifier that uniquely represents this descriptor."]
    #[serde(rename = "descriptorId")]
    pub descriptor_id: String,
}
