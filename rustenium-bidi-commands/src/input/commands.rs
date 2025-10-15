// Generated commands for module

use serde::{Serialize, Deserialize};
use crate::browsing_context::types::BrowsingContext;
use crate::script::types::SharedReference;
use crate::EmptyResult;
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputCommand {
    PerformActions(PerformActions),
    ReleaseActions(ReleaseActions),
    SetFiles(SetFiles),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputPerformActionsMethod {
    #[serde(rename = "input.performActions")]
    InputPerformActions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputReleaseActionsMethod {
    #[serde(rename = "input.releaseActions")]
    InputReleaseActions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputSetFilesMethod {
    #[serde(rename = "input.setFiles")]
    InputSetFiles,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformActionsParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "actions")]
    pub actions: Vec<SourceActions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseActionsParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetFilesParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "element")]
    pub element: SharedReference,
    #[serde(rename = "files")]
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformActions {
    #[serde(rename = "method")]
    pub method: InputPerformActionsMethod,
    #[serde(rename = "params")]
    pub params: PerformActionsParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseActions {
    #[serde(rename = "method")]
    pub method: InputReleaseActionsMethod,
    #[serde(rename = "params")]
    pub params: ReleaseActionsParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetFiles {
    #[serde(rename = "method")]
    pub method: InputSetFilesMethod,
    #[serde(rename = "params")]
    pub params: SetFilesParameters,
}

// Generated results

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputResult {
    PerformActionsResult(PerformActionsResult),
    ReleaseActionsResult(ReleaseActionsResult),
    SetFilesResult(SetFilesResult),
}


pub type PerformActionsResult = EmptyResult;


pub type ReleaseActionsResult = EmptyResult;


pub type SetFilesResult = EmptyResult;


