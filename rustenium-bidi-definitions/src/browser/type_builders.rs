use super::types::*;
impl ClientWindowInfo {
    pub fn builder() -> ClientWindowInfoBuilder {
        <ClientWindowInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ClientWindowInfoBuilder {
    active: Option<bool>,
    client_window: Option<ClientWindow>,
    height: Option<u64>,
    state: Option<ClientWindowInfoState>,
    width: Option<u64>,
    x: Option<i64>,
    y: Option<i64>,
}
impl ClientWindowInfoBuilder {
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.active = Some(active.into());
        self
    }
    pub fn client_window(mut self, client_window: impl Into<ClientWindow>) -> Self {
        self.client_window = Some(client_window.into());
        self
    }
    pub fn height(mut self, height: impl Into<u64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn state(mut self, state: impl Into<ClientWindowInfoState>) -> Self {
        self.state = Some(state.into());
        self
    }
    pub fn width(mut self, width: impl Into<u64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn x(mut self, x: impl Into<i64>) -> Self {
        self.x = Some(x.into());
        self
    }
    pub fn y(mut self, y: impl Into<i64>) -> Self {
        self.y = Some(y.into());
        self
    }
    pub fn build(self) -> Result<ClientWindowInfo, String> {
        Ok(ClientWindowInfo {
            active: self
                .active
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(active)))?,
            client_window: self.client_window.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(client_window))
            })?,
            height: self
                .height
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(height)))?,
            state: self
                .state
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(state)))?,
            width: self
                .width
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(width)))?,
            x: self
                .x
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
            y: self
                .y
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
        })
    }
}
impl UserContextInfo {
    pub fn builder() -> UserContextInfoBuilder {
        <UserContextInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UserContextInfoBuilder {
    user_context: Option<UserContext>,
}
impl UserContextInfoBuilder {
    pub fn user_context(mut self, user_context: impl Into<UserContext>) -> Self {
        self.user_context = Some(user_context.into());
        self
    }
    pub fn build(self) -> Result<UserContextInfo, String> {
        Ok(UserContextInfo {
            user_context: self.user_context.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(user_context))
            })?,
        })
    }
}
impl ClientWindowNamedState {
    pub fn builder() -> ClientWindowNamedStateBuilder {
        <ClientWindowNamedStateBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ClientWindowNamedStateBuilder {
    state: Option<ClientWindowNamedStateState>,
}
impl ClientWindowNamedStateBuilder {
    pub fn state(mut self, state: impl Into<ClientWindowNamedStateState>) -> Self {
        self.state = Some(state.into());
        self
    }
    pub fn build(self) -> Result<ClientWindowNamedState, String> {
        Ok(ClientWindowNamedState {
            state: self
                .state
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(state)))?,
        })
    }
}
impl ClientWindowRectState {
    pub fn builder() -> ClientWindowRectStateBuilder {
        <ClientWindowRectStateBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ClientWindowRectStateBuilder {
    state: Option<ClientWindowRectStateState>,
    width: Option<u64>,
    height: Option<u64>,
    x: Option<i64>,
    y: Option<i64>,
}
impl ClientWindowRectStateBuilder {
    pub fn state(mut self, state: impl Into<ClientWindowRectStateState>) -> Self {
        self.state = Some(state.into());
        self
    }
    pub fn width(mut self, width: impl Into<u64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn height(mut self, height: impl Into<u64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn x(mut self, x: impl Into<i64>) -> Self {
        self.x = Some(x.into());
        self
    }
    pub fn y(mut self, y: impl Into<i64>) -> Self {
        self.y = Some(y.into());
        self
    }
    pub fn build(self) -> Result<ClientWindowRectState, String> {
        Ok(ClientWindowRectState {
            state: self
                .state
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(state)))?,
            width: self.width,
            height: self.height,
            x: self.x,
            y: self.y,
        })
    }
}
impl DownloadBehaviorAllowed {
    pub fn builder() -> DownloadBehaviorAllowedBuilder {
        <DownloadBehaviorAllowedBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DownloadBehaviorAllowedBuilder {
    r#type: Option<DownloadBehaviorAllowedType>,
    destination_folder: Option<String>,
}
impl DownloadBehaviorAllowedBuilder {
    pub fn r#type(mut self, r#type: impl Into<DownloadBehaviorAllowedType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn destination_folder(mut self, destination_folder: impl Into<String>) -> Self {
        self.destination_folder = Some(destination_folder.into());
        self
    }
    pub fn build(self) -> Result<DownloadBehaviorAllowed, String> {
        Ok(DownloadBehaviorAllowed {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            destination_folder: self.destination_folder.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(destination_folder)
                )
            })?,
        })
    }
}
impl DownloadBehaviorDenied {
    pub fn builder() -> DownloadBehaviorDeniedBuilder {
        <DownloadBehaviorDeniedBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DownloadBehaviorDeniedBuilder {
    r#type: Option<DownloadBehaviorDeniedType>,
}
impl DownloadBehaviorDeniedBuilder {
    pub fn r#type(mut self, r#type: impl Into<DownloadBehaviorDeniedType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<DownloadBehaviorDenied, String> {
        Ok(DownloadBehaviorDenied {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
