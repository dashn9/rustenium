use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSimulatedCentralStateResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulatePreconnectedPeripheralResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateAdvertisementResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateGattOperationResponseResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateCharacteristicOperationResponseResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateDescriptorOperationResponseResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddServiceResult {
    #[doc = "An identifier that uniquely represents this service."]
    #[serde(rename = "serviceId")]
    pub service_id: String,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveServiceResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddCharacteristicResult {
    #[doc = "An identifier that uniquely represents this characteristic."]
    #[serde(rename = "characteristicId")]
    pub characteristic_id: String,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveCharacteristicResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddDescriptorResult {
    #[doc = "An identifier that uniquely represents this descriptor."]
    #[serde(rename = "descriptorId")]
    pub descriptor_id: String,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveDescriptorResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateGattDisconnectionResult {}
