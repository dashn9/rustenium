use super::commands::*;
impl New {
    pub fn builder() -> NewBuilder {
        NewBuilder::default()
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
impl Subscribe {
    pub fn builder() -> SubscribeBuilder {
        SubscribeBuilder::default()
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
