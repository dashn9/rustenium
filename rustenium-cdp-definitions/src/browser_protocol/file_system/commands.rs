use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDirectoryParams {
    #[serde(rename = "bucketFileSystemLocator")]
    pub bucket_file_system_locator: super::types::BucketFileSystemLocator,
}
impl GetDirectoryParams {
    pub fn new(
        bucket_file_system_locator: impl Into<super::types::BucketFileSystemLocator>,
    ) -> Self {
        Self {
            bucket_file_system_locator: bucket_file_system_locator.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetDirectoryMethod {
    #[serde(rename = "FileSystem.getDirectory")]
    GetDirectory,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDirectory {
    pub method: GetDirectoryMethod,
    pub params: GetDirectoryParams,
}
impl GetDirectory {
    pub const IDENTIFIER: &'static str = "FileSystem.getDirectory";
}
impl crate::CommandResult for GetDirectory {
    type Result = super::results::GetDirectoryResult;
}
group_enum ! (FileSystemCommands { GetDirectory (GetDirectory) });
