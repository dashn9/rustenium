use super::types::*;
impl CreditCard {
    pub fn builder() -> CreditCardBuilder {
        <CreditCardBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CreditCardBuilder {
    number: Option<String>,
    name: Option<String>,
    expiry_month: Option<String>,
    expiry_year: Option<String>,
    cvc: Option<String>,
}
impl CreditCardBuilder {
    pub fn number(mut self, number: impl Into<String>) -> Self {
        self.number = Some(number.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn expiry_month(mut self, expiry_month: impl Into<String>) -> Self {
        self.expiry_month = Some(expiry_month.into());
        self
    }
    pub fn expiry_year(mut self, expiry_year: impl Into<String>) -> Self {
        self.expiry_year = Some(expiry_year.into());
        self
    }
    pub fn cvc(mut self, cvc: impl Into<String>) -> Self {
        self.cvc = Some(cvc.into());
        self
    }
    pub fn build(self) -> Result<CreditCard, String> {
        Ok(CreditCard {
            number: self
                .number
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(number)))?,
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            expiry_month: self.expiry_month.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(expiry_month))
            })?,
            expiry_year: self
                .expiry_year
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(expiry_year)))?,
            cvc: self
                .cvc
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cvc)))?,
        })
    }
}
impl AddressField {
    pub fn builder() -> AddressFieldBuilder {
        <AddressFieldBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AddressFieldBuilder {
    name: Option<String>,
    value: Option<String>,
}
impl AddressFieldBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<AddressField, String> {
        Ok(AddressField {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl AddressFields {
    pub fn builder() -> AddressFieldsBuilder {
        <AddressFieldsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AddressFieldsBuilder {
    fields: Option<Vec<AddressField>>,
}
impl AddressFieldsBuilder {
    pub fn field(mut self, field: impl Into<AddressField>) -> Self {
        let v = self.fields.get_or_insert(Vec::new());
        v.push(field.into());
        self
    }
    pub fn fields<I, S>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AddressField>,
    {
        let v = self.fields.get_or_insert(Vec::new());
        for val in fields {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<AddressFields, String> {
        Ok(AddressFields {
            fields: self
                .fields
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(fields)))?,
        })
    }
}
impl Address {
    pub fn builder() -> AddressBuilder {
        <AddressBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AddressBuilder {
    fields: Option<Vec<AddressField>>,
}
impl AddressBuilder {
    pub fn field(mut self, field: impl Into<AddressField>) -> Self {
        let v = self.fields.get_or_insert(Vec::new());
        v.push(field.into());
        self
    }
    pub fn fields<I, S>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AddressField>,
    {
        let v = self.fields.get_or_insert(Vec::new());
        for val in fields {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<Address, String> {
        Ok(Address {
            fields: self
                .fields
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(fields)))?,
        })
    }
}
impl AddressUi {
    pub fn builder() -> AddressUiBuilder {
        <AddressUiBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AddressUiBuilder {
    address_fields: Option<Vec<AddressFields>>,
}
impl AddressUiBuilder {
    pub fn address_field(mut self, address_field: impl Into<AddressFields>) -> Self {
        let v = self.address_fields.get_or_insert(Vec::new());
        v.push(address_field.into());
        self
    }
    pub fn address_fields<I, S>(mut self, address_fields: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AddressFields>,
    {
        let v = self.address_fields.get_or_insert(Vec::new());
        for val in address_fields {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<AddressUi, String> {
        Ok(AddressUi {
            address_fields: self.address_fields.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(address_fields))
            })?,
        })
    }
}
impl FilledField {
    pub fn builder() -> FilledFieldBuilder {
        <FilledFieldBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct FilledFieldBuilder {
    html_type: Option<String>,
    id: Option<String>,
    name: Option<String>,
    value: Option<String>,
    autofill_type: Option<String>,
    filling_strategy: Option<FillingStrategy>,
    frame_id: Option<crate::browser_protocol::page::types::FrameId>,
    field_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
}
impl FilledFieldBuilder {
    pub fn html_type(mut self, html_type: impl Into<String>) -> Self {
        self.html_type = Some(html_type.into());
        self
    }
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn autofill_type(mut self, autofill_type: impl Into<String>) -> Self {
        self.autofill_type = Some(autofill_type.into());
        self
    }
    pub fn filling_strategy(mut self, filling_strategy: impl Into<FillingStrategy>) -> Self {
        self.filling_strategy = Some(filling_strategy.into());
        self
    }
    pub fn frame_id(
        mut self,
        frame_id: impl Into<crate::browser_protocol::page::types::FrameId>,
    ) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn field_id(
        mut self,
        field_id: impl Into<crate::browser_protocol::dom::types::BackendNodeId>,
    ) -> Self {
        self.field_id = Some(field_id.into());
        self
    }
    pub fn build(self) -> Result<FilledField, String> {
        Ok(FilledField {
            html_type: self
                .html_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(html_type)))?,
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            autofill_type: self.autofill_type.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(autofill_type))
            })?,
            filling_strategy: self.filling_strategy.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(filling_strategy)
                )
            })?,
            frame_id: self
                .frame_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(frame_id)))?,
            field_id: self
                .field_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(field_id)))?,
        })
    }
}
