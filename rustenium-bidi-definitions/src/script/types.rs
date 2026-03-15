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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
impl From<RemoteReference> for LocalValue {
    fn from(v: RemoteReference) -> Self {
        LocalValue::RemoteReference(v)
    }
}
impl TryFrom<LocalValue> for RemoteReference {
    type Error = LocalValue;
    fn try_from(e: LocalValue) -> Result<Self, Self::Error> {
        match e {
            LocalValue::RemoteReference(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<PrimitiveProtocolValue> for LocalValue {
    fn from(v: PrimitiveProtocolValue) -> Self {
        LocalValue::PrimitiveProtocolValue(v)
    }
}
impl TryFrom<LocalValue> for PrimitiveProtocolValue {
    type Error = LocalValue;
    fn try_from(e: LocalValue) -> Result<Self, Self::Error> {
        match e {
            LocalValue::PrimitiveProtocolValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<ChannelValue> for LocalValue {
    fn from(v: ChannelValue) -> Self {
        LocalValue::ChannelValue(v)
    }
}
impl TryFrom<LocalValue> for ChannelValue {
    type Error = LocalValue;
    fn try_from(e: LocalValue) -> Result<Self, Self::Error> {
        match e {
            LocalValue::ChannelValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<ArrayLocalValue> for LocalValue {
    fn from(v: ArrayLocalValue) -> Self {
        LocalValue::ArrayLocalValue(v)
    }
}
impl TryFrom<LocalValue> for ArrayLocalValue {
    type Error = LocalValue;
    fn try_from(e: LocalValue) -> Result<Self, Self::Error> {
        match e {
            LocalValue::ArrayLocalValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<DateLocalValue> for LocalValue {
    fn from(v: DateLocalValue) -> Self {
        LocalValue::DateLocalValue(v)
    }
}
impl TryFrom<LocalValue> for DateLocalValue {
    type Error = LocalValue;
    fn try_from(e: LocalValue) -> Result<Self, Self::Error> {
        match e {
            LocalValue::DateLocalValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<MapLocalValue> for LocalValue {
    fn from(v: MapLocalValue) -> Self {
        LocalValue::MapLocalValue(v)
    }
}
impl TryFrom<LocalValue> for MapLocalValue {
    type Error = LocalValue;
    fn try_from(e: LocalValue) -> Result<Self, Self::Error> {
        match e {
            LocalValue::MapLocalValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<ObjectLocalValue> for LocalValue {
    fn from(v: ObjectLocalValue) -> Self {
        LocalValue::ObjectLocalValue(v)
    }
}
impl TryFrom<LocalValue> for ObjectLocalValue {
    type Error = LocalValue;
    fn try_from(e: LocalValue) -> Result<Self, Self::Error> {
        match e {
            LocalValue::ObjectLocalValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<RegExpLocalValue> for LocalValue {
    fn from(v: RegExpLocalValue) -> Self {
        LocalValue::RegExpLocalValue(v)
    }
}
impl TryFrom<LocalValue> for RegExpLocalValue {
    type Error = LocalValue;
    fn try_from(e: LocalValue) -> Result<Self, Self::Error> {
        match e {
            LocalValue::RegExpLocalValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<SetLocalValue> for LocalValue {
    fn from(v: SetLocalValue) -> Self {
        LocalValue::SetLocalValue(v)
    }
}
impl TryFrom<LocalValue> for SetLocalValue {
    type Error = LocalValue;
    fn try_from(e: LocalValue) -> Result<Self, Self::Error> {
        match e {
            LocalValue::SetLocalValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
impl From<UndefinedValue> for PrimitiveProtocolValue {
    fn from(v: UndefinedValue) -> Self {
        PrimitiveProtocolValue::UndefinedValue(v)
    }
}
impl TryFrom<PrimitiveProtocolValue> for UndefinedValue {
    type Error = PrimitiveProtocolValue;
    fn try_from(e: PrimitiveProtocolValue) -> Result<Self, Self::Error> {
        match e {
            PrimitiveProtocolValue::UndefinedValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<NullValue> for PrimitiveProtocolValue {
    fn from(v: NullValue) -> Self {
        PrimitiveProtocolValue::NullValue(v)
    }
}
impl TryFrom<PrimitiveProtocolValue> for NullValue {
    type Error = PrimitiveProtocolValue;
    fn try_from(e: PrimitiveProtocolValue) -> Result<Self, Self::Error> {
        match e {
            PrimitiveProtocolValue::NullValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<StringValue> for PrimitiveProtocolValue {
    fn from(v: StringValue) -> Self {
        PrimitiveProtocolValue::StringValue(v)
    }
}
impl TryFrom<PrimitiveProtocolValue> for StringValue {
    type Error = PrimitiveProtocolValue;
    fn try_from(e: PrimitiveProtocolValue) -> Result<Self, Self::Error> {
        match e {
            PrimitiveProtocolValue::StringValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<NumberValue> for PrimitiveProtocolValue {
    fn from(v: NumberValue) -> Self {
        PrimitiveProtocolValue::NumberValue(v)
    }
}
impl TryFrom<PrimitiveProtocolValue> for NumberValue {
    type Error = PrimitiveProtocolValue;
    fn try_from(e: PrimitiveProtocolValue) -> Result<Self, Self::Error> {
        match e {
            PrimitiveProtocolValue::NumberValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<BooleanValue> for PrimitiveProtocolValue {
    fn from(v: BooleanValue) -> Self {
        PrimitiveProtocolValue::BooleanValue(v)
    }
}
impl TryFrom<PrimitiveProtocolValue> for BooleanValue {
    type Error = PrimitiveProtocolValue;
    fn try_from(e: PrimitiveProtocolValue) -> Result<Self, Self::Error> {
        match e {
            PrimitiveProtocolValue::BooleanValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<BigIntValue> for PrimitiveProtocolValue {
    fn from(v: BigIntValue) -> Self {
        PrimitiveProtocolValue::BigIntValue(v)
    }
}
impl TryFrom<PrimitiveProtocolValue> for BigIntValue {
    type Error = PrimitiveProtocolValue;
    fn try_from(e: PrimitiveProtocolValue) -> Result<Self, Self::Error> {
        match e {
            PrimitiveProtocolValue::BigIntValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
impl From<SharedReference> for RemoteReference {
    fn from(v: SharedReference) -> Self {
        RemoteReference::SharedReference(v)
    }
}
impl TryFrom<RemoteReference> for SharedReference {
    type Error = RemoteReference;
    fn try_from(e: RemoteReference) -> Result<Self, Self::Error> {
        match e {
            RemoteReference::SharedReference(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<RemoteObjectReference> for RemoteReference {
    fn from(v: RemoteObjectReference) -> Self {
        RemoteReference::RemoteObjectReference(v)
    }
}
impl TryFrom<RemoteReference> for RemoteObjectReference {
    type Error = RemoteReference;
    fn try_from(e: RemoteReference) -> Result<Self, Self::Error> {
        match e {
            RemoteReference::RemoteObjectReference(inner) => Ok(inner),
            other => Err(other),
        }
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
impl From<PrimitiveProtocolValue> for RemoteValue {
    fn from(v: PrimitiveProtocolValue) -> Self {
        RemoteValue::PrimitiveProtocolValue(v)
    }
}
impl TryFrom<RemoteValue> for PrimitiveProtocolValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::PrimitiveProtocolValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<SymbolRemoteValue> for RemoteValue {
    fn from(v: SymbolRemoteValue) -> Self {
        RemoteValue::SymbolRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for SymbolRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::SymbolRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<ArrayRemoteValue> for RemoteValue {
    fn from(v: ArrayRemoteValue) -> Self {
        RemoteValue::ArrayRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for ArrayRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::ArrayRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<ObjectRemoteValue> for RemoteValue {
    fn from(v: ObjectRemoteValue) -> Self {
        RemoteValue::ObjectRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for ObjectRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::ObjectRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<FunctionRemoteValue> for RemoteValue {
    fn from(v: FunctionRemoteValue) -> Self {
        RemoteValue::FunctionRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for FunctionRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::FunctionRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<RegExpRemoteValue> for RemoteValue {
    fn from(v: RegExpRemoteValue) -> Self {
        RemoteValue::RegExpRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for RegExpRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::RegExpRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<DateRemoteValue> for RemoteValue {
    fn from(v: DateRemoteValue) -> Self {
        RemoteValue::DateRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for DateRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::DateRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<MapRemoteValue> for RemoteValue {
    fn from(v: MapRemoteValue) -> Self {
        RemoteValue::MapRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for MapRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::MapRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<SetRemoteValue> for RemoteValue {
    fn from(v: SetRemoteValue) -> Self {
        RemoteValue::SetRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for SetRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::SetRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<WeakMapRemoteValue> for RemoteValue {
    fn from(v: WeakMapRemoteValue) -> Self {
        RemoteValue::WeakMapRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for WeakMapRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::WeakMapRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<WeakSetRemoteValue> for RemoteValue {
    fn from(v: WeakSetRemoteValue) -> Self {
        RemoteValue::WeakSetRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for WeakSetRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::WeakSetRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<GeneratorRemoteValue> for RemoteValue {
    fn from(v: GeneratorRemoteValue) -> Self {
        RemoteValue::GeneratorRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for GeneratorRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::GeneratorRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<ErrorRemoteValue> for RemoteValue {
    fn from(v: ErrorRemoteValue) -> Self {
        RemoteValue::ErrorRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for ErrorRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::ErrorRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<ProxyRemoteValue> for RemoteValue {
    fn from(v: ProxyRemoteValue) -> Self {
        RemoteValue::ProxyRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for ProxyRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::ProxyRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<PromiseRemoteValue> for RemoteValue {
    fn from(v: PromiseRemoteValue) -> Self {
        RemoteValue::PromiseRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for PromiseRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::PromiseRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<TypedArrayRemoteValue> for RemoteValue {
    fn from(v: TypedArrayRemoteValue) -> Self {
        RemoteValue::TypedArrayRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for TypedArrayRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::TypedArrayRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<ArrayBufferRemoteValue> for RemoteValue {
    fn from(v: ArrayBufferRemoteValue) -> Self {
        RemoteValue::ArrayBufferRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for ArrayBufferRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::ArrayBufferRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<NodeListRemoteValue> for RemoteValue {
    fn from(v: NodeListRemoteValue) -> Self {
        RemoteValue::NodeListRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for NodeListRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::NodeListRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<HtmlCollectionRemoteValue> for RemoteValue {
    fn from(v: HtmlCollectionRemoteValue) -> Self {
        RemoteValue::HtmlCollectionRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for HtmlCollectionRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::HtmlCollectionRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<NodeRemoteValue> for RemoteValue {
    fn from(v: NodeRemoteValue) -> Self {
        RemoteValue::NodeRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for NodeRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::NodeRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<WindowProxyRemoteValue> for RemoteValue {
    fn from(v: WindowProxyRemoteValue) -> Self {
        RemoteValue::WindowProxyRemoteValue(v)
    }
}
impl TryFrom<RemoteValue> for WindowProxyRemoteValue {
    type Error = RemoteValue;
    fn try_from(e: RemoteValue) -> Result<Self, Self::Error> {
        match e {
            RemoteValue::WindowProxyRemoteValue(inner) => Ok(inner),
            other => Err(other),
        }
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Target {
    ContextTarget(ContextTarget),
    RealmTarget(RealmTarget),
}
impl From<ContextTarget> for Target {
    fn from(v: ContextTarget) -> Self {
        Target::ContextTarget(v)
    }
}
impl TryFrom<Target> for ContextTarget {
    type Error = Target;
    fn try_from(e: Target) -> Result<Self, Self::Error> {
        match e {
            Target::ContextTarget(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<RealmTarget> for Target {
    fn from(v: RealmTarget) -> Self {
        Target::RealmTarget(v)
    }
}
impl TryFrom<Target> for RealmTarget {
    type Error = Target;
    fn try_from(e: Target) -> Result<Self, Self::Error> {
        match e {
            Target::RealmTarget(inner) => Ok(inner),
            other => Err(other),
        }
    }
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
impl From<WindowRealmInfo> for RealmInfo {
    fn from(v: WindowRealmInfo) -> Self {
        RealmInfo::WindowRealmInfo(v)
    }
}
impl TryFrom<RealmInfo> for WindowRealmInfo {
    type Error = RealmInfo;
    fn try_from(e: RealmInfo) -> Result<Self, Self::Error> {
        match e {
            RealmInfo::WindowRealmInfo(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<DedicatedWorkerRealmInfo> for RealmInfo {
    fn from(v: DedicatedWorkerRealmInfo) -> Self {
        RealmInfo::DedicatedWorkerRealmInfo(v)
    }
}
impl TryFrom<RealmInfo> for DedicatedWorkerRealmInfo {
    type Error = RealmInfo;
    fn try_from(e: RealmInfo) -> Result<Self, Self::Error> {
        match e {
            RealmInfo::DedicatedWorkerRealmInfo(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<SharedWorkerRealmInfo> for RealmInfo {
    fn from(v: SharedWorkerRealmInfo) -> Self {
        RealmInfo::SharedWorkerRealmInfo(v)
    }
}
impl TryFrom<RealmInfo> for SharedWorkerRealmInfo {
    type Error = RealmInfo;
    fn try_from(e: RealmInfo) -> Result<Self, Self::Error> {
        match e {
            RealmInfo::SharedWorkerRealmInfo(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<ServiceWorkerRealmInfo> for RealmInfo {
    fn from(v: ServiceWorkerRealmInfo) -> Self {
        RealmInfo::ServiceWorkerRealmInfo(v)
    }
}
impl TryFrom<RealmInfo> for ServiceWorkerRealmInfo {
    type Error = RealmInfo;
    fn try_from(e: RealmInfo) -> Result<Self, Self::Error> {
        match e {
            RealmInfo::ServiceWorkerRealmInfo(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<WorkerRealmInfo> for RealmInfo {
    fn from(v: WorkerRealmInfo) -> Self {
        RealmInfo::WorkerRealmInfo(v)
    }
}
impl TryFrom<RealmInfo> for WorkerRealmInfo {
    type Error = RealmInfo;
    fn try_from(e: RealmInfo) -> Result<Self, Self::Error> {
        match e {
            RealmInfo::WorkerRealmInfo(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<PaintWorkletRealmInfo> for RealmInfo {
    fn from(v: PaintWorkletRealmInfo) -> Self {
        RealmInfo::PaintWorkletRealmInfo(v)
    }
}
impl TryFrom<RealmInfo> for PaintWorkletRealmInfo {
    type Error = RealmInfo;
    fn try_from(e: RealmInfo) -> Result<Self, Self::Error> {
        match e {
            RealmInfo::PaintWorkletRealmInfo(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<AudioWorkletRealmInfo> for RealmInfo {
    fn from(v: AudioWorkletRealmInfo) -> Self {
        RealmInfo::AudioWorkletRealmInfo(v)
    }
}
impl TryFrom<RealmInfo> for AudioWorkletRealmInfo {
    type Error = RealmInfo;
    fn try_from(e: RealmInfo) -> Result<Self, Self::Error> {
        match e {
            RealmInfo::AudioWorkletRealmInfo(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<WorkletRealmInfo> for RealmInfo {
    fn from(v: WorkletRealmInfo) -> Self {
        RealmInfo::WorkletRealmInfo(v)
    }
}
impl TryFrom<RealmInfo> for WorkletRealmInfo {
    type Error = RealmInfo;
    fn try_from(e: RealmInfo) -> Result<Self, Self::Error> {
        match e {
            RealmInfo::WorkletRealmInfo(inner) => Ok(inner),
            other => Err(other),
        }
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
