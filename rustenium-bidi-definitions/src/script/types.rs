use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct Channel(String);
impl Channel {
    pub fn new(val: impl Into<String>) -> Self {
        Channel(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for Channel {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<Channel> for String {
    fn from(el: Channel) -> String {
        el.0
    }
}
impl From<String> for Channel {
    fn from(expr: String) -> Self {
        Channel(expr)
    }
}
impl Channel {
    pub const IDENTIFIER: &'static str = "script.Channel";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelValue {
    #[serde(rename = "type")]
    pub r#type: ChannelValueType,
    #[serde(rename = "value")]
    pub value: ChannelProperties,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChannelValueType {
    #[serde(rename = "channel")]
    Channel,
}
impl ChannelValue {
    pub fn new(r#type: impl Into<ChannelValueType>, value: impl Into<ChannelProperties>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl ChannelValue {
    pub const IDENTIFIER: &'static str = "script.ChannelValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelProperties {
    #[serde(rename = "channel")]
    pub channel: Channel,
    #[serde(rename = "serializationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub serialization_options: Option<SerializationOptions>,
    #[serde(rename = "ownership")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ownership: Option<ResultOwnership>,
}
impl ChannelProperties {
    pub fn new(channel: impl Into<Channel>) -> Self {
        Self {
            channel: channel.into(),
            serialization_options: None,
            ownership: None,
        }
    }
}
impl ChannelProperties {
    pub const IDENTIFIER: &'static str = "script.ChannelProperties";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvaluateResultSuccess {
    #[serde(rename = "type")]
    pub r#type: EvaluateResultSuccessType,
    #[serde(rename = "result")]
    pub result: RemoteValue,
    #[serde(rename = "realm")]
    pub realm: Realm,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvaluateResultSuccessType {
    #[serde(rename = "success")]
    Success,
}
impl EvaluateResultSuccess {
    pub fn new(
        r#type: impl Into<EvaluateResultSuccessType>,
        result: impl Into<RemoteValue>,
        realm: impl Into<Realm>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            result: result.into(),
            realm: realm.into(),
        }
    }
}
impl EvaluateResultSuccess {
    pub const IDENTIFIER: &'static str = "script.EvaluateResultSuccess";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvaluateResultException {
    #[serde(rename = "type")]
    pub r#type: EvaluateResultExceptionType,
    #[serde(rename = "exceptionDetails")]
    pub exception_details: ExceptionDetails,
    #[serde(rename = "realm")]
    pub realm: Realm,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvaluateResultExceptionType {
    #[serde(rename = "exception")]
    Exception,
}
impl EvaluateResultException {
    pub fn new(
        r#type: impl Into<EvaluateResultExceptionType>,
        exception_details: impl Into<ExceptionDetails>,
        realm: impl Into<Realm>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            exception_details: exception_details.into(),
            realm: realm.into(),
        }
    }
}
impl EvaluateResultException {
    pub const IDENTIFIER: &'static str = "script.EvaluateResultException";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExceptionDetails {
    #[serde(rename = "columnNumber")]
    pub column_number: u64,
    #[serde(rename = "exception")]
    pub exception: RemoteValue,
    #[serde(rename = "lineNumber")]
    pub line_number: u64,
    #[serde(rename = "stackTrace")]
    pub stack_trace: StackTrace,
    #[serde(rename = "text")]
    pub text: String,
}
impl ExceptionDetails {
    pub const IDENTIFIER: &'static str = "script.ExceptionDetails";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct Handle(String);
impl Handle {
    pub fn new(val: impl Into<String>) -> Self {
        Handle(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for Handle {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<Handle> for String {
    fn from(el: Handle) -> String {
        el.0
    }
}
impl From<String> for Handle {
    fn from(expr: String) -> Self {
        Handle(expr)
    }
}
impl Handle {
    pub const IDENTIFIER: &'static str = "script.Handle";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct InternalId(String);
impl InternalId {
    pub fn new(val: impl Into<String>) -> Self {
        InternalId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for InternalId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<InternalId> for String {
    fn from(el: InternalId) -> String {
        el.0
    }
}
impl From<String> for InternalId {
    fn from(expr: String) -> Self {
        InternalId(expr)
    }
}
impl std::borrow::Borrow<str> for InternalId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl InternalId {
    pub const IDENTIFIER: &'static str = "script.InternalId";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LocalValue(serde_json::Value);
impl LocalValue {
    pub fn new(val: impl Into<serde_json::Value>) -> Self {
        LocalValue(val.into())
    }
    pub fn inner(&self) -> &serde_json::Value {
        &self.0
    }
}
impl LocalValue {
    pub const IDENTIFIER: &'static str = "script.LocalValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListLocalValue(Vec<LocalValue>);
impl ListLocalValue {
    pub fn new(val: impl Into<Vec<LocalValue>>) -> Self {
        ListLocalValue(val.into())
    }
    pub fn inner(&self) -> &Vec<LocalValue> {
        &self.0
    }
}
impl ListLocalValue {
    pub const IDENTIFIER: &'static str = "script.ListLocalValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayLocalValue {
    #[serde(rename = "type")]
    pub r#type: ArrayLocalValueType,
    #[serde(rename = "value")]
    pub value: ListLocalValue,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArrayLocalValueType {
    #[serde(rename = "array")]
    Array,
}
impl ArrayLocalValue {
    pub fn new(r#type: impl Into<ArrayLocalValueType>, value: impl Into<ListLocalValue>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl ArrayLocalValue {
    pub const IDENTIFIER: &'static str = "script.ArrayLocalValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateLocalValue {
    #[serde(rename = "type")]
    pub r#type: DateLocalValueType,
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DateLocalValueType {
    #[serde(rename = "date")]
    Date,
}
impl DateLocalValue {
    pub fn new(r#type: impl Into<DateLocalValueType>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl DateLocalValue {
    pub const IDENTIFIER: &'static str = "script.DateLocalValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MappingLocalValue(Vec<Vec<serde_json::Value>>);
impl MappingLocalValue {
    pub fn new(val: impl Into<Vec<Vec<serde_json::Value>>>) -> Self {
        MappingLocalValue(val.into())
    }
    pub fn inner(&self) -> &Vec<Vec<serde_json::Value>> {
        &self.0
    }
}
impl MappingLocalValue {
    pub const IDENTIFIER: &'static str = "script.MappingLocalValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapLocalValue {
    #[serde(rename = "type")]
    pub r#type: MapLocalValueType,
    #[serde(rename = "value")]
    pub value: MappingLocalValue,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MapLocalValueType {
    #[serde(rename = "map")]
    Map,
}
impl MapLocalValue {
    pub fn new(r#type: impl Into<MapLocalValueType>, value: impl Into<MappingLocalValue>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl MapLocalValue {
    pub const IDENTIFIER: &'static str = "script.MapLocalValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectLocalValue {
    #[serde(rename = "type")]
    pub r#type: ObjectLocalValueType,
    #[serde(rename = "value")]
    pub value: MappingLocalValue,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectLocalValueType {
    #[serde(rename = "object")]
    Object,
}
impl ObjectLocalValue {
    pub fn new(
        r#type: impl Into<ObjectLocalValueType>,
        value: impl Into<MappingLocalValue>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl ObjectLocalValue {
    pub const IDENTIFIER: &'static str = "script.ObjectLocalValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegExpValue {
    #[serde(rename = "pattern")]
    pub pattern: String,
    #[serde(rename = "flags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub flags: Option<String>,
}
impl RegExpValue {
    pub fn new(pattern: impl Into<String>) -> Self {
        Self {
            pattern: pattern.into(),
            flags: None,
        }
    }
}
impl<T: Into<String>> From<T> for RegExpValue {
    fn from(url: T) -> Self {
        RegExpValue::new(url)
    }
}
impl RegExpValue {
    pub const IDENTIFIER: &'static str = "script.RegExpValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegExpLocalValue {
    #[serde(rename = "type")]
    pub r#type: RegExpLocalValueType,
    #[serde(rename = "value")]
    pub value: RegExpValue,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegExpLocalValueType {
    #[serde(rename = "regexp")]
    Regexp,
}
impl RegExpLocalValue {
    pub fn new(r#type: impl Into<RegExpLocalValueType>, value: impl Into<RegExpValue>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl RegExpLocalValue {
    pub const IDENTIFIER: &'static str = "script.RegExpLocalValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLocalValue {
    #[serde(rename = "type")]
    pub r#type: SetLocalValueType,
    #[serde(rename = "value")]
    pub value: ListLocalValue,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetLocalValueType {
    #[serde(rename = "set")]
    Set,
}
impl SetLocalValue {
    pub fn new(r#type: impl Into<SetLocalValueType>, value: impl Into<ListLocalValue>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl SetLocalValue {
    pub const IDENTIFIER: &'static str = "script.SetLocalValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct PreloadScript(String);
impl PreloadScript {
    pub fn new(val: impl Into<String>) -> Self {
        PreloadScript(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for PreloadScript {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<PreloadScript> for String {
    fn from(el: PreloadScript) -> String {
        el.0
    }
}
impl From<String> for PreloadScript {
    fn from(expr: String) -> Self {
        PreloadScript(expr)
    }
}
impl PreloadScript {
    pub const IDENTIFIER: &'static str = "script.PreloadScript";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct Realm(String);
impl Realm {
    pub fn new(val: impl Into<String>) -> Self {
        Realm(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for Realm {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<Realm> for String {
    fn from(el: Realm) -> String {
        el.0
    }
}
impl From<String> for Realm {
    fn from(expr: String) -> Self {
        Realm(expr)
    }
}
impl Realm {
    pub const IDENTIFIER: &'static str = "script.Realm";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrimitiveProtocolValue {
    UndefinedValue(UndefinedValue),
    NullValue(NullValue),
    StringValue(StringValue),
    NumberValue(NumberValue),
    BooleanValue(BooleanValue),
    BigIntValue(BigIntValue),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UndefinedValue {
    #[serde(rename = "type")]
    pub r#type: UndefinedValueType,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UndefinedValueType {
    #[serde(rename = "undefined")]
    Undefined,
}
impl UndefinedValue {
    pub fn new(r#type: impl Into<UndefinedValueType>) -> Self {
        Self {
            r#type: r#type.into(),
        }
    }
}
impl UndefinedValue {
    pub const IDENTIFIER: &'static str = "script.UndefinedValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NullValue {
    #[serde(rename = "type")]
    pub r#type: NullValueType,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NullValueType {
    #[serde(rename = "null")]
    Null,
}
impl NullValue {
    pub fn new(r#type: impl Into<NullValueType>) -> Self {
        Self {
            r#type: r#type.into(),
        }
    }
}
impl NullValue {
    pub const IDENTIFIER: &'static str = "script.NullValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringValue {
    #[serde(rename = "type")]
    pub r#type: StringValueType,
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StringValueType {
    #[serde(rename = "string")]
    String,
}
impl StringValue {
    pub fn new(r#type: impl Into<StringValueType>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl StringValue {
    pub const IDENTIFIER: &'static str = "script.StringValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct SpecialNumber(String);
impl SpecialNumber {
    pub fn new(val: impl Into<String>) -> Self {
        SpecialNumber(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for SpecialNumber {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<SpecialNumber> for String {
    fn from(el: SpecialNumber) -> String {
        el.0
    }
}
impl From<String> for SpecialNumber {
    fn from(expr: String) -> Self {
        SpecialNumber(expr)
    }
}
impl SpecialNumber {
    pub const IDENTIFIER: &'static str = "script.SpecialNumber";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberValue {
    #[serde(rename = "type")]
    pub r#type: NumberValueType,
    #[serde(rename = "value")]
    pub value: serde_json::Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NumberValueType {
    #[serde(rename = "number")]
    Number,
}
impl NumberValue {
    pub fn new(r#type: impl Into<NumberValueType>, value: impl Into<serde_json::Value>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl NumberValue {
    pub const IDENTIFIER: &'static str = "script.NumberValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BooleanValue {
    #[serde(rename = "type")]
    pub r#type: BooleanValueType,
    #[serde(rename = "value")]
    pub value: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BooleanValueType {
    #[serde(rename = "boolean")]
    Boolean,
}
impl BooleanValue {
    pub fn new(r#type: impl Into<BooleanValueType>, value: impl Into<bool>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl BooleanValue {
    pub const IDENTIFIER: &'static str = "script.BooleanValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BigIntValue {
    #[serde(rename = "type")]
    pub r#type: BigIntValueType,
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BigIntValueType {
    #[serde(rename = "bigint")]
    Bigint,
}
impl BigIntValue {
    pub fn new(r#type: impl Into<BigIntValueType>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl BigIntValue {
    pub const IDENTIFIER: &'static str = "script.BigIntValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RealmType {
    #[serde(rename = "window")]
    Window,
    #[serde(rename = "dedicated-worker")]
    DedicatedWorker,
    #[serde(rename = "shared-worker")]
    SharedWorker,
    #[serde(rename = "service-worker")]
    ServiceWorker,
    #[serde(rename = "worker")]
    Worker,
    #[serde(rename = "paint-worklet")]
    PaintWorklet,
    #[serde(rename = "audio-worklet")]
    AudioWorklet,
    #[serde(rename = "worklet")]
    Worklet,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoteReference {
    SharedReference(SharedReference),
    RemoteObjectReference(RemoteObjectReference),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedReference {
    #[serde(rename = "sharedId")]
    pub shared_id: SharedId,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl SharedReference {
    pub fn new(
        shared_id: impl Into<SharedId>,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            shared_id: shared_id.into(),
            extensible: extensible.into(),
            handle: None,
        }
    }
}
impl SharedReference {
    pub const IDENTIFIER: &'static str = "script.SharedReference";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoteObjectReference {
    #[serde(rename = "handle")]
    pub handle: Handle,
    #[serde(rename = "sharedId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub shared_id: Option<SharedId>,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl RemoteObjectReference {
    pub fn new(
        handle: impl Into<Handle>,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            handle: handle.into(),
            extensible: extensible.into(),
            shared_id: None,
        }
    }
}
impl RemoteObjectReference {
    pub const IDENTIFIER: &'static str = "script.RemoteObjectReference";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoteValue {
    PrimitiveProtocolValue(PrimitiveProtocolValue),
    SymbolRemoteValue(SymbolRemoteValue),
    ArrayRemoteValue(ArrayRemoteValue),
    ObjectRemoteValue(ObjectRemoteValue),
    FunctionRemoteValue(FunctionRemoteValue),
    RegExpRemoteValue(RegExpRemoteValue),
    DateRemoteValue(DateRemoteValue),
    MapRemoteValue(MapRemoteValue),
    SetRemoteValue(SetRemoteValue),
    WeakMapRemoteValue(WeakMapRemoteValue),
    WeakSetRemoteValue(WeakSetRemoteValue),
    GeneratorRemoteValue(GeneratorRemoteValue),
    ErrorRemoteValue(ErrorRemoteValue),
    ProxyRemoteValue(ProxyRemoteValue),
    PromiseRemoteValue(PromiseRemoteValue),
    TypedArrayRemoteValue(TypedArrayRemoteValue),
    ArrayBufferRemoteValue(ArrayBufferRemoteValue),
    NodeListRemoteValue(NodeListRemoteValue),
    HtmlCollectionRemoteValue(HtmlCollectionRemoteValue),
    NodeRemoteValue(NodeRemoteValue),
    WindowProxyRemoteValue(WindowProxyRemoteValue),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListRemoteValue(Vec<RemoteValue>);
impl ListRemoteValue {
    pub fn new(val: impl Into<Vec<RemoteValue>>) -> Self {
        ListRemoteValue(val.into())
    }
    pub fn inner(&self) -> &Vec<RemoteValue> {
        &self.0
    }
}
impl ListRemoteValue {
    pub const IDENTIFIER: &'static str = "script.ListRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MappingRemoteValue(Vec<Vec<serde_json::Value>>);
impl MappingRemoteValue {
    pub fn new(val: impl Into<Vec<Vec<serde_json::Value>>>) -> Self {
        MappingRemoteValue(val.into())
    }
    pub fn inner(&self) -> &Vec<Vec<serde_json::Value>> {
        &self.0
    }
}
impl MappingRemoteValue {
    pub const IDENTIFIER: &'static str = "script.MappingRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolRemoteValue {
    #[serde(rename = "type")]
    pub r#type: SymbolRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolRemoteValueType {
    #[serde(rename = "symbol")]
    Symbol,
}
impl SymbolRemoteValue {
    pub fn new(r#type: impl Into<SymbolRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
        }
    }
}
impl SymbolRemoteValue {
    pub const IDENTIFIER: &'static str = "script.SymbolRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayRemoteValue {
    #[serde(rename = "type")]
    pub r#type: ArrayRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<ListRemoteValue>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArrayRemoteValueType {
    #[serde(rename = "array")]
    Array,
}
impl ArrayRemoteValue {
    pub fn new(r#type: impl Into<ArrayRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
            value: None,
        }
    }
}
impl ArrayRemoteValue {
    pub const IDENTIFIER: &'static str = "script.ArrayRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectRemoteValue {
    #[serde(rename = "type")]
    pub r#type: ObjectRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<MappingRemoteValue>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectRemoteValueType {
    #[serde(rename = "object")]
    Object,
}
impl ObjectRemoteValue {
    pub fn new(r#type: impl Into<ObjectRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
            value: None,
        }
    }
}
impl ObjectRemoteValue {
    pub const IDENTIFIER: &'static str = "script.ObjectRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionRemoteValue {
    #[serde(rename = "type")]
    pub r#type: FunctionRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FunctionRemoteValueType {
    #[serde(rename = "function")]
    Function,
}
impl FunctionRemoteValue {
    pub fn new(r#type: impl Into<FunctionRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
        }
    }
}
impl FunctionRemoteValue {
    pub const IDENTIFIER: &'static str = "script.FunctionRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegExpRemoteValue {
    #[serde(flatten)]
    #[serde(default)]
    pub reg_exp_local_value: RegExpLocalValue,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
}
impl RegExpRemoteValue {
    pub fn new(reg_exp_local_value: impl Into<RegExpLocalValue>) -> Self {
        Self {
            reg_exp_local_value: reg_exp_local_value.into(),
            handle: None,
            internal_id: None,
        }
    }
}
impl RegExpRemoteValue {
    pub const IDENTIFIER: &'static str = "script.RegExpRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateRemoteValue {
    #[serde(flatten)]
    #[serde(default)]
    pub date_local_value: DateLocalValue,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
}
impl DateRemoteValue {
    pub fn new(date_local_value: impl Into<DateLocalValue>) -> Self {
        Self {
            date_local_value: date_local_value.into(),
            handle: None,
            internal_id: None,
        }
    }
}
impl DateRemoteValue {
    pub const IDENTIFIER: &'static str = "script.DateRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapRemoteValue {
    #[serde(rename = "type")]
    pub r#type: MapRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<MappingRemoteValue>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MapRemoteValueType {
    #[serde(rename = "map")]
    Map,
}
impl MapRemoteValue {
    pub fn new(r#type: impl Into<MapRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
            value: None,
        }
    }
}
impl MapRemoteValue {
    pub const IDENTIFIER: &'static str = "script.MapRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRemoteValue {
    #[serde(rename = "type")]
    pub r#type: SetRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<ListRemoteValue>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetRemoteValueType {
    #[serde(rename = "set")]
    Set,
}
impl SetRemoteValue {
    pub fn new(r#type: impl Into<SetRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
            value: None,
        }
    }
}
impl SetRemoteValue {
    pub const IDENTIFIER: &'static str = "script.SetRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeakMapRemoteValue {
    #[serde(rename = "type")]
    pub r#type: WeakMapRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeakMapRemoteValueType {
    #[serde(rename = "weakmap")]
    Weakmap,
}
impl WeakMapRemoteValue {
    pub fn new(r#type: impl Into<WeakMapRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
        }
    }
}
impl WeakMapRemoteValue {
    pub const IDENTIFIER: &'static str = "script.WeakMapRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeakSetRemoteValue {
    #[serde(rename = "type")]
    pub r#type: WeakSetRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeakSetRemoteValueType {
    #[serde(rename = "weakset")]
    Weakset,
}
impl WeakSetRemoteValue {
    pub fn new(r#type: impl Into<WeakSetRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
        }
    }
}
impl WeakSetRemoteValue {
    pub const IDENTIFIER: &'static str = "script.WeakSetRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneratorRemoteValue {
    #[serde(rename = "type")]
    pub r#type: GeneratorRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeneratorRemoteValueType {
    #[serde(rename = "generator")]
    Generator,
}
impl GeneratorRemoteValue {
    pub fn new(r#type: impl Into<GeneratorRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
        }
    }
}
impl GeneratorRemoteValue {
    pub const IDENTIFIER: &'static str = "script.GeneratorRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorRemoteValue {
    #[serde(rename = "type")]
    pub r#type: ErrorRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorRemoteValueType {
    #[serde(rename = "error")]
    Error,
}
impl ErrorRemoteValue {
    pub fn new(r#type: impl Into<ErrorRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
        }
    }
}
impl ErrorRemoteValue {
    pub const IDENTIFIER: &'static str = "script.ErrorRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProxyRemoteValue {
    #[serde(rename = "type")]
    pub r#type: ProxyRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProxyRemoteValueType {
    #[serde(rename = "proxy")]
    Proxy,
}
impl ProxyRemoteValue {
    pub fn new(r#type: impl Into<ProxyRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
        }
    }
}
impl ProxyRemoteValue {
    pub const IDENTIFIER: &'static str = "script.ProxyRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromiseRemoteValue {
    #[serde(rename = "type")]
    pub r#type: PromiseRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PromiseRemoteValueType {
    #[serde(rename = "promise")]
    Promise,
}
impl PromiseRemoteValue {
    pub fn new(r#type: impl Into<PromiseRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
        }
    }
}
impl PromiseRemoteValue {
    pub const IDENTIFIER: &'static str = "script.PromiseRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedArrayRemoteValue {
    #[serde(rename = "type")]
    pub r#type: TypedArrayRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypedArrayRemoteValueType {
    #[serde(rename = "typedarray")]
    Typedarray,
}
impl TypedArrayRemoteValue {
    pub fn new(r#type: impl Into<TypedArrayRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
        }
    }
}
impl TypedArrayRemoteValue {
    pub const IDENTIFIER: &'static str = "script.TypedArrayRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayBufferRemoteValue {
    #[serde(rename = "type")]
    pub r#type: ArrayBufferRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArrayBufferRemoteValueType {
    #[serde(rename = "arraybuffer")]
    Arraybuffer,
}
impl ArrayBufferRemoteValue {
    pub fn new(r#type: impl Into<ArrayBufferRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
        }
    }
}
impl ArrayBufferRemoteValue {
    pub const IDENTIFIER: &'static str = "script.ArrayBufferRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeListRemoteValue {
    #[serde(rename = "type")]
    pub r#type: NodeListRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<ListRemoteValue>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeListRemoteValueType {
    #[serde(rename = "nodelist")]
    Nodelist,
}
impl NodeListRemoteValue {
    pub fn new(r#type: impl Into<NodeListRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
            value: None,
        }
    }
}
impl NodeListRemoteValue {
    pub const IDENTIFIER: &'static str = "script.NodeListRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HtmlCollectionRemoteValue {
    #[serde(rename = "type")]
    pub r#type: HtmlCollectionRemoteValueType,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<ListRemoteValue>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HtmlCollectionRemoteValueType {
    #[serde(rename = "htmlcollection")]
    Htmlcollection,
}
impl HtmlCollectionRemoteValue {
    pub fn new(r#type: impl Into<HtmlCollectionRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            handle: None,
            internal_id: None,
            value: None,
        }
    }
}
impl HtmlCollectionRemoteValue {
    pub const IDENTIFIER: &'static str = "script.HTMLCollectionRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeRemoteValue {
    #[serde(rename = "type")]
    pub r#type: NodeRemoteValueType,
    #[serde(rename = "sharedId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub shared_id: Option<SharedId>,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<Box<NodeProperties>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeRemoteValueType {
    #[serde(rename = "node")]
    Node,
}
impl NodeRemoteValue {
    pub fn new(r#type: impl Into<NodeRemoteValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            shared_id: None,
            handle: None,
            internal_id: None,
            value: None,
        }
    }
}
impl NodeRemoteValue {
    pub const IDENTIFIER: &'static str = "script.NodeRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeProperties {
    #[serde(rename = "nodeType")]
    pub node_type: u64,
    #[serde(rename = "childNodeCount")]
    pub child_node_count: u64,
    #[serde(rename = "attributes")]
    #[serde(default)]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub children: Option<Vec<NodeRemoteValue>>,
    #[serde(rename = "localName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub local_name: Option<String>,
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mode: Option<NodePropertiesMode>,
    #[serde(rename = "namespaceURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub namespace_uri: Option<String>,
    #[serde(rename = "nodeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_value: Option<String>,
    #[serde(rename = "shadowRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub shadow_root: Option<Box<NodeRemoteValue>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodePropertiesMode {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}
impl NodeProperties {
    pub fn new(node_type: impl Into<u64>, child_node_count: impl Into<u64>) -> Self {
        Self {
            node_type: node_type.into(),
            child_node_count: child_node_count.into(),
            attributes: None,
            children: None,
            local_name: None,
            mode: None,
            namespace_uri: None,
            node_value: None,
            shadow_root: None,
        }
    }
}
impl NodeProperties {
    pub const IDENTIFIER: &'static str = "script.NodeProperties";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowProxyRemoteValue {
    #[serde(rename = "type")]
    pub r#type: WindowProxyRemoteValueType,
    #[serde(rename = "value")]
    pub value: WindowProxyProperties,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_id: Option<InternalId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WindowProxyRemoteValueType {
    #[serde(rename = "window")]
    Window,
}
impl WindowProxyRemoteValue {
    pub fn new(
        r#type: impl Into<WindowProxyRemoteValueType>,
        value: impl Into<WindowProxyProperties>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
            handle: None,
            internal_id: None,
        }
    }
}
impl WindowProxyRemoteValue {
    pub const IDENTIFIER: &'static str = "script.WindowProxyRemoteValue";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowProxyProperties {
    #[serde(rename = "context")]
    pub context: crate::browsing_context::types::BrowsingContext,
}
impl WindowProxyProperties {
    pub fn new(context: impl Into<crate::browsing_context::types::BrowsingContext>) -> Self {
        Self {
            context: context.into(),
        }
    }
}
impl WindowProxyProperties {
    pub const IDENTIFIER: &'static str = "script.WindowProxyProperties";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResultOwnership {
    #[serde(rename = "root")]
    Root,
    #[serde(rename = "none")]
    None,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SerializationOptions {
    #[serde(rename = "maxDomDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_serialization_options_max_dom_depth")]
    pub max_dom_depth: Option<u64>,
    #[serde(rename = "maxObjectDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_object_depth: Option<u64>,
    #[serde(rename = "includeShadowTree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_shadow_tree: Option<SerializationOptionsIncludeShadowTree>,
}
fn default_serialization_options_max_dom_depth() -> Option<u64> {
    Some(0u64)
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SerializationOptionsIncludeShadowTree {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "all")]
    All,
}
impl SerializationOptions {
    pub const IDENTIFIER: &'static str = "script.SerializationOptions";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct SharedId(String);
impl SharedId {
    pub fn new(val: impl Into<String>) -> Self {
        SharedId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for SharedId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<SharedId> for String {
    fn from(el: SharedId) -> String {
        el.0
    }
}
impl From<String> for SharedId {
    fn from(expr: String) -> Self {
        SharedId(expr)
    }
}
impl std::borrow::Borrow<str> for SharedId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl SharedId {
    pub const IDENTIFIER: &'static str = "script.SharedId";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StackFrame {
    #[serde(rename = "columnNumber")]
    pub column_number: u64,
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(rename = "lineNumber")]
    pub line_number: u64,
    #[serde(rename = "url")]
    pub url: String,
}
impl StackFrame {
    pub fn new(
        column_number: impl Into<u64>,
        function_name: impl Into<String>,
        line_number: impl Into<u64>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            column_number: column_number.into(),
            function_name: function_name.into(),
            line_number: line_number.into(),
            url: url.into(),
        }
    }
}
impl StackFrame {
    pub const IDENTIFIER: &'static str = "script.StackFrame";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StackTrace {
    #[serde(rename = "callFrames")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub call_frames: Vec<StackFrame>,
}
impl StackTrace {
    pub fn new(call_frames: Vec<StackFrame>) -> Self {
        Self { call_frames }
    }
}
impl StackTrace {
    pub const IDENTIFIER: &'static str = "script.StackTrace";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealmTarget {
    #[serde(rename = "realm")]
    pub realm: Realm,
}
impl RealmTarget {
    pub fn new(realm: impl Into<Realm>) -> Self {
        Self {
            realm: realm.into(),
        }
    }
}
impl RealmTarget {
    pub const IDENTIFIER: &'static str = "script.RealmTarget";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextTarget {
    #[serde(rename = "context")]
    pub context: crate::browsing_context::types::BrowsingContext,
    #[serde(rename = "sandbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sandbox: Option<String>,
}
impl ContextTarget {
    pub fn new(context: impl Into<crate::browsing_context::types::BrowsingContext>) -> Self {
        Self {
            context: context.into(),
            sandbox: None,
        }
    }
}
impl ContextTarget {
    pub const IDENTIFIER: &'static str = "script.ContextTarget";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Target {
    ContextTarget(ContextTarget),
    RealmTarget(RealmTarget),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealmInfo {
    WindowRealmInfo(WindowRealmInfo),
    DedicatedWorkerRealmInfo(DedicatedWorkerRealmInfo),
    SharedWorkerRealmInfo(SharedWorkerRealmInfo),
    ServiceWorkerRealmInfo(ServiceWorkerRealmInfo),
    WorkerRealmInfo(WorkerRealmInfo),
    PaintWorkletRealmInfo(PaintWorkletRealmInfo),
    AudioWorkletRealmInfo(AudioWorkletRealmInfo),
    WorkletRealmInfo(WorkletRealmInfo),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseRealmInfo {
    #[serde(rename = "realm")]
    pub realm: Realm,
    #[serde(rename = "origin")]
    pub origin: String,
}
impl BaseRealmInfo {
    pub fn new(realm: impl Into<Realm>, origin: impl Into<String>) -> Self {
        Self {
            realm: realm.into(),
            origin: origin.into(),
        }
    }
}
impl BaseRealmInfo {
    pub const IDENTIFIER: &'static str = "script.BaseRealmInfo";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowRealmInfo {
    #[serde(flatten)]
    #[serde(default)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: WindowRealmInfoType,
    #[serde(rename = "context")]
    pub context: crate::browsing_context::types::BrowsingContext,
    #[serde(rename = "sandbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sandbox: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WindowRealmInfoType {
    #[serde(rename = "window")]
    Window,
}
impl WindowRealmInfo {
    pub fn new(
        base_realm_info: impl Into<BaseRealmInfo>,
        r#type: impl Into<WindowRealmInfoType>,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        Self {
            base_realm_info: base_realm_info.into(),
            r#type: r#type.into(),
            context: context.into(),
            sandbox: None,
        }
    }
}
impl WindowRealmInfo {
    pub const IDENTIFIER: &'static str = "script.WindowRealmInfo";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DedicatedWorkerRealmInfo {
    #[serde(flatten)]
    #[serde(default)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: DedicatedWorkerRealmInfoType,
    #[serde(rename = "owners")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub owners: Vec<Realm>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DedicatedWorkerRealmInfoType {
    #[serde(rename = "dedicated-worker")]
    DedicatedWorker,
}
impl DedicatedWorkerRealmInfo {
    pub fn new(
        base_realm_info: impl Into<BaseRealmInfo>,
        r#type: impl Into<DedicatedWorkerRealmInfoType>,
        owners: Vec<Realm>,
    ) -> Self {
        Self {
            base_realm_info: base_realm_info.into(),
            r#type: r#type.into(),
            owners,
        }
    }
}
impl DedicatedWorkerRealmInfo {
    pub const IDENTIFIER: &'static str = "script.DedicatedWorkerRealmInfo";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedWorkerRealmInfo {
    #[serde(flatten)]
    #[serde(default)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: SharedWorkerRealmInfoType,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SharedWorkerRealmInfoType {
    #[serde(rename = "shared-worker")]
    SharedWorker,
}
impl SharedWorkerRealmInfo {
    pub fn new(
        base_realm_info: impl Into<BaseRealmInfo>,
        r#type: impl Into<SharedWorkerRealmInfoType>,
    ) -> Self {
        Self {
            base_realm_info: base_realm_info.into(),
            r#type: r#type.into(),
        }
    }
}
impl SharedWorkerRealmInfo {
    pub const IDENTIFIER: &'static str = "script.SharedWorkerRealmInfo";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceWorkerRealmInfo {
    #[serde(flatten)]
    #[serde(default)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: ServiceWorkerRealmInfoType,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceWorkerRealmInfoType {
    #[serde(rename = "service-worker")]
    ServiceWorker,
}
impl ServiceWorkerRealmInfo {
    pub fn new(
        base_realm_info: impl Into<BaseRealmInfo>,
        r#type: impl Into<ServiceWorkerRealmInfoType>,
    ) -> Self {
        Self {
            base_realm_info: base_realm_info.into(),
            r#type: r#type.into(),
        }
    }
}
impl ServiceWorkerRealmInfo {
    pub const IDENTIFIER: &'static str = "script.ServiceWorkerRealmInfo";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerRealmInfo {
    #[serde(flatten)]
    #[serde(default)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: WorkerRealmInfoType,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkerRealmInfoType {
    #[serde(rename = "worker")]
    Worker,
}
impl WorkerRealmInfo {
    pub fn new(
        base_realm_info: impl Into<BaseRealmInfo>,
        r#type: impl Into<WorkerRealmInfoType>,
    ) -> Self {
        Self {
            base_realm_info: base_realm_info.into(),
            r#type: r#type.into(),
        }
    }
}
impl WorkerRealmInfo {
    pub const IDENTIFIER: &'static str = "script.WorkerRealmInfo";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaintWorkletRealmInfo {
    #[serde(flatten)]
    #[serde(default)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: PaintWorkletRealmInfoType,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaintWorkletRealmInfoType {
    #[serde(rename = "paint-worklet")]
    PaintWorklet,
}
impl PaintWorkletRealmInfo {
    pub fn new(
        base_realm_info: impl Into<BaseRealmInfo>,
        r#type: impl Into<PaintWorkletRealmInfoType>,
    ) -> Self {
        Self {
            base_realm_info: base_realm_info.into(),
            r#type: r#type.into(),
        }
    }
}
impl PaintWorkletRealmInfo {
    pub const IDENTIFIER: &'static str = "script.PaintWorkletRealmInfo";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioWorkletRealmInfo {
    #[serde(flatten)]
    #[serde(default)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: AudioWorkletRealmInfoType,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioWorkletRealmInfoType {
    #[serde(rename = "audio-worklet")]
    AudioWorklet,
}
impl AudioWorkletRealmInfo {
    pub fn new(
        base_realm_info: impl Into<BaseRealmInfo>,
        r#type: impl Into<AudioWorkletRealmInfoType>,
    ) -> Self {
        Self {
            base_realm_info: base_realm_info.into(),
            r#type: r#type.into(),
        }
    }
}
impl AudioWorkletRealmInfo {
    pub const IDENTIFIER: &'static str = "script.AudioWorkletRealmInfo";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkletRealmInfo {
    #[serde(flatten)]
    #[serde(default)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: WorkletRealmInfoType,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkletRealmInfoType {
    #[serde(rename = "worklet")]
    Worklet,
}
impl WorkletRealmInfo {
    pub fn new(
        base_realm_info: impl Into<BaseRealmInfo>,
        r#type: impl Into<WorkletRealmInfoType>,
    ) -> Self {
        Self {
            base_realm_info: base_realm_info.into(),
            r#type: r#type.into(),
        }
    }
}
impl WorkletRealmInfo {
    pub const IDENTIFIER: &'static str = "script.WorkletRealmInfo";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "realm")]
    pub realm: Realm,
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub context: Option<crate::browsing_context::types::BrowsingContext>,
}
impl Source {
    pub fn new(realm: impl Into<Realm>) -> Self {
        Self {
            realm: realm.into(),
            context: None,
        }
    }
}
impl Source {
    pub const IDENTIFIER: &'static str = "script.Source";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
group_enum ! (ScriptType { Channel (Channel) , ChannelValue (ChannelValue) , ChannelProperties (ChannelProperties) , EvaluateResultSuccess (EvaluateResultSuccess) , EvaluateResultException (EvaluateResultException) , ExceptionDetails (ExceptionDetails) , Handle (Handle) , InternalId (InternalId) , LocalValue (LocalValue) , ListLocalValue (ListLocalValue) , ArrayLocalValue (ArrayLocalValue) , DateLocalValue (DateLocalValue) , MappingLocalValue (MappingLocalValue) , MapLocalValue (MapLocalValue) , ObjectLocalValue (ObjectLocalValue) , RegExpValue (RegExpValue) , RegExpLocalValue (RegExpLocalValue) , SetLocalValue (SetLocalValue) , PreloadScript (PreloadScript) , Realm (Realm) , SpecialNumber (SpecialNumber) , RealmType (RealmType) , RemoteReference (RemoteReference) , RemoteValue (RemoteValue) , ListRemoteValue (ListRemoteValue) , MappingRemoteValue (MappingRemoteValue) , HtmlCollectionRemoteValue (HtmlCollectionRemoteValue) , NodeProperties (NodeProperties) , WindowProxyProperties (WindowProxyProperties) , ResultOwnership (ResultOwnership) , SerializationOptions (SerializationOptions) , SharedId (SharedId) , StackFrame (StackFrame) , StackTrace (StackTrace) , Target (Target) , RealmInfo (RealmInfo) , BaseRealmInfo (BaseRealmInfo) , Source (Source) });
