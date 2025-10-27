// Generated types for module
use serde::{Serialize, Deserialize};
use crate::browsing_context::types::BrowsingContext;
use crate::Extensible;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelEnum {
    #[serde(rename = "channel")]
    Channel,
}

pub type Channel = String;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializationOptionsincludeShadowTreeUnion {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "all")]
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializationOptions {
    #[serde(rename = "maxDomDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_dom_depth: Option<Option<u64>>,
    #[serde(rename = "maxObjectDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_object_depth: Option<Option<u64>>,
    #[serde(rename = "includeShadowTree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shadow_tree: Option<SerializationOptionsincludeShadowTreeUnion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResultOwnership {
    #[serde(rename = "root")]
    Root,
    #[serde(rename = "none")]
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelProperties {
    #[serde(rename = "channel")]
    pub channel: Channel,
    #[serde(rename = "serializationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialization_options: Option<SerializationOptions>,
    #[serde(rename = "ownership")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<ResultOwnership>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelValue {
    #[serde(rename = "type")]
    pub r#type: ChannelEnum,
    #[serde(rename = "value")]
    pub value: ChannelProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextTarget {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "sandbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<String>,
}

pub type Realm = String;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmTarget {
    #[serde(rename = "realm")]
    pub realm: Realm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Target {
    ContextTarget(ContextTarget),
    RealmTarget(RealmTarget),
}

pub type SharedId = String;


pub type Handle = String;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedReference {
    #[serde(rename = "sharedId")]
    pub shared_id: SharedId,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteObjectReference {
    #[serde(rename = "handle")]
    pub handle: Handle,
    #[serde(rename = "sharedId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_id: Option<SharedId>,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoteReference {
    SharedReference(SharedReference),
    RemoteObjectReference(RemoteObjectReference),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UndefinedEnum {
    #[serde(rename = "undefined")]
    Undefined,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndefinedValue {
    #[serde(rename = "type")]
    pub r#type: UndefinedEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NullEnum {
    #[serde(rename = "null")]
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NullValue {
    #[serde(rename = "type")]
    pub r#type: NullEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StringEnum {
    #[serde(rename = "string")]
    String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringValue {
    #[serde(rename = "type")]
    pub r#type: StringEnum,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NumberEnum {
    #[serde(rename = "number")]
    Number,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecialNumber {
    #[serde(rename = "NaN")]
    NaN,
    #[serde(rename = "-0")]
    Negative0,
    #[serde(rename = "Infinity")]
    Infinity,
    #[serde(rename = "-Infinity")]
    NegativeInfinity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumberValuevalueUnion {
    Number(serde_json::value::Number),
    SpecialNumber(SpecialNumber),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberValue {
    #[serde(rename = "type")]
    pub r#type: NumberEnum,
    #[serde(rename = "value")]
    pub value: NumberValuevalueUnion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BooleanEnum {
    #[serde(rename = "boolean")]
    Boolean,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BooleanValue {
    #[serde(rename = "type")]
    pub r#type: BooleanEnum,
    #[serde(rename = "value")]
    pub value: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BigintEnum {
    #[serde(rename = "bigint")]
    Bigint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BigIntValue {
    #[serde(rename = "type")]
    pub r#type: BigintEnum,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrimitiveProtocolValue {
    UndefinedValue(UndefinedValue),
    NullValue(NullValue),
    StringValue(StringValue),
    NumberValue(NumberValue),
    BooleanValue(BooleanValue),
    BigIntValue(BigIntValue),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrayEnum {
    #[serde(rename = "array")]
    Array,
}

pub type ListLocalValue = Vec<LocalValue>;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayLocalValue {
    #[serde(rename = "type")]
    pub r#type: ArrayEnum,
    #[serde(rename = "value")]
    pub value: ListLocalValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DateEnum {
    #[serde(rename = "date")]
    Date,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateLocalValue {
    #[serde(rename = "type")]
    pub r#type: DateEnum,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapEnum {
    #[serde(rename = "map")]
    Map,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MappingLocalValueMappingLocalValueUnion {
    LocalValue(LocalValue),
    String(String),
}

pub type MappingLocalValue = Vec<(MappingLocalValueMappingLocalValueUnion, LocalValue)>;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapLocalValue {
    #[serde(rename = "type")]
    pub r#type: MapEnum,
    #[serde(rename = "value")]
    pub value: MappingLocalValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectEnum {
    #[serde(rename = "object")]
    Object,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectLocalValue {
    #[serde(rename = "type")]
    pub r#type: ObjectEnum,
    #[serde(rename = "value")]
    pub value: MappingLocalValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegexpEnum {
    #[serde(rename = "regexp")]
    Regexp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegExpValue {
    #[serde(rename = "pattern")]
    pub pattern: String,
    #[serde(rename = "flags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegExpLocalValue {
    #[serde(rename = "type")]
    pub r#type: RegexpEnum,
    #[serde(rename = "value")]
    pub value: RegExpValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SetEnum {
    #[serde(rename = "set")]
    Set,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetLocalValue {
    #[serde(rename = "type")]
    pub r#type: SetEnum,
    #[serde(rename = "value")]
    pub value: ListLocalValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LocalValue {
    RemoteReference(RemoteReference),
    PrimitiveProtocolValue(PrimitiveProtocolValue),
    ChannelValue(ChannelValue),
    ArrayLocalValue(ArrayLocalValue),
    DateLocalValue(DateLocalValue),
    MapLocalValue(MapLocalValue),
    ObjectLocalValue(ObjectLocalValue),
    RegExpLocalValue(RegExpLocalValue),
    SetLocalValue(SetLocalValue),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub type PreloadScript = String;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuccessEnum {
    #[serde(rename = "success")]
    Success,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymbolEnum {
    #[serde(rename = "symbol")]
    Symbol,
}

pub type InternalId = String;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolRemoteValue {
    #[serde(rename = "type")]
    pub r#type: SymbolEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

pub type ListRemoteValue = Vec<RemoteValue>;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayRemoteValue {
    #[serde(rename = "type")]
    pub r#type: ArrayEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ListRemoteValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MappingRemoteValueMappingRemoteValueUnion {
    RemoteValue(RemoteValue),
    String(String),
}

pub type MappingRemoteValue = Vec<(MappingRemoteValueMappingRemoteValueUnion, RemoteValue)>;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectRemoteValue {
    #[serde(rename = "type")]
    pub r#type: ObjectEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MappingRemoteValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FunctionEnum {
    #[serde(rename = "function")]
    Function,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionRemoteValue {
    #[serde(rename = "type")]
    pub r#type: FunctionEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegExpRemoteValue {
    #[serde(flatten)]
    pub reg_exp_local_value: RegExpLocalValue,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRemoteValue {
    #[serde(flatten)]
    pub date_local_value: DateLocalValue,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapRemoteValue {
    #[serde(rename = "type")]
    pub r#type: MapEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MappingRemoteValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetRemoteValue {
    #[serde(rename = "type")]
    pub r#type: SetEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ListRemoteValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeakmapEnum {
    #[serde(rename = "weakmap")]
    Weakmap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeakMapRemoteValue {
    #[serde(rename = "type")]
    pub r#type: WeakmapEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaksetEnum {
    #[serde(rename = "weakset")]
    Weakset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeakSetRemoteValue {
    #[serde(rename = "type")]
    pub r#type: WeaksetEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeneratorEnum {
    #[serde(rename = "generator")]
    Generator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorRemoteValue {
    #[serde(rename = "type")]
    pub r#type: GeneratorEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorEnum {
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRemoteValue {
    #[serde(rename = "type")]
    pub r#type: ErrorEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProxyEnum {
    #[serde(rename = "proxy")]
    Proxy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyRemoteValue {
    #[serde(rename = "type")]
    pub r#type: ProxyEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PromiseEnum {
    #[serde(rename = "promise")]
    Promise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromiseRemoteValue {
    #[serde(rename = "type")]
    pub r#type: PromiseEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypedarrayEnum {
    #[serde(rename = "typedarray")]
    Typedarray,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedArrayRemoteValue {
    #[serde(rename = "type")]
    pub r#type: TypedarrayEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArraybufferEnum {
    #[serde(rename = "arraybuffer")]
    Arraybuffer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayBufferRemoteValue {
    #[serde(rename = "type")]
    pub r#type: ArraybufferEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodelistEnum {
    #[serde(rename = "nodelist")]
    Nodelist,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeListRemoteValue {
    #[serde(rename = "type")]
    pub r#type: NodelistEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ListRemoteValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HtmlcollectionEnum {
    #[serde(rename = "htmlcollection")]
    Htmlcollection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTMLCollectionRemoteValue {
    #[serde(rename = "type")]
    pub r#type: HtmlcollectionEnum,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ListRemoteValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeEnum {
    #[serde(rename = "node")]
    Node,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodePropertiesModeUnion {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeProperties {
    #[serde(rename = "nodeType")]
    pub node_type: u64,
    #[serde(rename = "childNodeCount")]
    pub child_node_count: u64,
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<NodeRemoteValue>>,
    #[serde(rename = "localName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<NodePropertiesModeUnion>,
    #[serde(rename = "namespaceURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_uri: Option<String>,
    #[serde(rename = "nodeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_value: Option<String>,
    #[serde(rename = "shadowRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_root: Option<Option<Box<NodeRemoteValue>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeRemoteValue {
    #[serde(rename = "type")]
    pub r#type: NodeEnum,
    #[serde(rename = "sharedId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_id: Option<SharedId>,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<NodeProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowEnum {
    #[serde(rename = "window")]
    Window,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowProxyProperties {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowProxyRemoteValue {
    #[serde(rename = "type")]
    pub r#type: WindowEnum,
    #[serde(rename = "value")]
    pub value: WindowProxyProperties,
    #[serde(rename = "handle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    HTMLCollectionRemoteValue(HTMLCollectionRemoteValue),
    NodeRemoteValue(NodeRemoteValue),
    WindowProxyRemoteValue(WindowProxyRemoteValue),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluateResultSuccess {
    #[serde(rename = "type")]
    pub r#type: SuccessEnum,
    #[serde(rename = "result")]
    pub result: RemoteValue,
    #[serde(rename = "realm")]
    pub realm: Realm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExceptionEnum {
    #[serde(rename = "exception")]
    Exception,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackTrace {
    #[serde(rename = "callFrames")]
    pub call_frames: Vec<StackFrame>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluateResultException {
    #[serde(rename = "type")]
    pub r#type: ExceptionEnum,
    #[serde(rename = "exceptionDetails")]
    pub exception_details: ExceptionDetails,
    #[serde(rename = "realm")]
    pub realm: Realm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseRealmInfo {
    #[serde(rename = "realm")]
    pub realm: Realm,
    #[serde(rename = "origin")]
    pub origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowRealmInfo {
    #[serde(flatten)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: WindowEnum,
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "sandbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DedicatedWorkerEnum {
    #[serde(rename = "dedicated-worker")]
    DedicatedWorker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedicatedWorkerRealmInfo {
    #[serde(flatten)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: DedicatedWorkerEnum,
    #[serde(rename = "owners")]
    pub owners: Realm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SharedWorkerEnum {
    #[serde(rename = "shared-worker")]
    SharedWorker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedWorkerRealmInfo {
    #[serde(flatten)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: SharedWorkerEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceWorkerEnum {
    #[serde(rename = "service-worker")]
    ServiceWorker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceWorkerRealmInfo {
    #[serde(flatten)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: ServiceWorkerEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkerEnum {
    #[serde(rename = "worker")]
    Worker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerRealmInfo {
    #[serde(flatten)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: WorkerEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaintWorkletEnum {
    #[serde(rename = "paint-worklet")]
    PaintWorklet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaintWorkletRealmInfo {
    #[serde(flatten)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: PaintWorkletEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioWorkletEnum {
    #[serde(rename = "audio-worklet")]
    AudioWorklet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioWorkletRealmInfo {
    #[serde(flatten)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: AudioWorkletEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkletEnum {
    #[serde(rename = "worklet")]
    Worklet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkletRealmInfo {
    #[serde(flatten)]
    pub base_realm_info: BaseRealmInfo,
    #[serde(rename = "type")]
    pub r#type: WorkletEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "realm")]
    pub realm: Realm,
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<BrowsingContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageParameters {
    #[serde(rename = "channel")]
    pub channel: Channel,
    #[serde(rename = "data")]
    pub data: RemoteValue,
    #[serde(rename = "source")]
    pub source: Source,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmDestroyedParameters {
    #[serde(rename = "realm")]
    pub realm: Realm,
}

