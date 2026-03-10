use super::commands::*;
#[derive(Debug, Clone, Default)]
pub struct StatusBuilder;
impl StatusBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Status {
        Status {
            method: StatusMethod::Status,
            params: StatusParams {},
        }
    }
}
impl Status {
    pub fn builder() -> StatusBuilder {
        StatusBuilder
    }
}
impl New {
    pub fn builder() -> NewBuilder {
        <NewBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct NewBuilder {
    capabilities: Option<super::types::CapabilitiesRequest>,
}
impl NewBuilder {
    pub fn capabilities(
        mut self,
        capabilities: impl Into<super::types::CapabilitiesRequest>,
    ) -> Self {
        self.capabilities = Some(capabilities.into());
        self
    }
    pub fn build(self) -> Result<New, String> {
        Ok(New {
            method: NewMethod::New,
            params: NewParams {
                capabilities: self.capabilities.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(capabilities))
                })?,
            },
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct EndBuilder;
impl EndBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> End {
        End {
            method: EndMethod::End,
            params: EndParams {},
        }
    }
}
impl End {
    pub fn builder() -> EndBuilder {
        EndBuilder
    }
}
impl Subscribe {
    pub fn builder() -> SubscribeBuilder {
        <SubscribeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SubscribeBuilder {
    events: Option<Vec<String>>,
    contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl SubscribeBuilder {
    pub fn event(mut self, event: impl Into<String>) -> Self {
        let v = self.events.get_or_insert(Vec::new());
        v.push(event.into());
        self
    }
    pub fn events<I, S>(mut self, events: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.events.get_or_insert(Vec::new());
        for val in events {
            v.push(val.into());
        }
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        let v = self.contexts.get_or_insert(Vec::new());
        v.push(context.into());
        self
    }
    pub fn contexts<I, S>(mut self, contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browsing_context::types::BrowsingContext>,
    {
        let v = self.contexts.get_or_insert(Vec::new());
        for val in contexts {
            v.push(val.into());
        }
        self
    }
    pub fn user_context(
        mut self,
        user_context: impl Into<crate::browser::types::UserContext>,
    ) -> Self {
        let v = self.user_contexts.get_or_insert(Vec::new());
        v.push(user_context.into());
        self
    }
    pub fn user_contexts<I, S>(mut self, user_contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser::types::UserContext>,
    {
        let v = self.user_contexts.get_or_insert(Vec::new());
        for val in user_contexts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<Subscribe, String> {
        Ok(Subscribe {
            method: SubscribeMethod::Subscribe,
            params: SubscribeParams {
                events: self
                    .events
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(events)))?,
                contexts: self.contexts,
                user_contexts: self.user_contexts,
            },
        })
    }
}
impl Unsubscribe {
    pub fn builder() -> UnsubscribeBuilder {
        <UnsubscribeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UnsubscribeBuilder {
    unsubscribe_parameters: Option<super::types::UnsubscribeParameters>,
}
impl UnsubscribeBuilder {
    pub fn unsubscribe_parameters(
        mut self,
        unsubscribe_parameters: impl Into<super::types::UnsubscribeParameters>,
    ) -> Self {
        self.unsubscribe_parameters = Some(unsubscribe_parameters.into());
        self
    }
    pub fn build(self) -> Result<Unsubscribe, String> {
        Ok(Unsubscribe {
            method: UnsubscribeMethod::Unsubscribe,
            params: UnsubscribeParams {
                unsubscribe_parameters: self.unsubscribe_parameters.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(unsubscribe_parameters)
                    )
                })?,
            },
        })
    }
}
