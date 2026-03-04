use super::commands::*;
impl Trigger {
    pub fn builder() -> TriggerBuilder {
        TriggerBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct TriggerBuilder {
    field_id: Option<super::super::dom::types::BackendNodeId>,
    frame_id: Option<super::super::page::types::FrameId>,
    card: Option<super::types::CreditCard>,
    address: Option<super::types::Address>,
}
impl TriggerBuilder {
    pub fn field_id(
        mut self,
        field_id: impl Into<super::super::dom::types::BackendNodeId>,
    ) -> Self {
        self.field_id = Some(field_id.into());
        self
    }
    pub fn frame_id(mut self, frame_id: impl Into<super::super::page::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn card(mut self, card: impl Into<super::types::CreditCard>) -> Self {
        self.card = Some(card.into());
        self
    }
    pub fn address(mut self, address: impl Into<super::types::Address>) -> Self {
        self.address = Some(address.into());
        self
    }
    pub fn build(self) -> Result<Trigger, String> {
        Ok(Trigger {
            method: TriggerMethod::Trigger,
            params: TriggerParams {
                field_id: self.field_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(field_id))
                })?,
                frame_id: self.frame_id,
                card: self.card,
                address: self.address,
            },
        })
    }
}
impl SetAddresses {
    pub fn builder() -> SetAddressesBuilder {
        SetAddressesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetAddressesBuilder {
    addresses: Option<Vec<super::types::Address>>,
}
impl SetAddressesBuilder {
    pub fn addresse(mut self, addresse: impl Into<super::types::Address>) -> Self {
        let v = self.addresses.get_or_insert(Vec::new());
        v.push(addresse.into());
        self
    }
    pub fn addresses<I, S>(mut self, addresses: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::Address>,
    {
        let v = self.addresses.get_or_insert(Vec::new());
        for val in addresses {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetAddresses, String> {
        Ok(SetAddresses {
            method: SetAddressesMethod::SetAddresses,
            params: SetAddressesParams {
                addresses: self.addresses.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(addresses))
                })?,
            },
        })
    }
}
