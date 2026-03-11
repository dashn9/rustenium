use serde::{Deserialize, Serialize};
#[doc = "Close the stream, discard any temporary backing storage.\n[close](https://chromedevtools.github.io/devtools-protocol/tot/IO/#method-close)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseParams {
    #[doc = "Handle of the stream to close."]
    #[serde(rename = "handle")]
    pub handle: super::types::StreamHandle,
}
impl CloseParams {
    pub fn new(handle: impl Into<super::types::StreamHandle>) -> Self {
        Self {
            handle: handle.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CloseMethod {
    #[serde(rename = "IO.close")]
    Close,
}
#[doc = "Close the stream, discard any temporary backing storage.\n[close](https://chromedevtools.github.io/devtools-protocol/tot/IO/#method-close)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Close {
    pub method: CloseMethod,
    pub params: CloseParams,
}
impl Close {
    pub const IDENTIFIER: &'static str = "IO.close";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Close {
    type Result = super::results::CloseResult;
}
#[doc = "Read a chunk of the stream\n[read](https://chromedevtools.github.io/devtools-protocol/tot/IO/#method-read)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadParams {
    #[doc = "Handle of the stream to read."]
    #[serde(rename = "handle")]
    pub handle: super::types::StreamHandle,
    #[doc = "Seek to the specified offset before reading (if not specified, proceed with offset\nfollowing the last read). Some types of streams may only support sequential reads."]
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub offset: Option<i64>,
    #[doc = "Maximum number of bytes to read (left upon the agent discretion if not specified)."]
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub size: Option<i64>,
}
impl ReadParams {
    pub fn new(handle: impl Into<super::types::StreamHandle>) -> Self {
        Self {
            handle: handle.into(),
            offset: None,
            size: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReadMethod {
    #[serde(rename = "IO.read")]
    Read,
}
#[doc = "Read a chunk of the stream\n[read](https://chromedevtools.github.io/devtools-protocol/tot/IO/#method-read)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Read {
    pub method: ReadMethod,
    pub params: ReadParams,
}
impl Read {
    pub const IDENTIFIER: &'static str = "IO.read";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Read {
    type Result = super::results::ReadResult;
}
#[doc = "Return UUID of Blob object specified by a remote object id.\n[resolveBlob](https://chromedevtools.github.io/devtools-protocol/tot/IO/#method-resolveBlob)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveBlobParams {
    #[doc = "Object id of a Blob object wrapper."]
    #[serde(rename = "objectId")]
    pub object_id: crate::js_protocol::runtime::types::RemoteObjectId,
}
impl ResolveBlobParams {
    pub fn new(object_id: impl Into<crate::js_protocol::runtime::types::RemoteObjectId>) -> Self {
        Self {
            object_id: object_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResolveBlobMethod {
    #[serde(rename = "IO.resolveBlob")]
    ResolveBlob,
}
#[doc = "Return UUID of Blob object specified by a remote object id.\n[resolveBlob](https://chromedevtools.github.io/devtools-protocol/tot/IO/#method-resolveBlob)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveBlob {
    pub method: ResolveBlobMethod,
    pub params: ResolveBlobParams,
}
impl ResolveBlob {
    pub const IDENTIFIER: &'static str = "IO.resolveBlob";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ResolveBlob {
    type Result = super::results::ResolveBlobResult;
}
group_enum ! (IoCommands { Close (Close) , Read (Read) , ResolveBlob (ResolveBlob) });
