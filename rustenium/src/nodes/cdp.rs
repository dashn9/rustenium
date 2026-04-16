use std::collections::HashMap;
use rustenium_cdp_definitions::browser_protocol::dom::types::Node as DomNode;
use crate::nodes::node::NodeType;

/// A DOM node fetched via CDP `DOM.describeNode`.
/// Preserves all CDP data and reconstructs the child tree.
#[derive(Clone)]
pub struct CdpNode {
    pub node_id: i64,
    pub backend_node_id: i64,
    pub node_type: i64,
    pub node_name: String,
    pub local_name: String,
    pub node_value: String,
    pub child_node_count: Option<i64>,
    pub attributes: HashMap<String, String>,
    pub children: Vec<CdpNode>,
}

impl std::fmt::Debug for CdpNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "node_id: {} | type: {}", self.node_id, self.node_name)?;
        if !self.local_name.is_empty() {
            write!(f, " | local_name: {}", self.local_name)?;
        }
        if !self.node_value.is_empty() {
            write!(f, " | value: {}", self.node_value)?;
        }
        for (k, v) in &self.attributes {
            write!(f, "\n  {}: {}", k, v)?;
        }
        for child in &self.children {
            let s = format!("{:?}", child);
            for line in s.lines() {
                write!(f, "\n  {}", line)?;
            }
        }
        Ok(())
    }
}

impl CdpNode {
    /// Build a `CdpNode` from a CDP `DomNode`.
    pub fn from_dom(node: DomNode) -> Self {
        let attributes = parse_flat_attributes(node.attributes.unwrap_or_default());
        let children = node.children
            .unwrap_or_default()
            .into_iter()
            .map(CdpNode::from_dom)
            .collect();

        CdpNode {
            node_id: *node.node_id.inner(),
            backend_node_id: *node.backend_node_id.inner(),
            node_type: *node.node_type,
            node_name: node.node_name,
            local_name: node.local_name,
            node_value: node.node_value,
            child_node_count: node.child_node_count,
            attributes,
            children,
        }
    }

    pub fn get_local_name(&self) -> &str {
        &self.local_name
    }

    pub fn get_node_type(&self) -> Option<NodeType> {
        NodeType::from_u16(self.node_type as u16)
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(String::as_str)
    }

    pub fn get_attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    pub fn get_node_name(&self) -> &str {
        &self.node_name
    }

    pub fn get_node_value(&self) -> &str {
        &self.node_value
    }

    pub fn get_children(&self) -> &Vec<CdpNode> {
        &self.children
    }

    pub fn is_element(&self) -> bool {
        self.node_type == 1
    }
}

/// CDP attributes come as a flat `[name, value, name, value, ...]` array.
fn parse_flat_attributes(flat: Vec<String>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut iter = flat.into_iter();
    while let (Some(k), Some(v)) = (iter.next(), iter.next()) {
        map.insert(k, v);
    }
    map
}
