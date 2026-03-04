use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSnapshotReturns {
    #[doc = "The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document."]
    #[serde(rename = "domNodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dom_nodes: Vec<super::types::DomNode>,
    #[doc = "The nodes in the layout tree."]
    #[serde(rename = "layoutTreeNodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub layout_tree_nodes: Vec<super::types::LayoutTreeNode>,
    #[doc = "Whitelisted ComputedStyle properties for each node in the layout tree."]
    #[serde(rename = "computedStyles")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub computed_styles: Vec<super::types::ComputedStyle>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureSnapshotReturns {
    #[doc = "The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document."]
    #[serde(rename = "documents")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub documents: Vec<super::types::DocumentSnapshot>,
    #[doc = "Shared string table that all string properties refer to with indexes."]
    #[serde(rename = "strings")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub strings: Vec<String>,
}
