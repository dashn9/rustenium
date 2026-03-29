use crate::browsers::chrome::tab::ChromeTab;
use crate::conduit::cdp::adapter::CdpAdapter;
use crate::error::cdp::{CreateTabError, EmulateDeviceMetricsError, NavigateError};
use rustenium_cdp_definitions::Command;
use rustenium_cdp_definitions::base::CommandResponse;
use rustenium_cdp_definitions::browser_protocol::emulation::commands::SetDeviceMetricsOverride;
use rustenium_cdp_definitions::browser_protocol::emulation::types::ScreenOrientation;
use rustenium_cdp_definitions::browser_protocol::page::commands::Navigate;
use rustenium_cdp_definitions::browser_protocol::page::results::NavigateResult;
use rustenium_cdp_definitions::browser_protocol::page::types::{ReferrerPolicy, TransitionType, Viewport};
use rustenium_cdp_definitions::browser_protocol::target::commands::CreateTarget;
use rustenium_core::error::CdpSessionSendError;
use rustenium_core::transport::WebsocketConnectionTransport;

#[derive(Debug, Clone, Default)]
pub struct NavigateOptions {
    pub referrer: Option<String>,
    pub transition_type: Option<TransitionType>,
    pub referrer_policy: Option<ReferrerPolicy>,
}

#[derive(Default, Clone)]
pub struct NavigateOptionsBuilder {
    referrer: Option<String>,
    transition_type: Option<TransitionType>,
    referrer_policy: Option<ReferrerPolicy>,
}

impl NavigateOptionsBuilder {
    pub fn referrer(mut self, v: impl Into<String>) -> Self {
        self.referrer = Some(v.into());
        self
    }
    pub fn transition_type(mut self, v: impl Into<TransitionType>) -> Self {
        self.transition_type = Some(v.into());
        self
    }
    pub fn referrer_policy(mut self, v: impl Into<ReferrerPolicy>) -> Self {
        self.referrer_policy = Some(v.into());
        self
    }
    pub fn build(self) -> NavigateOptions {
        NavigateOptions {
            referrer: self.referrer,
            transition_type: self.transition_type,
            referrer_policy: self.referrer_policy,
        }
    }
}

pub trait CdpBrowser {
    fn adapter(&self) -> &CdpAdapter<WebsocketConnectionTransport>;

    fn adapter_mut(&mut self) -> &mut CdpAdapter<WebsocketConnectionTransport>;

    fn navigate(
        &mut self,
        url: impl Into<String>,
    ) -> impl Future<Output = Result<NavigateResult, NavigateError>> {
        self.navigate_with_options(url, NavigateOptions::default())
    }

    fn navigate_with_options(
        &mut self,
        url: impl Into<String>,
        options: NavigateOptions,
    ) -> impl Future<Output = Result<NavigateResult, NavigateError>> {
        let mut builder = Navigate::builder().url(url);
        if let Some(referrer) = options.referrer {
            builder = builder.referrer(referrer);
        }
        if let Some(transition_type) = options.transition_type {
            builder = builder.transition_type(transition_type);
        }
        if let Some(referrer_policy) = options.referrer_policy {
            builder = builder.referrer_policy(referrer_policy);
        }
        let command = builder.build().unwrap();
        let adapter = self.adapter_mut();
        async move { adapter.navigate(command).await }
    }

    fn create_tab(
        &mut self,
        url: impl Into<String>,
    ) -> impl Future<Output = Result<ChromeTab, CreateTabError>> {
        self.create_tab_with_options(url, CreateTabOptions::default())
    }

