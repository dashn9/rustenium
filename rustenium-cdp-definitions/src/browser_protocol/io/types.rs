use serde::{Deserialize, Serialize};
#[doc = "This is either obtained from another method or specified as `blob:<uuid>` where\n`<uuid>` is an UUID of a Blob.\n[StreamHandle](https://chromedevtools.github.io/devtools-protocol/tot/IO/#type-StreamHandle)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct StreamHandle(String);
impl StreamHandle {
    pub fn new(val: impl Into<String>) -> Self {
        StreamHandle(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for StreamHandle {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<StreamHandle> for String {
    fn from(el: StreamHandle) -> String {
        el.0
    }
}
impl From<String> for StreamHandle {
    fn from(expr: String) -> Self {
        StreamHandle(expr)
    }
}
impl StreamHandle {
    pub const IDENTIFIER: &'static str = "IO.StreamHandle";
}
group_enum ! (Type { StreamHandle (StreamHandle) });
