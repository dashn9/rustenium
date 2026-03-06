use super::commands::*;
impl AddPreloadScript {
    pub fn builder() -> AddPreloadScriptBuilder {
        AddPreloadScriptBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AddPreloadScriptBuilder {
    function_declaration: Option<String>,
    arguments: Option<Vec<super::types::ChannelValue>>,
    contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    user_contexts: Option<Vec<crate::browser::types::UserContext>>,
    sandbox: Option<String>,
}
impl AddPreloadScriptBuilder {
    pub fn function_declaration(mut self, function_declaration: impl Into<String>) -> Self {
        self.function_declaration = Some(function_declaration.into());
        self
    }
    pub fn argument(mut self, argument: impl Into<super::types::ChannelValue>) -> Self {
        let v = self.arguments.get_or_insert(Vec::new());
        v.push(argument.into());
        self
    }
    pub fn arguments<I, S>(mut self, arguments: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::ChannelValue>,
    {
        let v = self.arguments.get_or_insert(Vec::new());
        for val in arguments {
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
    pub fn sandbox(mut self, sandbox: impl Into<String>) -> Self {
        self.sandbox = Some(sandbox.into());
        self
    }
    pub fn build(self) -> Result<AddPreloadScript, String> {
        Ok(AddPreloadScript {
            method: AddPreloadScriptMethod::AddPreloadScript,
            params: AddPreloadScriptParams {
                function_declaration: self.function_declaration.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(function_declaration)
                    )
                })?,
                arguments: self.arguments,
                contexts: self.contexts,
                user_contexts: self.user_contexts,
                sandbox: self.sandbox,
            },
        })
    }
}
impl Disown {
    pub fn builder() -> DisownBuilder {
        DisownBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DisownBuilder {
    handles: Option<Vec<super::types::Handle>>,
    target: Option<super::types::Target>,
}
impl DisownBuilder {
    pub fn handle(mut self, handle: impl Into<super::types::Handle>) -> Self {
        let v = self.handles.get_or_insert(Vec::new());
        v.push(handle.into());
        self
    }
    pub fn handles<I, S>(mut self, handles: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::Handle>,
    {
        let v = self.handles.get_or_insert(Vec::new());
        for val in handles {
            v.push(val.into());
        }
        self
    }
    pub fn target(mut self, target: impl Into<super::types::Target>) -> Self {
        self.target = Some(target.into());
        self
    }
    pub fn build(self) -> Result<Disown, String> {
        Ok(Disown {
            method: DisownMethod::Disown,
            params: DisownParams {
                handles: self
                    .handles
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(handles)))?,
                target: self
                    .target
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(target)))?,
            },
        })
    }
}
impl CallFunction {
    pub fn builder() -> CallFunctionBuilder {
        CallFunctionBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CallFunctionBuilder {
    function_declaration: Option<String>,
    await_promise: Option<bool>,
    target: Option<super::types::Target>,
    arguments: Option<Vec<super::types::LocalValue>>,
    result_ownership: Option<super::types::ResultOwnership>,
    serialization_options: Option<super::types::SerializationOptions>,
    this: Option<super::types::LocalValue>,
    user_activation: Option<bool>,
}
impl CallFunctionBuilder {
    pub fn function_declaration(mut self, function_declaration: impl Into<String>) -> Self {
        self.function_declaration = Some(function_declaration.into());
        self
    }
    pub fn await_promise(mut self, await_promise: impl Into<bool>) -> Self {
        self.await_promise = Some(await_promise.into());
        self
    }
    pub fn target(mut self, target: impl Into<super::types::Target>) -> Self {
        self.target = Some(target.into());
        self
    }
    pub fn argument(mut self, argument: impl Into<super::types::LocalValue>) -> Self {
        let v = self.arguments.get_or_insert(Vec::new());
        v.push(argument.into());
        self
    }
    pub fn arguments<I, S>(mut self, arguments: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::LocalValue>,
    {
        let v = self.arguments.get_or_insert(Vec::new());
        for val in arguments {
            v.push(val.into());
        }
        self
    }
    pub fn result_ownership(
        mut self,
        result_ownership: impl Into<super::types::ResultOwnership>,
    ) -> Self {
        self.result_ownership = Some(result_ownership.into());
        self
    }
    pub fn serialization_options(
        mut self,
        serialization_options: impl Into<super::types::SerializationOptions>,
    ) -> Self {
        self.serialization_options = Some(serialization_options.into());
        self
    }
    pub fn this(mut self, this: impl Into<super::types::LocalValue>) -> Self {
        self.this = Some(this.into());
        self
    }
    pub fn user_activation(mut self, user_activation: impl Into<bool>) -> Self {
        self.user_activation = Some(user_activation.into());
        self
    }
    pub fn build(self) -> Result<CallFunction, String> {
        Ok(CallFunction {
            method: CallFunctionMethod::CallFunction,
            params: CallFunctionParams {
                function_declaration: self.function_declaration.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(function_declaration)
                    )
                })?,
                await_promise: self.await_promise.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(await_promise))
                })?,
                target: self
                    .target
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(target)))?,
                arguments: self.arguments,
                result_ownership: self.result_ownership,
                serialization_options: self.serialization_options,
                this: self.this,
                user_activation: self.user_activation,
            },
        })
    }
}
impl Evaluate {
    pub fn builder() -> EvaluateBuilder {
        EvaluateBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct EvaluateBuilder {
    expression: Option<String>,
    target: Option<super::types::Target>,
    await_promise: Option<bool>,
    result_ownership: Option<super::types::ResultOwnership>,
    serialization_options: Option<super::types::SerializationOptions>,
    user_activation: Option<bool>,
}
impl EvaluateBuilder {
    pub fn expression(mut self, expression: impl Into<String>) -> Self {
        self.expression = Some(expression.into());
        self
    }
    pub fn target(mut self, target: impl Into<super::types::Target>) -> Self {
        self.target = Some(target.into());
        self
    }
    pub fn await_promise(mut self, await_promise: impl Into<bool>) -> Self {
        self.await_promise = Some(await_promise.into());
        self
    }
    pub fn result_ownership(
        mut self,
        result_ownership: impl Into<super::types::ResultOwnership>,
    ) -> Self {
        self.result_ownership = Some(result_ownership.into());
        self
    }
    pub fn serialization_options(
        mut self,
        serialization_options: impl Into<super::types::SerializationOptions>,
    ) -> Self {
        self.serialization_options = Some(serialization_options.into());
        self
    }
    pub fn user_activation(mut self, user_activation: impl Into<bool>) -> Self {
        self.user_activation = Some(user_activation.into());
        self
    }
    pub fn build(self) -> Result<Evaluate, String> {
        Ok(Evaluate {
            method: EvaluateMethod::Evaluate,
            params: EvaluateParams {
                expression: self.expression.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(expression))
                })?,
                target: self
                    .target
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(target)))?,
                await_promise: self.await_promise.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(await_promise))
                })?,
                result_ownership: self.result_ownership,
                serialization_options: self.serialization_options,
                user_activation: self.user_activation,
            },
        })
    }
}
impl GetRealms {
    pub fn builder() -> GetRealmsBuilder {
        GetRealmsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetRealmsBuilder {
    context: Option<crate::browsing_context::types::BrowsingContext>,
    r#type: Option<super::types::RealmType>,
}
impl GetRealmsBuilder {
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<super::types::RealmType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> GetRealms {
        GetRealms {
            method: GetRealmsMethod::GetRealms,
            params: GetRealmsParams {
                context: self.context,
                r#type: self.r#type,
            },
        }
    }
}
impl RemovePreloadScript {
    pub fn builder() -> RemovePreloadScriptBuilder {
        RemovePreloadScriptBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct RemovePreloadScriptBuilder {
    script: Option<super::types::PreloadScript>,
}
impl RemovePreloadScriptBuilder {
    pub fn script(mut self, script: impl Into<super::types::PreloadScript>) -> Self {
        self.script = Some(script.into());
        self
    }
    pub fn build(self) -> Result<RemovePreloadScript, String> {
        Ok(RemovePreloadScript {
            method: RemovePreloadScriptMethod::RemovePreloadScript,
            params: RemovePreloadScriptParams {
                script: self
                    .script
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(script)))?,
            },
        })
    }
}
