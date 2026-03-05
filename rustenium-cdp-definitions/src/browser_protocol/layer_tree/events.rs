use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerPainted {
    #[doc = "The id of the painted layer."]
    #[serde(rename = "layerId")]
    pub layer_id: super::types::LayerId,
    #[doc = "Clip rectangle."]
    #[serde(rename = "clip")]
    pub clip: super::super::dom::types::Rect,
}
impl LayerPainted {
    pub const IDENTIFIER: &'static str = "LayerTree.layerPainted";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LayerTreeDidChange {
    #[doc = "Layer tree, absent if not in the compositing mode."]
    #[serde(rename = "layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub layers: Option<Vec<super::types::Layer>>,
}
impl LayerTreeDidChange {
    pub const IDENTIFIER: &'static str = "LayerTree.layerTreeDidChange";
}
group_enum ! (LayerTreeEvents { LayerPainted (LayerPainted) , LayerTreeDidChange (LayerTreeDidChange) });
