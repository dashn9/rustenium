use serde::{Deserialize, Serialize};
#[doc = "Activates (focuses) the target.\n[activateTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-activateTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivateTargetParams {
    #[serde(rename = "targetId")]
    pub target_id: super::types::TargetId,
}
impl ActivateTargetParams {
    pub fn new(target_id: impl Into<super::types::TargetId>) -> Self {
        Self {
            target_id: target_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActivateTargetMethod {
    #[serde(rename = "Target.activateTarget")]
    ActivateTarget,
}
#[doc = "Activates (focuses) the target.\n[activateTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-activateTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivateTarget {
    pub method: ActivateTargetMethod,
    pub params: ActivateTargetParams,
}
impl ActivateTarget {
    pub const IDENTIFIER: &'static str = "Target.activateTarget";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ActivateTarget {
    type Result = super::results::ActivateTargetResult;
}
#[doc = "Attaches to the target with given id.\n[attachToTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-attachToTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachToTargetParams {
    #[serde(rename = "targetId")]
    pub target_id: super::types::TargetId,
    #[doc = "Enables \"flat\" access to the session via specifying sessionId attribute in the commands.\nWe plan to make this the default, deprecate non-flattened mode,\nand eventually retire it. See crbug.com/991325."]
    #[serde(rename = "flatten")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub flatten: Option<bool>,
}
impl AttachToTargetParams {
    pub fn new(target_id: impl Into<super::types::TargetId>) -> Self {
        Self {
            target_id: target_id.into(),
            flatten: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttachToTargetMethod {
    #[serde(rename = "Target.attachToTarget")]
    AttachToTarget,
}
#[doc = "Attaches to the target with given id.\n[attachToTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-attachToTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachToTarget {
    pub method: AttachToTargetMethod,
    pub params: AttachToTargetParams,
}
impl AttachToTarget {
    pub const IDENTIFIER: &'static str = "Target.attachToTarget";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for AttachToTarget {
    type Result = super::results::AttachToTargetResult;
}
#[doc = "Attaches to the browser target, only uses flat sessionId mode.\n[attachToBrowserTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-attachToBrowserTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachToBrowserTargetParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttachToBrowserTargetMethod {
    #[serde(rename = "Target.attachToBrowserTarget")]
    AttachToBrowserTarget,
}
#[doc = "Attaches to the browser target, only uses flat sessionId mode.\n[attachToBrowserTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-attachToBrowserTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachToBrowserTarget {
    pub method: AttachToBrowserTargetMethod,
    pub params: AttachToBrowserTargetParams,
}
impl AttachToBrowserTarget {
    pub const IDENTIFIER: &'static str = "Target.attachToBrowserTarget";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for AttachToBrowserTarget {
    type Result = super::results::AttachToBrowserTargetResult;
}
#[doc = "Closes the target. If the target is a page that gets closed too.\n[closeTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-closeTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseTargetParams {
    #[serde(rename = "targetId")]
    pub target_id: super::types::TargetId,
}
impl CloseTargetParams {
    pub fn new(target_id: impl Into<super::types::TargetId>) -> Self {
        Self {
            target_id: target_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CloseTargetMethod {
    #[serde(rename = "Target.closeTarget")]
    CloseTarget,
}
#[doc = "Closes the target. If the target is a page that gets closed too.\n[closeTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-closeTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseTarget {
    pub method: CloseTargetMethod,
    pub params: CloseTargetParams,
}
impl CloseTarget {
    pub const IDENTIFIER: &'static str = "Target.closeTarget";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for CloseTarget {
    type Result = super::results::CloseTargetResult;
}
#[doc = "Inject object to the target's main frame that provides a communication\nchannel with browser target.\n\nInjected object will be available as `window[bindingName]`.\n\nThe object has the following API:\n- `binding.send(json)` - a method to send messages over the remote debugging protocol\n- `binding.onmessage = json => handleMessage(json)` - a callback that will be called for the protocol notifications and command responses.\n[exposeDevToolsProtocol](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-exposeDevToolsProtocol)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExposeDevToolsProtocolParams {
    #[serde(rename = "targetId")]
    pub target_id: super::types::TargetId,
    #[doc = "Binding name, 'cdp' if not specified."]
    #[serde(rename = "bindingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub binding_name: Option<String>,
    #[doc = "If true, inherits the current root session's permissions (default: false)."]
    #[serde(rename = "inheritPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub inherit_permissions: Option<bool>,
}
impl ExposeDevToolsProtocolParams {
    pub fn new(target_id: impl Into<super::types::TargetId>) -> Self {
        Self {
            target_id: target_id.into(),
            binding_name: None,
            inherit_permissions: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExposeDevToolsProtocolMethod {
    #[serde(rename = "Target.exposeDevToolsProtocol")]
    ExposeDevToolsProtocol,
}
#[doc = "Inject object to the target's main frame that provides a communication\nchannel with browser target.\n\nInjected object will be available as `window[bindingName]`.\n\nThe object has the following API:\n- `binding.send(json)` - a method to send messages over the remote debugging protocol\n- `binding.onmessage = json => handleMessage(json)` - a callback that will be called for the protocol notifications and command responses.\n[exposeDevToolsProtocol](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-exposeDevToolsProtocol)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExposeDevToolsProtocol {
    pub method: ExposeDevToolsProtocolMethod,
    pub params: ExposeDevToolsProtocolParams,
}
impl ExposeDevToolsProtocol {
    pub const IDENTIFIER: &'static str = "Target.exposeDevToolsProtocol";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ExposeDevToolsProtocol {
    type Result = super::results::ExposeDevToolsProtocolResult;
}
#[doc = "Creates a new empty BrowserContext. Similar to an incognito profile but you can have more than\none.\n[createBrowserContext](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-createBrowserContext)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateBrowserContextParams {
    #[doc = "If specified, disposes this context when debugging session disconnects."]
    #[serde(rename = "disposeOnDetach")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dispose_on_detach: Option<bool>,
    #[doc = "Proxy server, similar to the one passed to --proxy-server"]
    #[serde(rename = "proxyServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub proxy_server: Option<String>,
    #[doc = "Proxy bypass list, similar to the one passed to --proxy-bypass-list"]
    #[serde(rename = "proxyBypassList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub proxy_bypass_list: Option<String>,
    #[doc = "An optional list of origins to grant unlimited cross-origin access to.\nParts of the URL other than those constituting origin are ignored."]
    #[serde(rename = "originsWithUniversalNetworkAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub origins_with_universal_network_access: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateBrowserContextMethod {
    #[serde(rename = "Target.createBrowserContext")]
    CreateBrowserContext,
}
#[doc = "Creates a new empty BrowserContext. Similar to an incognito profile but you can have more than\none.\n[createBrowserContext](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-createBrowserContext)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateBrowserContext {
    pub method: CreateBrowserContextMethod,
    pub params: CreateBrowserContextParams,
}
impl CreateBrowserContext {
    pub const IDENTIFIER: &'static str = "Target.createBrowserContext";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for CreateBrowserContext {
    type Result = super::results::CreateBrowserContextResult;
}
#[doc = "Returns all browser contexts created with `Target.createBrowserContext` method.\n[getBrowserContexts](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-getBrowserContexts)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBrowserContextsParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetBrowserContextsMethod {
    #[serde(rename = "Target.getBrowserContexts")]
    GetBrowserContexts,
}
#[doc = "Returns all browser contexts created with `Target.createBrowserContext` method.\n[getBrowserContexts](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-getBrowserContexts)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBrowserContexts {
    pub method: GetBrowserContextsMethod,
    pub params: GetBrowserContextsParams,
}
impl GetBrowserContexts {
    pub const IDENTIFIER: &'static str = "Target.getBrowserContexts";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetBrowserContexts {
    type Result = super::results::GetBrowserContextsResult;
}
#[doc = "Creates a new page.\n[createTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-createTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTargetParams {
    #[doc = "The initial URL the page will be navigated to. An empty string indicates about:blank."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Frame left origin in DIP (requires newWindow to be true or headless shell)."]
    #[serde(rename = "left")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub left: Option<i64>,
    #[doc = "Frame top origin in DIP (requires newWindow to be true or headless shell)."]
    #[serde(rename = "top")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub top: Option<i64>,
    #[doc = "Frame width in DIP (requires newWindow to be true or headless shell)."]
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub width: Option<i64>,
    #[doc = "Frame height in DIP (requires newWindow to be true or headless shell)."]
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub height: Option<i64>,
    #[doc = "Frame window state (requires newWindow to be true or headless shell).\nDefault is normal."]
    #[serde(rename = "windowState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub window_state: Option<super::types::WindowState>,
    #[doc = "The browser context to create the page in."]
    #[serde(rename = "browserContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub browser_context_id: Option<crate::browser_protocol::browser::types::BrowserContextId>,
    #[doc = "Whether BeginFrames for this target will be controlled via DevTools (headless shell only,\nnot supported on MacOS yet, false by default)."]
    #[serde(rename = "enableBeginFrameControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enable_begin_frame_control: Option<bool>,
    #[doc = "Whether to create a new Window or Tab (false by default, not supported by headless shell)."]
    #[serde(rename = "newWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub new_window: Option<bool>,
    #[doc = "Whether to create the target in background or foreground (false by default, not supported\nby headless shell)."]
    #[serde(rename = "background")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub background: Option<bool>,
    #[doc = "Whether to create the target of type \"tab\"."]
    #[serde(rename = "forTab")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub for_tab: Option<bool>,
    #[doc = "Whether to create a hidden target. The hidden target is observable via protocol, but not\npresent in the tab UI strip. Cannot be created with `forTab: true`, `newWindow: true` or\n`background: false`. The life-time of the tab is limited to the life-time of the session."]
    #[serde(rename = "hidden")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub hidden: Option<bool>,
    #[doc = "If specified, the option is used to determine if the new target should\nbe focused or not. By default, the focus behavior depends on the\nvalue of the background field. For example, background=false and focus=false\nwill result in the target tab being opened but the browser window remain\nunchanged (if it was in the background, it will remain in the background)\nand background=false with focus=undefined will result in the window being focused.\nUsing background: true and focus: true is not supported and will result in an error."]
    #[serde(rename = "focus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub focus: Option<bool>,
}
impl CreateTargetParams {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            left: None,
            top: None,
            width: None,
            height: None,
            window_state: None,
            browser_context_id: None,
            enable_begin_frame_control: None,
            new_window: None,
            background: None,
            for_tab: None,
            hidden: None,
            focus: None,
        }
    }
}
impl<T: Into<String>> From<T> for CreateTargetParams {
    fn from(url: T) -> Self {
        CreateTargetParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateTargetMethod {
    #[serde(rename = "Target.createTarget")]
    CreateTarget,
}
#[doc = "Creates a new page.\n[createTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-createTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTarget {
    pub method: CreateTargetMethod,
    pub params: CreateTargetParams,
}
impl CreateTarget {
    pub const IDENTIFIER: &'static str = "Target.createTarget";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for CreateTarget {
    type Result = super::results::CreateTargetResult;
}
#[doc = "Detaches session with given id.\n[detachFromTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-detachFromTarget)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DetachFromTargetParams {
    #[doc = "Session to detach."]
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub session_id: Option<super::types::SessionId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DetachFromTargetMethod {
    #[serde(rename = "Target.detachFromTarget")]
    DetachFromTarget,
}
#[doc = "Detaches session with given id.\n[detachFromTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-detachFromTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetachFromTarget {
    pub method: DetachFromTargetMethod,
    pub params: DetachFromTargetParams,
}
impl DetachFromTarget {
    pub const IDENTIFIER: &'static str = "Target.detachFromTarget";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for DetachFromTarget {
    type Result = super::results::DetachFromTargetResult;
}
#[doc = "Deletes a BrowserContext. All the belonging pages will be closed without calling their\nbeforeunload hooks.\n[disposeBrowserContext](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-disposeBrowserContext)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisposeBrowserContextParams {
    #[serde(rename = "browserContextId")]
    pub browser_context_id: crate::browser_protocol::browser::types::BrowserContextId,
}
impl DisposeBrowserContextParams {
    pub fn new(
        browser_context_id: impl Into<crate::browser_protocol::browser::types::BrowserContextId>,
    ) -> Self {
        Self {
            browser_context_id: browser_context_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisposeBrowserContextMethod {
    #[serde(rename = "Target.disposeBrowserContext")]
    DisposeBrowserContext,
}
#[doc = "Deletes a BrowserContext. All the belonging pages will be closed without calling their\nbeforeunload hooks.\n[disposeBrowserContext](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-disposeBrowserContext)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisposeBrowserContext {
    pub method: DisposeBrowserContextMethod,
    pub params: DisposeBrowserContextParams,
}
impl DisposeBrowserContext {
    pub const IDENTIFIER: &'static str = "Target.disposeBrowserContext";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for DisposeBrowserContext {
    type Result = super::results::DisposeBrowserContextResult;
}
#[doc = "Returns information about a target.\n[getTargetInfo](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-getTargetInfo)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetTargetInfoParams {
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub target_id: Option<super::types::TargetId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetTargetInfoMethod {
    #[serde(rename = "Target.getTargetInfo")]
    GetTargetInfo,
}
#[doc = "Returns information about a target.\n[getTargetInfo](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-getTargetInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTargetInfo {
    pub method: GetTargetInfoMethod,
    pub params: GetTargetInfoParams,
}
impl GetTargetInfo {
    pub const IDENTIFIER: &'static str = "Target.getTargetInfo";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetTargetInfo {
    type Result = super::results::GetTargetInfoResult;
}
#[doc = "Retrieves a list of available targets.\n[getTargets](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-getTargets)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetTargetsParams {
    #[doc = "Only targets matching filter will be reported. If filter is not specified\nand target discovery is currently enabled, a filter used for target discovery\nis used for consistency."]
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub filter: Option<super::types::TargetFilter>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetTargetsMethod {
    #[serde(rename = "Target.getTargets")]
    GetTargets,
}
#[doc = "Retrieves a list of available targets.\n[getTargets](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-getTargets)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTargets {
    pub method: GetTargetsMethod,
    pub params: GetTargetsParams,
}
impl GetTargets {
    pub const IDENTIFIER: &'static str = "Target.getTargets";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetTargets {
    type Result = super::results::GetTargetsResult;
}
#[doc = "Controls whether to automatically attach to new targets which are considered\nto be directly related to this one (for example, iframes or workers).\nWhen turned on, attaches to all existing related targets as well. When turned off,\nautomatically detaches from all currently attached targets.\nThis also clears all targets added by `autoAttachRelated` from the list of targets to watch\nfor creation of related targets.\nYou might want to call this recursively for auto-attached targets to attach\nto all available targets.\n[setAutoAttach](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-setAutoAttach)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAutoAttachParams {
    #[doc = "Whether to auto-attach to related targets."]
    #[serde(rename = "autoAttach")]
    pub auto_attach: bool,
    #[doc = "Whether to pause new targets when attaching to them. Use `Runtime.runIfWaitingForDebugger`\nto run paused targets."]
    #[serde(rename = "waitForDebuggerOnStart")]
    pub wait_for_debugger_on_start: bool,
    #[doc = "Enables \"flat\" access to the session via specifying sessionId attribute in the commands.\nWe plan to make this the default, deprecate non-flattened mode,\nand eventually retire it. See crbug.com/991325."]
    #[serde(rename = "flatten")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub flatten: Option<bool>,
    #[doc = "Only targets matching filter will be attached."]
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub filter: Option<super::types::TargetFilter>,
}
impl SetAutoAttachParams {
    pub fn new(auto_attach: impl Into<bool>, wait_for_debugger_on_start: impl Into<bool>) -> Self {
        Self {
            auto_attach: auto_attach.into(),
            wait_for_debugger_on_start: wait_for_debugger_on_start.into(),
            flatten: None,
            filter: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetAutoAttachMethod {
    #[serde(rename = "Target.setAutoAttach")]
    SetAutoAttach,
}
#[doc = "Controls whether to automatically attach to new targets which are considered\nto be directly related to this one (for example, iframes or workers).\nWhen turned on, attaches to all existing related targets as well. When turned off,\nautomatically detaches from all currently attached targets.\nThis also clears all targets added by `autoAttachRelated` from the list of targets to watch\nfor creation of related targets.\nYou might want to call this recursively for auto-attached targets to attach\nto all available targets.\n[setAutoAttach](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-setAutoAttach)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAutoAttach {
    pub method: SetAutoAttachMethod,
    pub params: SetAutoAttachParams,
}
impl SetAutoAttach {
    pub const IDENTIFIER: &'static str = "Target.setAutoAttach";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetAutoAttach {
    type Result = super::results::SetAutoAttachResult;
}
#[doc = "Adds the specified target to the list of targets that will be monitored for any related target\ncreation (such as child frames, child workers and new versions of service worker) and reported\nthrough `attachedToTarget`. The specified target is also auto-attached.\nThis cancels the effect of any previous `setAutoAttach` and is also cancelled by subsequent\n`setAutoAttach`. Only available at the Browser target.\n[autoAttachRelated](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-autoAttachRelated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoAttachRelatedParams {
    #[serde(rename = "targetId")]
    pub target_id: super::types::TargetId,
    #[doc = "Whether to pause new targets when attaching to them. Use `Runtime.runIfWaitingForDebugger`\nto run paused targets."]
    #[serde(rename = "waitForDebuggerOnStart")]
    pub wait_for_debugger_on_start: bool,
    #[doc = "Only targets matching filter will be attached."]
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub filter: Option<super::types::TargetFilter>,
}
impl AutoAttachRelatedParams {
    pub fn new(
        target_id: impl Into<super::types::TargetId>,
        wait_for_debugger_on_start: impl Into<bool>,
    ) -> Self {
        Self {
            target_id: target_id.into(),
            wait_for_debugger_on_start: wait_for_debugger_on_start.into(),
            filter: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AutoAttachRelatedMethod {
    #[serde(rename = "Target.autoAttachRelated")]
    AutoAttachRelated,
}
#[doc = "Adds the specified target to the list of targets that will be monitored for any related target\ncreation (such as child frames, child workers and new versions of service worker) and reported\nthrough `attachedToTarget`. The specified target is also auto-attached.\nThis cancels the effect of any previous `setAutoAttach` and is also cancelled by subsequent\n`setAutoAttach`. Only available at the Browser target.\n[autoAttachRelated](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-autoAttachRelated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoAttachRelated {
    pub method: AutoAttachRelatedMethod,
    pub params: AutoAttachRelatedParams,
}
impl AutoAttachRelated {
    pub const IDENTIFIER: &'static str = "Target.autoAttachRelated";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for AutoAttachRelated {
    type Result = super::results::AutoAttachRelatedResult;
}
#[doc = "Controls whether to discover available targets and notify via\n`targetCreated/targetInfoChanged/targetDestroyed` events.\n[setDiscoverTargets](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-setDiscoverTargets)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDiscoverTargetsParams {
    #[doc = "Whether to discover available targets."]
    #[serde(rename = "discover")]
    pub discover: bool,
    #[doc = "Only targets matching filter will be attached. If `discover` is false,\n`filter` must be omitted or empty."]
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub filter: Option<super::types::TargetFilter>,
}
impl SetDiscoverTargetsParams {
    pub fn new(discover: impl Into<bool>) -> Self {
        Self {
            discover: discover.into(),
            filter: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDiscoverTargetsMethod {
    #[serde(rename = "Target.setDiscoverTargets")]
    SetDiscoverTargets,
}
#[doc = "Controls whether to discover available targets and notify via\n`targetCreated/targetInfoChanged/targetDestroyed` events.\n[setDiscoverTargets](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-setDiscoverTargets)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDiscoverTargets {
    pub method: SetDiscoverTargetsMethod,
    pub params: SetDiscoverTargetsParams,
}
impl SetDiscoverTargets {
    pub const IDENTIFIER: &'static str = "Target.setDiscoverTargets";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetDiscoverTargets {
    type Result = super::results::SetDiscoverTargetsResult;
}
#[doc = "Enables target discovery for the specified locations, when `setDiscoverTargets` was set to\n`true`.\n[setRemoteLocations](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-setRemoteLocations)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRemoteLocationsParams {
    #[doc = "List of remote locations."]
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<super::types::RemoteLocation>,
}
impl SetRemoteLocationsParams {
    pub fn new(locations: Vec<super::types::RemoteLocation>) -> Self {
        Self { locations }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetRemoteLocationsMethod {
    #[serde(rename = "Target.setRemoteLocations")]
    SetRemoteLocations,
}
#[doc = "Enables target discovery for the specified locations, when `setDiscoverTargets` was set to\n`true`.\n[setRemoteLocations](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-setRemoteLocations)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRemoteLocations {
    pub method: SetRemoteLocationsMethod,
    pub params: SetRemoteLocationsParams,
}
impl SetRemoteLocations {
    pub const IDENTIFIER: &'static str = "Target.setRemoteLocations";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetRemoteLocations {
    type Result = super::results::SetRemoteLocationsResult;
}
#[doc = "Gets the targetId of the DevTools page target opened for the given target\n(if any).\n[getDevToolsTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-getDevToolsTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDevToolsTargetParams {
    #[doc = "Page or tab target ID."]
    #[serde(rename = "targetId")]
    pub target_id: super::types::TargetId,
}
impl GetDevToolsTargetParams {
    pub fn new(target_id: impl Into<super::types::TargetId>) -> Self {
        Self {
            target_id: target_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetDevToolsTargetMethod {
    #[serde(rename = "Target.getDevToolsTarget")]
    GetDevToolsTarget,
}
#[doc = "Gets the targetId of the DevTools page target opened for the given target\n(if any).\n[getDevToolsTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-getDevToolsTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDevToolsTarget {
    pub method: GetDevToolsTargetMethod,
    pub params: GetDevToolsTargetParams,
}
impl GetDevToolsTarget {
    pub const IDENTIFIER: &'static str = "Target.getDevToolsTarget";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetDevToolsTarget {
    type Result = super::results::GetDevToolsTargetResult;
}
#[doc = "Opens a DevTools window for the target.\n[openDevTools](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-openDevTools)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenDevToolsParams {
    #[doc = "This can be the page or tab target ID."]
    #[serde(rename = "targetId")]
    pub target_id: super::types::TargetId,
    #[doc = "The id of the panel we want DevTools to open initially. Currently\nsupported panels are elements, console, network, sources, resources\nand performance."]
    #[serde(rename = "panelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub panel_id: Option<String>,
}
impl OpenDevToolsParams {
    pub fn new(target_id: impl Into<super::types::TargetId>) -> Self {
        Self {
            target_id: target_id.into(),
            panel_id: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OpenDevToolsMethod {
    #[serde(rename = "Target.openDevTools")]
    OpenDevTools,
}
#[doc = "Opens a DevTools window for the target.\n[openDevTools](https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-openDevTools)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenDevTools {
    pub method: OpenDevToolsMethod,
    pub params: OpenDevToolsParams,
}
impl OpenDevTools {
    pub const IDENTIFIER: &'static str = "Target.openDevTools";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for OpenDevTools {
    type Result = super::results::OpenDevToolsResult;
}
group_enum ! (TargetCommands { ActivateTarget (ActivateTarget) , AttachToTarget (AttachToTarget) , AttachToBrowserTarget (AttachToBrowserTarget) , CloseTarget (CloseTarget) , ExposeDevToolsProtocol (ExposeDevToolsProtocol) , CreateBrowserContext (CreateBrowserContext) , GetBrowserContexts (GetBrowserContexts) , CreateTarget (CreateTarget) , DetachFromTarget (DetachFromTarget) , DisposeBrowserContext (DisposeBrowserContext) , GetTargetInfo (GetTargetInfo) , GetTargets (GetTargets) , SetAutoAttach (SetAutoAttach) , AutoAttachRelated (AutoAttachRelated) , SetDiscoverTargets (SetDiscoverTargets) , SetRemoteLocations (SetRemoteLocations) , GetDevToolsTarget (GetDevToolsTarget) , OpenDevTools (OpenDevTools) });
