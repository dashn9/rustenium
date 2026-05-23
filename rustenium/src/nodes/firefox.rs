use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use rustenium_bidi_definitions::browsing_context::types::{BrowsingContext, Locator};
use rustenium_bidi_definitions::script::types::{Handle, NodeRemoteValue, SharedId};
use rustenium_core::BidiSession;
use rustenium_core::transport::ConnectionTransport;

use crate::error::node::{NodeActionError, NodeInputError, NodeMouseError, NodeScreenshotError};
use crate::input::{BidiKeyboard, BidiMouse, Keyboard, Mouse, MouseClickOptions, MouseMoveOptions};
use crate::nodes::NodePosition;
use crate::nodes::bidi::node::BidiNode;
use crate::nodes::node::{Node, NodeScreenShotOptions, NodeType};

pub struct FirefoxNode<
    T: ConnectionTransport,
    M: Mouse + Send + Sync = BidiMouse<T>,
    K: Keyboard + Send + Sync = BidiKeyboard<T>,
> {
    bidi_node: BidiNode<T>,
    children: Vec<FirefoxNode<T, M, K>>,
    mouse: Arc<M>,
    keyboard: Arc<K>,
}

impl<T: ConnectionTransport, M: Mouse + Send + Sync, K: Keyboard + Send + Sync> std::fmt::Debug
    for FirefoxNode<T, M, K>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.bidi_node.fmt(f)
    }
}

impl<T: ConnectionTransport, M: Mouse + Send + Sync + 'static, K: Keyboard + Send + Sync + 'static>
    FirefoxNode<T, M, K>
{
    pub fn from_bidi(
        raw_bidi_node: NodeRemoteValue,
        locator: Locator,
        session: Arc<Mutex<BidiSession<T>>>,
        context: BrowsingContext,
        mouse: Arc<M>,
        keyboard: Arc<K>,
    ) -> Self {
        let bidi_node = BidiNode::new(
            raw_bidi_node,
            locator.clone(),
            session.clone(),
            context.clone(),
        );

        let children = bidi_node
            .children
            .iter()
            .map(|bidi_child| {
                FirefoxNode::from_bidi(
                    bidi_child.get_raw_node_ref().clone(),
                    locator.clone(),
                    session.clone(),
                    context.clone(),
                    mouse.clone(),
                    keyboard.clone(),
                )
            })
            .collect();

        Self {
            bidi_node,
            children,
            mouse,
            keyboard,
        }
    }

    pub async fn mouse_move(&mut self) -> Result<(), NodeMouseError> {
        self.bidi_node
            .mouse_move(self.mouse.as_ref(), MouseMoveOptions::default())
            .await
            .map_err(NodeMouseError::from)
    }

    pub async fn mouse_move_with_options(
        &mut self,
        options: MouseMoveOptions,
    ) -> Result<(), NodeMouseError> {
        self.bidi_node
            .mouse_move(self.mouse.as_ref(), options)
            .await
            .map_err(NodeMouseError::from)
    }

    pub async fn mouse_click(&mut self) -> Result<(), NodeMouseError> {
        self.bidi_node
            .mouse_click(self.mouse.as_ref(), MouseClickOptions::default())
            .await
            .map_err(NodeMouseError::from)
    }

    pub async fn mouse_click_with_options(
        &mut self,
        options: MouseClickOptions,
    ) -> Result<(), NodeMouseError> {
        self.bidi_node
            .mouse_click(self.mouse.as_ref(), options)
            .await
            .map_err(NodeMouseError::from)
    }
}

impl<T: ConnectionTransport, M: Mouse + Send + Sync + 'static, K: Keyboard + Send + Sync + 'static>
    Node for FirefoxNode<T, M, K>
{
    #[allow(refining_impl_trait)]
    fn get_children_nodes(&self) -> &Vec<FirefoxNode<T, M, K>> {
        &self.children
    }

    fn get_bidi_locator(&self) -> &Locator {
        &self.bidi_node.locator
    }

    fn get_local_name(&self) -> Option<&str> {
        self.bidi_node.get_local_name()
    }

    fn get_node_type(&self) -> Option<NodeType> {
        self.bidi_node.get_node_type()
    }

    async fn get_inner_text(&self) -> String {
        self.bidi_node.get_inner_text().await.unwrap()
    }

    async fn get_text_content(&self) -> String {
        self.bidi_node.get_text_content().await.unwrap()
    }

    async fn get_html(&self) -> String {
        self.bidi_node.get_inner_html().await.unwrap()
    }

    fn get_attribute(&self, attribute_name: &str) -> Option<serde_json::Value> {
        self.bidi_node.get_attribute(attribute_name)
    }

    fn get_attributes(&self) -> HashMap<String, serde_json::Value> {
        self.bidi_node.get_attributes()
    }

    async fn get_position(&mut self) -> Option<&NodePosition> {
        self.bidi_node.get_position().await.unwrap_or(None)
    }

    fn get_context_id(&self) -> &BrowsingContext {
        &self.bidi_node.context_id
    }

    fn get_shared_id(&self) -> Option<&SharedId> {
        self.bidi_node.get_raw_node_ref().shared_id.as_ref()
    }

    fn get_handle(&self) -> &Option<Handle> {
        &self.bidi_node.get_raw_node_ref().handle
    }

    fn set_position(&mut self, position: NodePosition) {
        self.bidi_node.position = Some(position);
    }

    async fn scroll_into_view(&self) -> Result<(), NodeActionError> {
        self.bidi_node
            .scroll_into_view()
            .await
            .map_err(NodeActionError::from)
    }

    async fn is_visible(&self) -> Result<bool, NodeActionError> {
        self.bidi_node
            .is_visible()
            .await
            .map_err(NodeActionError::from)
    }

    async fn delete(&self) -> Result<(), NodeActionError> {
        self.bidi_node.delete().await.map_err(NodeActionError::from)
    }

    async fn mouse_move(&mut self) -> Result<(), NodeMouseError> {
        self.bidi_node
            .mouse_move(self.mouse.as_ref(), MouseMoveOptions::default())
            .await
            .map_err(NodeMouseError::from)
    }

    async fn mouse_move_with_options(
        &mut self,
        options: MouseMoveOptions,
    ) -> Result<(), NodeMouseError> {
        self.bidi_node
            .mouse_move(self.mouse.as_ref(), options)
            .await
            .map_err(NodeMouseError::from)
    }

    async fn mouse_click(&mut self) -> Result<(), NodeMouseError> {
        self.bidi_node
            .mouse_click(self.mouse.as_ref(), MouseClickOptions::default())
            .await
            .map_err(NodeMouseError::from)
    }

    async fn mouse_click_with_options(
        &mut self,
        options: MouseClickOptions,
    ) -> Result<(), NodeMouseError> {
        self.bidi_node
            .mouse_click(self.mouse.as_ref(), options)
            .await
            .map_err(NodeMouseError::from)
    }

    async fn screenshot(&mut self) -> Result<String, NodeScreenshotError> {
        self.bidi_node
            .screenshot(NodeScreenShotOptions::default())
            .await
            .map_err(NodeScreenshotError::from)
    }

    async fn screenshot_with_options(
        &mut self,
        options: NodeScreenShotOptions,
    ) -> Result<String, NodeScreenshotError> {
        self.bidi_node
            .screenshot(options)
            .await
            .map_err(NodeScreenshotError::from)
    }

    async fn type_text(&mut self, text: String) -> Result<(), NodeInputError> {
        self.bidi_node
            .type_text(self.keyboard.as_ref(), text)
            .await
            .map_err(NodeInputError::from)
    }
}
