use super::commands::*;
impl Close {
    pub fn builder() -> CloseBuilder {
        CloseBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CloseBuilder {
    handle: Option<super::types::StreamHandle>,
}
impl CloseBuilder {
    pub fn handle(mut self, handle: impl Into<super::types::StreamHandle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn build(self) -> Result<Close, String> {
        Ok(Close {
            method: CloseMethod::Close,
            params: CloseParams {
                handle: self
                    .handle
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(handle)))?,
            },
        })
    }
}
impl Read {
    pub fn builder() -> ReadBuilder {
        ReadBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ReadBuilder {
    handle: Option<super::types::StreamHandle>,
    offset: Option<i64>,
    size: Option<i64>,
}
impl ReadBuilder {
    pub fn handle(mut self, handle: impl Into<super::types::StreamHandle>) -> Self {
        self.handle = Some(handle.into());
        self
    }
    pub fn offset(mut self, offset: impl Into<i64>) -> Self {
        self.offset = Some(offset.into());
        self
    }
    pub fn size(mut self, size: impl Into<i64>) -> Self {
        self.size = Some(size.into());
        self
    }
    pub fn build(self) -> Result<Read, String> {
        Ok(Read {
            method: ReadMethod::Read,
            params: ReadParams {
                handle: self
                    .handle
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(handle)))?,
                offset: self.offset,
                size: self.size,
            },
        })
    }
}
impl ResolveBlob {
    pub fn builder() -> ResolveBlobBuilder {
        ResolveBlobBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ResolveBlobBuilder {
    object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
}
impl ResolveBlobBuilder {
    pub fn object_id(
        mut self,
        object_id: impl Into<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
    ) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn build(self) -> Result<ResolveBlob, String> {
        Ok(ResolveBlob {
            method: ResolveBlobMethod::ResolveBlob,
            params: ResolveBlobParams {
                object_id: self.object_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(object_id))
                })?,
            },
        })
    }
}
