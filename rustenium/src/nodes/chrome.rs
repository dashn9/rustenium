use std::collections::HashMap;
use rustenium_bidi_commands::browsing_context::types::Locator;
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
}

impl<T: ConnectionTransport> ChromeNode<T> {
    pub fn from_bidi(
        _raw_bidi_node: NodeRemoteValue,
        locator: Locator,
        session: Arc<Mutex<Session<T>>>,
        context: String,
    ) -> Self {
        Self {
            bidi_node: BidiNode::new(_raw_bidi_node, locator, session, context),
        }
    }
}

impl<T: ConnectionTransport> Node for ChromeNode<T> {
    fn get_children_nodes(&self) -> &Vec<ChromeNode<T>> {
        todo!()
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
}
