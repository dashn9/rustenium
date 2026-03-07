use super::types::*;
impl SerializationOptions {
    pub fn builder() -> SerializationOptionsBuilder {
        <SerializationOptionsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SerializationOptionsBuilder {
    serialization: Option<SerializationOptionsSerialization>,
    max_depth: Option<i64>,
    additional_parameters: Option<serde_json::Value>,
}
impl SerializationOptionsBuilder {
    pub fn serialization(
        mut self,
        serialization: impl Into<SerializationOptionsSerialization>,
    ) -> Self {
        self.serialization = Some(serialization.into());
        self
    }
    pub fn max_depth(mut self, max_depth: impl Into<i64>) -> Self {
        self.max_depth = Some(max_depth.into());
        self
    }
    pub fn additional_parameters(
        mut self,
        additional_parameters: impl Into<serde_json::Value>,
    ) -> Self {
        self.additional_parameters = Some(additional_parameters.into());
        self
    }
    pub fn build(self) -> Result<SerializationOptions, String> {
        Ok(SerializationOptions {
            serialization: self.serialization.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(serialization))
            })?,
            max_depth: self.max_depth,
            additional_parameters: self.additional_parameters,
        })
    }
}
impl DeepSerializedValue {
    pub fn builder() -> DeepSerializedValueBuilder {
        <DeepSerializedValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeepSerializedValueBuilder {
    r#type: Option<DeepSerializedValueType>,
    value: Option<serde_json::Value>,
    object_id: Option<String>,
    weak_local_object_reference: Option<i64>,
}
impl DeepSerializedValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<DeepSerializedValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<serde_json::Value>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn object_id(mut self, object_id: impl Into<String>) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn weak_local_object_reference(
        mut self,
        weak_local_object_reference: impl Into<i64>,
    ) -> Self {
        self.weak_local_object_reference = Some(weak_local_object_reference.into());
        self
    }
    pub fn build(self) -> Result<DeepSerializedValue, String> {
        Ok(DeepSerializedValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self.value,
            object_id: self.object_id,
            weak_local_object_reference: self.weak_local_object_reference,
        })
    }
}
impl RemoteObject {
    pub fn builder() -> RemoteObjectBuilder {
        <RemoteObjectBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoteObjectBuilder {
    r#type: Option<RemoteObjectType>,
    subtype: Option<RemoteObjectSubtype>,
    class_name: Option<String>,
    value: Option<serde_json::Value>,
    unserializable_value: Option<UnserializableValue>,
    description: Option<String>,
    deep_serialized_value: Option<DeepSerializedValue>,
    object_id: Option<RemoteObjectId>,
    preview: Option<ObjectPreview>,
    custom_preview: Option<CustomPreview>,
}
impl RemoteObjectBuilder {
    pub fn r#type(mut self, r#type: impl Into<RemoteObjectType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn subtype(mut self, subtype: impl Into<RemoteObjectSubtype>) -> Self {
        self.subtype = Some(subtype.into());
        self
    }
    pub fn class_name(mut self, class_name: impl Into<String>) -> Self {
        self.class_name = Some(class_name.into());
        self
    }
    pub fn value(mut self, value: impl Into<serde_json::Value>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn unserializable_value(
        mut self,
        unserializable_value: impl Into<UnserializableValue>,
    ) -> Self {
        self.unserializable_value = Some(unserializable_value.into());
        self
    }
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn deep_serialized_value(
        mut self,
        deep_serialized_value: impl Into<DeepSerializedValue>,
    ) -> Self {
        self.deep_serialized_value = Some(deep_serialized_value.into());
        self
    }
    pub fn object_id(mut self, object_id: impl Into<RemoteObjectId>) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn preview(mut self, preview: impl Into<ObjectPreview>) -> Self {
        self.preview = Some(preview.into());
        self
    }
    pub fn custom_preview(mut self, custom_preview: impl Into<CustomPreview>) -> Self {
        self.custom_preview = Some(custom_preview.into());
        self
    }
    pub fn build(self) -> Result<RemoteObject, String> {
        Ok(RemoteObject {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            subtype: self.subtype,
            class_name: self.class_name,
            value: self.value,
            unserializable_value: self.unserializable_value,
            description: self.description,
            deep_serialized_value: self.deep_serialized_value,
            object_id: self.object_id,
            preview: self.preview,
            custom_preview: self.custom_preview,
        })
    }
}
impl CustomPreview {
    pub fn builder() -> CustomPreviewBuilder {
        <CustomPreviewBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CustomPreviewBuilder {
    header: Option<String>,
    body_getter_id: Option<RemoteObjectId>,
}
impl CustomPreviewBuilder {
    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }
    pub fn body_getter_id(mut self, body_getter_id: impl Into<RemoteObjectId>) -> Self {
        self.body_getter_id = Some(body_getter_id.into());
        self
    }
    pub fn build(self) -> Result<CustomPreview, String> {
        Ok(CustomPreview {
            header: self
                .header
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(header)))?,
            body_getter_id: self.body_getter_id,
        })
    }
}
impl ObjectPreview {
    pub fn builder() -> ObjectPreviewBuilder {
        <ObjectPreviewBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ObjectPreviewBuilder {
    r#type: Option<ObjectPreviewType>,
    subtype: Option<ObjectPreviewSubtype>,
    description: Option<String>,
    overflow: Option<bool>,
    properties: Option<Vec<PropertyPreview>>,
    entries: Option<Vec<EntryPreview>>,
}
impl ObjectPreviewBuilder {
    pub fn r#type(mut self, r#type: impl Into<ObjectPreviewType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn subtype(mut self, subtype: impl Into<ObjectPreviewSubtype>) -> Self {
        self.subtype = Some(subtype.into());
        self
    }
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn overflow(mut self, overflow: impl Into<bool>) -> Self {
        self.overflow = Some(overflow.into());
        self
    }
    pub fn propertie(mut self, propertie: impl Into<PropertyPreview>) -> Self {
        let v = self.properties.get_or_insert(Vec::new());
        v.push(propertie.into());
        self
    }
    pub fn properties<I, S>(mut self, properties: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<PropertyPreview>,
    {
        let v = self.properties.get_or_insert(Vec::new());
        for val in properties {
            v.push(val.into());
        }
        self
    }
    pub fn entrie(mut self, entrie: impl Into<EntryPreview>) -> Self {
        let v = self.entries.get_or_insert(Vec::new());
        v.push(entrie.into());
        self
    }
    pub fn entries<I, S>(mut self, entries: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<EntryPreview>,
    {
        let v = self.entries.get_or_insert(Vec::new());
        for val in entries {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<ObjectPreview, String> {
        Ok(ObjectPreview {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            subtype: self.subtype,
            description: self.description,
            overflow: self
                .overflow
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(overflow)))?,
            properties: self
                .properties
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(properties)))?,
            entries: self.entries,
        })
    }
}
impl PropertyPreview {
    pub fn builder() -> PropertyPreviewBuilder {
        <PropertyPreviewBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PropertyPreviewBuilder {
    name: Option<String>,
    r#type: Option<PropertyPreviewType>,
    value: Option<String>,
    value_preview: Option<ObjectPreview>,
    subtype: Option<PropertyPreviewSubtype>,
}
impl PropertyPreviewBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<PropertyPreviewType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn value_preview(mut self, value_preview: impl Into<ObjectPreview>) -> Self {
        self.value_preview = Some(value_preview.into());
        self
    }
    pub fn subtype(mut self, subtype: impl Into<PropertyPreviewSubtype>) -> Self {
        self.subtype = Some(subtype.into());
        self
    }
    pub fn build(self) -> Result<PropertyPreview, String> {
        Ok(PropertyPreview {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self.value,
            value_preview: self.value_preview,
            subtype: self.subtype,
        })
    }
}
impl EntryPreview {
    pub fn builder() -> EntryPreviewBuilder {
        <EntryPreviewBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct EntryPreviewBuilder {
    key: Option<ObjectPreview>,
    value: Option<ObjectPreview>,
}
impl EntryPreviewBuilder {
    pub fn key(mut self, key: impl Into<ObjectPreview>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn value(mut self, value: impl Into<ObjectPreview>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<EntryPreview, String> {
        Ok(EntryPreview {
            key: self.key,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl PropertyDescriptor {
    pub fn builder() -> PropertyDescriptorBuilder {
        <PropertyDescriptorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PropertyDescriptorBuilder {
    name: Option<String>,
    value: Option<RemoteObject>,
    writable: Option<bool>,
    get: Option<RemoteObject>,
    set: Option<RemoteObject>,
    configurable: Option<bool>,
    enumerable: Option<bool>,
    was_thrown: Option<bool>,
    is_own: Option<bool>,
    symbol: Option<RemoteObject>,
}
impl PropertyDescriptorBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<RemoteObject>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn writable(mut self, writable: impl Into<bool>) -> Self {
        self.writable = Some(writable.into());
        self
    }
    pub fn get(mut self, get: impl Into<RemoteObject>) -> Self {
        self.get = Some(get.into());
        self
    }
    pub fn set(mut self, set: impl Into<RemoteObject>) -> Self {
        self.set = Some(set.into());
        self
    }
    pub fn configurable(mut self, configurable: impl Into<bool>) -> Self {
        self.configurable = Some(configurable.into());
        self
    }
    pub fn enumerable(mut self, enumerable: impl Into<bool>) -> Self {
        self.enumerable = Some(enumerable.into());
        self
    }
    pub fn was_thrown(mut self, was_thrown: impl Into<bool>) -> Self {
        self.was_thrown = Some(was_thrown.into());
        self
    }
    pub fn is_own(mut self, is_own: impl Into<bool>) -> Self {
        self.is_own = Some(is_own.into());
        self
    }
    pub fn symbol(mut self, symbol: impl Into<RemoteObject>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    pub fn build(self) -> Result<PropertyDescriptor, String> {
        Ok(PropertyDescriptor {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self.value,
            writable: self.writable,
            get: self.get,
            set: self.set,
            configurable: self.configurable.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(configurable))
            })?,
            enumerable: self
                .enumerable
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enumerable)))?,
            was_thrown: self.was_thrown,
            is_own: self.is_own,
            symbol: self.symbol,
        })
    }
}
impl InternalPropertyDescriptor {
    pub fn builder() -> InternalPropertyDescriptorBuilder {
        <InternalPropertyDescriptorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct InternalPropertyDescriptorBuilder {
    name: Option<String>,
    value: Option<RemoteObject>,
}
impl InternalPropertyDescriptorBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<RemoteObject>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<InternalPropertyDescriptor, String> {
        Ok(InternalPropertyDescriptor {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self.value,
        })
    }
}
impl PrivatePropertyDescriptor {
    pub fn builder() -> PrivatePropertyDescriptorBuilder {
        <PrivatePropertyDescriptorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PrivatePropertyDescriptorBuilder {
    name: Option<String>,
    value: Option<RemoteObject>,
    get: Option<RemoteObject>,
    set: Option<RemoteObject>,
}
impl PrivatePropertyDescriptorBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<RemoteObject>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn get(mut self, get: impl Into<RemoteObject>) -> Self {
        self.get = Some(get.into());
        self
    }
    pub fn set(mut self, set: impl Into<RemoteObject>) -> Self {
        self.set = Some(set.into());
        self
    }
    pub fn build(self) -> Result<PrivatePropertyDescriptor, String> {
        Ok(PrivatePropertyDescriptor {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self.value,
            get: self.get,
            set: self.set,
        })
    }
}
impl CallArgument {
    pub fn builder() -> CallArgumentBuilder {
        <CallArgumentBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CallArgumentBuilder {
    value: Option<serde_json::Value>,
    unserializable_value: Option<UnserializableValue>,
    object_id: Option<RemoteObjectId>,
}
impl CallArgumentBuilder {
    pub fn value(mut self, value: impl Into<serde_json::Value>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn unserializable_value(
        mut self,
        unserializable_value: impl Into<UnserializableValue>,
    ) -> Self {
        self.unserializable_value = Some(unserializable_value.into());
        self
    }
    pub fn object_id(mut self, object_id: impl Into<RemoteObjectId>) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn build(self) -> CallArgument {
        CallArgument {
            value: self.value,
            unserializable_value: self.unserializable_value,
            object_id: self.object_id,
        }
    }
}
impl ExecutionContextDescription {
    pub fn builder() -> ExecutionContextDescriptionBuilder {
        <ExecutionContextDescriptionBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ExecutionContextDescriptionBuilder {
    id: Option<ExecutionContextId>,
    origin: Option<String>,
    name: Option<String>,
    unique_id: Option<String>,
    aux_data: Option<serde_json::Value>,
}
impl ExecutionContextDescriptionBuilder {
    pub fn id(mut self, id: impl Into<ExecutionContextId>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn unique_id(mut self, unique_id: impl Into<String>) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
    pub fn aux_data(mut self, aux_data: impl Into<serde_json::Value>) -> Self {
        self.aux_data = Some(aux_data.into());
        self
    }
    pub fn build(self) -> Result<ExecutionContextDescription, String> {
        Ok(ExecutionContextDescription {
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            unique_id: self
                .unique_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(unique_id)))?,
            aux_data: self.aux_data,
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
    exception_id: Option<i64>,
    text: Option<String>,
    line_number: Option<i64>,
    column_number: Option<i64>,
    script_id: Option<ScriptId>,
    url: Option<String>,
    stack_trace: Option<StackTrace>,
    exception: Option<RemoteObject>,
    execution_context_id: Option<ExecutionContextId>,
    exception_meta_data: Option<serde_json::Value>,
}
impl ExceptionDetailsBuilder {
    pub fn exception_id(mut self, exception_id: impl Into<i64>) -> Self {
        self.exception_id = Some(exception_id.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn line_number(mut self, line_number: impl Into<i64>) -> Self {
        self.line_number = Some(line_number.into());
        self
    }
    pub fn column_number(mut self, column_number: impl Into<i64>) -> Self {
        self.column_number = Some(column_number.into());
        self
    }
    pub fn script_id(mut self, script_id: impl Into<ScriptId>) -> Self {
        self.script_id = Some(script_id.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn stack_trace(mut self, stack_trace: impl Into<StackTrace>) -> Self {
        self.stack_trace = Some(stack_trace.into());
        self
    }
    pub fn exception(mut self, exception: impl Into<RemoteObject>) -> Self {
        self.exception = Some(exception.into());
        self
    }
    pub fn execution_context_id(
        mut self,
        execution_context_id: impl Into<ExecutionContextId>,
    ) -> Self {
        self.execution_context_id = Some(execution_context_id.into());
        self
    }
    pub fn exception_meta_data(
        mut self,
        exception_meta_data: impl Into<serde_json::Value>,
    ) -> Self {
        self.exception_meta_data = Some(exception_meta_data.into());
        self
    }
    pub fn build(self) -> Result<ExceptionDetails, String> {
        Ok(ExceptionDetails {
            exception_id: self.exception_id.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(exception_id))
            })?,
            text: self
                .text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
            line_number: self
                .line_number
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(line_number)))?,
            column_number: self.column_number.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(column_number))
            })?,
            script_id: self.script_id,
            url: self.url,
            stack_trace: self.stack_trace,
            exception: self.exception,
            execution_context_id: self.execution_context_id,
            exception_meta_data: self.exception_meta_data,
        })
    }
}
impl CallFrame {
    pub fn builder() -> CallFrameBuilder {
        <CallFrameBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CallFrameBuilder {
    function_name: Option<String>,
    script_id: Option<ScriptId>,
    url: Option<String>,
    line_number: Option<i64>,
    column_number: Option<i64>,
}
impl CallFrameBuilder {
    pub fn function_name(mut self, function_name: impl Into<String>) -> Self {
        self.function_name = Some(function_name.into());
        self
    }
    pub fn script_id(mut self, script_id: impl Into<ScriptId>) -> Self {
        self.script_id = Some(script_id.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn line_number(mut self, line_number: impl Into<i64>) -> Self {
        self.line_number = Some(line_number.into());
        self
    }
    pub fn column_number(mut self, column_number: impl Into<i64>) -> Self {
        self.column_number = Some(column_number.into());
        self
    }
    pub fn build(self) -> Result<CallFrame, String> {
        Ok(CallFrame {
            function_name: self.function_name.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(function_name))
            })?,
            script_id: self
                .script_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(script_id)))?,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            line_number: self
                .line_number
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(line_number)))?,
            column_number: self.column_number.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(column_number))
            })?,
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
    description: Option<String>,
    call_frames: Option<Vec<CallFrame>>,
    parent: Option<StackTrace>,
    parent_id: Option<StackTraceId>,
}
impl StackTraceBuilder {
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn call_frame(mut self, call_frame: impl Into<CallFrame>) -> Self {
        let v = self.call_frames.get_or_insert(Vec::new());
        v.push(call_frame.into());
        self
    }
    pub fn call_frames<I, S>(mut self, call_frames: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CallFrame>,
    {
        let v = self.call_frames.get_or_insert(Vec::new());
        for val in call_frames {
            v.push(val.into());
        }
        self
    }
    pub fn parent(mut self, parent: impl Into<StackTrace>) -> Self {
        self.parent = Some(parent.into());
        self
    }
    pub fn parent_id(mut self, parent_id: impl Into<StackTraceId>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }
    pub fn build(self) -> Result<StackTrace, String> {
        Ok(StackTrace {
            description: self.description,
            call_frames: self
                .call_frames
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(call_frames)))?,
            parent: self.parent.map(Box::new),
            parent_id: self.parent_id,
        })
    }
}
impl StackTraceId {
    pub fn builder() -> StackTraceIdBuilder {
        <StackTraceIdBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StackTraceIdBuilder {
    id: Option<String>,
    debugger_id: Option<UniqueDebuggerId>,
}
impl StackTraceIdBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn debugger_id(mut self, debugger_id: impl Into<UniqueDebuggerId>) -> Self {
        self.debugger_id = Some(debugger_id.into());
        self
    }
    pub fn build(self) -> Result<StackTraceId, String> {
        Ok(StackTraceId {
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            debugger_id: self.debugger_id,
        })
    }
}
