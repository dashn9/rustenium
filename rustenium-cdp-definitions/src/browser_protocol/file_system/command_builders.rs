use super::commands::*;
impl GetDirectory {
    pub fn builder() -> GetDirectoryBuilder {
        <GetDirectoryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetDirectoryBuilder {
    bucket_file_system_locator: Option<super::types::BucketFileSystemLocator>,
}
impl GetDirectoryBuilder {
    pub fn bucket_file_system_locator(
        mut self,
        bucket_file_system_locator: impl Into<super::types::BucketFileSystemLocator>,
    ) -> Self {
        self.bucket_file_system_locator = Some(bucket_file_system_locator.into());
        self
    }
    pub fn build(self) -> Result<GetDirectory, String> {
        Ok(GetDirectory {
            method: GetDirectoryMethod::GetDirectory,
            params: GetDirectoryParams {
                bucket_file_system_locator: self.bucket_file_system_locator.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(bucket_file_system_locator)
                    )
                })?,
            },
        })
    }
}
