use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformActionsParams {
    #[serde(rename = "context")]
    pub context: crate::browsing_context::types::BrowsingContext,
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<super::types::SourceActions>,
}
impl PerformActionsParams {
    pub fn new(
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
        actions: Vec<super::types::SourceActions>,
    ) -> Self {
        Self {
            context: context.into(),
            actions,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PerformActionsMethod {
    #[serde(rename = "input.performActions")]
    PerformActions,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformActions {
    pub method: PerformActionsMethod,
    pub params: PerformActionsParams,
}
impl PerformActions {
    pub const IDENTIFIER: &'static str = "input.performActions";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for PerformActions {
    type Result = super::results::PerformActionsResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseActionsParams {
    #[serde(rename = "context")]
    pub context: crate::browsing_context::types::BrowsingContext,
}
impl ReleaseActionsParams {
    pub fn new(context: impl Into<crate::browsing_context::types::BrowsingContext>) -> Self {
        Self {
            context: context.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReleaseActionsMethod {
    #[serde(rename = "input.releaseActions")]
    ReleaseActions,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseActions {
    pub method: ReleaseActionsMethod,
    pub params: ReleaseActionsParams,
}
impl ReleaseActions {
    pub const IDENTIFIER: &'static str = "input.releaseActions";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ReleaseActions {
    type Result = super::results::ReleaseActionsResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetFilesParams {
    #[serde(rename = "context")]
    pub context: crate::browsing_context::types::BrowsingContext,
    #[serde(rename = "element")]
    pub element: crate::script::types::SharedReference,
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
}
impl SetFilesParams {
    pub fn new(
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
        element: impl Into<crate::script::types::SharedReference>,
        files: Vec<String>,
    ) -> Self {
        Self {
            context: context.into(),
            element: element.into(),
            files,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetFilesMethod {
    #[serde(rename = "input.setFiles")]
    SetFiles,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetFiles {
    pub method: SetFilesMethod,
    pub params: SetFilesParams,
}
impl SetFiles {
    pub const IDENTIFIER: &'static str = "input.setFiles";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetFiles {
    type Result = super::results::SetFilesResult;
}
group_enum ! (InputCommand { PerformActions (PerformActions) , ReleaseActions (ReleaseActions) , SetFiles (SetFiles) } + identifiable);
