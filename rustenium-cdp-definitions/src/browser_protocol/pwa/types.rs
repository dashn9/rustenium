use serde::{Deserialize, Serialize};
#[doc = "The following types are the replica of\nhttps://crsrc.org/c/chrome/browser/web_applications/proto/web_app_os_integration_state.proto;drc=9910d3be894c8f142c977ba1023f30a656bc13fc;l=67\n[FileHandlerAccept](https://chromedevtools.github.io/devtools-protocol/tot/PWA/#type-FileHandlerAccept)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileHandlerAccept {
    #[doc = "New name of the mimetype according to\nhttps://www.iana.org/assignments/media-types/media-types.xhtml"]
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "fileExtensions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_extensions: Vec<String>,
}
impl FileHandlerAccept {
    pub fn new(media_type: impl Into<String>, file_extensions: Vec<String>) -> Self {
        Self {
            media_type: media_type.into(),
            file_extensions,
        }
    }
}
impl FileHandlerAccept {
    pub const IDENTIFIER: &'static str = "PWA.FileHandlerAccept";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileHandler {
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "accepts")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accepts: Vec<FileHandlerAccept>,
    #[serde(rename = "displayName")]
    pub display_name: String,
}
impl FileHandler {
    pub fn new(
        action: impl Into<String>,
        accepts: Vec<FileHandlerAccept>,
        display_name: impl Into<String>,
    ) -> Self {
        Self {
            action: action.into(),
            accepts,
            display_name: display_name.into(),
        }
    }
}
impl FileHandler {
    pub const IDENTIFIER: &'static str = "PWA.FileHandler";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "If user prefers opening the app in browser or an app window."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisplayMode {
    #[serde(rename = "standalone")]
    Standalone,
    #[serde(rename = "browser")]
    Browser,
}
group_enum ! (PwaTypes { FileHandlerAccept (FileHandlerAccept) , FileHandler (FileHandler) , DisplayMode (DisplayMode) });
