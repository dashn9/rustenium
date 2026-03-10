use super::commands::*;
#[derive(Debug, Clone, Default)]
pub struct CloseBuilder;
impl CloseBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Close {
        Close {
            method: CloseMethod::Close,
            params: CloseParams {},
        }
    }
}
impl Close {
    pub fn builder() -> CloseBuilder {
        CloseBuilder
    }
}
impl CreateUserContext {
    pub fn builder() -> CreateUserContextBuilder {
        <CreateUserContextBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CreateUserContextBuilder {
    accept_insecure_certs: Option<bool>,
    proxy: Option<crate::session::types::ProxyConfiguration>,
    unhandled_prompt_behavior: Option<crate::session::types::UserPromptHandler>,
}
impl CreateUserContextBuilder {
    pub fn accept_insecure_certs(mut self, accept_insecure_certs: impl Into<bool>) -> Self {
        self.accept_insecure_certs = Some(accept_insecure_certs.into());
        self
    }
    pub fn proxy(mut self, proxy: impl Into<crate::session::types::ProxyConfiguration>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }
    pub fn unhandled_prompt_behavior(
        mut self,
        unhandled_prompt_behavior: impl Into<crate::session::types::UserPromptHandler>,
    ) -> Self {
        self.unhandled_prompt_behavior = Some(unhandled_prompt_behavior.into());
        self
    }
    pub fn build(self) -> CreateUserContext {
        CreateUserContext {
            method: CreateUserContextMethod::CreateUserContext,
            params: CreateUserContextParams {
                accept_insecure_certs: self.accept_insecure_certs,
                proxy: self.proxy,
                unhandled_prompt_behavior: self.unhandled_prompt_behavior,
            },
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct GetClientWindowsBuilder;
impl GetClientWindowsBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> GetClientWindows {
        GetClientWindows {
            method: GetClientWindowsMethod::GetClientWindows,
            params: GetClientWindowsParams {},
        }
    }
}
impl GetClientWindows {
    pub fn builder() -> GetClientWindowsBuilder {
        GetClientWindowsBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct GetUserContextsBuilder;
impl GetUserContextsBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> GetUserContexts {
        GetUserContexts {
            method: GetUserContextsMethod::GetUserContexts,
            params: GetUserContextsParams {},
        }
    }
}
impl GetUserContexts {
    pub fn builder() -> GetUserContextsBuilder {
        GetUserContextsBuilder
    }
}
impl RemoveUserContext {
    pub fn builder() -> RemoveUserContextBuilder {
        <RemoveUserContextBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveUserContextBuilder {
    user_context: Option<super::types::UserContext>,
}
impl RemoveUserContextBuilder {
    pub fn user_context(mut self, user_context: impl Into<super::types::UserContext>) -> Self {
        self.user_context = Some(user_context.into());
        self
    }
    pub fn build(self) -> Result<RemoveUserContext, String> {
        Ok(RemoveUserContext {
            method: RemoveUserContextMethod::RemoveUserContext,
            params: RemoveUserContextParams {
                user_context: self.user_context.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(user_context))
                })?,
            },
        })
    }
}
impl SetClientWindowState {
    pub fn builder() -> SetClientWindowStateBuilder {
        <SetClientWindowStateBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetClientWindowStateBuilder {
    client_window: Option<super::types::ClientWindow>,
    client_window_named_state_client_window_rect_state_union:
        Option<super::types::ClientWindowNamedStateClientWindowRectStateUnion>,
}
impl SetClientWindowStateBuilder {
    pub fn client_window(mut self, client_window: impl Into<super::types::ClientWindow>) -> Self {
        self.client_window = Some(client_window.into());
        self
    }
    pub fn client_window_named_state_client_window_rect_state_union(
        mut self,
        client_window_named_state_client_window_rect_state_union: impl Into<
            super::types::ClientWindowNamedStateClientWindowRectStateUnion,
        >,
    ) -> Self {
        self.client_window_named_state_client_window_rect_state_union =
            Some(client_window_named_state_client_window_rect_state_union.into());
        self
    }
    pub fn build(self) -> Result<SetClientWindowState, String> {
        Ok(SetClientWindowState {
            method: SetClientWindowStateMethod::SetClientWindowState,
            params: SetClientWindowStateParams {
                client_window: self.client_window.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(client_window))
                })?,
                client_window_named_state_client_window_rect_state_union: self
                    .client_window_named_state_client_window_rect_state_union
                    .ok_or_else(|| {
                        format!(
                            "Field `{}` is mandatory.",
                            std::stringify!(
                                client_window_named_state_client_window_rect_state_union
                            )
                        )
                    })?,
            },
        })
    }
}
impl SetDownloadBehavior {
    pub fn builder() -> SetDownloadBehaviorBuilder {
        <SetDownloadBehaviorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetDownloadBehaviorBuilder {
    download_behavior: Option<super::types::DownloadBehavior>,
    user_contexts: Option<Vec<super::types::UserContext>>,
}
impl SetDownloadBehaviorBuilder {
    pub fn download_behavior(
        mut self,
        download_behavior: impl Into<super::types::DownloadBehavior>,
    ) -> Self {
        self.download_behavior = Some(download_behavior.into());
        self
    }
    pub fn user_context(mut self, user_context: impl Into<super::types::UserContext>) -> Self {
        let v = self.user_contexts.get_or_insert(Vec::new());
        v.push(user_context.into());
        self
    }
    pub fn user_contexts<I, S>(mut self, user_contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::UserContext>,
    {
        let v = self.user_contexts.get_or_insert(Vec::new());
        for val in user_contexts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> SetDownloadBehavior {
        SetDownloadBehavior {
            method: SetDownloadBehaviorMethod::SetDownloadBehavior,
            params: SetDownloadBehaviorParams {
                download_behavior: self.download_behavior,
                user_contexts: self.user_contexts,
            },
        }
    }
}
