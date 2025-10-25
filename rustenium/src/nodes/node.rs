use std::collections::HashMap;

struct NodePosition {

}
trait Node {
    fn get_all_children_nodes(&self) -> &Vec<impl Node>;

    fn get_text(&self) -> String;

    fn get_attributes(&self) -> Vec<HashMap<String, String>>;

    fn get_position(&self) -> NodePosition;

    fn get_child_node(&self) -> &impl Node;

}