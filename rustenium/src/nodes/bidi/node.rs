use rustenium_bidi_commands::script::types::NodeRemoteValue;

pub struct BidiNode {
    _raw_node: NodeRemoteValue
}

impl BidiNode {
    pub fn new (_raw_node: NodeRemoteValue) -> Self {
        Self { _raw_node }
    }
}