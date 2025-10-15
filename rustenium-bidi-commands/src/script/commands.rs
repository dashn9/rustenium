// Generated commands for module

use serde::{Serialize, Deserialize};
use crate::browser::types::UserContext;
use crate::browsing_context::types::BrowsingContext;
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
    pub arguments: Option<Vec<ChannelValue>>,
    #[serde(rename = "contexts")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    pub user_contexts: Option<Vec<UserContext>>,
    #[serde(rename = "sandbox")]
    pub sandbox: Option<String>,
}

fn call_function_parameters_default_user_activation() -> bool {
    false
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
    pub arguments: Option<Vec<LocalValue>>,
    #[serde(rename = "resultOwnership")]
    pub result_ownership: Option<ResultOwnership>,
    #[serde(rename = "serializationOptions")]
    pub serialization_options: Option<SerializationOptions>,
    #[serde(rename = "this")]
    pub this: Option<LocalValue>,
    #[serde(rename = "userActivation")]
    #[serde(default = "call_function_parameters_default_user_activation")]
    pub user_activation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisownParameters {
    #[serde(rename = "handles")]
    pub handles: Vec<Handle>,
    #[serde(rename = "target")]
    pub target: Target,
}

fn evaluate_parameters_default_user_activation() -> bool {
    false
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
    pub result_ownership: Option<ResultOwnership>,
    #[serde(rename = "serializationOptions")]
    pub serialization_options: Option<SerializationOptions>,
    #[serde(rename = "userActivation")]
    #[serde(default = "evaluate_parameters_default_user_activation")]
    pub user_activation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRealmsParameters {
    #[serde(rename = "context")]
    pub context: Option<BrowsingContext>,
    #[serde(rename = "type")]
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
    GetRealmsResult(GetRealmsResult),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPreloadScriptResult {
    #[serde(rename = "script")]
    pub script: PreloadScript,
}

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

