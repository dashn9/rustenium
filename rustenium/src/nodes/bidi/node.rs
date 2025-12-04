use rustenium_bidi_commands::browsing_context::types::{ClipRectangle, ElementClipRectangle, ElementEnum, ImageFormat, Locator, OriginUnion};
use rustenium_bidi_commands::script::types::{
    ContextTarget, LocalValue, NodeRemoteValue,
    PrimitiveProtocolValue, RemoteReference, RemoteValue, SharedReference, Target
};
use rustenium_bidi_commands::script::commands::{CallFunction, CallFunctionParameters, EvaluateResult, ScriptCallFunctionMethod};
use rustenium_bidi_commands::{BrowsingContextCommand, BrowsingContextResult, CommandData, ResultData, ScriptCommand, ScriptResult};
use rustenium_bidi_commands::browsing_context::commands::{
    BrowsingContextCaptureScreenshotMethod, CaptureScreenshot, CaptureScreenshotParameters,
};
use rustenium_core::transport::ConnectionTransport;
use rustenium_core::{CommandResponseState, Session};
use rustenium_core::error::{CommandResultError, ResponseReceiveTimeoutError, SessionSendError};
use crate::nodes::NodePosition;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;

pub struct BidiNode<T: ConnectionTransport = rustenium_core::transport::WebsocketConnectionTransport> {
    _raw_node: NodeRemoteValue,
    children: Vec<BidiNode<T>>,
    pub locator: Locator,
    pub position: Option<NodePosition>,
    pub session: Option<Arc<Mutex<Session<T>>>>,
    pub context: Option<String>,
}

impl<T: ConnectionTransport> BidiNode<T> {
    pub fn new(
        _raw_node: NodeRemoteValue,
        locator: Locator,
        session: Arc<Mutex<Session<T>>>,
        context: String,
    ) -> Self {
        let mut children = Vec::new();
        if let Some(node_properties) = &_raw_node.value {
            if let Some(node_properties_children) = node_properties.children.clone() {
                children.extend(Self::process_node_value_to_children(
                    node_properties_children,
                    &locator,
                    session.clone(),
                    context.clone(),
                ));
            }
        }
        Self {
            _raw_node,
            children,
            locator,
            position: None,
            session: Some(session),
            context: Some(context),
        }
    }

    fn process_node_value_to_children(
        children: Vec<NodeRemoteValue>,
        locator: &Locator,
        session: Arc<Mutex<Session<T>>>,
        context: String,
    ) -> Vec<BidiNode<T>> {
        let mut chrome_node_children = Vec::new();
        for child in children {
            let chrome_node = BidiNode::new(child, locator.clone(), session.clone(), context.clone());
            chrome_node_children.push(chrome_node);
        }
        chrome_node_children
    }

    pub fn get_raw_node_ref(&self) -> &NodeRemoteValue{
        &self._raw_node
    }

    /// Private helper to send a command via the session
    async fn send_command(&self, command: CommandData) -> Result<ResultData, SessionSendError> {
        let session = self.session.as_ref().ok_or_else(|| {
            SessionSendError::ResponseReceiveTimeoutError(ResponseReceiveTimeoutError)
        })?;

        let rx = {
            let mut sess = session.lock().await;
            sess.send_and_get_receiver(command).await
        };

        match timeout(Duration::from_secs(100), rx).await {
            Ok(Ok(command_result)) => match command_result {
                CommandResponseState::Success(response) => Ok(response.result),
                CommandResponseState::Error(err) => Err(SessionSendError::ErrorResponse(err))
            },
            Ok(Err(_)) => Err(SessionSendError::ResponseReceiveTimeoutError(ResponseReceiveTimeoutError)),
            Err(_) => Err(SessionSendError::ResponseReceiveTimeoutError(ResponseReceiveTimeoutError)),
        }
    }

    /// Get the position of the node, updating it if not available
    pub async fn get_position(&mut self) -> Result<Option<&NodePosition>, crate::error::EvaluateResultError> {
        if self.position.is_none() {
            self.update_position().await?;
        }
        Ok(self.position.as_ref())
    }

