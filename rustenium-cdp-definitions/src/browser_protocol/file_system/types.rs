use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Timestamp"]
    #[serde(rename = "lastModified")]
    pub last_modified: super::super::network::types::TimeSinceEpoch,
    #[doc = "Size in bytes"]
    #[serde(rename = "size")]
    pub size: f64,
    #[serde(rename = "type")]
    pub r#type: String,
}
impl File {
    pub fn new(
        name: impl Into<String>,
        last_modified: impl Into<super::super::network::types::TimeSinceEpoch>,
        size: impl Into<f64>,
        r#type: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            last_modified: last_modified.into(),
            size: size.into(),
            r#type: r#type.into(),
        }
    }
}
impl File {
    pub const IDENTIFIER: &'static str = "FileSystem.File";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Directory {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nestedDirectories")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nested_directories: Vec<String>,
    #[doc = "Files that are directly nested under this directory."]
    #[serde(rename = "nestedFiles")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nested_files: Vec<File>,
}
impl Directory {
    pub fn new(
        name: impl Into<String>,
        nested_directories: Vec<String>,
        nested_files: Vec<File>,
    ) -> Self {
        Self {
            name: name.into(),
            nested_directories,
            nested_files,
        }
    }
}
impl Directory {
    pub const IDENTIFIER: &'static str = "FileSystem.Directory";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BucketFileSystemLocator {
    #[doc = "Storage key"]
    #[serde(rename = "storageKey")]
    pub storage_key: super::super::storage::types::SerializedStorageKey,
    #[doc = "Bucket name. Not passing a `bucketName` will retrieve the default Bucket. (https://developer.mozilla.org/en-US/docs/Web/API/Storage_API#storage_buckets)"]
    #[serde(rename = "bucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bucket_name: Option<String>,
    #[doc = "Path to the directory using each path component as an array item."]
    #[serde(rename = "pathComponents")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub path_components: Vec<String>,
}
impl BucketFileSystemLocator {
    pub fn new(
        storage_key: impl Into<super::super::storage::types::SerializedStorageKey>,
        path_components: Vec<String>,
    ) -> Self {
        Self {
            storage_key: storage_key.into(),
            path_components,
            bucket_name: None,
        }
    }
}
impl BucketFileSystemLocator {
    pub const IDENTIFIER: &'static str = "FileSystem.BucketFileSystemLocator";
}
group_enum ! (FileSystemTypes { File (File) , Directory (Directory) , BucketFileSystemLocator (BucketFileSystemLocator) });
