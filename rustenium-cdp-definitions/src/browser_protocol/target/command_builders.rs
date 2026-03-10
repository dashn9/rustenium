use super::commands::*;
impl ActivateTarget {
    pub fn builder() -> ActivateTargetBuilder {
        <ActivateTargetBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ActivateTargetBuilder {
    target_id: Option<super::types::TargetId>,
}
impl ActivateTargetBuilder {
    pub fn target_id(mut self, target_id: impl Into<super::types::TargetId>) -> Self {
        self.target_id = Some(target_id.into());
        self
    }
    pub fn build(self) -> Result<ActivateTarget, String> {
        Ok(ActivateTarget {
            method: ActivateTargetMethod::ActivateTarget,
            params: ActivateTargetParams {
                target_id: self.target_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(target_id))
                })?,
            },
        })
    }
}
impl AttachToTarget {
    pub fn builder() -> AttachToTargetBuilder {
        <AttachToTargetBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AttachToTargetBuilder {
    target_id: Option<super::types::TargetId>,
    flatten: Option<bool>,
}
impl AttachToTargetBuilder {
    pub fn target_id(mut self, target_id: impl Into<super::types::TargetId>) -> Self {
        self.target_id = Some(target_id.into());
        self
    }
    pub fn flatten(mut self, flatten: impl Into<bool>) -> Self {
        self.flatten = Some(flatten.into());
        self
    }
    pub fn build(self) -> Result<AttachToTarget, String> {
        Ok(AttachToTarget {
            method: AttachToTargetMethod::AttachToTarget,
            params: AttachToTargetParams {
                target_id: self.target_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(target_id))
                })?,
                flatten: self.flatten,
            },
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct AttachToBrowserTargetBuilder;
impl AttachToBrowserTargetBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> AttachToBrowserTarget {
        AttachToBrowserTarget {
            method: AttachToBrowserTargetMethod::AttachToBrowserTarget,
            params: AttachToBrowserTargetParams {},
        }
    }
}
impl AttachToBrowserTarget {
    pub fn builder() -> AttachToBrowserTargetBuilder {
        AttachToBrowserTargetBuilder
    }
}
impl CloseTarget {
    pub fn builder() -> CloseTargetBuilder {
        <CloseTargetBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CloseTargetBuilder {
    target_id: Option<super::types::TargetId>,
}
impl CloseTargetBuilder {
    pub fn target_id(mut self, target_id: impl Into<super::types::TargetId>) -> Self {
        self.target_id = Some(target_id.into());
        self
    }
    pub fn build(self) -> Result<CloseTarget, String> {
        Ok(CloseTarget {
            method: CloseTargetMethod::CloseTarget,
            params: CloseTargetParams {
                target_id: self.target_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(target_id))
                })?,
            },
        })
    }
}
impl ExposeDevToolsProtocol {
    pub fn builder() -> ExposeDevToolsProtocolBuilder {
        <ExposeDevToolsProtocolBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ExposeDevToolsProtocolBuilder {
    target_id: Option<super::types::TargetId>,
    binding_name: Option<String>,
    inherit_permissions: Option<bool>,
}
impl ExposeDevToolsProtocolBuilder {
    pub fn target_id(mut self, target_id: impl Into<super::types::TargetId>) -> Self {
        self.target_id = Some(target_id.into());
        self
    }
    pub fn binding_name(mut self, binding_name: impl Into<String>) -> Self {
        self.binding_name = Some(binding_name.into());
        self
    }
    pub fn inherit_permissions(mut self, inherit_permissions: impl Into<bool>) -> Self {
        self.inherit_permissions = Some(inherit_permissions.into());
        self
    }
    pub fn build(self) -> Result<ExposeDevToolsProtocol, String> {
        Ok(ExposeDevToolsProtocol {
            method: ExposeDevToolsProtocolMethod::ExposeDevToolsProtocol,
            params: ExposeDevToolsProtocolParams {
                target_id: self.target_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(target_id))
                })?,
                binding_name: self.binding_name,
                inherit_permissions: self.inherit_permissions,
            },
        })
    }
}
impl CreateBrowserContext {
    pub fn builder() -> CreateBrowserContextBuilder {
        <CreateBrowserContextBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CreateBrowserContextBuilder {
    dispose_on_detach: Option<bool>,
    proxy_server: Option<String>,
    proxy_bypass_list: Option<String>,
    origins_with_universal_network_access: Option<Vec<String>>,
}
impl CreateBrowserContextBuilder {
    pub fn dispose_on_detach(mut self, dispose_on_detach: impl Into<bool>) -> Self {
        self.dispose_on_detach = Some(dispose_on_detach.into());
        self
    }
    pub fn proxy_server(mut self, proxy_server: impl Into<String>) -> Self {
        self.proxy_server = Some(proxy_server.into());
        self
    }
    pub fn proxy_bypass_list(mut self, proxy_bypass_list: impl Into<String>) -> Self {
        self.proxy_bypass_list = Some(proxy_bypass_list.into());
        self
    }
    pub fn origins_with_universal_network_acces(
        mut self,
        origins_with_universal_network_acces: impl Into<String>,
    ) -> Self {
        let v = self
            .origins_with_universal_network_access
            .get_or_insert(Vec::new());
        v.push(origins_with_universal_network_acces.into());
        self
    }
    pub fn origins_with_universal_network_access<I, S>(
        mut self,
        origins_with_universal_network_access: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self
            .origins_with_universal_network_access
            .get_or_insert(Vec::new());
        for val in origins_with_universal_network_access {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> CreateBrowserContext {
        CreateBrowserContext {
            method: CreateBrowserContextMethod::CreateBrowserContext,
            params: CreateBrowserContextParams {
                dispose_on_detach: self.dispose_on_detach,
                proxy_server: self.proxy_server,
                proxy_bypass_list: self.proxy_bypass_list,
                origins_with_universal_network_access: self.origins_with_universal_network_access,
            },
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct GetBrowserContextsBuilder;
impl GetBrowserContextsBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> GetBrowserContexts {
        GetBrowserContexts {
            method: GetBrowserContextsMethod::GetBrowserContexts,
            params: GetBrowserContextsParams {},
        }
    }
}
impl GetBrowserContexts {
    pub fn builder() -> GetBrowserContextsBuilder {
        GetBrowserContextsBuilder
    }
}
impl CreateTarget {
    pub fn builder() -> CreateTargetBuilder {
        <CreateTargetBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CreateTargetBuilder {
    url: Option<String>,
    left: Option<i64>,
    top: Option<i64>,
    width: Option<i64>,
    height: Option<i64>,
    window_state: Option<super::types::WindowState>,
    browser_context_id: Option<crate::browser_protocol::browser::types::BrowserContextId>,
    enable_begin_frame_control: Option<bool>,
    new_window: Option<bool>,
    background: Option<bool>,
    for_tab: Option<bool>,
    hidden: Option<bool>,
    focus: Option<bool>,
}
impl CreateTargetBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn left(mut self, left: impl Into<i64>) -> Self {
        self.left = Some(left.into());
        self
    }
    pub fn top(mut self, top: impl Into<i64>) -> Self {
        self.top = Some(top.into());
        self
    }
    pub fn width(mut self, width: impl Into<i64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn height(mut self, height: impl Into<i64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn window_state(mut self, window_state: impl Into<super::types::WindowState>) -> Self {
        self.window_state = Some(window_state.into());
        self
    }
    pub fn browser_context_id(
        mut self,
        browser_context_id: impl Into<crate::browser_protocol::browser::types::BrowserContextId>,
    ) -> Self {
        self.browser_context_id = Some(browser_context_id.into());
        self
    }
    pub fn enable_begin_frame_control(
        mut self,
        enable_begin_frame_control: impl Into<bool>,
    ) -> Self {
        self.enable_begin_frame_control = Some(enable_begin_frame_control.into());
        self
    }
    pub fn new_window(mut self, new_window: impl Into<bool>) -> Self {
        self.new_window = Some(new_window.into());
        self
    }
    pub fn background(mut self, background: impl Into<bool>) -> Self {
        self.background = Some(background.into());
        self
    }
    pub fn for_tab(mut self, for_tab: impl Into<bool>) -> Self {
        self.for_tab = Some(for_tab.into());
        self
    }
    pub fn hidden(mut self, hidden: impl Into<bool>) -> Self {
        self.hidden = Some(hidden.into());
        self
    }
    pub fn focus(mut self, focus: impl Into<bool>) -> Self {
        self.focus = Some(focus.into());
        self
    }
    pub fn build(self) -> Result<CreateTarget, String> {
        Ok(CreateTarget {
            method: CreateTargetMethod::CreateTarget,
            params: CreateTargetParams {
                url: self
                    .url
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
                left: self.left,
                top: self.top,
                width: self.width,
                height: self.height,
                window_state: self.window_state,
                browser_context_id: self.browser_context_id,
                enable_begin_frame_control: self.enable_begin_frame_control,
                new_window: self.new_window,
                background: self.background,
                for_tab: self.for_tab,
                hidden: self.hidden,
                focus: self.focus,
            },
        })
    }
}
impl DetachFromTarget {
    pub fn builder() -> DetachFromTargetBuilder {
        <DetachFromTargetBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DetachFromTargetBuilder {
    session_id: Option<super::types::SessionId>,
}
impl DetachFromTargetBuilder {
    pub fn session_id(mut self, session_id: impl Into<super::types::SessionId>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }
    pub fn build(self) -> DetachFromTarget {
        DetachFromTarget {
            method: DetachFromTargetMethod::DetachFromTarget,
            params: DetachFromTargetParams {
                session_id: self.session_id,
            },
        }
    }
}
impl DisposeBrowserContext {
    pub fn builder() -> DisposeBrowserContextBuilder {
        <DisposeBrowserContextBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DisposeBrowserContextBuilder {
    browser_context_id: Option<crate::browser_protocol::browser::types::BrowserContextId>,
}
impl DisposeBrowserContextBuilder {
    pub fn browser_context_id(
        mut self,
        browser_context_id: impl Into<crate::browser_protocol::browser::types::BrowserContextId>,
    ) -> Self {
        self.browser_context_id = Some(browser_context_id.into());
        self
    }
    pub fn build(self) -> Result<DisposeBrowserContext, String> {
        Ok(DisposeBrowserContext {
            method: DisposeBrowserContextMethod::DisposeBrowserContext,
            params: DisposeBrowserContextParams {
                browser_context_id: self.browser_context_id.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(browser_context_id)
                    )
                })?,
            },
        })
    }
}
impl GetTargetInfo {
    pub fn builder() -> GetTargetInfoBuilder {
        <GetTargetInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetTargetInfoBuilder {
    target_id: Option<super::types::TargetId>,
}
impl GetTargetInfoBuilder {
    pub fn target_id(mut self, target_id: impl Into<super::types::TargetId>) -> Self {
        self.target_id = Some(target_id.into());
        self
    }
    pub fn build(self) -> GetTargetInfo {
        GetTargetInfo {
            method: GetTargetInfoMethod::GetTargetInfo,
            params: GetTargetInfoParams {
                target_id: self.target_id,
            },
        }
    }
}
impl GetTargets {
    pub fn builder() -> GetTargetsBuilder {
        <GetTargetsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetTargetsBuilder {
    filter: Option<super::types::TargetFilter>,
}
impl GetTargetsBuilder {
    pub fn filter(mut self, filter: impl Into<super::types::TargetFilter>) -> Self {
        self.filter = Some(filter.into());
        self
    }
    pub fn build(self) -> GetTargets {
        GetTargets {
            method: GetTargetsMethod::GetTargets,
            params: GetTargetsParams {
                filter: self.filter,
            },
        }
    }
}
impl SetAutoAttach {
    pub fn builder() -> SetAutoAttachBuilder {
        <SetAutoAttachBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetAutoAttachBuilder {
    auto_attach: Option<bool>,
    wait_for_debugger_on_start: Option<bool>,
    flatten: Option<bool>,
    filter: Option<super::types::TargetFilter>,
}
impl SetAutoAttachBuilder {
    pub fn auto_attach(mut self, auto_attach: impl Into<bool>) -> Self {
        self.auto_attach = Some(auto_attach.into());
        self
    }
    pub fn wait_for_debugger_on_start(
        mut self,
        wait_for_debugger_on_start: impl Into<bool>,
    ) -> Self {
        self.wait_for_debugger_on_start = Some(wait_for_debugger_on_start.into());
        self
    }
    pub fn flatten(mut self, flatten: impl Into<bool>) -> Self {
        self.flatten = Some(flatten.into());
        self
    }
    pub fn filter(mut self, filter: impl Into<super::types::TargetFilter>) -> Self {
        self.filter = Some(filter.into());
        self
    }
    pub fn build(self) -> Result<SetAutoAttach, String> {
        Ok(SetAutoAttach {
            method: SetAutoAttachMethod::SetAutoAttach,
            params: SetAutoAttachParams {
                auto_attach: self.auto_attach.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(auto_attach))
                })?,
                wait_for_debugger_on_start: self.wait_for_debugger_on_start.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(wait_for_debugger_on_start)
                    )
                })?,
                flatten: self.flatten,
                filter: self.filter,
            },
        })
    }
}
impl AutoAttachRelated {
    pub fn builder() -> AutoAttachRelatedBuilder {
        <AutoAttachRelatedBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AutoAttachRelatedBuilder {
    target_id: Option<super::types::TargetId>,
    wait_for_debugger_on_start: Option<bool>,
    filter: Option<super::types::TargetFilter>,
}
impl AutoAttachRelatedBuilder {
    pub fn target_id(mut self, target_id: impl Into<super::types::TargetId>) -> Self {
        self.target_id = Some(target_id.into());
        self
    }
    pub fn wait_for_debugger_on_start(
        mut self,
        wait_for_debugger_on_start: impl Into<bool>,
    ) -> Self {
        self.wait_for_debugger_on_start = Some(wait_for_debugger_on_start.into());
        self
    }
    pub fn filter(mut self, filter: impl Into<super::types::TargetFilter>) -> Self {
        self.filter = Some(filter.into());
        self
    }
    pub fn build(self) -> Result<AutoAttachRelated, String> {
        Ok(AutoAttachRelated {
            method: AutoAttachRelatedMethod::AutoAttachRelated,
            params: AutoAttachRelatedParams {
                target_id: self.target_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(target_id))
                })?,
                wait_for_debugger_on_start: self.wait_for_debugger_on_start.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(wait_for_debugger_on_start)
                    )
                })?,
                filter: self.filter,
            },
        })
    }
}
impl SetDiscoverTargets {
    pub fn builder() -> SetDiscoverTargetsBuilder {
        <SetDiscoverTargetsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetDiscoverTargetsBuilder {
    discover: Option<bool>,
    filter: Option<super::types::TargetFilter>,
}
impl SetDiscoverTargetsBuilder {
    pub fn discover(mut self, discover: impl Into<bool>) -> Self {
        self.discover = Some(discover.into());
        self
    }
    pub fn filter(mut self, filter: impl Into<super::types::TargetFilter>) -> Self {
        self.filter = Some(filter.into());
        self
    }
    pub fn build(self) -> Result<SetDiscoverTargets, String> {
        Ok(SetDiscoverTargets {
            method: SetDiscoverTargetsMethod::SetDiscoverTargets,
            params: SetDiscoverTargetsParams {
                discover: self.discover.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(discover))
                })?,
                filter: self.filter,
            },
        })
    }
}
impl SetRemoteLocations {
    pub fn builder() -> SetRemoteLocationsBuilder {
        <SetRemoteLocationsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetRemoteLocationsBuilder {
    locations: Option<Vec<super::types::RemoteLocation>>,
}
impl SetRemoteLocationsBuilder {
    pub fn location(mut self, location: impl Into<super::types::RemoteLocation>) -> Self {
        let v = self.locations.get_or_insert(Vec::new());
        v.push(location.into());
        self
    }
    pub fn locations<I, S>(mut self, locations: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::RemoteLocation>,
    {
        let v = self.locations.get_or_insert(Vec::new());
        for val in locations {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetRemoteLocations, String> {
        Ok(SetRemoteLocations {
            method: SetRemoteLocationsMethod::SetRemoteLocations,
            params: SetRemoteLocationsParams {
                locations: self.locations.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(locations))
                })?,
            },
        })
    }
}
impl GetDevToolsTarget {
    pub fn builder() -> GetDevToolsTargetBuilder {
        <GetDevToolsTargetBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetDevToolsTargetBuilder {
    target_id: Option<super::types::TargetId>,
}
impl GetDevToolsTargetBuilder {
    pub fn target_id(mut self, target_id: impl Into<super::types::TargetId>) -> Self {
        self.target_id = Some(target_id.into());
        self
    }
    pub fn build(self) -> Result<GetDevToolsTarget, String> {
        Ok(GetDevToolsTarget {
            method: GetDevToolsTargetMethod::GetDevToolsTarget,
            params: GetDevToolsTargetParams {
                target_id: self.target_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(target_id))
                })?,
            },
        })
    }
}
impl OpenDevTools {
    pub fn builder() -> OpenDevToolsBuilder {
        <OpenDevToolsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct OpenDevToolsBuilder {
    target_id: Option<super::types::TargetId>,
    panel_id: Option<String>,
}
impl OpenDevToolsBuilder {
    pub fn target_id(mut self, target_id: impl Into<super::types::TargetId>) -> Self {
        self.target_id = Some(target_id.into());
        self
    }
    pub fn panel_id(mut self, panel_id: impl Into<String>) -> Self {
        self.panel_id = Some(panel_id.into());
        self
    }
    pub fn build(self) -> Result<OpenDevTools, String> {
        Ok(OpenDevTools {
            method: OpenDevToolsMethod::OpenDevTools,
            params: OpenDevToolsParams {
                target_id: self.target_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(target_id))
                })?,
                panel_id: self.panel_id,
            },
        })
    }
}
