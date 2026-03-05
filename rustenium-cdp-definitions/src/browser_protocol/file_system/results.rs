use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDirectoryResult {
    #[doc = "Returns the directory object at the path."]
    #[serde(rename = "directory")]
    pub directory: super::types::Directory,
}
