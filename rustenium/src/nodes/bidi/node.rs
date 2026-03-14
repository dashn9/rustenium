use crate::error::ScreenshotError;
use crate::nodes::NodePosition;
use rustenium_bidi_definitions::Command;
use rustenium_bidi_definitions::base::CommandResponse;
use rustenium_bidi_definitions::browsing_context::commands::CaptureScreenshot;
use rustenium_bidi_definitions::browsing_context::results::CaptureScreenshotResult;
use rustenium_bidi_definitions::browsing_context::type_builders::ElementClipRectangleBuilder;
use rustenium_bidi_definitions::browsing_context::types::{BrowsingContext, ElementClipRectangleType, Locator};
use rustenium_bidi_definitions::script::command_builders::CallFunctionBuilder;
use rustenium_bidi_definitions::script::results::{CallFunctionResult, EvaluateResult};
use rustenium_bidi_definitions::script::type_builders::{
    ContextTargetBuilder, SharedReferenceBuilder,
};
use rustenium_bidi_definitions::script::types::{
    ContextTarget, NodeRemoteValue, PrimitiveProtocolValue, RemoteReference, RemoteValue,
};
use rustenium_core::error::{CommandResultError, ResponseReceiveTimeoutError, SessionSendError};
use rustenium_core::transport::ConnectionTransport;
use rustenium_core::{BidiSession, CommandResponseState};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;

pub(crate) struct BidiNode<
    T: ConnectionTransport = rustenium_core::transport::WebsocketConnectionTransport,
> {
    _raw_node: NodeRemoteValue,
    pub children: Vec<BidiNode<T>>,
    pub locator: Locator,
    pub position: Option<NodePosition>,
    pub session: Option<Arc<Mutex<BidiSession<T>>>>,
    pub context_id: BrowsingContext,
}

impl<T: ConnectionTransport> BidiNode<T> {
    pub fn new(
        _raw_node: NodeRemoteValue,
        locator: Locator,
        session: Arc<Mutex<BidiSession<T>>>,
        context_id: impl Into<BrowsingContext>,
    ) -> Self {
        let mut children = Vec::new();
        let context_id = context_id.into();
        if let Some(node_properties) = &_raw_node.value {
            if let Some(node_properties_children) = node_properties.children.clone() {
                children.extend(Self::process_node_value_to_children(
                    node_properties_children,
                    &locator,
                    session.clone(),
                    context_id.clone(),
                ));
            }
        }
        Self {
            _raw_node,
            children,
            locator,
            position: None,
            session: Some(session),
            context_id: context_id,
        }
    }

    fn process_node_value_to_children(
        children: Vec<NodeRemoteValue>,
        locator: &Locator,
        session: Arc<Mutex<BidiSession<T>>>,
        context_id: BrowsingContext,
    ) -> Vec<BidiNode<T>> {
        let mut chrome_node_children = Vec::new();
        for child in children {
            let chrome_node =
                BidiNode::new(child, locator.to_owned(), session.clone(), context_id.clone());
            chrome_node_children.push(chrome_node);
        }
        chrome_node_children
    }

    pub fn get_raw_node_ref(&self) -> &NodeRemoteValue {
        &self._raw_node
    }

    async fn send_command(
        &self,
        command: impl Into<Command>,
    ) -> Result<CommandResponse, SessionSendError> {
        let session = self.session.as_ref().ok_or_else(|| {
            SessionSendError::ResponseReceiveTimeoutError(ResponseReceiveTimeoutError)
        })?;

        let rx = {
            let mut sess = session.lock().await;
            sess.send_and_get_receiver(command.into()).await
        };

        match timeout(Duration::from_secs(100), rx).await {
            Ok(Ok(command_result)) => match command_result {
                CommandResponseState::Success(response) => Ok(response),
                CommandResponseState::Error(err) => Err(SessionSendError::ErrorResponse(err)),
            },
            Ok(Err(_)) => Err(SessionSendError::ResponseReceiveTimeoutError(
                ResponseReceiveTimeoutError,
            )),
            Err(_) => Err(SessionSendError::ResponseReceiveTimeoutError(
                ResponseReceiveTimeoutError,
            )),
        }
    }

