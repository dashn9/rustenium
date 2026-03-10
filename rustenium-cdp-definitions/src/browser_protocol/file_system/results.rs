use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDirectoryResult {
    #[doc = "Returns the directory object at the path."]
    #[serde(rename = "directory")]
    pub directory: super::types::Directory,
}
impl TryFrom<serde_json::Value> for GetDirectoryResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
