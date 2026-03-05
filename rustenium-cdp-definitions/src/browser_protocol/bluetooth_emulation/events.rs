use serde::{Deserialize, Serialize};
#[doc = "Event for when a GATT operation of |type| to the peripheral with |address|\nhappened.\n[gattOperationReceived](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#event-gattOperationReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GattOperationReceived {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "type")]
    pub r#type: super::types::GattOperationType,
}
impl GattOperationReceived {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.gattOperationReceived";
}
#[doc = "Event for when a characteristic operation of |type| to the characteristic\nrespresented by |characteristicId| happened. |data| and |writeType| is\nexpected to exist when |type| is write.\n[characteristicOperationReceived](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#event-characteristicOperationReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacteristicOperationReceived {
    #[serde(rename = "characteristicId")]
    pub characteristic_id: String,
    #[serde(rename = "type")]
    pub r#type: super::types::CharacteristicOperationType,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data: Option<super::super::super::Binary>,
    #[serde(rename = "writeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub write_type: Option<super::types::CharacteristicWriteType>,
}
impl CharacteristicOperationReceived {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.characteristicOperationReceived";
}
#[doc = "Event for when a descriptor operation of |type| to the descriptor\nrespresented by |descriptorId| happened. |data| is expected to exist when\n|type| is write.\n[descriptorOperationReceived](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#event-descriptorOperationReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescriptorOperationReceived {
    #[serde(rename = "descriptorId")]
    pub descriptor_id: String,
    #[serde(rename = "type")]
    pub r#type: super::types::DescriptorOperationType,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data: Option<super::super::super::Binary>,
}
impl DescriptorOperationReceived {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.descriptorOperationReceived";
}
group_enum ! (BluetoothEmulationEvents { GattOperationReceived (GattOperationReceived) , CharacteristicOperationReceived (CharacteristicOperationReceived) , DescriptorOperationReceived (DescriptorOperationReceived) });
