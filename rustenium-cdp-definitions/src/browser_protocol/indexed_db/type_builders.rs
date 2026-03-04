use super::types::*;
impl DatabaseWithObjectStores {
    pub fn builder() -> DatabaseWithObjectStoresBuilder {
        DatabaseWithObjectStoresBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DatabaseWithObjectStoresBuilder {
    name: Option<String>,
    version: Option<f64>,
    object_stores: Option<Vec<ObjectStore>>,
}
impl DatabaseWithObjectStoresBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn version(mut self, version: impl Into<f64>) -> Self {
        self.version = Some(version.into());
        self
    }
    pub fn object_store(mut self, object_store: impl Into<ObjectStore>) -> Self {
        let v = self.object_stores.get_or_insert(Vec::new());
        v.push(object_store.into());
        self
    }
    pub fn object_stores<I, S>(mut self, object_stores: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<ObjectStore>,
    {
        let v = self.object_stores.get_or_insert(Vec::new());
        for val in object_stores {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<DatabaseWithObjectStores, String> {
        Ok(DatabaseWithObjectStores {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            version: self
                .version
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(version)))?,
            object_stores: self.object_stores.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(object_stores))
            })?,
        })
    }
}
impl ObjectStore {
    pub fn builder() -> ObjectStoreBuilder {
        ObjectStoreBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ObjectStoreBuilder {
    name: Option<String>,
    key_path: Option<KeyPath>,
    auto_increment: Option<bool>,
    indexes: Option<Vec<ObjectStoreIndex>>,
}
impl ObjectStoreBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn key_path(mut self, key_path: impl Into<KeyPath>) -> Self {
        self.key_path = Some(key_path.into());
        self
    }
    pub fn auto_increment(mut self, auto_increment: impl Into<bool>) -> Self {
        self.auto_increment = Some(auto_increment.into());
        self
    }
    pub fn indexe(mut self, indexe: impl Into<ObjectStoreIndex>) -> Self {
        let v = self.indexes.get_or_insert(Vec::new());
        v.push(indexe.into());
        self
    }
    pub fn indexes<I, S>(mut self, indexes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<ObjectStoreIndex>,
    {
        let v = self.indexes.get_or_insert(Vec::new());
        for val in indexes {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<ObjectStore, String> {
        Ok(ObjectStore {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            key_path: self
                .key_path
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key_path)))?,
            auto_increment: self.auto_increment.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(auto_increment))
            })?,
            indexes: self
                .indexes
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(indexes)))?,
        })
    }
}
impl ObjectStoreIndex {
    pub fn builder() -> ObjectStoreIndexBuilder {
        ObjectStoreIndexBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ObjectStoreIndexBuilder {
    name: Option<String>,
    key_path: Option<KeyPath>,
    unique: Option<bool>,
    multi_entry: Option<bool>,
}
impl ObjectStoreIndexBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn key_path(mut self, key_path: impl Into<KeyPath>) -> Self {
        self.key_path = Some(key_path.into());
        self
    }
    pub fn unique(mut self, unique: impl Into<bool>) -> Self {
        self.unique = Some(unique.into());
        self
    }
    pub fn multi_entry(mut self, multi_entry: impl Into<bool>) -> Self {
        self.multi_entry = Some(multi_entry.into());
        self
    }
    pub fn build(self) -> Result<ObjectStoreIndex, String> {
        Ok(ObjectStoreIndex {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            key_path: self
                .key_path
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key_path)))?,
            unique: self
                .unique
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(unique)))?,
            multi_entry: self
                .multi_entry
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(multi_entry)))?,
        })
    }
}
impl Key {
    pub fn builder() -> KeyBuilder {
        KeyBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct KeyBuilder {
    r#type: Option<KeyType>,
    number: Option<f64>,
    string: Option<String>,
    date: Option<f64>,
    array: Option<Vec<Key>>,
}
impl KeyBuilder {
    pub fn r#type(mut self, r#type: impl Into<KeyType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn number(mut self, number: impl Into<f64>) -> Self {
        self.number = Some(number.into());
        self
    }
    pub fn string(mut self, string: impl Into<String>) -> Self {
        self.string = Some(string.into());
        self
    }
    pub fn date(mut self, date: impl Into<f64>) -> Self {
        self.date = Some(date.into());
        self
    }
    pub fn array(mut self, array: impl Into<Key>) -> Self {
        let v = self.array.get_or_insert(Vec::new());
        v.push(array.into());
        self
    }
    pub fn arrays<I, S>(mut self, arrays: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Key>,
    {
        let v = self.array.get_or_insert(Vec::new());
        for val in arrays {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<Key, String> {
        Ok(Key {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            number: self.number,
            string: self.string,
            date: self.date,
            array: self.array,
        })
    }
}
impl KeyRange {
    pub fn builder() -> KeyRangeBuilder {
        KeyRangeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct KeyRangeBuilder {
    lower: Option<Key>,
    upper: Option<Key>,
    lower_open: Option<bool>,
    upper_open: Option<bool>,
}
impl KeyRangeBuilder {
    pub fn lower(mut self, lower: impl Into<Key>) -> Self {
        self.lower = Some(lower.into());
        self
    }
    pub fn upper(mut self, upper: impl Into<Key>) -> Self {
        self.upper = Some(upper.into());
        self
    }
    pub fn lower_open(mut self, lower_open: impl Into<bool>) -> Self {
        self.lower_open = Some(lower_open.into());
        self
    }
    pub fn upper_open(mut self, upper_open: impl Into<bool>) -> Self {
        self.upper_open = Some(upper_open.into());
        self
    }
    pub fn build(self) -> Result<KeyRange, String> {
        Ok(KeyRange {
            lower: self.lower,
            upper: self.upper,
            lower_open: self
                .lower_open
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(lower_open)))?,
            upper_open: self
                .upper_open
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(upper_open)))?,
        })
    }
}
impl DataEntry {
    pub fn builder() -> DataEntryBuilder {
        DataEntryBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DataEntryBuilder {
    key: Option<super::super::super::js_protocol::runtime::types::RemoteObject>,
    primary_key: Option<super::super::super::js_protocol::runtime::types::RemoteObject>,
    value: Option<super::super::super::js_protocol::runtime::types::RemoteObject>,
}
impl DataEntryBuilder {
    pub fn key(
        mut self,
        key: impl Into<super::super::super::js_protocol::runtime::types::RemoteObject>,
    ) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn primary_key(
        mut self,
        primary_key: impl Into<super::super::super::js_protocol::runtime::types::RemoteObject>,
    ) -> Self {
        self.primary_key = Some(primary_key.into());
        self
    }
    pub fn value(
        mut self,
        value: impl Into<super::super::super::js_protocol::runtime::types::RemoteObject>,
    ) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<DataEntry, String> {
        Ok(DataEntry {
            key: self
                .key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key)))?,
            primary_key: self
                .primary_key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(primary_key)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl KeyPath {
    pub fn builder() -> KeyPathBuilder {
        KeyPathBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct KeyPathBuilder {
    r#type: Option<KeyPathType>,
    string: Option<String>,
    array: Option<Vec<String>>,
}
impl KeyPathBuilder {
    pub fn r#type(mut self, r#type: impl Into<KeyPathType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn string(mut self, string: impl Into<String>) -> Self {
        self.string = Some(string.into());
        self
    }
    pub fn array(mut self, array: impl Into<String>) -> Self {
        let v = self.array.get_or_insert(Vec::new());
        v.push(array.into());
        self
    }
    pub fn arrays<I, S>(mut self, arrays: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.array.get_or_insert(Vec::new());
        for val in arrays {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<KeyPath, String> {
        Ok(KeyPath {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            string: self.string,
            array: self.array,
        })
    }
}
