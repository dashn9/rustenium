use std::collections::HashMap;
use serde::Deserialize;
use rustenium_bidi_commands::browsing_context::types::Locator;
use rustenium_bidi_commands::script::types::{Handle, SharedId};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct NodePosition {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

pub trait Node {
    fn get_children_nodes(&self) -> &Vec<impl Node>;

    fn get_bidi_locator(&self) -> &Locator;

    fn get_text(&self) -> String;

    fn get_attributes(&self) -> Vec<HashMap<String, String>>;

    fn get_position(&self) -> &Option<NodePosition>;

    fn get_shared_id(&self) -> &Option<SharedId>;

    fn get_handle(&self) -> &Option<Handle>;

    fn set_position(&mut self, position: NodePosition) -> ();

}