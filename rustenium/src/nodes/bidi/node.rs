use rustenium_bidi_commands::browsing_context::types::Locator;
use rustenium_bidi_commands::script::types::{Handle, NodeRemoteValue, SharedId};
use crate::nodes::NodePosition;

pub struct BidiNode {
    _raw_node: NodeRemoteValue,
    children: Vec<BidiNode>,
    pub locator: Locator,
    pub position: Option<NodePosition>,
}

impl BidiNode {
    pub fn new (_raw_node: NodeRemoteValue, locator: Locator) -> Self {
        let mut children = Vec::new();
        if let Some(node_properties) = &_raw_node.value {
            if let Some(node_properties_children) = node_properties.children.clone() {
                children.extend(Self::process_node_value_to_children(
                    node_properties_children,
                    &locator
                ));
            }
        }
        Self {
            _raw_node,
            children,
            locator,
            position: None,
        }
    }

    fn process_node_value_to_children(children: Vec<NodeRemoteValue>, locator: &Locator) -> Vec<BidiNode> {
        let mut chrome_node_children = Vec::new();
        for child in children {
            let chrome_node = BidiNode::new(child, locator.clone());
            chrome_node_children.push(chrome_node);
        }
        chrome_node_children
    }
    
    pub fn get_raw_node_ref(&self) -> &NodeRemoteValue{
        &self._raw_node
    }
}