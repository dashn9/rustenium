use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerPaintedParams {
    #[doc = "The id of the painted layer."]
    #[serde(rename = "layerId")]
    pub layer_id: super::types::LayerId,
    #[doc = "Clip rectangle."]
    #[serde(rename = "clip")]
    pub clip: crate::browser_protocol::dom::types::Rect,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LayerPaintedMethod {
    #[serde(rename = "LayerTree.layerPainted")]
    LayerPainted,
}
impl LayerPaintedMethod {
    pub const IDENTIFIER: &'static str = "LayerTree.layerPainted";
}
#[derive(Debug, Clone, PartialEq)]
pub struct LayerPainted {
    pub method: LayerPaintedMethod,
    pub params: LayerPaintedParams,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LayerTreeDidChangeParams {
    #[doc = "Layer tree, absent if not in the compositing mode."]
    #[serde(rename = "layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub layers: Option<Vec<super::types::Layer>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LayerTreeDidChangeMethod {
    #[serde(rename = "LayerTree.layerTreeDidChange")]
    LayerTreeDidChange,
}
impl LayerTreeDidChangeMethod {
    pub const IDENTIFIER: &'static str = "LayerTree.layerTreeDidChange";
}
#[derive(Debug, Clone, PartialEq)]
pub struct LayerTreeDidChange {
    pub method: LayerTreeDidChangeMethod,
    pub params: LayerTreeDidChangeParams,
}
group_enum ! (LayerTreeEvents { LayerPainted (LayerPainted) , LayerTreeDidChange (LayerTreeDidChange) });
