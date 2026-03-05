use serde::{Deserialize, Serialize};
#[doc = "Unique script identifier.\n[ScriptId](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-ScriptId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct ScriptId(String);
impl ScriptId {
    pub fn new(val: impl Into<String>) -> Self {
        ScriptId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for ScriptId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<ScriptId> for String {
    fn from(el: ScriptId) -> String {
        el.0
    }
}
impl From<String> for ScriptId {
    fn from(expr: String) -> Self {
        ScriptId(expr)
    }
}
impl std::borrow::Borrow<str> for ScriptId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl ScriptId {
    pub const IDENTIFIER: &'static str = "Runtime.ScriptId";
}
#[doc = "Represents options for serialization. Overrides `generatePreview` and `returnByValue`.\n[SerializationOptions](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-SerializationOptions)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerializationOptions {
    #[serde(rename = "serialization")]
    pub serialization: SerializationOptionsSerialization,
    #[doc = "Deep serialization depth. Default is full depth. Respected only in `deep` serialization mode."]
    #[serde(rename = "maxDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_depth: Option<i64>,
    #[doc = "Embedder-specific parameters. For example if connected to V8 in Chrome these control DOM\nserialization via `maxNodeDepth: integer` and `includeShadowTree: \"none\" | \"open\" | \"all\"`.\nValues can be only of type string or integer."]
    #[serde(rename = "additionalParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub additional_parameters: Option<serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SerializationOptionsSerialization {
    #[doc = "Whether the result should be deep-serialized. The result is put into\n`deepSerializedValue` and `ObjectId` is provided."]
    #[serde(rename = "deep")]
    Deep,
    #[doc = "Whether the result is expected to be a JSON object which should be sent by value.\nThe result is put either into `value` or into `unserializableValue`. Synonym of\n`returnByValue: true`. Overrides `returnByValue`."]
    #[serde(rename = "json")]
    Json,
    #[doc = "Only remote object id is put in the result. Same bahaviour as if no\n`serializationOptions`, `generatePreview` nor `returnByValue` are provided."]
    #[serde(rename = "idOnly")]
    IdOnly,
}
impl SerializationOptions {
    pub fn new(serialization: impl Into<SerializationOptionsSerialization>) -> Self {
        Self {
            serialization: serialization.into(),
            max_depth: None,
            additional_parameters: None,
        }
    }
}
impl SerializationOptions {
    pub const IDENTIFIER: &'static str = "Runtime.SerializationOptions";
}
#[doc = "Represents deep serialized value.\n[DeepSerializedValue](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-DeepSerializedValue)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeepSerializedValue {
    #[serde(rename = "type")]
    pub r#type: DeepSerializedValueType,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<String>,
    #[doc = "Set if value reference met more then once during serialization. In such\ncase, value is provided only to one of the serialized values. Unique\nper value in the scope of one CDP call."]
    #[serde(rename = "weakLocalObjectReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub weak_local_object_reference: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeepSerializedValueType {
    #[serde(rename = "undefined")]
    Undefined,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "bigint")]
    Bigint,
    #[serde(rename = "regexp")]
    Regexp,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "symbol")]
    Symbol,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "weakmap")]
    Weakmap,
    #[serde(rename = "weakset")]
    Weakset,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "promise")]
    Promise,
    #[serde(rename = "typedarray")]
    Typedarray,
    #[serde(rename = "arraybuffer")]
    Arraybuffer,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "window")]
    Window,
    #[serde(rename = "generator")]
    Generator,
}
impl DeepSerializedValue {
    pub fn new(r#type: impl Into<DeepSerializedValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            value: None,
            object_id: None,
            weak_local_object_reference: None,
        }
    }
}
impl DeepSerializedValue {
    pub const IDENTIFIER: &'static str = "Runtime.DeepSerializedValue";
}
#[doc = "Unique object identifier.\n[RemoteObjectId](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-RemoteObjectId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct RemoteObjectId(String);
impl RemoteObjectId {
    pub fn new(val: impl Into<String>) -> Self {
        RemoteObjectId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for RemoteObjectId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<RemoteObjectId> for String {
    fn from(el: RemoteObjectId) -> String {
        el.0
    }
}
impl From<String> for RemoteObjectId {
    fn from(expr: String) -> Self {
        RemoteObjectId(expr)
    }
}
impl std::borrow::Borrow<str> for RemoteObjectId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl RemoteObjectId {
    pub const IDENTIFIER: &'static str = "Runtime.RemoteObjectId";
}
#[doc = "Primitive value which cannot be JSON-stringified. Includes values `-0`, `NaN`, `Infinity`,\n`-Infinity`, and bigint literals.\n[UnserializableValue](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-UnserializableValue)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct UnserializableValue(String);
impl UnserializableValue {
    pub fn new(val: impl Into<String>) -> Self {
        UnserializableValue(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for UnserializableValue {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<UnserializableValue> for String {
    fn from(el: UnserializableValue) -> String {
        el.0
    }
}
impl From<String> for UnserializableValue {
    fn from(expr: String) -> Self {
        UnserializableValue(expr)
    }
}
impl UnserializableValue {
    pub const IDENTIFIER: &'static str = "Runtime.UnserializableValue";
}
#[doc = "Mirror object referencing original JavaScript object.\n[RemoteObject](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-RemoteObject)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoteObject {
    #[doc = "Object type."]
    #[serde(rename = "type")]
    pub r#type: RemoteObjectType,
    #[doc = "Object subtype hint. Specified for `object` type values only.\nNOTE: If you change anything here, make sure to also update\n`subtype` in `ObjectPreview` and `PropertyPreview` below."]
    #[serde(rename = "subtype")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub subtype: Option<RemoteObjectSubtype>,
    #[doc = "Object class (constructor) name. Specified for `object` type values only."]
    #[serde(rename = "className")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub class_name: Option<String>,
    #[doc = "Remote object value in case of primitive values or JSON values (if it was requested)."]
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[doc = "Primitive value which can not be JSON-stringified does not have `value`, but gets this\nproperty."]
    #[serde(rename = "unserializableValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unserializable_value: Option<UnserializableValue>,
    #[doc = "String representation of the object."]
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[doc = "Deep serialized value."]
    #[serde(rename = "deepSerializedValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub deep_serialized_value: Option<DeepSerializedValue>,
    #[doc = "Unique object identifier (for non-primitive values)."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<RemoteObjectId>,
    #[doc = "Preview containing abbreviated property values. Specified for `object` type values only."]
    #[serde(rename = "preview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub preview: Option<ObjectPreview>,
    #[serde(rename = "customPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_preview: Option<CustomPreview>,
}
#[doc = "Object type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RemoteObjectType {
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "undefined")]
    Undefined,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "symbol")]
    Symbol,
    #[serde(rename = "bigint")]
    Bigint,
}
#[doc = "Object subtype hint. Specified for `object` type values only.\nNOTE: If you change anything here, make sure to also update\n`subtype` in `ObjectPreview` and `PropertyPreview` below."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RemoteObjectSubtype {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "regexp")]
    Regexp,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "weakmap")]
    Weakmap,
    #[serde(rename = "weakset")]
    Weakset,
    #[serde(rename = "iterator")]
    Iterator,
    #[serde(rename = "generator")]
    Generator,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "promise")]
    Promise,
    #[serde(rename = "typedarray")]
    Typedarray,
    #[serde(rename = "arraybuffer")]
    Arraybuffer,
    #[serde(rename = "dataview")]
    Dataview,
    #[serde(rename = "webassemblymemory")]
    Webassemblymemory,
    #[serde(rename = "wasmvalue")]
    Wasmvalue,
    #[doc = "blink's subtypes."]
    #[serde(rename = "trustedtype")]
    Trustedtype,
}
impl RemoteObject {
    pub fn new(r#type: impl Into<RemoteObjectType>) -> Self {
        Self {
            r#type: r#type.into(),
            subtype: None,
            class_name: None,
            value: None,
            unserializable_value: None,
            description: None,
            deep_serialized_value: None,
            object_id: None,
            preview: None,
            custom_preview: None,
        }
    }
}
impl RemoteObject {
    pub const IDENTIFIER: &'static str = "Runtime.RemoteObject";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomPreview {
    #[doc = "The JSON-stringified result of formatter.header(object, config) call.\nIt contains json ML array that represents RemoteObject."]
    #[serde(rename = "header")]
    pub header: String,
    #[doc = "If formatter returns true as a result of formatter.hasBody call then bodyGetterId will\ncontain RemoteObjectId for the function that returns result of formatter.body(object, config) call.\nThe result value is json ML array."]
    #[serde(rename = "bodyGetterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub body_getter_id: Option<RemoteObjectId>,
}
impl CustomPreview {
    pub fn new(header: impl Into<String>) -> Self {
        Self {
            header: header.into(),
            body_getter_id: None,
        }
    }
}
impl<T: Into<String>> From<T> for CustomPreview {
    fn from(url: T) -> Self {
        CustomPreview::new(url)
    }
}
impl CustomPreview {
    pub const IDENTIFIER: &'static str = "Runtime.CustomPreview";
}
#[doc = "Object containing abbreviated remote object value.\n[ObjectPreview](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-ObjectPreview)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectPreview {
    #[doc = "Object type."]
    #[serde(rename = "type")]
    pub r#type: ObjectPreviewType,
    #[doc = "Object subtype hint. Specified for `object` type values only."]
    #[serde(rename = "subtype")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub subtype: Option<ObjectPreviewSubtype>,
    #[doc = "String representation of the object."]
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[doc = "True iff some of the properties or entries of the original object did not fit."]
    #[serde(rename = "overflow")]
    pub overflow: bool,
    #[doc = "List of the properties."]
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<PropertyPreview>,
    #[doc = "List of the entries. Specified for `map` and `set` subtype values only."]
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub entries: Option<Vec<EntryPreview>>,
}
#[doc = "Object type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectPreviewType {
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "undefined")]
    Undefined,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "symbol")]
    Symbol,
    #[serde(rename = "bigint")]
    Bigint,
}
#[doc = "Object subtype hint. Specified for `object` type values only."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectPreviewSubtype {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "regexp")]
    Regexp,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "weakmap")]
    Weakmap,
    #[serde(rename = "weakset")]
    Weakset,
    #[serde(rename = "iterator")]
    Iterator,
    #[serde(rename = "generator")]
    Generator,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "promise")]
    Promise,
    #[serde(rename = "typedarray")]
    Typedarray,
    #[serde(rename = "arraybuffer")]
    Arraybuffer,
    #[serde(rename = "dataview")]
    Dataview,
    #[serde(rename = "webassemblymemory")]
    Webassemblymemory,
    #[serde(rename = "wasmvalue")]
    Wasmvalue,
    #[doc = "blink's subtypes."]
    #[serde(rename = "trustedtype")]
    Trustedtype,
}
impl ObjectPreview {
    pub fn new(
        r#type: impl Into<ObjectPreviewType>,
        overflow: impl Into<bool>,
        properties: Vec<PropertyPreview>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            overflow: overflow.into(),
            properties,
            subtype: None,
            description: None,
            entries: None,
        }
    }
}
impl ObjectPreview {
    pub const IDENTIFIER: &'static str = "Runtime.ObjectPreview";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyPreview {
    #[doc = "Property name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Object type. Accessor means that the property itself is an accessor property."]
    #[serde(rename = "type")]
    pub r#type: PropertyPreviewType,
    #[doc = "User-friendly property value string."]
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<String>,
    #[doc = "Nested value preview."]
    #[serde(rename = "valuePreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value_preview: Option<ObjectPreview>,
    #[doc = "Object subtype hint. Specified for `object` type values only."]
    #[serde(rename = "subtype")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub subtype: Option<PropertyPreviewSubtype>,
}
#[doc = "Object type. Accessor means that the property itself is an accessor property."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropertyPreviewType {
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "undefined")]
    Undefined,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "symbol")]
    Symbol,
    #[serde(rename = "accessor")]
    Accessor,
    #[serde(rename = "bigint")]
    Bigint,
}
#[doc = "Object subtype hint. Specified for `object` type values only."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropertyPreviewSubtype {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "regexp")]
    Regexp,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "weakmap")]
    Weakmap,
    #[serde(rename = "weakset")]
    Weakset,
    #[serde(rename = "iterator")]
    Iterator,
    #[serde(rename = "generator")]
    Generator,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "promise")]
    Promise,
    #[serde(rename = "typedarray")]
    Typedarray,
    #[serde(rename = "arraybuffer")]
    Arraybuffer,
    #[serde(rename = "dataview")]
    Dataview,
    #[serde(rename = "webassemblymemory")]
    Webassemblymemory,
    #[serde(rename = "wasmvalue")]
    Wasmvalue,
    #[doc = "blink's subtypes."]
    #[serde(rename = "trustedtype")]
    Trustedtype,
}
impl PropertyPreview {
    pub fn new(name: impl Into<String>, r#type: impl Into<PropertyPreviewType>) -> Self {
        Self {
            name: name.into(),
            r#type: r#type.into(),
            value: None,
            value_preview: None,
            subtype: None,
        }
    }
}
impl PropertyPreview {
    pub const IDENTIFIER: &'static str = "Runtime.PropertyPreview";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryPreview {
    #[doc = "Preview of the key. Specified for map-like collection entries."]
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub key: Option<ObjectPreview>,
    #[doc = "Preview of the value."]
    #[serde(rename = "value")]
    pub value: ObjectPreview,
}
impl EntryPreview {
    pub fn new(value: impl Into<ObjectPreview>) -> Self {
        Self {
            value: value.into(),
            key: None,
        }
    }
}
impl EntryPreview {
    pub const IDENTIFIER: &'static str = "Runtime.EntryPreview";
}
#[doc = "Object property descriptor.\n[PropertyDescriptor](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-PropertyDescriptor)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyDescriptor {
    #[doc = "Property name or symbol description."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The value associated with the property."]
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<RemoteObject>,
    #[doc = "True if the value associated with the property may be changed (data descriptors only)."]
    #[serde(rename = "writable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub writable: Option<bool>,
    #[doc = "A function which serves as a getter for the property, or `undefined` if there is no getter\n(accessor descriptors only)."]
    #[serde(rename = "get")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub get: Option<RemoteObject>,
    #[doc = "A function which serves as a setter for the property, or `undefined` if there is no setter\n(accessor descriptors only)."]
    #[serde(rename = "set")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub set: Option<RemoteObject>,
    #[doc = "True if the type of this property descriptor may be changed and if the property may be\ndeleted from the corresponding object."]
    #[serde(rename = "configurable")]
    pub configurable: bool,
    #[doc = "True if this property shows up during enumeration of the properties on the corresponding\nobject."]
    #[serde(rename = "enumerable")]
    pub enumerable: bool,
    #[doc = "True if the result was thrown during the evaluation."]
    #[serde(rename = "wasThrown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub was_thrown: Option<bool>,
    #[doc = "True if the property is owned for the object."]
    #[serde(rename = "isOwn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_own: Option<bool>,
    #[doc = "Property symbol object, if the property is of the `symbol` type."]
    #[serde(rename = "symbol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub symbol: Option<RemoteObject>,
}
impl PropertyDescriptor {
    pub fn new(
        name: impl Into<String>,
        configurable: impl Into<bool>,
        enumerable: impl Into<bool>,
    ) -> Self {
        Self {
            name: name.into(),
            configurable: configurable.into(),
            enumerable: enumerable.into(),
            value: None,
            writable: None,
            get: None,
            set: None,
            was_thrown: None,
            is_own: None,
            symbol: None,
        }
    }
}
impl PropertyDescriptor {
    pub const IDENTIFIER: &'static str = "Runtime.PropertyDescriptor";
}
#[doc = "Object internal property descriptor. This property isn't normally visible in JavaScript code.\n[InternalPropertyDescriptor](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-InternalPropertyDescriptor)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InternalPropertyDescriptor {
    #[doc = "Conventional property name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The value associated with the property."]
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<RemoteObject>,
}
impl InternalPropertyDescriptor {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
        }
    }
}
impl<T: Into<String>> From<T> for InternalPropertyDescriptor {
    fn from(url: T) -> Self {
        InternalPropertyDescriptor::new(url)
    }
}
impl InternalPropertyDescriptor {
    pub const IDENTIFIER: &'static str = "Runtime.InternalPropertyDescriptor";
}
#[doc = "Object private field descriptor.\n[PrivatePropertyDescriptor](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-PrivatePropertyDescriptor)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivatePropertyDescriptor {
    #[doc = "Private property name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The value associated with the private property."]
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<RemoteObject>,
    #[doc = "A function which serves as a getter for the private property,\nor `undefined` if there is no getter (accessor descriptors only)."]
    #[serde(rename = "get")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub get: Option<RemoteObject>,
    #[doc = "A function which serves as a setter for the private property,\nor `undefined` if there is no setter (accessor descriptors only)."]
    #[serde(rename = "set")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub set: Option<RemoteObject>,
}
impl PrivatePropertyDescriptor {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
            get: None,
            set: None,
        }
    }
}
impl<T: Into<String>> From<T> for PrivatePropertyDescriptor {
    fn from(url: T) -> Self {
        PrivatePropertyDescriptor::new(url)
    }
}
impl PrivatePropertyDescriptor {
    pub const IDENTIFIER: &'static str = "Runtime.PrivatePropertyDescriptor";
}
#[doc = "Represents function call argument. Either remote object id `objectId`, primitive `value`,\nunserializable primitive value or neither of (for undefined) them should be specified.\n[CallArgument](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-CallArgument)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CallArgument {
    #[doc = "Primitive value or serializable javascript object."]
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[doc = "Primitive value which can not be JSON-stringified."]
    #[serde(rename = "unserializableValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unserializable_value: Option<UnserializableValue>,
    #[doc = "Remote object handle."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<RemoteObjectId>,
}
impl CallArgument {
    pub const IDENTIFIER: &'static str = "Runtime.CallArgument";
}
#[doc = "Id of an execution context.\n[ExecutionContextId](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-ExecutionContextId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Copy, Hash)]
pub struct ExecutionContextId(i64);
impl ExecutionContextId {
    pub fn new(val: impl Into<i64>) -> Self {
        ExecutionContextId(val.into())
    }
    pub fn inner(&self) -> &i64 {
        &self.0
    }
}
impl ExecutionContextId {
    pub const IDENTIFIER: &'static str = "Runtime.ExecutionContextId";
}
#[doc = "Description of an isolated world.\n[ExecutionContextDescription](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-ExecutionContextDescription)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionContextDescription {
    #[doc = "Unique id of the execution context. It can be used to specify in which execution context\nscript evaluation should be performed."]
    #[serde(rename = "id")]
    pub id: ExecutionContextId,
    #[doc = "Execution context origin."]
    #[serde(rename = "origin")]
    pub origin: String,
    #[doc = "Human readable name describing given context."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "A system-unique execution context identifier. Unlike the id, this is unique across\nmultiple processes, so can be reliably used to identify specific context while backend\nperforms a cross-process navigation."]
    #[serde(rename = "uniqueId")]
    pub unique_id: String,
    #[doc = "Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}"]
    #[serde(rename = "auxData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub aux_data: Option<serde_json::Value>,
}
impl ExecutionContextDescription {
    pub fn new(
        id: impl Into<ExecutionContextId>,
        origin: impl Into<String>,
        name: impl Into<String>,
        unique_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            origin: origin.into(),
            name: name.into(),
            unique_id: unique_id.into(),
            aux_data: None,
        }
    }
}
impl ExecutionContextDescription {
    pub const IDENTIFIER: &'static str = "Runtime.ExecutionContextDescription";
}
#[doc = "Detailed information about exception (or error) that was thrown during script compilation or\nexecution.\n[ExceptionDetails](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-ExceptionDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExceptionDetails {
    #[doc = "Exception id."]
    #[serde(rename = "exceptionId")]
    pub exception_id: i64,
    #[doc = "Exception text, which should be used together with exception object when available."]
    #[serde(rename = "text")]
    pub text: String,
    #[doc = "Line number of the exception location (0-based)."]
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    #[doc = "Column number of the exception location (0-based)."]
    #[serde(rename = "columnNumber")]
    pub column_number: i64,
    #[doc = "Script ID of the exception location."]
    #[serde(rename = "scriptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub script_id: Option<ScriptId>,
    #[doc = "URL of the exception location, to be used when the script was not reported."]
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
    #[doc = "JavaScript stack trace if available."]
    #[serde(rename = "stackTrace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stack_trace: Option<StackTrace>,
    #[doc = "Exception object if available."]
    #[serde(rename = "exception")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception: Option<RemoteObject>,
    #[doc = "Identifier of the context where exception happened."]
    #[serde(rename = "executionContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub execution_context_id: Option<ExecutionContextId>,
    #[doc = "Dictionary with entries of meta data that the client associated\nwith this exception, such as information about associated network\nrequests, etc."]
    #[serde(rename = "exceptionMetaData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_meta_data: Option<serde_json::Value>,
}
impl ExceptionDetails {
    pub fn new(
        exception_id: impl Into<i64>,
        text: impl Into<String>,
        line_number: impl Into<i64>,
        column_number: impl Into<i64>,
    ) -> Self {
        Self {
            exception_id: exception_id.into(),
            text: text.into(),
            line_number: line_number.into(),
            column_number: column_number.into(),
            script_id: None,
            url: None,
            stack_trace: None,
            exception: None,
            execution_context_id: None,
            exception_meta_data: None,
        }
    }
}
impl ExceptionDetails {
    pub const IDENTIFIER: &'static str = "Runtime.ExceptionDetails";
}
#[doc = "Number of milliseconds since epoch.\n[Timestamp](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-Timestamp)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Timestamp(f64);
impl Timestamp {
    pub fn new(val: impl Into<f64>) -> Self {
        Timestamp(val.into())
    }
    pub fn inner(&self) -> &f64 {
        &self.0
    }
}
impl Timestamp {
    pub const IDENTIFIER: &'static str = "Runtime.Timestamp";
}
#[doc = "Number of milliseconds.\n[TimeDelta](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-TimeDelta)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TimeDelta(f64);
impl TimeDelta {
    pub fn new(val: impl Into<f64>) -> Self {
        TimeDelta(val.into())
    }
    pub fn inner(&self) -> &f64 {
        &self.0
    }
}
impl TimeDelta {
    pub const IDENTIFIER: &'static str = "Runtime.TimeDelta";
}
#[doc = "Stack entry for runtime errors and assertions.\n[CallFrame](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-CallFrame)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallFrame {
    #[doc = "JavaScript function name."]
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[doc = "JavaScript script id."]
    #[serde(rename = "scriptId")]
    pub script_id: ScriptId,
    #[doc = "JavaScript script name or url."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "JavaScript script line number (0-based)."]
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    #[doc = "JavaScript script column number (0-based)."]
    #[serde(rename = "columnNumber")]
    pub column_number: i64,
}
impl CallFrame {
    pub const IDENTIFIER: &'static str = "Runtime.CallFrame";
}
#[doc = "Call frames for assertions or error messages.\n[StackTrace](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-StackTrace)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StackTrace {
    #[doc = "String label of this stack trace. For async traces this may be a name of the function that\ninitiated the async call."]
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[doc = "JavaScript function name."]
    #[serde(rename = "callFrames")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub call_frames: Vec<CallFrame>,
    #[doc = "Asynchronous JavaScript stack trace that preceded this stack, if available."]
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent: Option<Box<StackTrace>>,
    #[doc = "Asynchronous JavaScript stack trace that preceded this stack, if available."]
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent_id: Option<StackTraceId>,
}
impl StackTrace {
    pub fn new(call_frames: Vec<CallFrame>) -> Self {
        Self {
            call_frames,
            description: None,
            parent: None,
            parent_id: None,
        }
    }
}
impl StackTrace {
    pub const IDENTIFIER: &'static str = "Runtime.StackTrace";
}
#[doc = "Unique identifier of current debugger.\n[UniqueDebuggerId](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-UniqueDebuggerId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct UniqueDebuggerId(String);
impl UniqueDebuggerId {
    pub fn new(val: impl Into<String>) -> Self {
        UniqueDebuggerId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for UniqueDebuggerId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<UniqueDebuggerId> for String {
    fn from(el: UniqueDebuggerId) -> String {
        el.0
    }
}
impl From<String> for UniqueDebuggerId {
    fn from(expr: String) -> Self {
        UniqueDebuggerId(expr)
    }
}
impl std::borrow::Borrow<str> for UniqueDebuggerId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl UniqueDebuggerId {
    pub const IDENTIFIER: &'static str = "Runtime.UniqueDebuggerId";
}
#[doc = "If `debuggerId` is set stack trace comes from another debugger and can be resolved there. This\nallows to track cross-debugger calls. See `Runtime.StackTrace` and `Debugger.paused` for usages.\n[StackTraceId](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-StackTraceId)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StackTraceId {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "debuggerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub debugger_id: Option<UniqueDebuggerId>,
}
impl StackTraceId {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            debugger_id: None,
        }
    }
}
impl<T: Into<String>> From<T> for StackTraceId {
    fn from(url: T) -> Self {
        StackTraceId::new(url)
    }
}
impl StackTraceId {
    pub const IDENTIFIER: &'static str = "Runtime.StackTraceId";
}
group_enum ! (RuntimeTypes { ScriptId (ScriptId) , SerializationOptions (SerializationOptions) , DeepSerializedValue (DeepSerializedValue) , RemoteObjectId (RemoteObjectId) , UnserializableValue (UnserializableValue) , RemoteObject (RemoteObject) , CustomPreview (CustomPreview) , ObjectPreview (ObjectPreview) , PropertyPreview (PropertyPreview) , EntryPreview (EntryPreview) , PropertyDescriptor (PropertyDescriptor) , InternalPropertyDescriptor (InternalPropertyDescriptor) , PrivatePropertyDescriptor (PrivatePropertyDescriptor) , CallArgument (CallArgument) , ExecutionContextId (ExecutionContextId) , ExecutionContextDescription (ExecutionContextDescription) , ExceptionDetails (ExceptionDetails) , Timestamp (Timestamp) , TimeDelta (TimeDelta) , CallFrame (CallFrame) , StackTrace (StackTrace) , UniqueDebuggerId (UniqueDebuggerId) , StackTraceId (StackTraceId) });
