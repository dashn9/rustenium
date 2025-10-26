use rustenium_bidi_commands::script::types::NodeRemoteValue;
use crate::nodes::chrome::ChromeNode;

pub struct BidiNode {
    _raw_node: NodeRemoteValue,
    children: Vec<ChromeNode>
}

impl BidiNode {
    pub fn new (_raw_node: NodeRemoteValue) -> Self {
        let mut children = Vec::new();
        if let Some(node_properties) = &_raw_node.value {
            if let Some(node_properties_children) = node_properties.children.clone() {
                children.extend(Self::process_node_value_to_children(
                    node_properties_children,
                ));
            }
        }
        Self {
            _raw_node,
            children,
        }
    }


    fn process_node_value_to_children(children: Vec<NodeRemoteValue>) -> Vec<ChromeNode> {
        let mut chrome_node_children = Vec::new();
        for child in children {
            let chrome_node = ChromeNode::from_bidi(child);
            chrome_node_children.push(chrome_node);
        }
        chrome_node_children
    }
}