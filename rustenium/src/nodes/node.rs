use std::collections::HashMap;
use serde::Deserialize;
use rustenium_bidi_commands::browsing_context::types::Locator;
use rustenium_bidi_commands::script::types::{Handle, SharedId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum NodeType {
    Element = 1,
    Attribute = 2,   
    Text = 3,
    CDataSection = 4,
    ProcessingInstruction = 7,
    Comment = 8,
    Document = 9,
    DocumentType = 10,
    DocumentFragment = 11,
}

impl NodeType {
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            1 => Some(NodeType::Element),
            2 => Some(NodeType::Attribute),
            3 => Some(NodeType::Text),
            4 => Some(NodeType::CDataSection),
            7 => Some(NodeType::ProcessingInstruction),
            8 => Some(NodeType::Comment),
            9 => Some(NodeType::Document),
            10 => Some(NodeType::DocumentType),
            11 => Some(NodeType::DocumentFragment),
            _ => None,
        }
    }

    pub fn as_u16(&self) -> u16 {
        *self as u16
    }

    pub fn is_element(&self) -> bool {
        matches!(self, NodeType::Element)
    }

    pub fn is_text(&self) -> bool {
        matches!(self, NodeType::Text)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct NodePosition {
    pub x: f64,
    pub y: f64,
    pub scroll_x: f64,
    pub scroll_y: f64,
    pub width: f64,
    pub height: f64,
}

pub trait Node {
    fn get_children_nodes(&self) -> &Vec<impl Node>;

    fn get_bidi_locator(&self) -> &Locator;

    async fn get_inner_text(&self) -> String;

    async fn get_text_content(&self) -> String;

    async fn get_inner_html(&self) -> String;

    fn get_attribute(&self, attribute_name: &str) -> Option<String>;

    fn get_attributes(&self) -> HashMap<String, String>;

    async fn get_position(&mut self) -> Option<&NodePosition>;

    fn get_shared_id(&self) -> Option<&SharedId>;

    fn get_handle(&self) -> &Option<Handle>;

    fn set_position(&mut self, position: NodePosition) -> ();

    async fn scroll_into_view(&self) -> Result<(), crate::error::EvaluateResultError>;

    async fn is_visible(&self) -> Result<bool, crate::error::EvaluateResultError>;

    async fn delete(&self) -> Result<(), crate::error::EvaluateResultError>;

}