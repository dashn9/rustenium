use super::types::*;
impl ChannelValue {
    pub fn builder() -> ChannelValueBuilder {
        <ChannelValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ChannelValueBuilder {
    r#type: Option<ChannelValueType>,
    value: Option<ChannelProperties>,
}
impl ChannelValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<ChannelValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<ChannelProperties>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<ChannelValue, String> {
        Ok(ChannelValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl ChannelProperties {
    pub fn builder() -> ChannelPropertiesBuilder {
        <ChannelPropertiesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ChannelPropertiesBuilder {
    channel: Option<Channel>,
    serialization_options: Option<SerializationOptions>,
    ownership: Option<ResultOwnership>,
}
impl ChannelPropertiesBuilder {
    pub fn channel(mut self, channel: impl Into<Channel>) -> Self {
        self.channel = Some(channel.into());
        self
    }
    pub fn serialization_options(
        mut self,
        serialization_options: impl Into<SerializationOptions>,
    ) -> Self {
        self.serialization_options = Some(serialization_options.into());
        self
    }
    pub fn ownership(mut self, ownership: impl Into<ResultOwnership>) -> Self {
        self.ownership = Some(ownership.into());
        self
    }
    pub fn build(self) -> Result<ChannelProperties, String> {
        Ok(ChannelProperties {
            channel: self
                .channel
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(channel)))?,
            serialization_options: self.serialization_options,
            ownership: self.ownership,
        })
    }
}
impl EvaluateResultSuccess {
    pub fn builder() -> EvaluateResultSuccessBuilder {
        <EvaluateResultSuccessBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct EvaluateResultSuccessBuilder {
    r#type: Option<EvaluateResultSuccessType>,
    result: Option<RemoteValue>,
    realm: Option<Realm>,
}
impl EvaluateResultSuccessBuilder {
    pub fn r#type(mut self, r#type: impl Into<EvaluateResultSuccessType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn result(mut self, result: impl Into<RemoteValue>) -> Self {
        self.result = Some(result.into());
        self
    }
    pub fn realm(mut self, realm: impl Into<Realm>) -> Self {
        self.realm = Some(realm.into());
        self
    }
    pub fn build(self) -> Result<EvaluateResultSuccess, String> {
        Ok(EvaluateResultSuccess {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            result: self
                .result
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(result)))?,
            realm: self
                .realm
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(realm)))?,
        })
    }
}
impl EvaluateResultException {
    pub fn builder() -> EvaluateResultExceptionBuilder {
        <EvaluateResultExceptionBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct EvaluateResultExceptionBuilder {
    r#type: Option<EvaluateResultExceptionType>,
    exception_details: Option<ExceptionDetails>,
    realm: Option<Realm>,
}
impl EvaluateResultExceptionBuilder {
    pub fn r#type(mut self, r#type: impl Into<EvaluateResultExceptionType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn exception_details(mut self, exception_details: impl Into<ExceptionDetails>) -> Self {
        self.exception_details = Some(exception_details.into());
        self
    }
    pub fn realm(mut self, realm: impl Into<Realm>) -> Self {
        self.realm = Some(realm.into());
        self
    }
    pub fn build(self) -> Result<EvaluateResultException, String> {
        Ok(EvaluateResultException {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            exception_details: self.exception_details.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(exception_details)
                )
            })?,
            realm: self
                .realm
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(realm)))?,
        })
    }
}
impl ExceptionDetails {
    pub fn builder() -> ExceptionDetailsBuilder {
        <ExceptionDetailsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ExceptionDetailsBuilder {
    column_number: Option<u64>,
    exception: Option<RemoteValue>,
    line_number: Option<u64>,
    stack_trace: Option<StackTrace>,
    text: Option<String>,
}
impl ExceptionDetailsBuilder {
    pub fn column_number(mut self, column_number: impl Into<u64>) -> Self {
        self.column_number = Some(column_number.into());
        self
    }
    pub fn exception(mut self, exception: impl Into<RemoteValue>) -> Self {
        self.exception = Some(exception.into());
        self
    }
    pub fn line_number(mut self, line_number: impl Into<u64>) -> Self {
        self.line_number = Some(line_number.into());
        self
    }
    pub fn stack_trace(mut self, stack_trace: impl Into<StackTrace>) -> Self {
        self.stack_trace = Some(stack_trace.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn build(self) -> Result<ExceptionDetails, String> {
        Ok(ExceptionDetails {
            column_number: self.column_number.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(column_number))
            })?,
            exception: self
                .exception
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(exception)))?,
            line_number: self
                .line_number
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(line_number)))?,
            stack_trace: self
                .stack_trace
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(stack_trace)))?,
            text: self
                .text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
        })
    }
}
impl ArrayLocalValue {
    pub fn builder() -> ArrayLocalValueBuilder {
        <ArrayLocalValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ArrayLocalValueBuilder {
    r#type: Option<ArrayLocalValueType>,
    value: Option<ListLocalValue>,
}
impl ArrayLocalValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<ArrayLocalValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<ListLocalValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<ArrayLocalValue, String> {
        Ok(ArrayLocalValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl DateLocalValue {
    pub fn builder() -> DateLocalValueBuilder {
        <DateLocalValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DateLocalValueBuilder {
    r#type: Option<DateLocalValueType>,
    value: Option<String>,
}
impl DateLocalValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<DateLocalValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<DateLocalValue, String> {
        Ok(DateLocalValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl MapLocalValue {
    pub fn builder() -> MapLocalValueBuilder {
        <MapLocalValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct MapLocalValueBuilder {
    r#type: Option<MapLocalValueType>,
    value: Option<MappingLocalValue>,
}
impl MapLocalValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<MapLocalValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<MappingLocalValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<MapLocalValue, String> {
        Ok(MapLocalValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl ObjectLocalValue {
    pub fn builder() -> ObjectLocalValueBuilder {
        <ObjectLocalValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ObjectLocalValueBuilder {
    r#type: Option<ObjectLocalValueType>,
    value: Option<MappingLocalValue>,
}
impl ObjectLocalValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<ObjectLocalValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<MappingLocalValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<ObjectLocalValue, String> {
        Ok(ObjectLocalValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl RegExpValue {
    pub fn builder() -> RegExpValueBuilder {
        <RegExpValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RegExpValueBuilder {
    pattern: Option<String>,
    flags: Option<String>,
}
impl RegExpValueBuilder {
    pub fn pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }
    pub fn flags(mut self, flags: impl Into<String>) -> Self {
        self.flags = Some(flags.into());
        self
    }
    pub fn build(self) -> Result<RegExpValue, String> {
        Ok(RegExpValue {
            pattern: self
                .pattern
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(pattern)))?,
            flags: self.flags,
        })
    }
}
impl RegExpLocalValue {
    pub fn builder() -> RegExpLocalValueBuilder {
        <RegExpLocalValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RegExpLocalValueBuilder {
    r#type: Option<RegExpLocalValueType>,
    value: Option<RegExpValue>,
}
impl RegExpLocalValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<RegExpLocalValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<RegExpValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<RegExpLocalValue, String> {
        Ok(RegExpLocalValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl SetLocalValue {
    pub fn builder() -> SetLocalValueBuilder {
        <SetLocalValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetLocalValueBuilder {
    r#type: Option<SetLocalValueType>,
    value: Option<ListLocalValue>,
}
impl SetLocalValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<SetLocalValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<ListLocalValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<SetLocalValue, String> {
        Ok(SetLocalValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl UndefinedValue {
    pub fn builder() -> UndefinedValueBuilder {
        <UndefinedValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UndefinedValueBuilder {
    r#type: Option<UndefinedValueType>,
}
impl UndefinedValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<UndefinedValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<UndefinedValue, String> {
        Ok(UndefinedValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl NullValue {
    pub fn builder() -> NullValueBuilder {
        <NullValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct NullValueBuilder {
    r#type: Option<NullValueType>,
}
impl NullValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<NullValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<NullValue, String> {
        Ok(NullValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl StringValue {
    pub fn builder() -> StringValueBuilder {
        <StringValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StringValueBuilder {
    r#type: Option<StringValueType>,
    value: Option<String>,
}
impl StringValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<StringValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<StringValue, String> {
        Ok(StringValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl NumberValue {
    pub fn builder() -> NumberValueBuilder {
        <NumberValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct NumberValueBuilder {
    r#type: Option<NumberValueType>,
    value: Option<serde_json::Value>,
}
impl NumberValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<NumberValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<serde_json::Value>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<NumberValue, String> {
        Ok(NumberValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl BooleanValue {
    pub fn builder() -> BooleanValueBuilder {
        <BooleanValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BooleanValueBuilder {
    r#type: Option<BooleanValueType>,
    value: Option<bool>,
}
impl BooleanValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<BooleanValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<bool>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<BooleanValue, String> {
        Ok(BooleanValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl BigIntValue {
    pub fn builder() -> BigIntValueBuilder {
        <BigIntValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BigIntValueBuilder {
    r#type: Option<BigIntValueType>,
    value: Option<String>,
}
impl BigIntValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<BigIntValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<BigIntValue, String> {
        Ok(BigIntValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl SharedReference {
    pub fn builder() -> SharedReferenceBuilder {
        <SharedReferenceBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SharedReferenceBuilder {
    shared_id: Option<SharedId>,
    handle: Option<Handle>,
    extensible: Option<std::collections::HashMap<String, serde_json::Value>>,
}
impl SharedReferenceBuilder {
    pub fn shared_id(mut self, shared_id: impl Into<SharedId>) -> Self {
        self.shared_id = Some(shared_id.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn extensible(
        mut self,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.extensible = Some(extensible.into());
        self
    }
    pub fn build(self) -> Result<SharedReference, String> {
        Ok(SharedReference {
            shared_id: self
                .shared_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(shared_id)))?,
            handle: self.handle,
            extensible: self.extensible.unwrap_or_default(),
        })
    }
}
impl RemoteObjectReference {
    pub fn builder() -> RemoteObjectReferenceBuilder {
        <RemoteObjectReferenceBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoteObjectReferenceBuilder {
    handle: Option<Handle>,
    shared_id: Option<SharedId>,
    extensible: Option<std::collections::HashMap<String, serde_json::Value>>,
}
impl RemoteObjectReferenceBuilder {
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn shared_id(mut self, shared_id: impl Into<SharedId>) -> Self {
        self.shared_id = Some(shared_id.into());
        self
    }
    pub fn extensible(
        mut self,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.extensible = Some(extensible.into());
        self
    }
    pub fn build(self) -> Result<RemoteObjectReference, String> {
        Ok(RemoteObjectReference {
            handle: self
                .handle
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(handle)))?,
            shared_id: self.shared_id,
            extensible: self.extensible.unwrap_or_default(),
        })
    }
}
impl SymbolRemoteValue {
    pub fn builder() -> SymbolRemoteValueBuilder {
        <SymbolRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SymbolRemoteValueBuilder {
    r#type: Option<SymbolRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
}
impl SymbolRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<SymbolRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn build(self) -> Result<SymbolRemoteValue, String> {
        Ok(SymbolRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
        })
    }
}
impl ArrayRemoteValue {
    pub fn builder() -> ArrayRemoteValueBuilder {
        <ArrayRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ArrayRemoteValueBuilder {
    r#type: Option<ArrayRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
    value: Option<ListRemoteValue>,
}
impl ArrayRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<ArrayRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn value(mut self, value: impl Into<ListRemoteValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<ArrayRemoteValue, String> {
        Ok(ArrayRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
            value: self.value,
        })
    }
}
impl ObjectRemoteValue {
    pub fn builder() -> ObjectRemoteValueBuilder {
        <ObjectRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ObjectRemoteValueBuilder {
    r#type: Option<ObjectRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
    value: Option<MappingRemoteValue>,
}
impl ObjectRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<ObjectRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn value(mut self, value: impl Into<MappingRemoteValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<ObjectRemoteValue, String> {
        Ok(ObjectRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
            value: self.value,
        })
    }
}
impl FunctionRemoteValue {
    pub fn builder() -> FunctionRemoteValueBuilder {
        <FunctionRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct FunctionRemoteValueBuilder {
    r#type: Option<FunctionRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
}
impl FunctionRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<FunctionRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn build(self) -> Result<FunctionRemoteValue, String> {
        Ok(FunctionRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
        })
    }
}
impl RegExpRemoteValue {
    pub fn builder() -> RegExpRemoteValueBuilder {
        <RegExpRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RegExpRemoteValueBuilder {
    reg_exp_local_value: Option<RegExpLocalValue>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
}
impl RegExpRemoteValueBuilder {
    pub fn reg_exp_local_value(mut self, reg_exp_local_value: impl Into<RegExpLocalValue>) -> Self {
        self.reg_exp_local_value = Some(reg_exp_local_value.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn build(self) -> Result<RegExpRemoteValue, String> {
        Ok(RegExpRemoteValue {
            reg_exp_local_value: self.reg_exp_local_value.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(reg_exp_local_value)
                )
            })?,
            handle: self.handle,
            internal_id: self.internal_id,
        })
    }
}
impl DateRemoteValue {
    pub fn builder() -> DateRemoteValueBuilder {
        <DateRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DateRemoteValueBuilder {
    date_local_value: Option<DateLocalValue>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
}
impl DateRemoteValueBuilder {
    pub fn date_local_value(mut self, date_local_value: impl Into<DateLocalValue>) -> Self {
        self.date_local_value = Some(date_local_value.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn build(self) -> Result<DateRemoteValue, String> {
        Ok(DateRemoteValue {
            date_local_value: self.date_local_value.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(date_local_value)
                )
            })?,
            handle: self.handle,
            internal_id: self.internal_id,
        })
    }
}
impl MapRemoteValue {
    pub fn builder() -> MapRemoteValueBuilder {
        <MapRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct MapRemoteValueBuilder {
    r#type: Option<MapRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
    value: Option<MappingRemoteValue>,
}
impl MapRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<MapRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn value(mut self, value: impl Into<MappingRemoteValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<MapRemoteValue, String> {
        Ok(MapRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
            value: self.value,
        })
    }
}
impl SetRemoteValue {
    pub fn builder() -> SetRemoteValueBuilder {
        <SetRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetRemoteValueBuilder {
    r#type: Option<SetRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
    value: Option<ListRemoteValue>,
}
impl SetRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<SetRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn value(mut self, value: impl Into<ListRemoteValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<SetRemoteValue, String> {
        Ok(SetRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
            value: self.value,
        })
    }
}
impl WeakMapRemoteValue {
    pub fn builder() -> WeakMapRemoteValueBuilder {
        <WeakMapRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct WeakMapRemoteValueBuilder {
    r#type: Option<WeakMapRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
}
impl WeakMapRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<WeakMapRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn build(self) -> Result<WeakMapRemoteValue, String> {
        Ok(WeakMapRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
        })
    }
}
impl WeakSetRemoteValue {
    pub fn builder() -> WeakSetRemoteValueBuilder {
        <WeakSetRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct WeakSetRemoteValueBuilder {
    r#type: Option<WeakSetRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
}
impl WeakSetRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<WeakSetRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn build(self) -> Result<WeakSetRemoteValue, String> {
        Ok(WeakSetRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
        })
    }
}
impl GeneratorRemoteValue {
    pub fn builder() -> GeneratorRemoteValueBuilder {
        <GeneratorRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GeneratorRemoteValueBuilder {
    r#type: Option<GeneratorRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
}
impl GeneratorRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<GeneratorRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn build(self) -> Result<GeneratorRemoteValue, String> {
        Ok(GeneratorRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
        })
    }
}
impl ErrorRemoteValue {
    pub fn builder() -> ErrorRemoteValueBuilder {
        <ErrorRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ErrorRemoteValueBuilder {
    r#type: Option<ErrorRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
}
impl ErrorRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<ErrorRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn build(self) -> Result<ErrorRemoteValue, String> {
        Ok(ErrorRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
        })
    }
}
impl ProxyRemoteValue {
    pub fn builder() -> ProxyRemoteValueBuilder {
        <ProxyRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ProxyRemoteValueBuilder {
    r#type: Option<ProxyRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
}
impl ProxyRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<ProxyRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn build(self) -> Result<ProxyRemoteValue, String> {
        Ok(ProxyRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
        })
    }
}
impl PromiseRemoteValue {
    pub fn builder() -> PromiseRemoteValueBuilder {
        <PromiseRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PromiseRemoteValueBuilder {
    r#type: Option<PromiseRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
}
impl PromiseRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<PromiseRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn build(self) -> Result<PromiseRemoteValue, String> {
        Ok(PromiseRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
        })
    }
}
impl TypedArrayRemoteValue {
    pub fn builder() -> TypedArrayRemoteValueBuilder {
        <TypedArrayRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct TypedArrayRemoteValueBuilder {
    r#type: Option<TypedArrayRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
}
impl TypedArrayRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<TypedArrayRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn build(self) -> Result<TypedArrayRemoteValue, String> {
        Ok(TypedArrayRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
        })
    }
}
impl ArrayBufferRemoteValue {
    pub fn builder() -> ArrayBufferRemoteValueBuilder {
        <ArrayBufferRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ArrayBufferRemoteValueBuilder {
    r#type: Option<ArrayBufferRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
}
impl ArrayBufferRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<ArrayBufferRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn build(self) -> Result<ArrayBufferRemoteValue, String> {
        Ok(ArrayBufferRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
        })
    }
}
impl NodeListRemoteValue {
    pub fn builder() -> NodeListRemoteValueBuilder {
        <NodeListRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct NodeListRemoteValueBuilder {
    r#type: Option<NodeListRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
    value: Option<ListRemoteValue>,
}
impl NodeListRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<NodeListRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn value(mut self, value: impl Into<ListRemoteValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<NodeListRemoteValue, String> {
        Ok(NodeListRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
            value: self.value,
        })
    }
}
impl HtmlCollectionRemoteValue {
    pub fn builder() -> HtmlCollectionRemoteValueBuilder {
        <HtmlCollectionRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct HtmlCollectionRemoteValueBuilder {
    r#type: Option<HtmlCollectionRemoteValueType>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
    value: Option<ListRemoteValue>,
}
impl HtmlCollectionRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<HtmlCollectionRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn value(mut self, value: impl Into<ListRemoteValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<HtmlCollectionRemoteValue, String> {
        Ok(HtmlCollectionRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            handle: self.handle,
            internal_id: self.internal_id,
            value: self.value,
        })
    }
}
impl NodeRemoteValue {
    pub fn builder() -> NodeRemoteValueBuilder {
        <NodeRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct NodeRemoteValueBuilder {
    r#type: Option<NodeRemoteValueType>,
    shared_id: Option<SharedId>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
    value: Option<NodeProperties>,
}
impl NodeRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<NodeRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn shared_id(mut self, shared_id: impl Into<SharedId>) -> Self {
        self.shared_id = Some(shared_id.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn value(mut self, value: impl Into<NodeProperties>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<NodeRemoteValue, String> {
        Ok(NodeRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            shared_id: self.shared_id,
            handle: self.handle,
            internal_id: self.internal_id,
            value: self.value.map(Box::new),
        })
    }
}
impl NodeProperties {
    pub fn builder() -> NodePropertiesBuilder {
        <NodePropertiesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct NodePropertiesBuilder {
    node_type: Option<u64>,
    child_node_count: Option<u64>,
    attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    children: Option<Vec<NodeRemoteValue>>,
    local_name: Option<String>,
    mode: Option<NodePropertiesMode>,
    namespace_uri: Option<String>,
    node_value: Option<String>,
    shadow_root: Option<NodeRemoteValue>,
}
impl NodePropertiesBuilder {
    pub fn node_type(mut self, node_type: impl Into<u64>) -> Self {
        self.node_type = Some(node_type.into());
        self
    }
    pub fn child_node_count(mut self, child_node_count: impl Into<u64>) -> Self {
        self.child_node_count = Some(child_node_count.into());
        self
    }
    pub fn attributes(
        mut self,
        attributes: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.attributes = Some(attributes.into());
        self
    }
    pub fn children(mut self, children: impl Into<NodeRemoteValue>) -> Self {
        let v = self.children.get_or_insert(Vec::new());
        v.push(children.into());
        self
    }
    pub fn childrens<I, S>(mut self, childrens: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<NodeRemoteValue>,
    {
        let v = self.children.get_or_insert(Vec::new());
        for val in childrens {
            v.push(val.into());
        }
        self
    }
    pub fn local_name(mut self, local_name: impl Into<String>) -> Self {
        self.local_name = Some(local_name.into());
        self
    }
    pub fn mode(mut self, mode: impl Into<NodePropertiesMode>) -> Self {
        self.mode = Some(mode.into());
        self
    }
    pub fn namespace_uri(mut self, namespace_uri: impl Into<String>) -> Self {
        self.namespace_uri = Some(namespace_uri.into());
        self
    }
    pub fn node_value(mut self, node_value: impl Into<String>) -> Self {
        self.node_value = Some(node_value.into());
        self
    }
    pub fn shadow_root(mut self, shadow_root: impl Into<NodeRemoteValue>) -> Self {
        self.shadow_root = Some(shadow_root.into());
        self
    }
    pub fn build(self) -> Result<NodeProperties, String> {
        Ok(NodeProperties {
            node_type: self
                .node_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_type)))?,
            child_node_count: self.child_node_count.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(child_node_count)
                )
            })?,
            attributes: self.attributes,
            children: self.children,
            local_name: self.local_name,
            mode: self.mode,
            namespace_uri: self.namespace_uri,
            node_value: self.node_value,
            shadow_root: self.shadow_root.map(Box::new),
        })
    }
}
impl WindowProxyRemoteValue {
    pub fn builder() -> WindowProxyRemoteValueBuilder {
        <WindowProxyRemoteValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct WindowProxyRemoteValueBuilder {
    r#type: Option<WindowProxyRemoteValueType>,
    value: Option<WindowProxyProperties>,
    handle: Option<Handle>,
    internal_id: Option<InternalId>,
}
impl WindowProxyRemoteValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<WindowProxyRemoteValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<WindowProxyProperties>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn handle(mut self, handle: impl Into<Handle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn internal_id(mut self, internal_id: impl Into<InternalId>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }
    pub fn build(self) -> Result<WindowProxyRemoteValue, String> {
        Ok(WindowProxyRemoteValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            handle: self.handle,
            internal_id: self.internal_id,
        })
    }
}
impl WindowProxyProperties {
    pub fn builder() -> WindowProxyPropertiesBuilder {
        <WindowProxyPropertiesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct WindowProxyPropertiesBuilder {
    context: Option<crate::browsing_context::types::BrowsingContext>,
}
impl WindowProxyPropertiesBuilder {
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn build(self) -> Result<WindowProxyProperties, String> {
        Ok(WindowProxyProperties {
            context: self
                .context
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
        })
    }
}
impl SerializationOptions {
    pub fn builder() -> SerializationOptionsBuilder {
        <SerializationOptionsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SerializationOptionsBuilder {
    max_dom_depth: Option<u64>,
    max_object_depth: Option<u64>,
    include_shadow_tree: Option<SerializationOptionsIncludeShadowTree>,
}
impl SerializationOptionsBuilder {
    pub fn max_dom_depth(mut self, max_dom_depth: impl Into<u64>) -> Self {
        self.max_dom_depth = Some(max_dom_depth.into());
        self
    }
    pub fn max_object_depth(mut self, max_object_depth: impl Into<u64>) -> Self {
        self.max_object_depth = Some(max_object_depth.into());
        self
    }
    pub fn include_shadow_tree(
        mut self,
        include_shadow_tree: impl Into<SerializationOptionsIncludeShadowTree>,
    ) -> Self {
        self.include_shadow_tree = Some(include_shadow_tree.into());
        self
    }
    pub fn build(self) -> SerializationOptions {
        SerializationOptions {
            max_dom_depth: self.max_dom_depth,
            max_object_depth: self.max_object_depth,
            include_shadow_tree: self.include_shadow_tree,
        }
    }
}
impl StackFrame {
    pub fn builder() -> StackFrameBuilder {
        <StackFrameBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StackFrameBuilder {
    column_number: Option<u64>,
    function_name: Option<String>,
    line_number: Option<u64>,
    url: Option<String>,
}
impl StackFrameBuilder {
    pub fn column_number(mut self, column_number: impl Into<u64>) -> Self {
        self.column_number = Some(column_number.into());
        self
    }
    pub fn function_name(mut self, function_name: impl Into<String>) -> Self {
        self.function_name = Some(function_name.into());
        self
    }
    pub fn line_number(mut self, line_number: impl Into<u64>) -> Self {
        self.line_number = Some(line_number.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn build(self) -> Result<StackFrame, String> {
        Ok(StackFrame {
            column_number: self.column_number.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(column_number))
            })?,
            function_name: self.function_name.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(function_name))
            })?,
            line_number: self
                .line_number
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(line_number)))?,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
        })
    }
}
impl StackTrace {
    pub fn builder() -> StackTraceBuilder {
        <StackTraceBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StackTraceBuilder {
    call_frames: Option<Vec<StackFrame>>,
}
impl StackTraceBuilder {
    pub fn call_frame(mut self, call_frame: impl Into<StackFrame>) -> Self {
        let v = self.call_frames.get_or_insert(Vec::new());
        v.push(call_frame.into());
        self
    }
    pub fn call_frames<I, S>(mut self, call_frames: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<StackFrame>,
    {
        let v = self.call_frames.get_or_insert(Vec::new());
        for val in call_frames {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<StackTrace, String> {
        Ok(StackTrace {
            call_frames: self
                .call_frames
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(call_frames)))?,
        })
    }
}
impl RealmTarget {
    pub fn builder() -> RealmTargetBuilder {
        <RealmTargetBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RealmTargetBuilder {
    realm: Option<Realm>,
}
impl RealmTargetBuilder {
    pub fn realm(mut self, realm: impl Into<Realm>) -> Self {
        self.realm = Some(realm.into());
        self
    }
    pub fn build(self) -> Result<RealmTarget, String> {
        Ok(RealmTarget {
            realm: self
                .realm
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(realm)))?,
        })
    }
}
impl ContextTarget {
    pub fn builder() -> ContextTargetBuilder {
        <ContextTargetBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ContextTargetBuilder {
    context: Option<crate::browsing_context::types::BrowsingContext>,
    sandbox: Option<String>,
}
impl ContextTargetBuilder {
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn sandbox(mut self, sandbox: impl Into<String>) -> Self {
        self.sandbox = Some(sandbox.into());
        self
    }
    pub fn build(self) -> Result<ContextTarget, String> {
        Ok(ContextTarget {
            context: self
                .context
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
            sandbox: self.sandbox,
        })
    }
}
impl BaseRealmInfo {
    pub fn builder() -> BaseRealmInfoBuilder {
        <BaseRealmInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BaseRealmInfoBuilder {
    realm: Option<Realm>,
    origin: Option<String>,
}
impl BaseRealmInfoBuilder {
    pub fn realm(mut self, realm: impl Into<Realm>) -> Self {
        self.realm = Some(realm.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn build(self) -> Result<BaseRealmInfo, String> {
        Ok(BaseRealmInfo {
            realm: self
                .realm
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(realm)))?,
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
        })
    }
}
impl WindowRealmInfo {
    pub fn builder() -> WindowRealmInfoBuilder {
        <WindowRealmInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct WindowRealmInfoBuilder {
    base_realm_info: Option<BaseRealmInfo>,
    r#type: Option<WindowRealmInfoType>,
    context: Option<crate::browsing_context::types::BrowsingContext>,
    sandbox: Option<String>,
}
impl WindowRealmInfoBuilder {
    pub fn base_realm_info(mut self, base_realm_info: impl Into<BaseRealmInfo>) -> Self {
        self.base_realm_info = Some(base_realm_info.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<WindowRealmInfoType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn sandbox(mut self, sandbox: impl Into<String>) -> Self {
        self.sandbox = Some(sandbox.into());
        self
    }
    pub fn build(self) -> Result<WindowRealmInfo, String> {
        Ok(WindowRealmInfo {
            base_realm_info: self.base_realm_info.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(base_realm_info))
            })?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            context: self
                .context
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
            sandbox: self.sandbox,
        })
    }
}
impl DedicatedWorkerRealmInfo {
    pub fn builder() -> DedicatedWorkerRealmInfoBuilder {
        <DedicatedWorkerRealmInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DedicatedWorkerRealmInfoBuilder {
    base_realm_info: Option<BaseRealmInfo>,
    r#type: Option<DedicatedWorkerRealmInfoType>,
    owners: Option<Vec<Realm>>,
}
impl DedicatedWorkerRealmInfoBuilder {
    pub fn base_realm_info(mut self, base_realm_info: impl Into<BaseRealmInfo>) -> Self {
        self.base_realm_info = Some(base_realm_info.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<DedicatedWorkerRealmInfoType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn owner(mut self, owner: impl Into<Realm>) -> Self {
        let v = self.owners.get_or_insert(Vec::new());
        v.push(owner.into());
        self
    }
    pub fn owners<I, S>(mut self, owners: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Realm>,
    {
        let v = self.owners.get_or_insert(Vec::new());
        for val in owners {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<DedicatedWorkerRealmInfo, String> {
        Ok(DedicatedWorkerRealmInfo {
            base_realm_info: self.base_realm_info.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(base_realm_info))
            })?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            owners: self
                .owners
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(owners)))?,
        })
    }
}
impl SharedWorkerRealmInfo {
    pub fn builder() -> SharedWorkerRealmInfoBuilder {
        <SharedWorkerRealmInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SharedWorkerRealmInfoBuilder {
    base_realm_info: Option<BaseRealmInfo>,
    r#type: Option<SharedWorkerRealmInfoType>,
}
impl SharedWorkerRealmInfoBuilder {
    pub fn base_realm_info(mut self, base_realm_info: impl Into<BaseRealmInfo>) -> Self {
        self.base_realm_info = Some(base_realm_info.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<SharedWorkerRealmInfoType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<SharedWorkerRealmInfo, String> {
        Ok(SharedWorkerRealmInfo {
            base_realm_info: self.base_realm_info.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(base_realm_info))
            })?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl ServiceWorkerRealmInfo {
    pub fn builder() -> ServiceWorkerRealmInfoBuilder {
        <ServiceWorkerRealmInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ServiceWorkerRealmInfoBuilder {
    base_realm_info: Option<BaseRealmInfo>,
    r#type: Option<ServiceWorkerRealmInfoType>,
}
impl ServiceWorkerRealmInfoBuilder {
    pub fn base_realm_info(mut self, base_realm_info: impl Into<BaseRealmInfo>) -> Self {
        self.base_realm_info = Some(base_realm_info.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<ServiceWorkerRealmInfoType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<ServiceWorkerRealmInfo, String> {
        Ok(ServiceWorkerRealmInfo {
            base_realm_info: self.base_realm_info.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(base_realm_info))
            })?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl WorkerRealmInfo {
    pub fn builder() -> WorkerRealmInfoBuilder {
        <WorkerRealmInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct WorkerRealmInfoBuilder {
    base_realm_info: Option<BaseRealmInfo>,
    r#type: Option<WorkerRealmInfoType>,
}
impl WorkerRealmInfoBuilder {
    pub fn base_realm_info(mut self, base_realm_info: impl Into<BaseRealmInfo>) -> Self {
        self.base_realm_info = Some(base_realm_info.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<WorkerRealmInfoType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<WorkerRealmInfo, String> {
        Ok(WorkerRealmInfo {
            base_realm_info: self.base_realm_info.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(base_realm_info))
            })?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl PaintWorkletRealmInfo {
    pub fn builder() -> PaintWorkletRealmInfoBuilder {
        <PaintWorkletRealmInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PaintWorkletRealmInfoBuilder {
    base_realm_info: Option<BaseRealmInfo>,
    r#type: Option<PaintWorkletRealmInfoType>,
}
impl PaintWorkletRealmInfoBuilder {
    pub fn base_realm_info(mut self, base_realm_info: impl Into<BaseRealmInfo>) -> Self {
        self.base_realm_info = Some(base_realm_info.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<PaintWorkletRealmInfoType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<PaintWorkletRealmInfo, String> {
        Ok(PaintWorkletRealmInfo {
            base_realm_info: self.base_realm_info.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(base_realm_info))
            })?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl AudioWorkletRealmInfo {
    pub fn builder() -> AudioWorkletRealmInfoBuilder {
        <AudioWorkletRealmInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AudioWorkletRealmInfoBuilder {
    base_realm_info: Option<BaseRealmInfo>,
    r#type: Option<AudioWorkletRealmInfoType>,
}
impl AudioWorkletRealmInfoBuilder {
    pub fn base_realm_info(mut self, base_realm_info: impl Into<BaseRealmInfo>) -> Self {
        self.base_realm_info = Some(base_realm_info.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<AudioWorkletRealmInfoType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<AudioWorkletRealmInfo, String> {
        Ok(AudioWorkletRealmInfo {
            base_realm_info: self.base_realm_info.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(base_realm_info))
            })?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl WorkletRealmInfo {
    pub fn builder() -> WorkletRealmInfoBuilder {
        <WorkletRealmInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct WorkletRealmInfoBuilder {
    base_realm_info: Option<BaseRealmInfo>,
    r#type: Option<WorkletRealmInfoType>,
}
impl WorkletRealmInfoBuilder {
    pub fn base_realm_info(mut self, base_realm_info: impl Into<BaseRealmInfo>) -> Self {
        self.base_realm_info = Some(base_realm_info.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<WorkletRealmInfoType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<WorkletRealmInfo, String> {
        Ok(WorkletRealmInfo {
            base_realm_info: self.base_realm_info.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(base_realm_info))
            })?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl Source {
    pub fn builder() -> SourceBuilder {
        <SourceBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SourceBuilder {
    realm: Option<Realm>,
    context: Option<crate::browsing_context::types::BrowsingContext>,
}
impl SourceBuilder {
    pub fn realm(mut self, realm: impl Into<Realm>) -> Self {
        self.realm = Some(realm.into());
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn build(self) -> Result<Source, String> {
        Ok(Source {
            realm: self
                .realm
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(realm)))?,
            context: self.context,
        })
    }
}
