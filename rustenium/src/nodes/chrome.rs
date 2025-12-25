use std::collections::HashMap;
use rustenium_bidi_commands::browsing_context::types::{ImageFormat, Locator, OriginUnion};
use crate::nodes::bidi::node::BidiNode;
use rustenium_bidi_commands::script::types::{Handle, NodeRemoteValue, SharedId};
use rustenium_core::transport::ConnectionTransport;
use rustenium_core::Session;
use crate::nodes::node::Node;
use crate::nodes::NodePosition;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ChromeNode<T: ConnectionTransport = rustenium_core::transport::WebsocketConnectionTransport> {
    bidi_node: BidiNode<T>,
    children: Vec<ChromeNode<T>>,
}

impl<T: ConnectionTransport> ChromeNode<T> {
    pub fn from_bidi(
        _raw_bidi_node: NodeRemoteValue,
        locator: Locator,
        session: Arc<Mutex<Session<T>>>,
        context: String,
    ) -> Self {
        let bidi_node = BidiNode::new(_raw_bidi_node, locator.clone(), session.clone(), context.clone());

        // Convert BidiNode children to ChromeNode children
        let children = bidi_node.children.iter()
            .map(|bidi_child| {
                ChromeNode::from_bidi(
                    bidi_child.get_raw_node_ref().clone(),
                    locator.clone(),
                    session.clone(),
                    context.clone(),
                )
            })
            .collect();

        Self {
            bidi_node,
            children,
        }
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
        self.bidi_node.screenshot(origin, format, save_path).await
    }
}

impl<T: ConnectionTransport> Node for ChromeNode<T> {
    #[allow(refining_impl_trait)]
    fn get_children_nodes(&self) -> &Vec<ChromeNode<T>> {
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

    fn get_attribute(&self, attribute_name: &str) -> Option<String> {
        self.bidi_node.get_attribute(attribute_name)
    }

    fn get_attributes(&self) -> HashMap<String, String> {
        self.bidi_node.get_attributes()
    }

    async fn get_position(&mut self) -> Option<&NodePosition> {
        // TODO: Add a log / warning if unable to get position
        self.bidi_node.get_position().await.unwrap_or(None)
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

    async fn scroll_into_view(&self) -> Result<(), crate::error::EvaluateResultError> {
        self.bidi_node.scroll_into_view().await
    }

    async fn is_visible(&self) -> Result<bool, crate::error::EvaluateResultError> {
        self.bidi_node.is_visible().await
    }

    async fn delete(&self) -> Result<(), crate::error::EvaluateResultError> {
        self.bidi_node.delete().await
    }
}
