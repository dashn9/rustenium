use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct TargetId(String);
impl TargetId {
    pub fn new(val: impl Into<String>) -> Self {
        TargetId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for TargetId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<TargetId> for String {
    fn from(el: TargetId) -> String {
        el.0
    }
}
impl From<String> for TargetId {
    fn from(expr: String) -> Self {
        TargetId(expr)
    }
}
impl std::borrow::Borrow<str> for TargetId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl TargetId {
    pub const IDENTIFIER: &'static str = "Target.TargetID";
}
#[doc = "Unique identifier of attached debugging session.\n[SessionID](https://chromedevtools.github.io/devtools-protocol/tot/Target/#type-SessionID)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct SessionId(String);
impl SessionId {
    pub fn new(val: impl Into<String>) -> Self {
        SessionId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for SessionId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<SessionId> for String {
    fn from(el: SessionId) -> String {
        el.0
    }
}
impl From<String> for SessionId {
    fn from(expr: String) -> Self {
        SessionId(expr)
    }
}
impl std::borrow::Borrow<str> for SessionId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl SessionId {
    pub const IDENTIFIER: &'static str = "Target.SessionID";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetInfo {
    #[serde(rename = "targetId")]
    pub target_id: TargetId,
    #[doc = "List of types: https://source.chromium.org/chromium/chromium/src/+/main:content/browser/devtools/devtools_agent_host_impl.cc?ss=chromium&q=f:devtools%20-f:out%20%22::kTypeTab%5B%5D%22"]
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Whether the target has an attached client."]
    #[serde(rename = "attached")]
    pub attached: bool,
    #[doc = "Opener target Id"]
    #[serde(rename = "openerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub opener_id: Option<TargetId>,
    #[doc = "Whether the target has access to the originating window."]
    #[serde(rename = "canAccessOpener")]
    pub can_access_opener: bool,
    #[doc = "Frame id of originating window (is only set if target has an opener)."]
    #[serde(rename = "openerFrameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub opener_frame_id: Option<crate::browser_protocol::page::types::FrameId>,
    #[doc = "Id of the parent frame, only present for the \"iframe\" targets."]
    #[serde(rename = "parentFrameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent_frame_id: Option<crate::browser_protocol::page::types::FrameId>,
    #[serde(rename = "browserContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub browser_context_id: Option<crate::browser_protocol::browser::types::BrowserContextId>,
    #[doc = "Provides additional details for specific target types. For example, for\nthe type of \"page\", this may be set to \"prerender\"."]
    #[serde(rename = "subtype")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub subtype: Option<String>,
}
impl TargetInfo {
    pub const IDENTIFIER: &'static str = "Target.TargetInfo";
}
#[doc = "A filter used by target query/discovery/auto-attach operations.\n[FilterEntry](https://chromedevtools.github.io/devtools-protocol/tot/Target/#type-FilterEntry)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FilterEntry {
    #[doc = "If set, causes exclusion of matching targets from the list."]
    #[serde(rename = "exclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exclude: Option<bool>,
    #[doc = "If not present, matches any type."]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub r#type: Option<String>,
}
impl FilterEntry {
    pub const IDENTIFIER: &'static str = "Target.FilterEntry";
}
#[doc = "The entries in TargetFilter are matched sequentially against targets and\nthe first entry that matches determines if the target is included or not,\ndepending on the value of `exclude` field in the entry.\nIf filter is not specified, the one assumed is\n[{type: \"browser\", exclude: true}, {type: \"tab\", exclude: true}, {}]\n(i.e. include everything but `browser` and `tab`).\n[TargetFilter](https://chromedevtools.github.io/devtools-protocol/tot/Target/#type-TargetFilter)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetFilter(Vec<FilterEntry>);
impl TargetFilter {
    pub fn new(val: impl Into<Vec<FilterEntry>>) -> Self {
        TargetFilter(val.into())
    }
    pub fn inner(&self) -> &Vec<FilterEntry> {
        &self.0
    }
}
impl TargetFilter {
    pub const IDENTIFIER: &'static str = "Target.TargetFilter";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoteLocation {
    #[serde(rename = "host")]
    pub host: String,
    #[serde(rename = "port")]
    pub port: i64,
}
impl RemoteLocation {
    pub fn new(host: impl Into<String>, port: impl Into<i64>) -> Self {
        Self {
            host: host.into(),
            port: port.into(),
        }
    }
}
impl RemoteLocation {
    pub const IDENTIFIER: &'static str = "Target.RemoteLocation";
}
#[doc = "The state of the target window."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WindowState {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "minimized")]
    Minimized,
    #[serde(rename = "maximized")]
    Maximized,
    #[serde(rename = "fullscreen")]
    Fullscreen,
}
group_enum ! (TargetTypes { TargetId (TargetId) , SessionId (SessionId) , TargetInfo (TargetInfo) , FilterEntry (FilterEntry) , TargetFilter (TargetFilter) , RemoteLocation (RemoteLocation) , WindowState (WindowState) });
