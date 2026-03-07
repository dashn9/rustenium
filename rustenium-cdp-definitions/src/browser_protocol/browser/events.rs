use serde::{Deserialize, Serialize};
#[doc = "Fired when page is about to start a download.\n[downloadWillBegin](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#event-downloadWillBegin)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadWillBeginParams {
    #[doc = "Id of the frame that caused the download to begin."]
    #[serde(rename = "frameId")]
    pub frame_id: crate::browser_protocol::page::types::FrameId,
    #[doc = "Global unique identifier of the download."]
    #[serde(rename = "guid")]
    pub guid: String,
    #[doc = "URL of the resource being downloaded."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Suggested file name of the resource (the actual name of the file saved on disk may differ)."]
    #[serde(rename = "suggestedFilename")]
    pub suggested_filename: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DownloadWillBeginMethod {
    #[serde(rename = "Browser.downloadWillBegin")]
    DownloadWillBegin,
}
#[doc = "Fired when page is about to start a download.\n[downloadWillBegin](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#event-downloadWillBegin)"]
#[derive(Debug, Clone, PartialEq)]
pub struct DownloadWillBegin {
    pub method: DownloadWillBeginMethod,
    pub params: DownloadWillBeginParams,
}
impl DownloadWillBegin {
    pub const IDENTIFIER: &'static str = "Browser.downloadWillBegin";
}
#[doc = "Fired when download makes progress. Last call has |done| == true.\n[downloadProgress](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#event-downloadProgress)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadProgressParams {
    #[doc = "Global unique identifier of the download."]
    #[serde(rename = "guid")]
    pub guid: String,
    #[doc = "Total expected bytes to download."]
    #[serde(rename = "totalBytes")]
    pub total_bytes: f64,
    #[doc = "Total bytes received."]
    #[serde(rename = "receivedBytes")]
    pub received_bytes: f64,
    #[doc = "Download status."]
    #[serde(rename = "state")]
    pub state: DownloadProgressState,
    #[doc = "If download is \"completed\", provides the path of the downloaded file.\nDepending on the platform, it is not guaranteed to be set, nor the file\nis guaranteed to exist."]
    #[serde(rename = "filePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub file_path: Option<String>,
}
#[doc = "Download status."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DownloadProgressState {
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "canceled")]
    Canceled,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DownloadProgressMethod {
    #[serde(rename = "Browser.downloadProgress")]
    DownloadProgress,
}
#[doc = "Fired when download makes progress. Last call has |done| == true.\n[downloadProgress](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#event-downloadProgress)"]
#[derive(Debug, Clone, PartialEq)]
pub struct DownloadProgress {
    pub method: DownloadProgressMethod,
    pub params: DownloadProgressParams,
}
impl DownloadProgress {
    pub const IDENTIFIER: &'static str = "Browser.downloadProgress";
}
group_enum ! (BrowserEvents { DownloadWillBegin (DownloadWillBegin) , DownloadProgress (DownloadProgress) });
