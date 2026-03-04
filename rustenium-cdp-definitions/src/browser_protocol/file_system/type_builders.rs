use super::types::*;
impl File {
    pub fn builder() -> FileBuilder {
        FileBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FileBuilder {
    name: Option<String>,
    last_modified: Option<super::super::network::types::TimeSinceEpoch>,
    size: Option<f64>,
    r#type: Option<String>,
}
impl FileBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn last_modified(
        mut self,
        last_modified: impl Into<super::super::network::types::TimeSinceEpoch>,
    ) -> Self {
        self.last_modified = Some(last_modified.into());
        self
    }
    pub fn size(mut self, size: impl Into<f64>) -> Self {
        self.size = Some(size.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<File, String> {
        Ok(File {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            last_modified: self.last_modified.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(last_modified))
            })?,
            size: self
                .size
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(size)))?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl Directory {
    pub fn builder() -> DirectoryBuilder {
        DirectoryBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DirectoryBuilder {
    name: Option<String>,
    nested_directories: Option<Vec<String>>,
    nested_files: Option<Vec<File>>,
}
impl DirectoryBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn nested_directorie(mut self, nested_directorie: impl Into<String>) -> Self {
        let v = self.nested_directories.get_or_insert(Vec::new());
        v.push(nested_directorie.into());
        self
    }
    pub fn nested_directories<I, S>(mut self, nested_directories: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.nested_directories.get_or_insert(Vec::new());
        for val in nested_directories {
            v.push(val.into());
        }
        self
    }
    pub fn nested_file(mut self, nested_file: impl Into<File>) -> Self {
        let v = self.nested_files.get_or_insert(Vec::new());
        v.push(nested_file.into());
        self
    }
    pub fn nested_files<I, S>(mut self, nested_files: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<File>,
    {
        let v = self.nested_files.get_or_insert(Vec::new());
        for val in nested_files {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<Directory, String> {
        Ok(Directory {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            nested_directories: self.nested_directories.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(nested_directories)
                )
            })?,
            nested_files: self.nested_files.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(nested_files))
            })?,
        })
    }
}
impl BucketFileSystemLocator {
    pub fn builder() -> BucketFileSystemLocatorBuilder {
        BucketFileSystemLocatorBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct BucketFileSystemLocatorBuilder {
    storage_key: Option<super::super::storage::types::SerializedStorageKey>,
    bucket_name: Option<String>,
    path_components: Option<Vec<String>>,
}
impl BucketFileSystemLocatorBuilder {
    pub fn storage_key(
        mut self,
        storage_key: impl Into<super::super::storage::types::SerializedStorageKey>,
    ) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn bucket_name(mut self, bucket_name: impl Into<String>) -> Self {
        self.bucket_name = Some(bucket_name.into());
        self
    }
    pub fn path_component(mut self, path_component: impl Into<String>) -> Self {
        let v = self.path_components.get_or_insert(Vec::new());
        v.push(path_component.into());
        self
    }
    pub fn path_components<I, S>(mut self, path_components: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.path_components.get_or_insert(Vec::new());
        for val in path_components {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<BucketFileSystemLocator, String> {
        Ok(BucketFileSystemLocator {
            storage_key: self
                .storage_key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(storage_key)))?,
            bucket_name: self.bucket_name,
            path_components: self.path_components.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(path_components))
            })?,
        })
    }
}
