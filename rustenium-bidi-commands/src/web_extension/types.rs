// Generated types for module

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArchivePathEnum {
    #[serde(rename = "archivePath")]
    ArchivePath,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionArchivePath {
    #[serde(rename = "type")]
    pub r#type: ArchivePathEnum,
    #[serde(rename = "path")]
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Base64Enum {
    #[serde(rename = "base64")]
    Base64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionBase64Encoded {
    #[serde(rename = "type")]
    pub r#type: Base64Enum,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathEnum {
    #[serde(rename = "path")]
    Path,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionPath {
    #[serde(rename = "type")]
    pub r#type: PathEnum,
    #[serde(rename = "path")]
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtensionData {
    ExtensionArchivePath(ExtensionArchivePath),
    ExtensionBase64Encoded(ExtensionBase64Encoded),
    ExtensionPath(ExtensionPath),
}

pub type Extension = String;