    /// Update the position of the node
    pub async fn update_position(&mut self) -> Result<bool, crate::error::EvaluateResultError> {
        let shared_id = match self._raw_node.shared_id.as_ref() {
            Some(id) => id.clone(),
            None => return Ok(false),
        };

        let shared_reference = LocalValue::RemoteReference(
            RemoteReference::SharedReference(SharedReference {
                shared_id,
                handle: self._raw_node.handle.clone(),
                extensible: Default::default(),
            }),
        );

        let script =
            "function() {
    if (!this) {
        return null;
    }
    const rect = this.getBoundingClientRect();
    const scroll_x = window.pageXOffset || document.documentElement.scrollLeft;
    const scroll_y = window.pageYOffset || document.documentElement.scrollTop;

    return JSON.stringify({
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
        scroll_x: rect.x + scroll_x,
        scroll_y: rect.y + scroll_y
    });
}";

        let context = self.context.as_ref().ok_or(crate::error::EvaluateResultError::NoSharedId)?;

        let target = Target::ContextTarget(ContextTarget {
            context: context.clone(),
            sandbox: None,
        });

        let command = CommandData::ScriptCommand(ScriptCommand::CallFunction(CallFunction {
            method: ScriptCallFunctionMethod::ScriptCallFunction,
            params: CallFunctionParameters {
                function_declaration: script.to_string(),
                await_promise: false,
                target,
                arguments: None,
                result_ownership: None,
                serialization_options: None,
                this: Some(shared_reference),
                user_activation: None,
            },
        }));

