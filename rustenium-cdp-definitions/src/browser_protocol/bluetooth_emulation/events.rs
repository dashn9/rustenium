use serde::{Deserialize, Serialize};
#[doc = "Event for when a GATT operation of |type| to the peripheral with |address|\nhappened.\n[gattOperationReceived](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#event-gattOperationReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GattOperationReceivedParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "type")]
    pub r#type: super::types::GattOperationType,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GattOperationReceivedMethod {
    #[serde(rename = "BluetoothEmulation.gattOperationReceived")]
    GattOperationReceived,
}
#[doc = "Event for when a GATT operation of |type| to the peripheral with |address|\nhappened.\n[gattOperationReceived](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#event-gattOperationReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GattOperationReceived {
    pub method: GattOperationReceivedMethod,
    pub params: GattOperationReceivedParams,
}
impl GattOperationReceived {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.gattOperationReceived";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Event for when a characteristic operation of |type| to the characteristic\nrespresented by |characteristicId| happened. |data| and |writeType| is\nexpected to exist when |type| is write.\n[characteristicOperationReceived](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#event-characteristicOperationReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacteristicOperationReceivedParams {
    #[serde(rename = "characteristicId")]
    pub characteristic_id: String,
    #[serde(rename = "type")]
    pub r#type: super::types::CharacteristicOperationType,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data: Option<crate::Binary>,
    #[serde(rename = "writeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub write_type: Option<super::types::CharacteristicWriteType>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CharacteristicOperationReceivedMethod {
    #[serde(rename = "BluetoothEmulation.characteristicOperationReceived")]
    CharacteristicOperationReceived,
}
#[doc = "Event for when a characteristic operation of |type| to the characteristic\nrespresented by |characteristicId| happened. |data| and |writeType| is\nexpected to exist when |type| is write.\n[characteristicOperationReceived](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#event-characteristicOperationReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacteristicOperationReceived {
    pub method: CharacteristicOperationReceivedMethod,
    pub params: CharacteristicOperationReceivedParams,
}
impl CharacteristicOperationReceived {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.characteristicOperationReceived";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Event for when a descriptor operation of |type| to the descriptor\nrespresented by |descriptorId| happened. |data| is expected to exist when\n|type| is write.\n[descriptorOperationReceived](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#event-descriptorOperationReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescriptorOperationReceivedParams {
    #[serde(rename = "descriptorId")]
    pub descriptor_id: String,
    #[serde(rename = "type")]
    pub r#type: super::types::DescriptorOperationType,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data: Option<crate::Binary>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DescriptorOperationReceivedMethod {
    #[serde(rename = "BluetoothEmulation.descriptorOperationReceived")]
    DescriptorOperationReceived,
}
#[doc = "Event for when a descriptor operation of |type| to the descriptor\nrespresented by |descriptorId| happened. |data| is expected to exist when\n|type| is write.\n[descriptorOperationReceived](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#event-descriptorOperationReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescriptorOperationReceived {
    pub method: DescriptorOperationReceivedMethod,
    pub params: DescriptorOperationReceivedParams,
}
impl DescriptorOperationReceived {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.descriptorOperationReceived";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (BluetoothEmulationEvents { GattOperationReceived (GattOperationReceived) , CharacteristicOperationReceived (CharacteristicOperationReceived) , DescriptorOperationReceived (DescriptorOperationReceived) } + identifiable);
