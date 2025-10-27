use std::collections::HashMap;
use serde::Deserialize;

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

trait Node {
    fn get_all_children_nodes(&self) -> &Vec<impl Node>;

    fn get_text(&self) -> String;

    fn get_attributes(&self) -> Vec<HashMap<String, String>>;

    fn get_position(&self) -> NodePosition;

    fn get_child_node(&self) -> &impl Node;

}