        let result = self.send_command(command).await
            .map_err(|e| crate::error::EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(e)
            ))?;

        match result {
            ResultData::ScriptResult(script_result) => {
                let evaluate_result = match script_result {
                    ScriptResult::CallFunctionResult(eval_result) => eval_result,
                    ScriptResult::EvaluateResult(eval_result) => eval_result,
                    _ => return Ok(false),
                };

                match evaluate_result {
                    EvaluateResult::EvaluateResultSuccess(evaluate_result_success) => {
                        if let RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(rv_sv)) =
                            evaluate_result_success.result
                        {
                            let position: Option<NodePosition> = serde_json::from_str(rv_sv.value.as_str()).ok();
                            if let Some(pos) = position {
                                self.position = Some(pos);
                                return Ok(true);
                            }
                        }
                        Ok(false)
                    }
                    EvaluateResult::EvaluateResultException(_) => Ok(false),
                }
            }
            _ => Ok(false),
        }
    }

    /// Get the inner text of the node
    pub async fn get_inner_text(&self) -> Result<String, crate::error::EvaluateResultError> {
        let shared_id = self._raw_node.shared_id.as_ref()
            .ok_or(crate::error::EvaluateResultError::NoSharedId)?;

        let shared_reference = LocalValue::RemoteReference(
            RemoteReference::SharedReference(SharedReference {
                shared_id: shared_id.clone(),
                handle: self._raw_node.handle.clone(),
                extensible: Default::default(),
            }),
        );

        let script = "function() { return this.innerText || ''; }";
        let context = self.context.as_ref().ok_or(crate::error::EvaluateResultError::NoSharedId)?;

        let target = Target::ContextTarget(ContextTarget {
            context: context.clone(),
            sandbox: None,
        });

        let command = CommandData::ScriptCommand(ScriptCommand::CallFunction(CallFunction {
            method: ScriptCallFunctionMethod::ScriptCallFunction,
            params: CallFunctionParameters {
                function_declaration: script.to_string(),
                await_promise: false,
                target,
                arguments: None,
                result_ownership: None,
                serialization_options: None,
                this: Some(shared_reference),
                user_activation: None,
            },
        }));

        let result = self.send_command(command).await
            .map_err(|e| crate::error::EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(e)
            ))?;

        match result {
            ResultData::ScriptResult(script_result) => {
                let evaluate_result = match script_result {
                    ScriptResult::CallFunctionResult(eval_result) => eval_result,
                    ScriptResult::EvaluateResult(eval_result) => eval_result,
                    _ => return Ok(String::new()),
                };

                match evaluate_result {
                    EvaluateResult::EvaluateResultSuccess(evaluate_result_success) => {
                        if let RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(rv_sv)) =
                            evaluate_result_success.result
                        {
                            return Ok(rv_sv.value);
                        }
                        Ok(String::new())
                    }
                    EvaluateResult::EvaluateResultException(_) => Ok(String::new()),
                }
            }
            _ => Ok(String::new()),
        }
    }

    /// Get the text content of the node
    pub async fn get_text_content(&self) -> Result<String, crate::error::EvaluateResultError> {
        let shared_id = self._raw_node.shared_id.as_ref()
            .ok_or(crate::error::EvaluateResultError::NoSharedId)?;

        let shared_reference = LocalValue::RemoteReference(
            RemoteReference::SharedReference(SharedReference {
                shared_id: shared_id.clone(),
                handle: self._raw_node.handle.clone(),
                extensible: Default::default(),
            }),
        );

        let script = "function() { return this.textContent || ''; }";
        let context = self.context.as_ref().ok_or(crate::error::EvaluateResultError::NoSharedId)?;

        let target = Target::ContextTarget(ContextTarget {
            context: context.clone(),
            sandbox: None,
        });

        let command = CommandData::ScriptCommand(ScriptCommand::CallFunction(CallFunction {
            method: ScriptCallFunctionMethod::ScriptCallFunction,
            params: CallFunctionParameters {
                function_declaration: script.to_string(),
                await_promise: false,
                target,
                arguments: None,
                result_ownership: None,
                serialization_options: None,
                this: Some(shared_reference),
                user_activation: None,
            },
        }));

        let result = self.send_command(command).await
            .map_err(|e| crate::error::EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(e)
            ))?;

        match result {
            ResultData::ScriptResult(script_result) => {
                let evaluate_result = match script_result {
                    ScriptResult::CallFunctionResult(eval_result) => eval_result,
                    ScriptResult::EvaluateResult(eval_result) => eval_result,
                    _ => return Ok(String::new()),
                };

                match evaluate_result {
                    EvaluateResult::EvaluateResultSuccess(evaluate_result_success) => {
                        if let RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(rv_sv)) =
                            evaluate_result_success.result
                        {
                            return Ok(rv_sv.value);
                        }
                        Ok(String::new())
                    }
                    EvaluateResult::EvaluateResultException(_) => Ok(String::new()),
                }
            }
            _ => Ok(String::new()),
        }
    }

    /// Get a specific attribute value from the node
    pub fn get_attribute(&self, attribute_name: &str) -> Option<String> {
        self._raw_node.value.as_ref()
            .and_then(|props| props.attributes.as_ref())
            .and_then(|attrs| attrs.get(attribute_name).cloned())
    }

    /// Get all attributes from the node
    pub fn get_attributes(&self) -> std::collections::HashMap<String, String> {
        self._raw_node.value.as_ref()
            .and_then(|props| props.attributes.clone())
            .unwrap_or_default()
    }

    /// Check if the node is visible in the viewport
    pub async fn is_visible(&self) -> Result<bool, crate::error::EvaluateResultError> {
        let shared_id = self._raw_node.shared_id.as_ref()
            .ok_or(crate::error::EvaluateResultError::NoSharedId)?;

        let shared_reference = LocalValue::RemoteReference(
            RemoteReference::SharedReference(SharedReference {
                shared_id: shared_id.clone(),
                handle: self._raw_node.handle.clone(),
                extensible: Default::default(),
            }),
        );

        let script = r#"function() {
            if (!this) return false;
            const rect = this.getBoundingClientRect();
            const style = window.getComputedStyle(this);
            return rect.width > 0 &&
                   rect.height > 0 &&
                   style.visibility !== 'hidden' &&
                   style.display !== 'none' &&
                   style.opacity !== '0';
        }"#;
        let context = self.context.as_ref().ok_or(crate::error::EvaluateResultError::NoSharedId)?;

        let target = Target::ContextTarget(ContextTarget {
            context: context.clone(),
            sandbox: None,
        });

        let command = CommandData::ScriptCommand(ScriptCommand::CallFunction(CallFunction {
            method: ScriptCallFunctionMethod::ScriptCallFunction,
            params: CallFunctionParameters {
                function_declaration: script.to_string(),
                await_promise: false,
                target,
                arguments: None,
                result_ownership: None,
                serialization_options: None,
                this: Some(shared_reference),
                user_activation: None,
            },
        }));

        let result = self.send_command(command).await
            .map_err(|e| crate::error::EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(e)
            ))?;

        match result {
            ResultData::ScriptResult(script_result) => {
                let evaluate_result = match script_result {
                    ScriptResult::CallFunctionResult(eval_result) => eval_result,
                    ScriptResult::EvaluateResult(eval_result) => eval_result,
                    _ => return Ok(false),
                };

                match evaluate_result {
                    EvaluateResult::EvaluateResultSuccess(evaluate_result_success) => {
                        if let RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::BooleanValue(bv)) =
                            evaluate_result_success.result
                        {
                            return Ok(bv.value);
                        }
                        Ok(false)
                    }
                    EvaluateResult::EvaluateResultException(_) => Ok(false),
                }
            }
            _ => Ok(false),
        }
    }

    /// Scroll the node into view
    pub async fn scroll_into_view(&self) -> Result<(), crate::error::EvaluateResultError> {
        let shared_id = self._raw_node.shared_id.as_ref()
            .ok_or(crate::error::EvaluateResultError::NoSharedId)?;

        let shared_reference = LocalValue::RemoteReference(
            RemoteReference::SharedReference(SharedReference {
                shared_id: shared_id.clone(),
                handle: self._raw_node.handle.clone(),
                extensible: Default::default(),
            }),
        );

        let script = "function() { if (!this) { return null; } this.scrollIntoView({block: 'center', inline: 'center', behavior: 'instant'}); }";
        let context = self.context.as_ref().ok_or(crate::error::EvaluateResultError::NoSharedId)?;

        let target = Target::ContextTarget(ContextTarget {
            context: context.clone(),
            sandbox: None,
        });

        let command = CommandData::ScriptCommand(ScriptCommand::CallFunction(CallFunction {
            method: ScriptCallFunctionMethod::ScriptCallFunction,
            params: CallFunctionParameters {
                function_declaration: script.to_string(),
                await_promise: false,
                target,
                arguments: None,
                result_ownership: None,
                serialization_options: None,
                this: Some(shared_reference),
                user_activation: None,
            },
        }));

        self.send_command(command).await
            .map_err(|e| crate::error::EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(e)
            ))?;

        Ok(())
    }

    /// Get the inner HTML of the node
    pub async fn get_inner_html(&self) -> Result<String, crate::error::EvaluateResultError> {
        let shared_id = self._raw_node.shared_id.as_ref()
            .ok_or(crate::error::EvaluateResultError::NoSharedId)?;

        let shared_reference = LocalValue::RemoteReference(
            RemoteReference::SharedReference(SharedReference {
                shared_id: shared_id.clone(),
                handle: self._raw_node.handle.clone(),
                extensible: Default::default(),
            }),
        );

        let script = "function() { return this.innerHTML || ''; }";
        let context = self.context.as_ref().ok_or(crate::error::EvaluateResultError::NoSharedId)?;

        let target = Target::ContextTarget(ContextTarget {
            context: context.clone(),
            sandbox: None,
        });

        let command = CommandData::ScriptCommand(ScriptCommand::CallFunction(CallFunction {
            method: ScriptCallFunctionMethod::ScriptCallFunction,
            params: CallFunctionParameters {
                function_declaration: script.to_string(),
                await_promise: false,
                target,
                arguments: None,
                result_ownership: None,
                serialization_options: None,
                this: Some(shared_reference),
                user_activation: None,
            },
        }));

        let result = self.send_command(command).await
            .map_err(|e| crate::error::EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(e)
            ))?;

        match result {
            ResultData::ScriptResult(script_result) => {
                let evaluate_result = match script_result {
                    ScriptResult::CallFunctionResult(eval_result) => eval_result,
                    ScriptResult::EvaluateResult(eval_result) => eval_result,
                    _ => return Ok(String::new()),
                };

                match evaluate_result {
                    EvaluateResult::EvaluateResultSuccess(evaluate_result_success) => {
                        if let RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(rv_sv)) =
                            evaluate_result_success.result
                        {
                            return Ok(rv_sv.value);
                        }
                        Ok(String::new())
                    }
                    EvaluateResult::EvaluateResultException(_) => Ok(String::new()),
                }
            }
            _ => Ok(String::new()),
        }
    }

    /// Delete the node from the DOM
    pub async fn delete(&self) -> Result<(), crate::error::EvaluateResultError> {
        let shared_id = self._raw_node.shared_id.as_ref()
            .ok_or(crate::error::EvaluateResultError::NoSharedId)?;

        let shared_reference = LocalValue::RemoteReference(
            RemoteReference::SharedReference(SharedReference {
                shared_id: shared_id.clone(),
                handle: self._raw_node.handle.clone(),
                extensible: Default::default(),
            }),
        );

        let script = "function() { if (this && this.parentNode) { this.parentNode.removeChild(this); } }";
        let context = self.context.as_ref().ok_or(crate::error::EvaluateResultError::NoSharedId)?;

        let target = Target::ContextTarget(ContextTarget {
            context: context.clone(),
            sandbox: None,
        });

        let command = CommandData::ScriptCommand(ScriptCommand::CallFunction(CallFunction {
            method: ScriptCallFunctionMethod::ScriptCallFunction,
            params: CallFunctionParameters {
                function_declaration: script.to_string(),
                await_promise: false,
                target,
                arguments: None,
                result_ownership: None,
                serialization_options: None,
                this: Some(shared_reference),
                user_activation: None,
            },
        }));

        self.send_command(command).await
            .map_err(|e| crate::error::EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(e)
            ))?;

        Ok(())
    }

    /// Capture a screenshot of this element
    /// If `save_path` is provided:
    ///   - If it's a directory, saves with auto-generated filename (screenshot_TIMESTAMP.png)
    ///   - If it's a file path, saves to that exact location
    ///   Returns the final path where the file was saved
    /// Otherwise, returns the base64-encoded image data
    pub async fn screenshot(
        &self,
        origin: Option<OriginUnion>,
        format: Option<ImageFormat>,
        save_path: Option<&str>,
    ) -> Result<String, crate::error::ScreenshotError> {
        let shared_id = self._raw_node.shared_id.as_ref()
            .ok_or(crate::error::ScreenshotError::NoSharedId)?;

        let context = self.context.as_ref()
            .ok_or(crate::error::ScreenshotError::NoContext)?;

        // Create clip rectangle for this element
        let clip = Some(ClipRectangle::ElementClipRectangle(ElementClipRectangle {
            r#type: ElementEnum::Element,
            element: SharedReference {
                shared_id: shared_id.clone(),
                handle: self._raw_node.handle.clone(),
                extensible: Default::default(),
            },
        }));

        let result = self
            .send_command(CommandData::BrowsingContextCommand(
                BrowsingContextCommand::CaptureScreenshot(CaptureScreenshot {
                    method: BrowsingContextCaptureScreenshotMethod::BrowsingContextCaptureScreenshot,
                    params: CaptureScreenshotParameters {
                        context: context.clone(),
                        origin,
                        format,
                        clip,
                    },
                }),
            ))
            .await;

        let base64_data = match result {
            Ok(ResultData::BrowsingContextResult(browsing_context_result)) => {
                match browsing_context_result {
                    BrowsingContextResult::CaptureScreenshotResult(screenshot_result) => {
                        screenshot_result.data
                    }
                    _ => return Err(crate::error::ScreenshotError::CommandResultError(
                        CommandResultError::InvalidResultTypeError(
                            ResultData::BrowsingContextResult(browsing_context_result),
                        ),
                    )),
                }
            }
            Ok(result) => return Err(crate::error::ScreenshotError::CommandResultError(
                CommandResultError::InvalidResultTypeError(result),
            )),
            Err(err) => return Err(crate::error::ScreenshotError::CommandResultError(
                CommandResultError::SessionSendError(err),
            )),
        };

        // If save_path is provided, save to file
        if let Some(path) = save_path {
            use std::path::Path;

            let path_obj = Path::new(path);

            // Determine the final file path
            let final_path = if path_obj.is_dir() {
                // Generate timestamp-based filename
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis())
                    .unwrap_or(0);
                let filename = format!("screenshot_{}.png", timestamp);
                path_obj.join(filename)
            } else {
                // Verify parent directory exists
                if let Some(parent) = path_obj.parent() {
                    if !parent.as_os_str().is_empty() && !parent.exists() {
                        return Err(crate::error::ScreenshotError::InvalidPath(
                            format!("Parent directory does not exist: {}", parent.display())
                        ));
                    }
                }
                path_obj.to_path_buf()
            };

            // Decode base64 and write to file
            use base64::{Engine as _, engine::general_purpose};
            let decoded = general_purpose::STANDARD.decode(&base64_data)
                .map_err(|e| crate::error::ScreenshotError::Base64DecodeError(e.to_string()))?;

            std::fs::write(&final_path, decoded)
                .map_err(|e| crate::error::ScreenshotError::FileWriteError(e.to_string()))?;

            Ok(final_path.to_string_lossy().to_string())
        } else {
            Ok(base64_data)
        }
    }
}