    fn shared_reference(&self) -> Option<RemoteReference> {
        let shared_id = self._raw_node.shared_id.as_ref()?.clone();
        let mut builder = SharedReferenceBuilder::default().shared_id(shared_id);
        if let Some(handle) = self._raw_node.handle.clone() {
            builder = builder.handle(handle);
        }
        Some(builder.build().unwrap().into())
    }

    fn context_target(&self) -> ContextTarget {
        ContextTargetBuilder::default()
            .context(self.context_id.clone())
            .build()
            .unwrap()
    }

    pub async fn get_position(
        &mut self,
    ) -> Result<Option<&NodePosition>, crate::error::EvaluateResultError> {
        if self.position.is_none() {
            self.update_position().await?;
        }
        Ok(self.position.as_ref())
    }

    pub async fn update_position(&mut self) -> Result<bool, crate::error::EvaluateResultError> {
        let remote_reference = self
            .shared_reference()
            .ok_or(crate::error::EvaluateResultError::NoSharedId)?;
        let target = self.context_target();

        let script = "function() {
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

        let command = CallFunctionBuilder::default()
            .function_declaration(script.to_string())
            .await_promise(false)
            .target(target)
            .this(remote_reference)
            .build()
            .unwrap();

        let response = self.send_command(command).await.map_err(|e| {
            crate::error::EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(e),
            )
        })?;

        let evaluate_result: CallFunctionResult = response.result.try_into().unwrap();
        match evaluate_result {
            EvaluateResult::EvaluateResultSuccess(evaluate_result_success) => {
                if let RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(
                    rv_sv,
                )) = evaluate_result_success.result
                {
                    let position: Option<NodePosition> =
                        serde_json::from_str(rv_sv.value.as_str()).ok();
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

    pub async fn get_inner_text(&self) -> Result<String, crate::error::EvaluateResultError> {
        let remote_reference = self
            .shared_reference()
            .ok_or(crate::error::EvaluateResultError::NoSharedId)?;
        let target = self.context_target();

        let command = CallFunctionBuilder::default()
            .function_declaration("function() { return this.innerText || ''; }".to_string())
            .await_promise(false)
            .target(target)
            .this(remote_reference)
            .build()
            .unwrap();

        let response = self.send_command(command).await.map_err(|e| {
            crate::error::EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(e),
            )
        })?;

        let result: CallFunctionResult = response.result.try_into().unwrap();
        match result {
            EvaluateResult::EvaluateResultSuccess(success) => {
                if let RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(
                    sv,
                )) = success.result
                {
                    Ok(sv.value)
                } else {
                    Ok(String::new())
                }
            }
            EvaluateResult::EvaluateResultException(_) => Ok(String::new()),
        }
    }

    pub async fn get_text_content(&self) -> Result<String, crate::error::EvaluateResultError> {
        let remote_reference = self
            .shared_reference()
            .ok_or(crate::error::EvaluateResultError::NoSharedId)?;
        let target = self.context_target();

        let command = CallFunctionBuilder::default()
            .function_declaration("function() { return this.textContent || ''; }".to_string())
            .await_promise(false)
            .target(target)
            .this(remote_reference)
            .build()
            .unwrap();

        let response = self.send_command(command).await.map_err(|e| {
            crate::error::EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(e),
            )
        })?;

