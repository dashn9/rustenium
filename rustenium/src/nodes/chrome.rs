use std::collections::HashMap;
use rustenium_bidi_commands::browsing_context::types::Locator;
use crate::nodes::bidi::node::BidiNode;
use rustenium_bidi_commands::script::types::{Handle, NodeRemoteValue, SharedId};
use crate::nodes::node::Node;
use crate::nodes::NodePosition;

pub struct ChromeNode {
    bidi_node: BidiNode,
}

impl ChromeNode {
    pub fn from_bidi(_raw_bidi_node: NodeRemoteValue, locator: Locator) -> Self {
        Self {
            bidi_node: BidiNode::new(_raw_bidi_node, locator),
        }
    }
}

impl Node for ChromeNode {
    fn get_children_nodes(&self) -> &Vec<ChromeNode> {
        todo!()
    }

    fn get_bidi_locator(&self) -> &Locator {
        &self.bidi_node.locator
    }

    fn get_text(&self) -> String {
        todo!()
    }

    fn get_attributes(&self) -> Vec<HashMap<String, String>> {
        todo!()
    }

    fn get_position(&self) -> &Option<NodePosition> {
        &self.bidi_node.position
    }

    fn get_shared_id(&self) -> &Option<SharedId> {
        &self.bidi_node.get_raw_node_ref().shared_id
    }

    fn get_handle(&self) -> &Option<Handle> {
        &self.bidi_node.get_raw_node_ref().handle
    }

    fn set_position(&mut self, position: NodePosition) -> () {
        self.bidi_node.position = Some(position);
    }
}