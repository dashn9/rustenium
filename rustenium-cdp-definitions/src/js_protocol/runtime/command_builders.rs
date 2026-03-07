use super::commands::*;
impl AwaitPromise {
    pub fn builder() -> AwaitPromiseBuilder {
        <AwaitPromiseBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AwaitPromiseBuilder {
    promise_object_id: Option<super::types::RemoteObjectId>,
    return_by_value: Option<bool>,
    generate_preview: Option<bool>,
}
impl AwaitPromiseBuilder {
    pub fn promise_object_id(
        mut self,
        promise_object_id: impl Into<super::types::RemoteObjectId>,
    ) -> Self {
        self.promise_object_id = Some(promise_object_id.into());
        self
    }
    pub fn return_by_value(mut self, return_by_value: impl Into<bool>) -> Self {
        self.return_by_value = Some(return_by_value.into());
        self
    }
    pub fn generate_preview(mut self, generate_preview: impl Into<bool>) -> Self {
        self.generate_preview = Some(generate_preview.into());
        self
    }
    pub fn build(self) -> Result<AwaitPromise, String> {
        Ok(AwaitPromise {
            method: AwaitPromiseMethod::AwaitPromise,
            params: AwaitPromiseParams {
                promise_object_id: self.promise_object_id.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(promise_object_id)
                    )
                })?,
                return_by_value: self.return_by_value,
                generate_preview: self.generate_preview,
            },
        })
    }
}
impl CallFunctionOn {
    pub fn builder() -> CallFunctionOnBuilder {
        <CallFunctionOnBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CallFunctionOnBuilder {
    function_declaration: Option<String>,
    object_id: Option<super::types::RemoteObjectId>,
    arguments: Option<Vec<super::types::CallArgument>>,
    silent: Option<bool>,
    return_by_value: Option<bool>,
    generate_preview: Option<bool>,
    user_gesture: Option<bool>,
    await_promise: Option<bool>,
    execution_context_id: Option<super::types::ExecutionContextId>,
    object_group: Option<String>,
    throw_on_side_effect: Option<bool>,
    unique_context_id: Option<String>,
    serialization_options: Option<super::types::SerializationOptions>,
}
impl CallFunctionOnBuilder {
    pub fn function_declaration(mut self, function_declaration: impl Into<String>) -> Self {
        self.function_declaration = Some(function_declaration.into());
        self
    }
    pub fn object_id(mut self, object_id: impl Into<super::types::RemoteObjectId>) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn argument(mut self, argument: impl Into<super::types::CallArgument>) -> Self {
        let v = self.arguments.get_or_insert(Vec::new());
        v.push(argument.into());
        self
    }
    pub fn arguments<I, S>(mut self, arguments: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::CallArgument>,
    {
        let v = self.arguments.get_or_insert(Vec::new());
        for val in arguments {
            v.push(val.into());
        }
        self
    }
    pub fn silent(mut self, silent: impl Into<bool>) -> Self {
        self.silent = Some(silent.into());
        self
    }
    pub fn return_by_value(mut self, return_by_value: impl Into<bool>) -> Self {
        self.return_by_value = Some(return_by_value.into());
        self
    }
    pub fn generate_preview(mut self, generate_preview: impl Into<bool>) -> Self {
        self.generate_preview = Some(generate_preview.into());
        self
    }
    pub fn user_gesture(mut self, user_gesture: impl Into<bool>) -> Self {
        self.user_gesture = Some(user_gesture.into());
        self
    }
    pub fn await_promise(mut self, await_promise: impl Into<bool>) -> Self {
        self.await_promise = Some(await_promise.into());
        self
    }
    pub fn execution_context_id(
        mut self,
        execution_context_id: impl Into<super::types::ExecutionContextId>,
    ) -> Self {
        self.execution_context_id = Some(execution_context_id.into());
        self
    }
    pub fn object_group(mut self, object_group: impl Into<String>) -> Self {
        self.object_group = Some(object_group.into());
        self
    }
    pub fn throw_on_side_effect(mut self, throw_on_side_effect: impl Into<bool>) -> Self {
        self.throw_on_side_effect = Some(throw_on_side_effect.into());
        self
    }
    pub fn unique_context_id(mut self, unique_context_id: impl Into<String>) -> Self {
        self.unique_context_id = Some(unique_context_id.into());
        self
    }
    pub fn serialization_options(
        mut self,
        serialization_options: impl Into<super::types::SerializationOptions>,
    ) -> Self {
        self.serialization_options = Some(serialization_options.into());
        self
    }
    pub fn build(self) -> Result<CallFunctionOn, String> {
        Ok(CallFunctionOn {
            method: CallFunctionOnMethod::CallFunctionOn,
            params: CallFunctionOnParams {
                function_declaration: self.function_declaration.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(function_declaration)
                    )
                })?,
                object_id: self.object_id,
                arguments: self.arguments,
                silent: self.silent,
                return_by_value: self.return_by_value,
                generate_preview: self.generate_preview,
                user_gesture: self.user_gesture,
                await_promise: self.await_promise,
                execution_context_id: self.execution_context_id,
                object_group: self.object_group,
                throw_on_side_effect: self.throw_on_side_effect,
                unique_context_id: self.unique_context_id,
                serialization_options: self.serialization_options,
            },
        })
    }
}
impl CompileScript {
    pub fn builder() -> CompileScriptBuilder {
        <CompileScriptBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CompileScriptBuilder {
    expression: Option<String>,
    source_url: Option<String>,
    persist_script: Option<bool>,
    execution_context_id: Option<super::types::ExecutionContextId>,
}
impl CompileScriptBuilder {
    pub fn expression(mut self, expression: impl Into<String>) -> Self {
        self.expression = Some(expression.into());
        self
    }
    pub fn source_url(mut self, source_url: impl Into<String>) -> Self {
        self.source_url = Some(source_url.into());
        self
    }
    pub fn persist_script(mut self, persist_script: impl Into<bool>) -> Self {
        self.persist_script = Some(persist_script.into());
        self
    }
    pub fn execution_context_id(
        mut self,
        execution_context_id: impl Into<super::types::ExecutionContextId>,
    ) -> Self {
        self.execution_context_id = Some(execution_context_id.into());
        self
    }
    pub fn build(self) -> Result<CompileScript, String> {
        Ok(CompileScript {
            method: CompileScriptMethod::CompileScript,
            params: CompileScriptParams {
                expression: self.expression.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(expression))
                })?,
                source_url: self.source_url.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(source_url))
                })?,
                persist_script: self.persist_script.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(persist_script))
                })?,
                execution_context_id: self.execution_context_id,
            },
        })
    }
}
impl Evaluate {
    pub fn builder() -> EvaluateBuilder {
        <EvaluateBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct EvaluateBuilder {
    expression: Option<String>,
    object_group: Option<String>,
    include_command_line_api: Option<bool>,
    silent: Option<bool>,
    context_id: Option<super::types::ExecutionContextId>,
    return_by_value: Option<bool>,
    generate_preview: Option<bool>,
    user_gesture: Option<bool>,
    await_promise: Option<bool>,
    throw_on_side_effect: Option<bool>,
    timeout: Option<super::types::TimeDelta>,
    disable_breaks: Option<bool>,
    repl_mode: Option<bool>,
    allow_unsafe_eval_blocked_by_csp: Option<bool>,
    unique_context_id: Option<String>,
    serialization_options: Option<super::types::SerializationOptions>,
}
impl EvaluateBuilder {
    pub fn expression(mut self, expression: impl Into<String>) -> Self {
        self.expression = Some(expression.into());
        self
    }
    pub fn object_group(mut self, object_group: impl Into<String>) -> Self {
        self.object_group = Some(object_group.into());
        self
    }
    pub fn include_command_line_api(mut self, include_command_line_api: impl Into<bool>) -> Self {
        self.include_command_line_api = Some(include_command_line_api.into());
        self
    }
    pub fn silent(mut self, silent: impl Into<bool>) -> Self {
        self.silent = Some(silent.into());
        self
    }
    pub fn context_id(mut self, context_id: impl Into<super::types::ExecutionContextId>) -> Self {
        self.context_id = Some(context_id.into());
        self
    }
    pub fn return_by_value(mut self, return_by_value: impl Into<bool>) -> Self {
        self.return_by_value = Some(return_by_value.into());
        self
    }
    pub fn generate_preview(mut self, generate_preview: impl Into<bool>) -> Self {
        self.generate_preview = Some(generate_preview.into());
        self
    }
    pub fn user_gesture(mut self, user_gesture: impl Into<bool>) -> Self {
        self.user_gesture = Some(user_gesture.into());
        self
    }
    pub fn await_promise(mut self, await_promise: impl Into<bool>) -> Self {
        self.await_promise = Some(await_promise.into());
        self
    }
    pub fn throw_on_side_effect(mut self, throw_on_side_effect: impl Into<bool>) -> Self {
        self.throw_on_side_effect = Some(throw_on_side_effect.into());
        self
    }
    pub fn timeout(mut self, timeout: impl Into<super::types::TimeDelta>) -> Self {
        self.timeout = Some(timeout.into());
        self
    }
    pub fn disable_breaks(mut self, disable_breaks: impl Into<bool>) -> Self {
        self.disable_breaks = Some(disable_breaks.into());
        self
    }
    pub fn repl_mode(mut self, repl_mode: impl Into<bool>) -> Self {
        self.repl_mode = Some(repl_mode.into());
        self
    }
    pub fn allow_unsafe_eval_blocked_by_csp(
        mut self,
        allow_unsafe_eval_blocked_by_csp: impl Into<bool>,
    ) -> Self {
        self.allow_unsafe_eval_blocked_by_csp = Some(allow_unsafe_eval_blocked_by_csp.into());
        self
    }
    pub fn unique_context_id(mut self, unique_context_id: impl Into<String>) -> Self {
        self.unique_context_id = Some(unique_context_id.into());
        self
    }
    pub fn serialization_options(
        mut self,
        serialization_options: impl Into<super::types::SerializationOptions>,
    ) -> Self {
        self.serialization_options = Some(serialization_options.into());
        self
    }
    pub fn build(self) -> Result<Evaluate, String> {
        Ok(Evaluate {
            method: EvaluateMethod::Evaluate,
            params: EvaluateParams {
                expression: self.expression.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(expression))
                })?,
                object_group: self.object_group,
                include_command_line_api: self.include_command_line_api,
                silent: self.silent,
                context_id: self.context_id,
                return_by_value: self.return_by_value,
                generate_preview: self.generate_preview,
                user_gesture: self.user_gesture,
                await_promise: self.await_promise,
                throw_on_side_effect: self.throw_on_side_effect,
                timeout: self.timeout,
                disable_breaks: self.disable_breaks,
                repl_mode: self.repl_mode,
                allow_unsafe_eval_blocked_by_csp: self.allow_unsafe_eval_blocked_by_csp,
                unique_context_id: self.unique_context_id,
                serialization_options: self.serialization_options,
            },
        })
    }
}
impl GetProperties {
    pub fn builder() -> GetPropertiesBuilder {
        <GetPropertiesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetPropertiesBuilder {
    object_id: Option<super::types::RemoteObjectId>,
    own_properties: Option<bool>,
    accessor_properties_only: Option<bool>,
    generate_preview: Option<bool>,
    non_indexed_properties_only: Option<bool>,
}
impl GetPropertiesBuilder {
    pub fn object_id(mut self, object_id: impl Into<super::types::RemoteObjectId>) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn own_properties(mut self, own_properties: impl Into<bool>) -> Self {
        self.own_properties = Some(own_properties.into());
        self
    }
    pub fn accessor_properties_only(mut self, accessor_properties_only: impl Into<bool>) -> Self {
        self.accessor_properties_only = Some(accessor_properties_only.into());
        self
    }
    pub fn generate_preview(mut self, generate_preview: impl Into<bool>) -> Self {
        self.generate_preview = Some(generate_preview.into());
        self
    }
    pub fn non_indexed_properties_only(
        mut self,
        non_indexed_properties_only: impl Into<bool>,
    ) -> Self {
        self.non_indexed_properties_only = Some(non_indexed_properties_only.into());
        self
    }
    pub fn build(self) -> Result<GetProperties, String> {
        Ok(GetProperties {
            method: GetPropertiesMethod::GetProperties,
            params: GetPropertiesParams {
                object_id: self.object_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(object_id))
                })?,
                own_properties: self.own_properties,
                accessor_properties_only: self.accessor_properties_only,
                generate_preview: self.generate_preview,
                non_indexed_properties_only: self.non_indexed_properties_only,
            },
        })
    }
}
impl GlobalLexicalScopeNames {
    pub fn builder() -> GlobalLexicalScopeNamesBuilder {
        <GlobalLexicalScopeNamesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GlobalLexicalScopeNamesBuilder {
    execution_context_id: Option<super::types::ExecutionContextId>,
}
impl GlobalLexicalScopeNamesBuilder {
    pub fn execution_context_id(
        mut self,
        execution_context_id: impl Into<super::types::ExecutionContextId>,
    ) -> Self {
        self.execution_context_id = Some(execution_context_id.into());
        self
    }
    pub fn build(self) -> GlobalLexicalScopeNames {
        GlobalLexicalScopeNames {
            method: GlobalLexicalScopeNamesMethod::GlobalLexicalScopeNames,
            params: GlobalLexicalScopeNamesParams {
                execution_context_id: self.execution_context_id,
            },
        }
    }
}
impl QueryObjects {
    pub fn builder() -> QueryObjectsBuilder {
        <QueryObjectsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct QueryObjectsBuilder {
    prototype_object_id: Option<super::types::RemoteObjectId>,
    object_group: Option<String>,
}
impl QueryObjectsBuilder {
    pub fn prototype_object_id(
        mut self,
        prototype_object_id: impl Into<super::types::RemoteObjectId>,
    ) -> Self {
        self.prototype_object_id = Some(prototype_object_id.into());
        self
    }
    pub fn object_group(mut self, object_group: impl Into<String>) -> Self {
        self.object_group = Some(object_group.into());
        self
    }
    pub fn build(self) -> Result<QueryObjects, String> {
        Ok(QueryObjects {
            method: QueryObjectsMethod::QueryObjects,
            params: QueryObjectsParams {
                prototype_object_id: self.prototype_object_id.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(prototype_object_id)
                    )
                })?,
                object_group: self.object_group,
            },
        })
    }
}
impl ReleaseObject {
    pub fn builder() -> ReleaseObjectBuilder {
        <ReleaseObjectBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ReleaseObjectBuilder {
    object_id: Option<super::types::RemoteObjectId>,
}
impl ReleaseObjectBuilder {
    pub fn object_id(mut self, object_id: impl Into<super::types::RemoteObjectId>) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn build(self) -> Result<ReleaseObject, String> {
        Ok(ReleaseObject {
            method: ReleaseObjectMethod::ReleaseObject,
            params: ReleaseObjectParams {
                object_id: self.object_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(object_id))
                })?,
            },
        })
    }
}
impl ReleaseObjectGroup {
    pub fn builder() -> ReleaseObjectGroupBuilder {
        <ReleaseObjectGroupBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ReleaseObjectGroupBuilder {
    object_group: Option<String>,
}
impl ReleaseObjectGroupBuilder {
    pub fn object_group(mut self, object_group: impl Into<String>) -> Self {
        self.object_group = Some(object_group.into());
        self
    }
    pub fn build(self) -> Result<ReleaseObjectGroup, String> {
        Ok(ReleaseObjectGroup {
            method: ReleaseObjectGroupMethod::ReleaseObjectGroup,
            params: ReleaseObjectGroupParams {
                object_group: self.object_group.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(object_group))
                })?,
            },
        })
    }
}
impl RunScript {
    pub fn builder() -> RunScriptBuilder {
        <RunScriptBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RunScriptBuilder {
    script_id: Option<super::types::ScriptId>,
    execution_context_id: Option<super::types::ExecutionContextId>,
    object_group: Option<String>,
    silent: Option<bool>,
    include_command_line_api: Option<bool>,
    return_by_value: Option<bool>,
    generate_preview: Option<bool>,
    await_promise: Option<bool>,
}
impl RunScriptBuilder {
    pub fn script_id(mut self, script_id: impl Into<super::types::ScriptId>) -> Self {
        self.script_id = Some(script_id.into());
        self
    }
    pub fn execution_context_id(
        mut self,
        execution_context_id: impl Into<super::types::ExecutionContextId>,
    ) -> Self {
        self.execution_context_id = Some(execution_context_id.into());
        self
    }
    pub fn object_group(mut self, object_group: impl Into<String>) -> Self {
        self.object_group = Some(object_group.into());
        self
    }
    pub fn silent(mut self, silent: impl Into<bool>) -> Self {
        self.silent = Some(silent.into());
        self
    }
    pub fn include_command_line_api(mut self, include_command_line_api: impl Into<bool>) -> Self {
        self.include_command_line_api = Some(include_command_line_api.into());
        self
    }
    pub fn return_by_value(mut self, return_by_value: impl Into<bool>) -> Self {
        self.return_by_value = Some(return_by_value.into());
        self
    }
    pub fn generate_preview(mut self, generate_preview: impl Into<bool>) -> Self {
        self.generate_preview = Some(generate_preview.into());
        self
    }
    pub fn await_promise(mut self, await_promise: impl Into<bool>) -> Self {
        self.await_promise = Some(await_promise.into());
        self
    }
    pub fn build(self) -> Result<RunScript, String> {
        Ok(RunScript {
            method: RunScriptMethod::RunScript,
            params: RunScriptParams {
                script_id: self.script_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(script_id))
                })?,
                execution_context_id: self.execution_context_id,
                object_group: self.object_group,
                silent: self.silent,
                include_command_line_api: self.include_command_line_api,
                return_by_value: self.return_by_value,
                generate_preview: self.generate_preview,
                await_promise: self.await_promise,
            },
        })
    }
}
impl SetAsyncCallStackDepth {
    pub fn builder() -> SetAsyncCallStackDepthBuilder {
        <SetAsyncCallStackDepthBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetAsyncCallStackDepthBuilder {
    max_depth: Option<i64>,
}
impl SetAsyncCallStackDepthBuilder {
    pub fn max_depth(mut self, max_depth: impl Into<i64>) -> Self {
        self.max_depth = Some(max_depth.into());
        self
    }
    pub fn build(self) -> Result<SetAsyncCallStackDepth, String> {
        Ok(SetAsyncCallStackDepth {
            method: SetAsyncCallStackDepthMethod::SetAsyncCallStackDepth,
            params: SetAsyncCallStackDepthParams {
                max_depth: self.max_depth.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(max_depth))
                })?,
            },
        })
    }
}
impl SetCustomObjectFormatterEnabled {
    pub fn builder() -> SetCustomObjectFormatterEnabledBuilder {
        <SetCustomObjectFormatterEnabledBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetCustomObjectFormatterEnabledBuilder {
    enabled: Option<bool>,
}
impl SetCustomObjectFormatterEnabledBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn build(self) -> Result<SetCustomObjectFormatterEnabled, String> {
        Ok(SetCustomObjectFormatterEnabled {
            method: SetCustomObjectFormatterEnabledMethod::SetCustomObjectFormatterEnabled,
            params: SetCustomObjectFormatterEnabledParams {
                enabled: self
                    .enabled
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
            },
        })
    }
}
impl SetMaxCallStackSizeToCapture {
    pub fn builder() -> SetMaxCallStackSizeToCaptureBuilder {
        <SetMaxCallStackSizeToCaptureBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetMaxCallStackSizeToCaptureBuilder {
    size: Option<i64>,
}
impl SetMaxCallStackSizeToCaptureBuilder {
    pub fn size(mut self, size: impl Into<i64>) -> Self {
        self.size = Some(size.into());
        self
    }
    pub fn build(self) -> Result<SetMaxCallStackSizeToCapture, String> {
        Ok(SetMaxCallStackSizeToCapture {
            method: SetMaxCallStackSizeToCaptureMethod::SetMaxCallStackSizeToCapture,
            params: SetMaxCallStackSizeToCaptureParams {
                size: self
                    .size
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(size)))?,
            },
        })
    }
}
impl AddBinding {
    pub fn builder() -> AddBindingBuilder {
        <AddBindingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AddBindingBuilder {
    name: Option<String>,
    execution_context_name: Option<String>,
}
impl AddBindingBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn execution_context_name(mut self, execution_context_name: impl Into<String>) -> Self {
        self.execution_context_name = Some(execution_context_name.into());
        self
    }
    pub fn build(self) -> Result<AddBinding, String> {
        Ok(AddBinding {
            method: AddBindingMethod::AddBinding,
            params: AddBindingParams {
                name: self
                    .name
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
                execution_context_name: self.execution_context_name,
            },
        })
    }
}
impl RemoveBinding {
    pub fn builder() -> RemoveBindingBuilder {
        <RemoveBindingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveBindingBuilder {
    name: Option<String>,
}
impl RemoveBindingBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn build(self) -> Result<RemoveBinding, String> {
        Ok(RemoveBinding {
            method: RemoveBindingMethod::RemoveBinding,
            params: RemoveBindingParams {
                name: self
                    .name
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            },
        })
    }
}
impl GetExceptionDetails {
    pub fn builder() -> GetExceptionDetailsBuilder {
        <GetExceptionDetailsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetExceptionDetailsBuilder {
    error_object_id: Option<super::types::RemoteObjectId>,
}
impl GetExceptionDetailsBuilder {
    pub fn error_object_id(
        mut self,
        error_object_id: impl Into<super::types::RemoteObjectId>,
    ) -> Self {
        self.error_object_id = Some(error_object_id.into());
        self
    }
    pub fn build(self) -> Result<GetExceptionDetails, String> {
        Ok(GetExceptionDetails {
            method: GetExceptionDetailsMethod::GetExceptionDetails,
            params: GetExceptionDetailsParams {
                error_object_id: self.error_object_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(error_object_id))
                })?,
            },
        })
    }
}
