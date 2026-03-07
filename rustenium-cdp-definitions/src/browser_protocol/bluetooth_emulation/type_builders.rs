use super::types::*;
impl ManufacturerData {
    pub fn builder() -> ManufacturerDataBuilder {
        <ManufacturerDataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ManufacturerDataBuilder {
    key: Option<i64>,
    data: Option<crate::Binary>,
}
impl ManufacturerDataBuilder {
    pub fn key(mut self, key: impl Into<i64>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn data(mut self, data: impl Into<crate::Binary>) -> Self {
        self.data = Some(data.into());
        self
    }
    pub fn build(self) -> Result<ManufacturerData, String> {
        Ok(ManufacturerData {
            key: self
                .key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key)))?,
            data: self
                .data
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(data)))?,
        })
    }
}
impl ScanRecord {
    pub fn builder() -> ScanRecordBuilder {
        <ScanRecordBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ScanRecordBuilder {
    name: Option<String>,
    uuids: Option<Vec<String>>,
    appearance: Option<i64>,
    tx_power: Option<i64>,
    manufacturer_data: Option<Vec<ManufacturerData>>,
}
impl ScanRecordBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        let v = self.uuids.get_or_insert(Vec::new());
        v.push(uuid.into());
        self
    }
    pub fn uuids<I, S>(mut self, uuids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.uuids.get_or_insert(Vec::new());
        for val in uuids {
            v.push(val.into());
        }
        self
    }
    pub fn appearance(mut self, appearance: impl Into<i64>) -> Self {
        self.appearance = Some(appearance.into());
        self
    }
    pub fn tx_power(mut self, tx_power: impl Into<i64>) -> Self {
        self.tx_power = Some(tx_power.into());
        self
    }
    pub fn manufacturer_data(mut self, manufacturer_data: impl Into<ManufacturerData>) -> Self {
        let v = self.manufacturer_data.get_or_insert(Vec::new());
        v.push(manufacturer_data.into());
        self
    }
    pub fn manufacturer_datas<I, S>(mut self, manufacturer_datas: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<ManufacturerData>,
    {
        let v = self.manufacturer_data.get_or_insert(Vec::new());
        for val in manufacturer_datas {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> ScanRecord {
        ScanRecord {
            name: self.name,
            uuids: self.uuids,
            appearance: self.appearance,
            tx_power: self.tx_power,
            manufacturer_data: self.manufacturer_data,
        }
    }
}
impl ScanEntry {
    pub fn builder() -> ScanEntryBuilder {
        <ScanEntryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ScanEntryBuilder {
    device_address: Option<String>,
    rssi: Option<i64>,
    scan_record: Option<ScanRecord>,
}
impl ScanEntryBuilder {
    pub fn device_address(mut self, device_address: impl Into<String>) -> Self {
        self.device_address = Some(device_address.into());
        self
    }
    pub fn rssi(mut self, rssi: impl Into<i64>) -> Self {
        self.rssi = Some(rssi.into());
        self
    }
    pub fn scan_record(mut self, scan_record: impl Into<ScanRecord>) -> Self {
        self.scan_record = Some(scan_record.into());
        self
    }
    pub fn build(self) -> Result<ScanEntry, String> {
        Ok(ScanEntry {
            device_address: self.device_address.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(device_address))
            })?,
            rssi: self
                .rssi
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(rssi)))?,
            scan_record: self
                .scan_record
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(scan_record)))?,
        })
    }
}
impl CharacteristicProperties {
    pub fn builder() -> CharacteristicPropertiesBuilder {
        <CharacteristicPropertiesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CharacteristicPropertiesBuilder {
    broadcast: Option<bool>,
    read: Option<bool>,
    write_without_response: Option<bool>,
    write: Option<bool>,
    notify: Option<bool>,
    indicate: Option<bool>,
    authenticated_signed_writes: Option<bool>,
    extended_properties: Option<bool>,
}
impl CharacteristicPropertiesBuilder {
    pub fn broadcast(mut self, broadcast: impl Into<bool>) -> Self {
        self.broadcast = Some(broadcast.into());
        self
    }
    pub fn read(mut self, read: impl Into<bool>) -> Self {
        self.read = Some(read.into());
        self
    }
    pub fn write_without_response(mut self, write_without_response: impl Into<bool>) -> Self {
        self.write_without_response = Some(write_without_response.into());
        self
    }
    pub fn write(mut self, write: impl Into<bool>) -> Self {
        self.write = Some(write.into());
        self
    }
    pub fn notify(mut self, notify: impl Into<bool>) -> Self {
        self.notify = Some(notify.into());
        self
    }
    pub fn indicate(mut self, indicate: impl Into<bool>) -> Self {
        self.indicate = Some(indicate.into());
        self
    }
    pub fn authenticated_signed_writes(
        mut self,
        authenticated_signed_writes: impl Into<bool>,
    ) -> Self {
        self.authenticated_signed_writes = Some(authenticated_signed_writes.into());
        self
    }
    pub fn extended_properties(mut self, extended_properties: impl Into<bool>) -> Self {
        self.extended_properties = Some(extended_properties.into());
        self
    }
    pub fn build(self) -> CharacteristicProperties {
        CharacteristicProperties {
            broadcast: self.broadcast,
            read: self.read,
            write_without_response: self.write_without_response,
            write: self.write,
            notify: self.notify,
            indicate: self.indicate,
            authenticated_signed_writes: self.authenticated_signed_writes,
            extended_properties: self.extended_properties,
        }
    }
}
