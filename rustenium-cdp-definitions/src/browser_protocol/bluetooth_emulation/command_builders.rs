use super::commands::*;
impl Enable {
    pub fn builder() -> EnableBuilder {
        <EnableBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct EnableBuilder {
    state: Option<super::types::CentralState>,
    le_supported: Option<bool>,
}
impl EnableBuilder {
    pub fn state(mut self, state: impl Into<super::types::CentralState>) -> Self {
        self.state = Some(state.into());
        self
    }
    pub fn le_supported(mut self, le_supported: impl Into<bool>) -> Self {
        self.le_supported = Some(le_supported.into());
        self
    }
    pub fn build(self) -> Result<Enable, String> {
        Ok(Enable {
            method: EnableMethod::Enable,
            params: EnableParams {
                state: self
                    .state
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(state)))?,
                le_supported: self.le_supported.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(le_supported))
                })?,
            },
        })
    }
}
impl SetSimulatedCentralState {
    pub fn builder() -> SetSimulatedCentralStateBuilder {
        <SetSimulatedCentralStateBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetSimulatedCentralStateBuilder {
    state: Option<super::types::CentralState>,
}
impl SetSimulatedCentralStateBuilder {
    pub fn state(mut self, state: impl Into<super::types::CentralState>) -> Self {
        self.state = Some(state.into());
        self
    }
    pub fn build(self) -> Result<SetSimulatedCentralState, String> {
        Ok(SetSimulatedCentralState {
            method: SetSimulatedCentralStateMethod::SetSimulatedCentralState,
            params: SetSimulatedCentralStateParams {
                state: self
                    .state
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(state)))?,
            },
        })
    }
}
impl SimulatePreconnectedPeripheral {
    pub fn builder() -> SimulatePreconnectedPeripheralBuilder {
        <SimulatePreconnectedPeripheralBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SimulatePreconnectedPeripheralBuilder {
    address: Option<String>,
    name: Option<String>,
    manufacturer_data: Option<Vec<super::types::ManufacturerData>>,
    known_service_uuids: Option<Vec<String>>,
}
impl SimulatePreconnectedPeripheralBuilder {
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn manufacturer_data(
        mut self,
        manufacturer_data: impl Into<super::types::ManufacturerData>,
    ) -> Self {
        let v = self.manufacturer_data.get_or_insert(Vec::new());
        v.push(manufacturer_data.into());
        self
    }
    pub fn manufacturer_datas<I, S>(mut self, manufacturer_datas: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::ManufacturerData>,
    {
        let v = self.manufacturer_data.get_or_insert(Vec::new());
        for val in manufacturer_datas {
            v.push(val.into());
        }
        self
    }
    pub fn known_service_uuid(mut self, known_service_uuid: impl Into<String>) -> Self {
        let v = self.known_service_uuids.get_or_insert(Vec::new());
        v.push(known_service_uuid.into());
        self
    }
    pub fn known_service_uuids<I, S>(mut self, known_service_uuids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.known_service_uuids.get_or_insert(Vec::new());
        for val in known_service_uuids {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SimulatePreconnectedPeripheral, String> {
        Ok(SimulatePreconnectedPeripheral {
            method: SimulatePreconnectedPeripheralMethod::SimulatePreconnectedPeripheral,
            params: SimulatePreconnectedPeripheralParams {
                address: self
                    .address
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(address)))?,
                name: self
                    .name
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
                manufacturer_data: self.manufacturer_data.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(manufacturer_data)
                    )
                })?,
                known_service_uuids: self.known_service_uuids.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(known_service_uuids)
                    )
                })?,
            },
        })
    }
}
impl SimulateAdvertisement {
    pub fn builder() -> SimulateAdvertisementBuilder {
        <SimulateAdvertisementBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SimulateAdvertisementBuilder {
    entry: Option<super::types::ScanEntry>,
}
impl SimulateAdvertisementBuilder {
    pub fn entry(mut self, entry: impl Into<super::types::ScanEntry>) -> Self {
        self.entry = Some(entry.into());
        self
    }
    pub fn build(self) -> Result<SimulateAdvertisement, String> {
        Ok(SimulateAdvertisement {
            method: SimulateAdvertisementMethod::SimulateAdvertisement,
            params: SimulateAdvertisementParams {
                entry: self
                    .entry
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(entry)))?,
            },
        })
    }
}
impl SimulateGattOperationResponse {
    pub fn builder() -> SimulateGattOperationResponseBuilder {
        <SimulateGattOperationResponseBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SimulateGattOperationResponseBuilder {
    address: Option<String>,
    r#type: Option<super::types::GattOperationType>,
    code: Option<i64>,
}
impl SimulateGattOperationResponseBuilder {
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<super::types::GattOperationType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn code(mut self, code: impl Into<i64>) -> Self {
        self.code = Some(code.into());
        self
    }
    pub fn build(self) -> Result<SimulateGattOperationResponse, String> {
        Ok(SimulateGattOperationResponse {
            method: SimulateGattOperationResponseMethod::SimulateGattOperationResponse,
            params: SimulateGattOperationResponseParams {
                address: self
                    .address
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(address)))?,
                r#type: self
                    .r#type
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
                code: self
                    .code
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(code)))?,
            },
        })
    }
}
impl SimulateCharacteristicOperationResponse {
    pub fn builder() -> SimulateCharacteristicOperationResponseBuilder {
        <SimulateCharacteristicOperationResponseBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SimulateCharacteristicOperationResponseBuilder {
    characteristic_id: Option<String>,
    r#type: Option<super::types::CharacteristicOperationType>,
    code: Option<i64>,
    data: Option<crate::Binary>,
}
impl SimulateCharacteristicOperationResponseBuilder {
    pub fn characteristic_id(mut self, characteristic_id: impl Into<String>) -> Self {
        self.characteristic_id = Some(characteristic_id.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<super::types::CharacteristicOperationType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn code(mut self, code: impl Into<i64>) -> Self {
        self.code = Some(code.into());
        self
    }
    pub fn data(mut self, data: impl Into<crate::Binary>) -> Self {
        self.data = Some(data.into());
        self
    }
    pub fn build(self) -> Result<SimulateCharacteristicOperationResponse, String> {
        Ok (SimulateCharacteristicOperationResponse { method : SimulateCharacteristicOperationResponseMethod :: SimulateCharacteristicOperationResponse , params : SimulateCharacteristicOperationResponseParams { characteristic_id : self . characteristic_id . ok_or_else (|| format ! ("Field `{}` is mandatory." , std :: stringify ! (characteristic_id))) ? , r#type : self . r#type . ok_or_else (|| format ! ("Field `{}` is mandatory." , std :: stringify ! (r#type))) ? , code : self . code . ok_or_else (|| format ! ("Field `{}` is mandatory." , std :: stringify ! (code))) ? , data : self . data , } , })
    }
}
impl SimulateDescriptorOperationResponse {
    pub fn builder() -> SimulateDescriptorOperationResponseBuilder {
        <SimulateDescriptorOperationResponseBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SimulateDescriptorOperationResponseBuilder {
    descriptor_id: Option<String>,
    r#type: Option<super::types::DescriptorOperationType>,
    code: Option<i64>,
    data: Option<crate::Binary>,
}
impl SimulateDescriptorOperationResponseBuilder {
    pub fn descriptor_id(mut self, descriptor_id: impl Into<String>) -> Self {
        self.descriptor_id = Some(descriptor_id.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<super::types::DescriptorOperationType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn code(mut self, code: impl Into<i64>) -> Self {
        self.code = Some(code.into());
        self
    }
    pub fn data(mut self, data: impl Into<crate::Binary>) -> Self {
        self.data = Some(data.into());
        self
    }
    pub fn build(self) -> Result<SimulateDescriptorOperationResponse, String> {
        Ok(SimulateDescriptorOperationResponse {
            method: SimulateDescriptorOperationResponseMethod::SimulateDescriptorOperationResponse,
            params: SimulateDescriptorOperationResponseParams {
                descriptor_id: self.descriptor_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(descriptor_id))
                })?,
                r#type: self
                    .r#type
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
                code: self
                    .code
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(code)))?,
                data: self.data,
            },
        })
    }
}
impl AddService {
    pub fn builder() -> AddServiceBuilder {
        <AddServiceBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AddServiceBuilder {
    address: Option<String>,
    service_uuid: Option<String>,
}
impl AddServiceBuilder {
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }
    pub fn service_uuid(mut self, service_uuid: impl Into<String>) -> Self {
        self.service_uuid = Some(service_uuid.into());
        self
    }
    pub fn build(self) -> Result<AddService, String> {
        Ok(AddService {
            method: AddServiceMethod::AddService,
            params: AddServiceParams {
                address: self
                    .address
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(address)))?,
                service_uuid: self.service_uuid.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(service_uuid))
                })?,
            },
        })
    }
}
impl RemoveService {
    pub fn builder() -> RemoveServiceBuilder {
        <RemoveServiceBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveServiceBuilder {
    service_id: Option<String>,
}
impl RemoveServiceBuilder {
    pub fn service_id(mut self, service_id: impl Into<String>) -> Self {
        self.service_id = Some(service_id.into());
        self
    }
    pub fn build(self) -> Result<RemoveService, String> {
        Ok(RemoveService {
            method: RemoveServiceMethod::RemoveService,
            params: RemoveServiceParams {
                service_id: self.service_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(service_id))
                })?,
            },
        })
    }
}
impl AddCharacteristic {
    pub fn builder() -> AddCharacteristicBuilder {
        <AddCharacteristicBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AddCharacteristicBuilder {
    service_id: Option<String>,
    characteristic_uuid: Option<String>,
    properties: Option<super::types::CharacteristicProperties>,
}
impl AddCharacteristicBuilder {
    pub fn service_id(mut self, service_id: impl Into<String>) -> Self {
        self.service_id = Some(service_id.into());
        self
    }
    pub fn characteristic_uuid(mut self, characteristic_uuid: impl Into<String>) -> Self {
        self.characteristic_uuid = Some(characteristic_uuid.into());
        self
    }
    pub fn properties(
        mut self,
        properties: impl Into<super::types::CharacteristicProperties>,
    ) -> Self {
        self.properties = Some(properties.into());
        self
    }
    pub fn build(self) -> Result<AddCharacteristic, String> {
        Ok(AddCharacteristic {
            method: AddCharacteristicMethod::AddCharacteristic,
            params: AddCharacteristicParams {
                service_id: self.service_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(service_id))
                })?,
                characteristic_uuid: self.characteristic_uuid.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(characteristic_uuid)
                    )
                })?,
                properties: self.properties.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(properties))
                })?,
            },
        })
    }
}
impl RemoveCharacteristic {
    pub fn builder() -> RemoveCharacteristicBuilder {
        <RemoveCharacteristicBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveCharacteristicBuilder {
    characteristic_id: Option<String>,
}
impl RemoveCharacteristicBuilder {
    pub fn characteristic_id(mut self, characteristic_id: impl Into<String>) -> Self {
        self.characteristic_id = Some(characteristic_id.into());
        self
    }
    pub fn build(self) -> Result<RemoveCharacteristic, String> {
        Ok(RemoveCharacteristic {
            method: RemoveCharacteristicMethod::RemoveCharacteristic,
            params: RemoveCharacteristicParams {
                characteristic_id: self.characteristic_id.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(characteristic_id)
                    )
                })?,
            },
        })
    }
}
impl AddDescriptor {
    pub fn builder() -> AddDescriptorBuilder {
        <AddDescriptorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AddDescriptorBuilder {
    characteristic_id: Option<String>,
    descriptor_uuid: Option<String>,
}
impl AddDescriptorBuilder {
    pub fn characteristic_id(mut self, characteristic_id: impl Into<String>) -> Self {
        self.characteristic_id = Some(characteristic_id.into());
        self
    }
    pub fn descriptor_uuid(mut self, descriptor_uuid: impl Into<String>) -> Self {
        self.descriptor_uuid = Some(descriptor_uuid.into());
        self
    }
    pub fn build(self) -> Result<AddDescriptor, String> {
        Ok(AddDescriptor {
            method: AddDescriptorMethod::AddDescriptor,
            params: AddDescriptorParams {
                characteristic_id: self.characteristic_id.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(characteristic_id)
                    )
                })?,
                descriptor_uuid: self.descriptor_uuid.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(descriptor_uuid))
                })?,
            },
        })
    }
}
impl RemoveDescriptor {
    pub fn builder() -> RemoveDescriptorBuilder {
        <RemoveDescriptorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveDescriptorBuilder {
    descriptor_id: Option<String>,
}
impl RemoveDescriptorBuilder {
    pub fn descriptor_id(mut self, descriptor_id: impl Into<String>) -> Self {
        self.descriptor_id = Some(descriptor_id.into());
        self
    }
    pub fn build(self) -> Result<RemoveDescriptor, String> {
        Ok(RemoveDescriptor {
            method: RemoveDescriptorMethod::RemoveDescriptor,
            params: RemoveDescriptorParams {
                descriptor_id: self.descriptor_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(descriptor_id))
                })?,
            },
        })
    }
}
impl SimulateGattDisconnection {
    pub fn builder() -> SimulateGattDisconnectionBuilder {
        <SimulateGattDisconnectionBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SimulateGattDisconnectionBuilder {
    address: Option<String>,
}
impl SimulateGattDisconnectionBuilder {
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }
    pub fn build(self) -> Result<SimulateGattDisconnection, String> {
        Ok(SimulateGattDisconnection {
            method: SimulateGattDisconnectionMethod::SimulateGattDisconnection,
            params: SimulateGattDisconnectionParams {
                address: self
                    .address
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(address)))?,
            },
        })
    }
}
