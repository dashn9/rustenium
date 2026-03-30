use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use rustenium_bidi_definitions::browsing_context::types::{BrowsingContext, Locator};
use rustenium_bidi_definitions::script::types::{Handle, NodeRemoteValue, SharedId};
use rustenium_core::transport::ConnectionTransport;
use rustenium_core::BidiSession;

use crate::error::bidi::{EvaluateResultError, MouseInputError, ScreenshotError};
use crate::input::{Mouse, MouseClickOptions, MouseMoveOptions};
use crate::input::BidiMouse;
use crate::nodes::bidi::node::{BidiNode, BidiNodeScreenshotOptions};
use crate::nodes::node::Node;
use crate::nodes::NodePosition;

pub struct ChromeNode<T: ConnectionTransport, M: Mouse + Send + Sync = BidiMouse<T>> {
    bidi_node: BidiNode<T>,
    children: Vec<ChromeNode<T, M>>,
    mouse: Arc<M>,
}

impl<T: ConnectionTransport, M: Mouse + Send + Sync + 'static> ChromeNode<T, M> {
    pub fn from_bidi(
        raw_bidi_node: NodeRemoteValue,
        locator: Locator,
        session: Arc<Mutex<BidiSession<T>>>,
        context: BrowsingContext,
        mouse: Arc<M>,
    ) -> Self {
        let bidi_node = BidiNode::new(raw_bidi_node, locator.clone(), session.clone(), context.clone());

        let children = bidi_node.children.iter()
            .map(|bidi_child| {
                ChromeNode::from_bidi(
                    bidi_child.get_raw_node_ref().clone(),
                    locator.clone(),
                    session.clone(),
                    context.clone(),
                    mouse.clone(),
                )
            })
            .collect();

        Self { bidi_node, children, mouse }
    }


    // ── Mouse move ───────────────────────────────────────────────────────────

    /// Scrolls the element into view and moves the mouse to its center.
    pub async fn mouse_move(&mut self) -> Result<(), MouseInputError> {
        self.bidi_node.mouse_move(self.mouse.as_ref(), MouseMoveOptions::default()).await
    }

    /// Scrolls the element into view and moves the mouse to its center with custom move options.
    pub async fn mouse_move_with_options(&mut self, options: MouseMoveOptions) -> Result<(), MouseInputError> {
        self.bidi_node.mouse_move(self.mouse.as_ref(), options).await
    }

    // ── Mouse click ──────────────────────────────────────────────────────────

    /// Scrolls the element into view, moves the mouse to its center, and clicks.
    pub async fn mouse_click(&mut self) -> Result<(), MouseInputError> {
        self.bidi_node.mouse_click(self.mouse.as_ref(), MouseClickOptions::default()).await
    }

    /// Scrolls the element into view, moves the mouse to its center, and clicks with custom click options.
    ///
    /// # Example
    /// ```ignore
    /// // Double click
    /// node.mouse_click_with_options(MouseClickOptions { count: Some(2), ..Default::default() }).await?;
    /// ```
    pub async fn mouse_click_with_options(&mut self, options: MouseClickOptions) -> Result<(), MouseInputError> {
        self.bidi_node.mouse_click(self.mouse.as_ref(), options).await
    }
}

impl<T: ConnectionTransport, M: Mouse + Send + Sync + 'static> Node for ChromeNode<T, M> {
    #[allow(refining_impl_trait)]
    fn get_children_nodes(&self) -> &Vec<ChromeNode<T, M>> {
        &self.children
    }

    fn get_bidi_locator(&self) -> &Locator {
        &self.bidi_node.locator
    }

    async fn get_inner_text(&self) -> String {
        self.bidi_node.get_inner_text().await.unwrap()
    }

    async fn get_text_content(&self) -> String {
        self.bidi_node.get_text_content().await.unwrap()
    }

    async fn get_inner_html(&self) -> String {
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

    fn set_position(&mut self, position: NodePosition) -> () {
        self.bidi_node.position = Some(position);
    }

    async fn scroll_into_view(&self) -> Result<(), EvaluateResultError> {
        self.bidi_node.scroll_into_view().await
    }

    async fn is_visible(&self) -> Result<bool, EvaluateResultError> {
        self.bidi_node.is_visible().await
    }

    async fn delete(&self) -> Result<(), EvaluateResultError> {
        self.bidi_node.delete().await
    }
    
    /// Scrolls the element into view and moves the mouse to its center.
    async fn mouse_move(&mut self) -> Result<(), MouseInputError> {
        self.bidi_node.mouse_move(self.mouse.as_ref(), MouseMoveOptions::default()).await
    }

    /// Scrolls the element into view and moves the mouse to its center with custom move options.
    async fn mouse_move_with_options(&mut self, options: MouseMoveOptions) -> Result<(), MouseInputError> {
        self.bidi_node.mouse_move(self.mouse.as_ref(), options).await
    }

    // ── Mouse click ──────────────────────────────────────────────────────────

    /// Scrolls the element into view, moves the mouse to its center, and clicks.
    async fn mouse_click(&mut self) -> Result<(), MouseInputError> {
        self.bidi_node.mouse_click(self.mouse.as_ref(), MouseClickOptions::default()).await
    }

    /// Scrolls the element into view, moves the mouse to its center, and clicks with custom click options.
    ///
    /// # Example
    /// ```ignore
    /// // Double click
    /// node.mouse_click_with_options(MouseClickOptions { count: Some(2), ..Default::default() }).await?;
    /// ```
    async fn mouse_click_with_options(&mut self, options: MouseClickOptions) -> Result<(), MouseInputError> {
        self.bidi_node.mouse_click(self.mouse.as_ref(), options).await
    }
    // ── Screenshot ───────────────────────────────────────────────────────────

    /// Captures a screenshot of the element and returns base64-encoded image data.
    async fn screenshot(&self) -> Result<String, ScreenshotError> {
        self.bidi_node.screenshot(BidiNodeScreenshotOptions::default()).await
    }

    /// Captures a screenshot of the element with custom options (format, origin, save path).
    ///
    /// If `save_path` is a directory, saves with an auto-generated filename.
    /// If `save_path` is a file path, saves to that exact location and returns the path.
    /// If `save_path` is `None`, returns base64-encoded image data.
    async fn screenshot_with_options(&self, options: BidiNodeScreenshotOptions) -> Result<String, ScreenshotError> {
        self.bidi_node.screenshot(options).await
    }
}