        let result: CallFunctionResult = response.result.try_into().unwrap();
        match result {
            EvaluateResult::EvaluateResultSuccess(success) => {
                if let RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(
                    sv,
                )) = success.result
                {
                    Ok(sv.value)
                } else {
                    Ok(String::new())
                }
            }
            EvaluateResult::EvaluateResultException(_) => Ok(String::new()),
        }
    }

    pub fn get_attribute(&self, attribute_name: &str) -> Option<serde_json::Value> {
        self._raw_node
            .value
            .as_ref()
            .and_then(|props| props.attributes.as_ref())
            .and_then(|attrs| attrs.get(attribute_name).cloned())
    }

    pub fn get_attributes(&self) -> std::collections::HashMap<String, serde_json::Value> {
        self._raw_node
            .value
            .as_ref()
            .and_then(|props| props.attributes.clone())
            .unwrap_or_default()
    }

    pub async fn is_visible(&self) -> Result<bool, crate::error::EvaluateResultError> {
        let remote_reference = self
            .shared_reference()
            .ok_or(crate::error::EvaluateResultError::NoSharedId)?;
        let target = self.context_target();

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

        let command = CallFunctionBuilder::default()
            .function_declaration(script.to_string())
            .await_promise(false)
            .target(target)
            .this(remote_reference)
            .build()
            .unwrap();

        let response = self.send_command(command).await.map_err(|e| {
            crate::error::EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(e),
            )
        })?;

        let result: CallFunctionResult = response.result.try_into().unwrap();
        match result {
            EvaluateResult::EvaluateResultSuccess(success) => {
                if let RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::BooleanValue(
                    bv,
                )) = success.result
                {
                    Ok(bv.value)
                } else {
                    Ok(false)
                }
            }
            EvaluateResult::EvaluateResultException(_) => Ok(false),
        }
    }

    pub async fn scroll_into_view(&self) -> Result<(), crate::error::EvaluateResultError> {
        let remote_reference = self
            .shared_reference()
            .ok_or(crate::error::EvaluateResultError::NoSharedId)?;
        let target = self.context_target();

        let command = CallFunctionBuilder::default()
            .function_declaration("function() { if (!this) { return null; } this.scrollIntoView({block: 'center', inline: 'center', behavior: 'instant'}); }".to_string())
            .await_promise(false)
            .target(target)
            .this(remote_reference)
            .build()
            .unwrap();

        self.send_command(command).await.map_err(|e| {
            crate::error::EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(e),
            )
        })?;

        Ok(())
    }

    pub async fn get_inner_html(&self) -> Result<String, crate::error::EvaluateResultError> {
        let remote_reference = self
            .shared_reference()
            .ok_or(crate::error::EvaluateResultError::NoSharedId)?;
        let target = self.context_target();

        let command = CallFunctionBuilder::default()
            .function_declaration("function() { return this.innerHTML || ''; }".to_string())
            .await_promise(false)
            .target(target)
            .this(remote_reference)
            .build()
            .unwrap();

        let response = self.send_command(command).await.map_err(|e| {
            crate::error::EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(e),
            )
        })?;

        let result: CallFunctionResult = response.result.try_into().unwrap();
        match result {
            EvaluateResult::EvaluateResultSuccess(success) => {
                if let RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(
                    sv,
                )) = success.result
                {
                    Ok(sv.value)
                } else {
                    Ok(String::new())
                }
            }
            EvaluateResult::EvaluateResultException(_) => Ok(String::new()),
        }
    }

    pub async fn delete(&self) -> Result<(), crate::error::EvaluateResultError> {
        let remote_reference = self
            .shared_reference()
            .ok_or(crate::error::EvaluateResultError::NoSharedId)?;
        let target = self.context_target();

        let command = CallFunctionBuilder::default()
            .function_declaration(
                "function() { if (this && this.parentNode) { this.parentNode.removeChild(this); } }"
                    .to_string(),
            )
            .await_promise(false)
            .target(target)
            .this(remote_reference)
            .build()
            .unwrap();

        self.send_command(command).await.map_err(|e| {
            crate::error::EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(e),
            )
        })?;

        Ok(())
    }

    pub async fn screenshot(
        &self,
        screenshot_command: impl Into<CaptureScreenshot>,
    ) -> Result<String, crate::error::ScreenshotError> {
        let shared_id = self
            ._raw_node
            .shared_id
            .as_ref()
            .ok_or(crate::error::ScreenshotError::NoSharedId)?;

        let mut shared_ref_builder = SharedReferenceBuilder::default().shared_id(shared_id.clone());
        if let Some(handle) = self._raw_node.handle.clone() {
            shared_ref_builder = shared_ref_builder.handle(handle);
        }

        let _clip = Some(
            ElementClipRectangleBuilder::default()
                .r#type(ElementClipRectangleType::Element)
                .element(shared_ref_builder.build().unwrap())
                .build()
                .unwrap(),
        );

        let response = self.send_command(screenshot_command.into()).await;

        match response {
            Ok(response) => {
                let context_result: CaptureScreenshotResult =
                    response.result.clone().try_into().map_err(|_| {
                        ScreenshotError::CommandResultError(
                            CommandResultError::InvalidResultTypeError(response.result),
                        )
                    })?;
                Ok(context_result.data)
            }
            Err(err) => Err(crate::error::ScreenshotError::CommandResultError(
                CommandResultError::SessionSendError(err),
            )),
        }
    }
}
