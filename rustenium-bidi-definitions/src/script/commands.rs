use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddPreloadScriptParams {
    #[serde(rename = "functionDeclaration")]
    pub function_declaration: String,
    #[serde(rename = "arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub arguments: Option<Vec<super::types::ChannelValue>>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_contexts: Option<Vec<crate::browser::types::UserContext>>,
    #[serde(rename = "sandbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sandbox: Option<String>,
}
impl AddPreloadScriptParams {
    pub fn new(function_declaration: impl Into<String>) -> Self {
        Self {
            function_declaration: function_declaration.into(),
            arguments: None,
            contexts: None,
            user_contexts: None,
            sandbox: None,
        }
    }
}
impl<T: Into<String>> From<T> for AddPreloadScriptParams {
    fn from(url: T) -> Self {
        AddPreloadScriptParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddPreloadScriptMethod {
    #[serde(rename = "script.addPreloadScript")]
    AddPreloadScript,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddPreloadScript {
    pub method: AddPreloadScriptMethod,
    pub params: AddPreloadScriptParams,
}
impl AddPreloadScript {
    pub const IDENTIFIER: &'static str = "script.addPreloadScript";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for AddPreloadScript {
    type Result = super::results::AddPreloadScriptResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisownParams {
    #[serde(rename = "handles")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub handles: Vec<super::types::Handle>,
    #[serde(rename = "target")]
    pub target: super::types::Target,
}
impl DisownParams {
    pub fn new(
        handles: Vec<super::types::Handle>,
        target: impl Into<super::types::Target>,
    ) -> Self {
        Self {
            handles,
            target: target.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisownMethod {
    #[serde(rename = "script.disown")]
    Disown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disown {
    pub method: DisownMethod,
    pub params: DisownParams,
}
impl Disown {
    pub const IDENTIFIER: &'static str = "script.disown";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Disown {
    type Result = super::results::DisownResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallFunctionParams {
    #[serde(rename = "functionDeclaration")]
    pub function_declaration: String,
    #[serde(rename = "awaitPromise")]
    pub await_promise: bool,
    #[serde(rename = "target")]
    pub target: super::types::Target,
    #[serde(rename = "arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub arguments: Option<Vec<super::types::LocalValue>>,
    #[serde(rename = "resultOwnership")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub result_ownership: Option<super::types::ResultOwnership>,
    #[serde(rename = "serializationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub serialization_options: Option<super::types::SerializationOptions>,
    #[serde(rename = "this")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub this: Option<super::types::LocalValue>,
    #[serde(rename = "userActivation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_call_function_params_user_activation")]
    pub user_activation: Option<bool>,
}
fn default_call_function_params_user_activation() -> Option<bool> {
    Some(false)
}
impl CallFunctionParams {
    pub fn new(
        function_declaration: impl Into<String>,
        await_promise: impl Into<bool>,
        target: impl Into<super::types::Target>,
    ) -> Self {
        Self {
            function_declaration: function_declaration.into(),
            await_promise: await_promise.into(),
            target: target.into(),
            arguments: None,
            result_ownership: None,
            serialization_options: None,
            this: None,
            user_activation: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CallFunctionMethod {
    #[serde(rename = "script.callFunction")]
    CallFunction,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallFunction {
    pub method: CallFunctionMethod,
    pub params: CallFunctionParams,
}
impl CallFunction {
    pub const IDENTIFIER: &'static str = "script.callFunction";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for CallFunction {
    type Result = super::results::CallFunctionResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvaluateParams {
    #[serde(rename = "expression")]
    pub expression: String,
    #[serde(rename = "target")]
    pub target: super::types::Target,
    #[serde(rename = "awaitPromise")]
    pub await_promise: bool,
    #[serde(rename = "resultOwnership")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub result_ownership: Option<super::types::ResultOwnership>,
    #[serde(rename = "serializationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub serialization_options: Option<super::types::SerializationOptions>,
    #[serde(rename = "userActivation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_evaluate_params_user_activation")]
    pub user_activation: Option<bool>,
}
fn default_evaluate_params_user_activation() -> Option<bool> {
    Some(false)
}
impl EvaluateParams {
    pub fn new(
        expression: impl Into<String>,
        target: impl Into<super::types::Target>,
        await_promise: impl Into<bool>,
    ) -> Self {
        Self {
            expression: expression.into(),
            target: target.into(),
            await_promise: await_promise.into(),
            result_ownership: None,
            serialization_options: None,
            user_activation: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EvaluateMethod {
    #[serde(rename = "script.evaluate")]
    Evaluate,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Evaluate {
    pub method: EvaluateMethod,
    pub params: EvaluateParams,
}
impl Evaluate {
    pub const IDENTIFIER: &'static str = "script.evaluate";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Evaluate {
    type Result = super::results::EvaluateResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetRealmsParams {
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub context: Option<crate::browsing_context::types::BrowsingContext>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub r#type: Option<super::types::RealmType>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetRealmsMethod {
    #[serde(rename = "script.getRealms")]
    GetRealms,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRealms {
    pub method: GetRealmsMethod,
    pub params: GetRealmsParams,
}
impl GetRealms {
    pub const IDENTIFIER: &'static str = "script.getRealms";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetRealms {
    type Result = super::results::GetRealmsResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemovePreloadScriptParams {
    #[serde(rename = "script")]
    pub script: super::types::PreloadScript,
}
impl RemovePreloadScriptParams {
    pub fn new(script: impl Into<super::types::PreloadScript>) -> Self {
        Self {
            script: script.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemovePreloadScriptMethod {
    #[serde(rename = "script.removePreloadScript")]
    RemovePreloadScript,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemovePreloadScript {
    pub method: RemovePreloadScriptMethod,
    pub params: RemovePreloadScriptParams,
}
impl RemovePreloadScript {
    pub const IDENTIFIER: &'static str = "script.removePreloadScript";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for RemovePreloadScript {
    type Result = super::results::RemovePreloadScriptResult;
}
group_enum ! (ScriptCommand { AddPreloadScript (AddPreloadScript) , Disown (Disown) , CallFunction (CallFunction) , Evaluate (Evaluate) , GetRealms (GetRealms) , RemovePreloadScript (RemovePreloadScript) } + identifiable);