    fn create_tab_with_options(
        &mut self,
        url: impl Into<String>,
        options: CreateTabOptions,
    ) -> impl Future<Output = Result<ChromeTab, CreateTabError>> {
        let mut builder = CreateTarget::builder().url(url);
        if let Some(v) = options.left {
            builder = builder.left(v);
        }
        if let Some(v) = options.top {
            builder = builder.top(v);
        }
        if let Some(v) = options.width {
            builder = builder.width(v);
        }
        if let Some(v) = options.height {
            builder = builder.height(v);
        }
        if let Some(v) = options.window_state {
            builder = builder.window_state(v);
        }
        if let Some(v) = options.browser_context_id {
            builder = builder.browser_context_id(v);
        }
        if let Some(v) = options.enable_begin_frame_control {
            builder = builder.enable_begin_frame_control(v);
        }
        if let Some(v) = options.new_window {
            builder = builder.new_window(v);
        }
        if let Some(v) = options.background {
            builder = builder.background(v);
        }
        if let Some(v) = options.for_tab {
            builder = builder.for_tab(v);
        }
        if let Some(v) = options.hidden {
            builder = builder.hidden(v);
        }
        if let Some(v) = options.focus {
            builder = builder.focus(v);
        }
        let command = builder.build().unwrap();
        let adapter = self.adapter_mut();
        async move {
            let target_id = adapter
                .create_target(command)
                .await
                .map_err(CreateTabError::CreateTargetError)?;
            Ok(ChromeTab::new(target_id))
        }
    }

    fn emulate_device_metrics(
        &mut self,
        width: i64,
        height: i64,
        device_scale_factor: f64,
        mobile: bool,
    ) -> impl Future<Output = Result<(), EmulateDeviceMetricsError>> {
        self.emulate_device_metrics_with_options(
            width,
            height,
            device_scale_factor,
            mobile,
            EmulateDeviceMetricsOptions::default(),
        )
    }

    fn emulate_device_metrics_with_options(
        &mut self,
        width: i64,
        height: i64,
        device_scale_factor: f64,
        mobile: bool,
        options: EmulateDeviceMetricsOptions,
    ) -> impl Future<Output = Result<(), EmulateDeviceMetricsError>> {
        let mut builder = SetDeviceMetricsOverride::builder()
            .width(width)
            .height(height)
            .device_scale_factor(device_scale_factor)
            .mobile(mobile);
        if let Some(v) = options.scale { builder = builder.scale(v); }
        if let Some(v) = options.screen_width { builder = builder.screen_width(v); }
        if let Some(v) = options.screen_height { builder = builder.screen_height(v); }
        if let Some(v) = options.position_x { builder = builder.position_x(v); }
        if let Some(v) = options.position_y { builder = builder.position_y(v); }
        if let Some(v) = options.dont_set_visible_size { builder = builder.dont_set_visible_size(v); }
        if let Some(v) = options.screen_orientation { builder = builder.screen_orientation(v); }
        if let Some(v) = options.viewport { builder = builder.viewport(v); }
        if let Some(v) = options.scrollbar_type { builder = builder.scrollbar_type(v); }
        let command = builder.build().unwrap();
        let adapter = self.adapter_mut();
        async move { adapter.emulate_device_metrics(command).await }
    }

