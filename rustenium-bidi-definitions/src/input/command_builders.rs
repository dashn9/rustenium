use super::commands::*;
impl PerformActions {
    pub fn builder() -> PerformActionsBuilder {
        <PerformActionsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PerformActionsBuilder {
    context: Option<crate::browsing_context::types::BrowsingContext>,
    actions: Option<Vec<super::types::SourceActions>>,
}
impl PerformActionsBuilder {
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn action(mut self, action: impl Into<super::types::SourceActions>) -> Self {
        let v = self.actions.get_or_insert(Vec::new());
        v.push(action.into());
        self
    }
    pub fn actions<I, S>(mut self, actions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::SourceActions>,
    {
        let v = self.actions.get_or_insert(Vec::new());
        for val in actions {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<PerformActions, String> {
        Ok(PerformActions {
            method: PerformActionsMethod::PerformActions,
            params: PerformActionsParams {
                context: self
                    .context
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
                actions: self
                    .actions
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(actions)))?,
            },
        })
    }
}
impl ReleaseActions {
    pub fn builder() -> ReleaseActionsBuilder {
        <ReleaseActionsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ReleaseActionsBuilder {
    context: Option<crate::browsing_context::types::BrowsingContext>,
}
impl ReleaseActionsBuilder {
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn build(self) -> Result<ReleaseActions, String> {
        Ok(ReleaseActions {
            method: ReleaseActionsMethod::ReleaseActions,
            params: ReleaseActionsParams {
                context: self
                    .context
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
            },
        })
    }
}
impl SetFiles {
    pub fn builder() -> SetFilesBuilder {
        <SetFilesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetFilesBuilder {
    context: Option<crate::browsing_context::types::BrowsingContext>,
    element: Option<crate::script::types::SharedReference>,
    files: Option<Vec<String>>,
}
impl SetFilesBuilder {
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn element(mut self, element: impl Into<crate::script::types::SharedReference>) -> Self {
        self.element = Some(element.into());
        self
    }
    pub fn file(mut self, file: impl Into<String>) -> Self {
        let v = self.files.get_or_insert(Vec::new());
        v.push(file.into());
        self
    }
    pub fn files<I, S>(mut self, files: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.files.get_or_insert(Vec::new());
        for val in files {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetFiles, String> {
        Ok(SetFiles {
            method: SetFilesMethod::SetFiles,
            params: SetFilesParams {
                context: self
                    .context
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
                element: self
                    .element
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(element)))?,
                files: self
                    .files
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(files)))?,
            },
        })
    }
}
