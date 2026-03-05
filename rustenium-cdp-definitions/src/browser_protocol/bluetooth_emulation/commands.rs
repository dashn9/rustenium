use serde::{Deserialize, Serialize};
#[doc = "Enable the BluetoothEmulation domain.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {
    #[doc = "State of the simulated central."]
    #[serde(rename = "state")]
    pub state: super::types::CentralState,
    #[doc = "If the simulated central supports low-energy."]
    #[serde(rename = "leSupported")]
    pub le_supported: bool,
}
impl EnableParams {
    pub fn new(
        state: impl Into<super::types::CentralState>,
        le_supported: impl Into<bool>,
    ) -> Self {
        Self {
            state: state.into(),
            le_supported: le_supported.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "BluetoothEmulation.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.enable";
}
#[doc = "Enable the BluetoothEmulation domain.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
#[doc = "Set the state of the simulated central.\n[setSimulatedCentralState](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-setSimulatedCentralState)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSimulatedCentralStateParams {
    #[doc = "State of the simulated central."]
    #[serde(rename = "state")]
    pub state: super::types::CentralState,
}
impl SetSimulatedCentralStateParams {
    pub fn new(state: impl Into<super::types::CentralState>) -> Self {
        Self {
            state: state.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetSimulatedCentralStateMethod {
    #[serde(rename = "BluetoothEmulation.setSimulatedCentralState")]
    SetSimulatedCentralState,
}
impl SetSimulatedCentralStateMethod {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.setSimulatedCentralState";
}
#[doc = "Set the state of the simulated central.\n[setSimulatedCentralState](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-setSimulatedCentralState)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetSimulatedCentralState {
    pub method: SetSimulatedCentralStateMethod,
    pub params: SetSimulatedCentralStateParams,
}
#[doc = "Disable the BluetoothEmulation domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "BluetoothEmulation.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.disable";
}
#[doc = "Disable the BluetoothEmulation domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
#[doc = "Simulates a peripheral with |address|, |name| and |knownServiceUuids|\nthat has already been connected to the system.\n[simulatePreconnectedPeripheral](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-simulatePreconnectedPeripheral)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulatePreconnectedPeripheralParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "manufacturerData")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub manufacturer_data: Vec<super::types::ManufacturerData>,
    #[serde(rename = "knownServiceUuids")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub known_service_uuids: Vec<String>,
}
impl SimulatePreconnectedPeripheralParams {
    pub fn new(
        address: impl Into<String>,
        name: impl Into<String>,
        manufacturer_data: Vec<super::types::ManufacturerData>,
        known_service_uuids: Vec<String>,
    ) -> Self {
        Self {
            address: address.into(),
            name: name.into(),
            manufacturer_data,
            known_service_uuids,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimulatePreconnectedPeripheralMethod {
    #[serde(rename = "BluetoothEmulation.simulatePreconnectedPeripheral")]
    SimulatePreconnectedPeripheral,
}
impl SimulatePreconnectedPeripheralMethod {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.simulatePreconnectedPeripheral";
}
#[doc = "Simulates a peripheral with |address|, |name| and |knownServiceUuids|\nthat has already been connected to the system.\n[simulatePreconnectedPeripheral](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-simulatePreconnectedPeripheral)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SimulatePreconnectedPeripheral {
    pub method: SimulatePreconnectedPeripheralMethod,
    pub params: SimulatePreconnectedPeripheralParams,
}
#[doc = "Simulates an advertisement packet described in |entry| being received by\nthe central.\n[simulateAdvertisement](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-simulateAdvertisement)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulateAdvertisementParams {
    #[serde(rename = "entry")]
    pub entry: super::types::ScanEntry,
}
impl SimulateAdvertisementParams {
    pub fn new(entry: impl Into<super::types::ScanEntry>) -> Self {
        Self {
            entry: entry.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimulateAdvertisementMethod {
    #[serde(rename = "BluetoothEmulation.simulateAdvertisement")]
    SimulateAdvertisement,
}
impl SimulateAdvertisementMethod {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.simulateAdvertisement";
}
#[doc = "Simulates an advertisement packet described in |entry| being received by\nthe central.\n[simulateAdvertisement](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-simulateAdvertisement)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SimulateAdvertisement {
    pub method: SimulateAdvertisementMethod,
    pub params: SimulateAdvertisementParams,
}
#[doc = "Simulates the response code from the peripheral with |address| for a\nGATT operation of |type|. The |code| value follows the HCI Error Codes from\nBluetooth Core Specification Vol 2 Part D 1.3 List Of Error Codes.\n[simulateGATTOperationResponse](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-simulateGATTOperationResponse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulateGattOperationResponseParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "type")]
    pub r#type: super::types::GattOperationType,
    #[serde(rename = "code")]
    pub code: i64,
}
impl SimulateGattOperationResponseParams {
    pub fn new(
        address: impl Into<String>,
        r#type: impl Into<super::types::GattOperationType>,
        code: impl Into<i64>,
    ) -> Self {
        Self {
            address: address.into(),
            r#type: r#type.into(),
            code: code.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimulateGattOperationResponseMethod {
    #[serde(rename = "BluetoothEmulation.simulateGATTOperationResponse")]
    SimulateGattOperationResponse,
}
impl SimulateGattOperationResponseMethod {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.simulateGATTOperationResponse";
}
#[doc = "Simulates the response code from the peripheral with |address| for a\nGATT operation of |type|. The |code| value follows the HCI Error Codes from\nBluetooth Core Specification Vol 2 Part D 1.3 List Of Error Codes.\n[simulateGATTOperationResponse](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-simulateGATTOperationResponse)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SimulateGattOperationResponse {
    pub method: SimulateGattOperationResponseMethod,
    pub params: SimulateGattOperationResponseParams,
}
#[doc = "Simulates the response from the characteristic with |characteristicId| for a\ncharacteristic operation of |type|. The |code| value follows the Error\nCodes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.\nThe |data| is expected to exist when simulating a successful read operation\nresponse.\n[simulateCharacteristicOperationResponse](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-simulateCharacteristicOperationResponse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulateCharacteristicOperationResponseParams {
    #[serde(rename = "characteristicId")]
    pub characteristic_id: String,
    #[serde(rename = "type")]
    pub r#type: super::types::CharacteristicOperationType,
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data: Option<super::super::super::Binary>,
}
impl SimulateCharacteristicOperationResponseParams {
    pub fn new(
        characteristic_id: impl Into<String>,
        r#type: impl Into<super::types::CharacteristicOperationType>,
        code: impl Into<i64>,
    ) -> Self {
        Self {
            characteristic_id: characteristic_id.into(),
            r#type: r#type.into(),
            code: code.into(),
            data: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimulateCharacteristicOperationResponseMethod {
    #[serde(rename = "BluetoothEmulation.simulateCharacteristicOperationResponse")]
    SimulateCharacteristicOperationResponse,
}
impl SimulateCharacteristicOperationResponseMethod {
    pub const IDENTIFIER: &'static str =
        "BluetoothEmulation.simulateCharacteristicOperationResponse";
}
#[doc = "Simulates the response from the characteristic with |characteristicId| for a\ncharacteristic operation of |type|. The |code| value follows the Error\nCodes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.\nThe |data| is expected to exist when simulating a successful read operation\nresponse.\n[simulateCharacteristicOperationResponse](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-simulateCharacteristicOperationResponse)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SimulateCharacteristicOperationResponse {
    pub method: SimulateCharacteristicOperationResponseMethod,
    pub params: SimulateCharacteristicOperationResponseParams,
}
#[doc = "Simulates the response from the descriptor with |descriptorId| for a\ndescriptor operation of |type|. The |code| value follows the Error\nCodes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.\nThe |data| is expected to exist when simulating a successful read operation\nresponse.\n[simulateDescriptorOperationResponse](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-simulateDescriptorOperationResponse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulateDescriptorOperationResponseParams {
    #[serde(rename = "descriptorId")]
    pub descriptor_id: String,
    #[serde(rename = "type")]
    pub r#type: super::types::DescriptorOperationType,
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data: Option<super::super::super::Binary>,
}
impl SimulateDescriptorOperationResponseParams {
    pub fn new(
        descriptor_id: impl Into<String>,
        r#type: impl Into<super::types::DescriptorOperationType>,
        code: impl Into<i64>,
    ) -> Self {
        Self {
            descriptor_id: descriptor_id.into(),
            r#type: r#type.into(),
            code: code.into(),
            data: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimulateDescriptorOperationResponseMethod {
    #[serde(rename = "BluetoothEmulation.simulateDescriptorOperationResponse")]
    SimulateDescriptorOperationResponse,
}
impl SimulateDescriptorOperationResponseMethod {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.simulateDescriptorOperationResponse";
}
#[doc = "Simulates the response from the descriptor with |descriptorId| for a\ndescriptor operation of |type|. The |code| value follows the Error\nCodes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.\nThe |data| is expected to exist when simulating a successful read operation\nresponse.\n[simulateDescriptorOperationResponse](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-simulateDescriptorOperationResponse)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SimulateDescriptorOperationResponse {
    pub method: SimulateDescriptorOperationResponseMethod,
    pub params: SimulateDescriptorOperationResponseParams,
}
#[doc = "Adds a service with |serviceUuid| to the peripheral with |address|.\n[addService](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-addService)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddServiceParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "serviceUuid")]
    pub service_uuid: String,
}
impl AddServiceParams {
    pub fn new(address: impl Into<String>, service_uuid: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            service_uuid: service_uuid.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddServiceMethod {
    #[serde(rename = "BluetoothEmulation.addService")]
    AddService,
}
impl AddServiceMethod {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.addService";
}
#[doc = "Adds a service with |serviceUuid| to the peripheral with |address|.\n[addService](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-addService)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AddService {
    pub method: AddServiceMethod,
    pub params: AddServiceParams,
}
#[doc = "Removes the service respresented by |serviceId| from the simulated central.\n[removeService](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-removeService)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveServiceParams {
    #[serde(rename = "serviceId")]
    pub service_id: String,
}
impl RemoveServiceParams {
    pub fn new(service_id: impl Into<String>) -> Self {
        Self {
            service_id: service_id.into(),
        }
    }
}
impl<T: Into<String>> From<T> for RemoveServiceParams {
    fn from(url: T) -> Self {
        RemoveServiceParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveServiceMethod {
    #[serde(rename = "BluetoothEmulation.removeService")]
    RemoveService,
}
impl RemoveServiceMethod {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.removeService";
}
#[doc = "Removes the service respresented by |serviceId| from the simulated central.\n[removeService](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-removeService)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RemoveService {
    pub method: RemoveServiceMethod,
    pub params: RemoveServiceParams,
}
#[doc = "Adds a characteristic with |characteristicUuid| and |properties| to the\nservice represented by |serviceId|.\n[addCharacteristic](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-addCharacteristic)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddCharacteristicParams {
    #[serde(rename = "serviceId")]
    pub service_id: String,
    #[serde(rename = "characteristicUuid")]
    pub characteristic_uuid: String,
    #[serde(rename = "properties")]
    pub properties: super::types::CharacteristicProperties,
}
impl AddCharacteristicParams {
    pub fn new(
        service_id: impl Into<String>,
        characteristic_uuid: impl Into<String>,
        properties: impl Into<super::types::CharacteristicProperties>,
    ) -> Self {
        Self {
            service_id: service_id.into(),
            characteristic_uuid: characteristic_uuid.into(),
            properties: properties.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddCharacteristicMethod {
    #[serde(rename = "BluetoothEmulation.addCharacteristic")]
    AddCharacteristic,
}
impl AddCharacteristicMethod {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.addCharacteristic";
}
#[doc = "Adds a characteristic with |characteristicUuid| and |properties| to the\nservice represented by |serviceId|.\n[addCharacteristic](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-addCharacteristic)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AddCharacteristic {
    pub method: AddCharacteristicMethod,
    pub params: AddCharacteristicParams,
}
#[doc = "Removes the characteristic respresented by |characteristicId| from the\nsimulated central.\n[removeCharacteristic](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-removeCharacteristic)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveCharacteristicParams {
    #[serde(rename = "characteristicId")]
    pub characteristic_id: String,
}
impl RemoveCharacteristicParams {
    pub fn new(characteristic_id: impl Into<String>) -> Self {
        Self {
            characteristic_id: characteristic_id.into(),
        }
    }
}
impl<T: Into<String>> From<T> for RemoveCharacteristicParams {
    fn from(url: T) -> Self {
        RemoveCharacteristicParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveCharacteristicMethod {
    #[serde(rename = "BluetoothEmulation.removeCharacteristic")]
    RemoveCharacteristic,
}
impl RemoveCharacteristicMethod {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.removeCharacteristic";
}
#[doc = "Removes the characteristic respresented by |characteristicId| from the\nsimulated central.\n[removeCharacteristic](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-removeCharacteristic)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RemoveCharacteristic {
    pub method: RemoveCharacteristicMethod,
    pub params: RemoveCharacteristicParams,
}
#[doc = "Adds a descriptor with |descriptorUuid| to the characteristic respresented\nby |characteristicId|.\n[addDescriptor](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-addDescriptor)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddDescriptorParams {
    #[serde(rename = "characteristicId")]
    pub characteristic_id: String,
    #[serde(rename = "descriptorUuid")]
    pub descriptor_uuid: String,
}
impl AddDescriptorParams {
    pub fn new(characteristic_id: impl Into<String>, descriptor_uuid: impl Into<String>) -> Self {
        Self {
            characteristic_id: characteristic_id.into(),
            descriptor_uuid: descriptor_uuid.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddDescriptorMethod {
    #[serde(rename = "BluetoothEmulation.addDescriptor")]
    AddDescriptor,
}
impl AddDescriptorMethod {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.addDescriptor";
}
#[doc = "Adds a descriptor with |descriptorUuid| to the characteristic respresented\nby |characteristicId|.\n[addDescriptor](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-addDescriptor)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AddDescriptor {
    pub method: AddDescriptorMethod,
    pub params: AddDescriptorParams,
}
#[doc = "Removes the descriptor with |descriptorId| from the simulated central.\n[removeDescriptor](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-removeDescriptor)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveDescriptorParams {
    #[serde(rename = "descriptorId")]
    pub descriptor_id: String,
}
impl RemoveDescriptorParams {
    pub fn new(descriptor_id: impl Into<String>) -> Self {
        Self {
            descriptor_id: descriptor_id.into(),
        }
    }
}
impl<T: Into<String>> From<T> for RemoveDescriptorParams {
    fn from(url: T) -> Self {
        RemoveDescriptorParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveDescriptorMethod {
    #[serde(rename = "BluetoothEmulation.removeDescriptor")]
    RemoveDescriptor,
}
impl RemoveDescriptorMethod {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.removeDescriptor";
}
#[doc = "Removes the descriptor with |descriptorId| from the simulated central.\n[removeDescriptor](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-removeDescriptor)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RemoveDescriptor {
    pub method: RemoveDescriptorMethod,
    pub params: RemoveDescriptorParams,
}
#[doc = "Simulates a GATT disconnection from the peripheral with |address|.\n[simulateGATTDisconnection](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-simulateGATTDisconnection)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulateGattDisconnectionParams {
    #[serde(rename = "address")]
    pub address: String,
}
impl SimulateGattDisconnectionParams {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
        }
    }
}
impl<T: Into<String>> From<T> for SimulateGattDisconnectionParams {
    fn from(url: T) -> Self {
        SimulateGattDisconnectionParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimulateGattDisconnectionMethod {
    #[serde(rename = "BluetoothEmulation.simulateGATTDisconnection")]
    SimulateGattDisconnection,
}
impl SimulateGattDisconnectionMethod {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.simulateGATTDisconnection";
}
#[doc = "Simulates a GATT disconnection from the peripheral with |address|.\n[simulateGATTDisconnection](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#method-simulateGATTDisconnection)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SimulateGattDisconnection {
    pub method: SimulateGattDisconnectionMethod,
    pub params: SimulateGattDisconnectionParams,
}
group_enum ! (BluetoothEmulationCommands { Enable (Enable) , SetSimulatedCentralState (SetSimulatedCentralState) , Disable (Disable) , SimulatePreconnectedPeripheral (SimulatePreconnectedPeripheral) , SimulateAdvertisement (SimulateAdvertisement) , SimulateGattOperationResponse (SimulateGattOperationResponse) , SimulateCharacteristicOperationResponse (SimulateCharacteristicOperationResponse) , SimulateDescriptorOperationResponse (SimulateDescriptorOperationResponse) , AddService (AddService) , RemoveService (RemoveService) , AddCharacteristic (AddCharacteristic) , RemoveCharacteristic (RemoveCharacteristic) , AddDescriptor (AddDescriptor) , RemoveDescriptor (RemoveDescriptor) , SimulateGattDisconnection (SimulateGattDisconnection) });