    fn send_command(
        &mut self,
        command: Command,
    ) -> impl Future<Output = Result<CommandResponse, CdpSessionSendError>> + Send {
        let adapter = self.adapter_mut();
        async move { adapter.send_command(command).await }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CreateTabOptions {
    pub left: Option<i64>,
    pub top: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub window_state:
        Option<rustenium_cdp_definitions::browser_protocol::target::types::WindowState>,
    pub browser_context_id:
        Option<rustenium_cdp_definitions::browser_protocol::browser::types::BrowserContextId>,
    pub enable_begin_frame_control: Option<bool>,
    pub new_window: Option<bool>,
    pub background: Option<bool>,
    pub for_tab: Option<bool>,
    pub hidden: Option<bool>,
    pub focus: Option<bool>,
}

#[derive(Default, Clone)]
pub struct CreateTabOptionsBuilder {
    left: Option<i64>,
    top: Option<i64>,
    width: Option<i64>,
    height: Option<i64>,
    window_state: Option<rustenium_cdp_definitions::browser_protocol::target::types::WindowState>,
    browser_context_id:
        Option<rustenium_cdp_definitions::browser_protocol::browser::types::BrowserContextId>,
    enable_begin_frame_control: Option<bool>,
    new_window: Option<bool>,
    background: Option<bool>,
    for_tab: Option<bool>,
    hidden: Option<bool>,
    focus: Option<bool>,
}

impl CreateTabOptionsBuilder {
    pub fn left(mut self, v: i64) -> Self {
        self.left = Some(v);
        self
    }
    pub fn top(mut self, v: i64) -> Self {
        self.top = Some(v);
        self
    }
    pub fn width(mut self, v: i64) -> Self {
        self.width = Some(v);
        self
    }
    pub fn height(mut self, v: i64) -> Self {
        self.height = Some(v);
        self
    }
    pub fn window_state(
        mut self,
        v: rustenium_cdp_definitions::browser_protocol::target::types::WindowState,
    ) -> Self {
        self.window_state = Some(v);
        self
    }
    pub fn browser_context_id(
        mut self,
        v: rustenium_cdp_definitions::browser_protocol::browser::types::BrowserContextId,
    ) -> Self {
        self.browser_context_id = Some(v);
        self
    }
    pub fn enable_begin_frame_control(mut self, v: bool) -> Self {
        self.enable_begin_frame_control = Some(v);
        self
    }
    pub fn new_window(mut self, v: bool) -> Self {
        self.new_window = Some(v);
        self
    }
    pub fn background(mut self, v: bool) -> Self {
        self.background = Some(v);
        self
    }
    pub fn for_tab(mut self, v: bool) -> Self {
        self.for_tab = Some(v);
        self
    }
    pub fn hidden(mut self, v: bool) -> Self {
        self.hidden = Some(v);
        self
    }
    pub fn focus(mut self, v: bool) -> Self {
        self.focus = Some(v);
        self
    }
    pub fn build(self) -> CreateTabOptions {
        CreateTabOptions {
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
        }
    }
}

use rustenium_cdp_definitions::browser_protocol::emulation::commands::SetDeviceMetricsOverrideScrollbarType;

#[derive(Debug, Clone, Default)]
pub struct EmulateDeviceMetricsOptions {
    pub scale: Option<f64>,
    pub screen_width: Option<i64>,
    pub screen_height: Option<i64>,
    pub position_x: Option<i64>,
    pub position_y: Option<i64>,
    pub dont_set_visible_size: Option<bool>,
    pub screen_orientation: Option<ScreenOrientation>,
    pub viewport: Option<Viewport>,
    pub scrollbar_type: Option<SetDeviceMetricsOverrideScrollbarType>,
}

#[derive(Default, Clone)]
pub struct EmulateDeviceMetricsOptionsBuilder {
    scale: Option<f64>,
    screen_width: Option<i64>,
    screen_height: Option<i64>,
    position_x: Option<i64>,
    position_y: Option<i64>,
    dont_set_visible_size: Option<bool>,
    screen_orientation: Option<ScreenOrientation>,
    viewport: Option<Viewport>,
    scrollbar_type: Option<SetDeviceMetricsOverrideScrollbarType>,
}

impl EmulateDeviceMetricsOptionsBuilder {
    pub fn scale(mut self, v: f64) -> Self { self.scale = Some(v); self }
    pub fn screen_width(mut self, v: i64) -> Self { self.screen_width = Some(v); self }
    pub fn screen_height(mut self, v: i64) -> Self { self.screen_height = Some(v); self }
    pub fn position_x(mut self, v: i64) -> Self { self.position_x = Some(v); self }
    pub fn position_y(mut self, v: i64) -> Self { self.position_y = Some(v); self }
    pub fn dont_set_visible_size(mut self, v: bool) -> Self { self.dont_set_visible_size = Some(v); self }
    pub fn screen_orientation(mut self, v: ScreenOrientation) -> Self { self.screen_orientation = Some(v); self }
    pub fn viewport(mut self, v: Viewport) -> Self { self.viewport = Some(v); self }
    pub fn scrollbar_type(mut self, v: SetDeviceMetricsOverrideScrollbarType) -> Self { self.scrollbar_type = Some(v); self }
    pub fn build(self) -> EmulateDeviceMetricsOptions {
        EmulateDeviceMetricsOptions {
            scale: self.scale,
            screen_width: self.screen_width,
            screen_height: self.screen_height,
            position_x: self.position_x,
            position_y: self.position_y,
            dont_set_visible_size: self.dont_set_visible_size,
            screen_orientation: self.screen_orientation,
            viewport: self.viewport,
            scrollbar_type: self.scrollbar_type,
        }
    }
}
