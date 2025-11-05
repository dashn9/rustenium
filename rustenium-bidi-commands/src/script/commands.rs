// Generated commands for module

use serde::{Serialize, Deserialize};
use crate::browser::types::UserContext;
use crate::browsing_context::types::BrowsingContext;
use crate::EmptyResult;
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptCommand {
    AddPreloadScript(AddPreloadScript),
    CallFunction(CallFunction),
    Disown(Disown),
    Evaluate(Evaluate),
    GetRealms(GetRealms),
    RemovePreloadScript(RemovePreloadScript),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptAddPreloadScriptMethod {
    #[serde(rename = "script.addPreloadScript")]
    ScriptAddPreloadScript,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptCallFunctionMethod {
    #[serde(rename = "script.callFunction")]
    ScriptCallFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptDisownMethod {
    #[serde(rename = "script.disown")]
    ScriptDisown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptEvaluateMethod {
    #[serde(rename = "script.evaluate")]
    ScriptEvaluate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptGetRealmsMethod {
    #[serde(rename = "script.getRealms")]
    ScriptGetRealms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptRemovePreloadScriptMethod {
    #[serde(rename = "script.removePreloadScript")]
    ScriptRemovePreloadScript,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPreloadScriptParameters {
    #[serde(rename = "functionDeclaration")]
    pub function_declaration: String,
    #[serde(rename = "arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<ChannelValue>>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_contexts: Option<Vec<UserContext>>,
    #[serde(rename = "sandbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallFunctionParameters {
    #[serde(rename = "functionDeclaration")]
    pub function_declaration: String,
    #[serde(rename = "awaitPromise")]
    pub await_promise: bool,
    #[serde(rename = "target")]
    pub target: Target,
    #[serde(rename = "arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<LocalValue>>,
    #[serde(rename = "resultOwnership")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_ownership: Option<ResultOwnership>,
    #[serde(rename = "serializationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialization_options: Option<SerializationOptions>,
    #[serde(rename = "this")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub this: Option<LocalValue>,
    #[serde(rename = "userActivation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_activation: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisownParameters {
    #[serde(rename = "handles")]
    pub handles: Vec<Handle>,
    #[serde(rename = "target")]
    pub target: Target,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluateParameters {
    #[serde(rename = "expression")]
    pub expression: String,
    #[serde(rename = "target")]
    pub target: Target,
    #[serde(rename = "awaitPromise")]
    pub await_promise: bool,
    #[serde(rename = "resultOwnership")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_ownership: Option<ResultOwnership>,
    #[serde(rename = "serializationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialization_options: Option<SerializationOptions>,
    #[serde(rename = "userActivation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_activation: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRealmsParameters {
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<BrowsingContext>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RealmType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovePreloadScriptParameters {
    #[serde(rename = "script")]
    pub script: PreloadScript,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPreloadScript {
    #[serde(rename = "method")]
    pub method: ScriptAddPreloadScriptMethod,
    #[serde(rename = "params")]
    pub params: AddPreloadScriptParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallFunction {
    #[serde(rename = "method")]
    pub method: ScriptCallFunctionMethod,
    #[serde(rename = "params")]
    pub params: CallFunctionParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disown {
    #[serde(rename = "method")]
    pub method: ScriptDisownMethod,
    #[serde(rename = "params")]
    pub params: DisownParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evaluate {
    #[serde(rename = "method")]
    pub method: ScriptEvaluateMethod,
    #[serde(rename = "params")]
    pub params: EvaluateParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRealms {
    #[serde(rename = "method")]
    pub method: ScriptGetRealmsMethod,
    #[serde(rename = "params")]
    pub params: GetRealmsParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovePreloadScript {
    #[serde(rename = "method")]
    pub method: ScriptRemovePreloadScriptMethod,
    #[serde(rename = "params")]
    pub params: RemovePreloadScriptParameters,
}

// Generated results

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptResult {
    AddPreloadScriptResult(AddPreloadScriptResult),
    EvaluateResult(EvaluateResult),
    CallFunctionResult(CallFunctionResult),
    DisownResult(DisownResult),
    GetRealmsResult(GetRealmsResult),
    RemovePreloadScriptResult(RemovePreloadScriptResult),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPreloadScriptResult {
    #[serde(rename = "script")]
    pub script: PreloadScript,
}

pub type CallFunctionResult = EvaluateResult;


pub type DisownResult = EmptyResult;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EvaluateResult {
    EvaluateResultSuccess(EvaluateResultSuccess),
    EvaluateResultException(EvaluateResultException),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRealmsResult {
    #[serde(rename = "realms")]
    pub realms: Vec<RealmInfo>,
}

pub type RemovePreloadScriptResult = EmptyResult;


