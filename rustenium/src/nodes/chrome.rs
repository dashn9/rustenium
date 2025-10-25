use rustenium_bidi_commands::script::types::NodeRemoteValue;
use crate::nodes::bidi::node::BidiNode;

pub struct ChromeNode {
    bidi_node: BidiNode
}

impl ChromeNode {
    fn from_bidi(_raw_bidi_node: NodeRemoteValue) -> Self {
        Self {
            bidi_node: BidiNode::new(_raw_bidi_node)
        }
    }
}