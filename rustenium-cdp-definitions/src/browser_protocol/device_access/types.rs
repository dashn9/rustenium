use serde::{Deserialize, Serialize};
#[doc = "Device request id.\n[RequestId](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#type-RequestId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct RequestId(String);
impl RequestId {
    pub fn new(val: impl Into<String>) -> Self {
        RequestId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for RequestId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<RequestId> for String {
    fn from(el: RequestId) -> String {
        el.0
    }
}
impl From<String> for RequestId {
    fn from(expr: String) -> Self {
        RequestId(expr)
    }
}
impl std::borrow::Borrow<str> for RequestId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl RequestId {
    pub const IDENTIFIER: &'static str = "DeviceAccess.RequestId";
}
#[doc = "A device id.\n[DeviceId](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#type-DeviceId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct DeviceId(String);
impl DeviceId {
    pub fn new(val: impl Into<String>) -> Self {
        DeviceId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for DeviceId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<DeviceId> for String {
    fn from(el: DeviceId) -> String {
        el.0
    }
}
impl From<String> for DeviceId {
    fn from(expr: String) -> Self {
        DeviceId(expr)
    }
}
impl std::borrow::Borrow<str> for DeviceId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl DeviceId {
    pub const IDENTIFIER: &'static str = "DeviceAccess.DeviceId";
}
#[doc = "Device information displayed in a user prompt to select a device.\n[PromptDevice](https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#type-PromptDevice)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptDevice {
    #[serde(rename = "id")]
    pub id: DeviceId,
    #[doc = "Display name as it appears in a device request user prompt."]
    #[serde(rename = "name")]
    pub name: String,
}
impl PromptDevice {
    pub fn new(id: impl Into<DeviceId>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }
}
impl PromptDevice {
    pub const IDENTIFIER: &'static str = "DeviceAccess.PromptDevice";
}
group_enum ! (DeviceAccessTypes { RequestId (RequestId) , DeviceId (DeviceId) , PromptDevice (PromptDevice) });
