use serde::{Deserialize, Serialize};
#[doc = "Database with an array of object stores.\n[DatabaseWithObjectStores](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#type-DatabaseWithObjectStores)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseWithObjectStores {
    #[doc = "Database name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Database version (type is not 'integer', as the standard\nrequires the version number to be 'unsigned long long')"]
    #[serde(rename = "version")]
    pub version: f64,
    #[doc = "Object stores in this database."]
    #[serde(rename = "objectStores")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub object_stores: Vec<ObjectStore>,
}
impl DatabaseWithObjectStores {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<f64>,
        object_stores: Vec<ObjectStore>,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            object_stores,
        }
    }
}
impl DatabaseWithObjectStores {
    pub const IDENTIFIER: &'static str = "IndexedDB.DatabaseWithObjectStores";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Object store.\n[ObjectStore](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#type-ObjectStore)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectStore {
    #[doc = "Object store name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Object store key path."]
    #[serde(rename = "keyPath")]
    pub key_path: KeyPath,
    #[doc = "If true, object store has auto increment flag set."]
    #[serde(rename = "autoIncrement")]
    pub auto_increment: bool,
    #[doc = "Indexes in this object store."]
    #[serde(rename = "indexes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub indexes: Vec<ObjectStoreIndex>,
}
impl ObjectStore {
    pub fn new(
        name: impl Into<String>,
        key_path: impl Into<KeyPath>,
        auto_increment: impl Into<bool>,
        indexes: Vec<ObjectStoreIndex>,
    ) -> Self {
        Self {
            name: name.into(),
            key_path: key_path.into(),
            auto_increment: auto_increment.into(),
            indexes,
        }
    }
}
impl ObjectStore {
    pub const IDENTIFIER: &'static str = "IndexedDB.ObjectStore";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Object store index.\n[ObjectStoreIndex](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#type-ObjectStoreIndex)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectStoreIndex {
    #[doc = "Index name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Index key path."]
    #[serde(rename = "keyPath")]
    pub key_path: KeyPath,
    #[doc = "If true, index is unique."]
    #[serde(rename = "unique")]
    pub unique: bool,
    #[doc = "If true, index allows multiple entries for a key."]
    #[serde(rename = "multiEntry")]
    pub multi_entry: bool,
}
impl ObjectStoreIndex {
    pub fn new(
        name: impl Into<String>,
        key_path: impl Into<KeyPath>,
        unique: impl Into<bool>,
        multi_entry: impl Into<bool>,
    ) -> Self {
        Self {
            name: name.into(),
            key_path: key_path.into(),
            unique: unique.into(),
            multi_entry: multi_entry.into(),
        }
    }
}
impl ObjectStoreIndex {
    pub const IDENTIFIER: &'static str = "IndexedDB.ObjectStoreIndex";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Key.\n[Key](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#type-Key)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Key {
    #[doc = "Key type."]
    #[serde(rename = "type")]
    pub r#type: KeyType,
    #[doc = "Number value."]
    #[serde(rename = "number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub number: Option<f64>,
    #[doc = "String value."]
    #[serde(rename = "string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub string: Option<String>,
    #[doc = "Date value."]
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub date: Option<f64>,
    #[doc = "Array value."]
    #[serde(rename = "array")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub array: Option<Vec<Key>>,
}
#[doc = "Key type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyType {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "array")]
    Array,
}
impl Key {
    pub fn new(r#type: impl Into<KeyType>) -> Self {
        Self {
            r#type: r#type.into(),
            number: None,
            string: None,
            date: None,
            array: None,
        }
    }
}
impl Key {
    pub const IDENTIFIER: &'static str = "IndexedDB.Key";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Key range.\n[KeyRange](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#type-KeyRange)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyRange {
    #[doc = "Lower bound."]
    #[serde(rename = "lower")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub lower: Option<Key>,
    #[doc = "Upper bound."]
    #[serde(rename = "upper")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub upper: Option<Key>,
    #[doc = "If true lower bound is open."]
    #[serde(rename = "lowerOpen")]
    pub lower_open: bool,
    #[doc = "If true upper bound is open."]
    #[serde(rename = "upperOpen")]
    pub upper_open: bool,
}
impl KeyRange {
    pub fn new(lower_open: impl Into<bool>, upper_open: impl Into<bool>) -> Self {
        Self {
            lower_open: lower_open.into(),
            upper_open: upper_open.into(),
            lower: None,
            upper: None,
        }
    }
}
impl KeyRange {
    pub const IDENTIFIER: &'static str = "IndexedDB.KeyRange";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Data entry.\n[DataEntry](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#type-DataEntry)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataEntry {
    #[doc = "Key object."]
    #[serde(rename = "key")]
    pub key: crate::js_protocol::runtime::types::RemoteObject,
    #[doc = "Primary key object."]
    #[serde(rename = "primaryKey")]
    pub primary_key: crate::js_protocol::runtime::types::RemoteObject,
    #[doc = "Value object."]
    #[serde(rename = "value")]
    pub value: crate::js_protocol::runtime::types::RemoteObject,
}
impl DataEntry {
    pub fn new(
        key: impl Into<crate::js_protocol::runtime::types::RemoteObject>,
        primary_key: impl Into<crate::js_protocol::runtime::types::RemoteObject>,
        value: impl Into<crate::js_protocol::runtime::types::RemoteObject>,
    ) -> Self {
        Self {
            key: key.into(),
            primary_key: primary_key.into(),
            value: value.into(),
        }
    }
}
impl DataEntry {
    pub const IDENTIFIER: &'static str = "IndexedDB.DataEntry";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Key path.\n[KeyPath](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#type-KeyPath)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyPath {
    #[doc = "Key path type."]
    #[serde(rename = "type")]
    pub r#type: KeyPathType,
    #[doc = "String value."]
    #[serde(rename = "string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub string: Option<String>,
    #[doc = "Array value."]
    #[serde(rename = "array")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub array: Option<Vec<String>>,
}
#[doc = "Key path type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyPathType {
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "array")]
    Array,
}
impl KeyPath {
    pub fn new(r#type: impl Into<KeyPathType>) -> Self {
        Self {
            r#type: r#type.into(),
            string: None,
            array: None,
        }
    }
}
impl KeyPath {
    pub const IDENTIFIER: &'static str = "IndexedDB.KeyPath";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (IndexedDbTypes { DatabaseWithObjectStores (DatabaseWithObjectStores) , ObjectStore (ObjectStore) , ObjectStoreIndex (ObjectStoreIndex) , Key (Key) , KeyRange (KeyRange) , DataEntry (DataEntry) , KeyPath (KeyPath) });
