use serde::{Deserialize, Serialize};
#[doc = "Indicates the various states of Central."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CentralState {
    #[serde(rename = "absent")]
    Absent,
    #[serde(rename = "powered-off")]
    PoweredOff,
    #[serde(rename = "powered-on")]
    PoweredOn,
}
#[doc = "Indicates the various types of GATT event."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GattOperationType {
    #[serde(rename = "connection")]
    Connection,
    #[serde(rename = "discovery")]
    Discovery,
}
#[doc = "Indicates the various types of characteristic write."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharacteristicWriteType {
    #[serde(rename = "write-default-deprecated")]
    WriteDefaultDeprecated,
    #[serde(rename = "write-with-response")]
    WriteWithResponse,
    #[serde(rename = "write-without-response")]
    WriteWithoutResponse,
}
#[doc = "Indicates the various types of characteristic operation."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharacteristicOperationType {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "subscribe-to-notifications")]
    SubscribeToNotifications,
    #[serde(rename = "unsubscribe-from-notifications")]
    UnsubscribeFromNotifications,
}
#[doc = "Indicates the various types of descriptor operation."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DescriptorOperationType {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
#[doc = "Stores the manufacturer data\n[ManufacturerData](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#type-ManufacturerData)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManufacturerData {
    #[doc = "Company identifier\nhttps://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/company_identifiers/company_identifiers.yaml\nhttps://usb.org/developers"]
    #[serde(rename = "key")]
    pub key: i64,
    #[doc = "Manufacturer-specific data"]
    #[serde(rename = "data")]
    pub data: super::super::super::Binary,
}
impl ManufacturerData {
    pub fn new(key: impl Into<i64>, data: impl Into<super::super::super::Binary>) -> Self {
        Self {
            key: key.into(),
            data: data.into(),
        }
    }
}
impl ManufacturerData {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.ManufacturerData";
}
#[doc = "Stores the byte data of the advertisement packet sent by a Bluetooth device.\n[ScanRecord](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#type-ScanRecord)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScanRecord {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "uuids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub uuids: Option<Vec<String>>,
    #[doc = "Stores the external appearance description of the device."]
    #[serde(rename = "appearance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub appearance: Option<i64>,
    #[doc = "Stores the transmission power of a broadcasting device."]
    #[serde(rename = "txPower")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tx_power: Option<i64>,
    #[doc = "Key is the company identifier and the value is an array of bytes of\nmanufacturer specific data."]
    #[serde(rename = "manufacturerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub manufacturer_data: Option<Vec<ManufacturerData>>,
}
impl ScanRecord {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.ScanRecord";
}
#[doc = "Stores the advertisement packet information that is sent by a Bluetooth device.\n[ScanEntry](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#type-ScanEntry)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanEntry {
    #[serde(rename = "deviceAddress")]
    pub device_address: String,
    #[serde(rename = "rssi")]
    pub rssi: i64,
    #[serde(rename = "scanRecord")]
    pub scan_record: ScanRecord,
}
impl ScanEntry {
    pub fn new(
        device_address: impl Into<String>,
        rssi: impl Into<i64>,
        scan_record: impl Into<ScanRecord>,
    ) -> Self {
        Self {
            device_address: device_address.into(),
            rssi: rssi.into(),
            scan_record: scan_record.into(),
        }
    }
}
impl ScanEntry {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.ScanEntry";
}
#[doc = "Describes the properties of a characteristic. This follows Bluetooth Core\nSpecification BT 4.2 Vol 3 Part G 3.3.1. Characteristic Properties.\n[CharacteristicProperties](https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#type-CharacteristicProperties)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CharacteristicProperties {
    #[serde(rename = "broadcast")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub broadcast: Option<bool>,
    #[serde(rename = "read")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub read: Option<bool>,
    #[serde(rename = "writeWithoutResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub write_without_response: Option<bool>,
    #[serde(rename = "write")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub write: Option<bool>,
    #[serde(rename = "notify")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub notify: Option<bool>,
    #[serde(rename = "indicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub indicate: Option<bool>,
    #[serde(rename = "authenticatedSignedWrites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub authenticated_signed_writes: Option<bool>,
    #[serde(rename = "extendedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub extended_properties: Option<bool>,
}
impl CharacteristicProperties {
    pub const IDENTIFIER: &'static str = "BluetoothEmulation.CharacteristicProperties";
}
group_enum ! (Type { CentralState (CentralState) , GattOperationType (GattOperationType) , CharacteristicWriteType (CharacteristicWriteType) , CharacteristicOperationType (CharacteristicOperationType) , DescriptorOperationType (DescriptorOperationType) , ManufacturerData (ManufacturerData) , ScanRecord (ScanRecord) , ScanEntry (ScanEntry) , CharacteristicProperties (CharacteristicProperties) });
