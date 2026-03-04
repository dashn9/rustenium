use super::commands::*;
impl GetHighlightObjectForTest {
    pub fn builder() -> GetHighlightObjectForTestBuilder {
        GetHighlightObjectForTestBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetHighlightObjectForTestBuilder {
    node_id: Option<super::super::dom::types::NodeId>,
    include_distance: Option<bool>,
    include_style: Option<bool>,
    color_format: Option<super::types::ColorFormat>,
    show_accessibility_info: Option<bool>,
}
impl GetHighlightObjectForTestBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn include_distance(mut self, include_distance: impl Into<bool>) -> Self {
        self.include_distance = Some(include_distance.into());
        self
    }
    pub fn include_style(mut self, include_style: impl Into<bool>) -> Self {
        self.include_style = Some(include_style.into());
        self
    }
    pub fn color_format(mut self, color_format: impl Into<super::types::ColorFormat>) -> Self {
        self.color_format = Some(color_format.into());
        self
    }
    pub fn show_accessibility_info(mut self, show_accessibility_info: impl Into<bool>) -> Self {
        self.show_accessibility_info = Some(show_accessibility_info.into());
        self
    }
    pub fn build(self) -> Result<GetHighlightObjectForTest, String> {
        Ok(GetHighlightObjectForTest {
            method: GetHighlightObjectForTestMethod::GetHighlightObjectForTest,
            params: GetHighlightObjectForTestParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                include_distance: self.include_distance,
                include_style: self.include_style,
                color_format: self.color_format,
                show_accessibility_info: self.show_accessibility_info,
            },
        })
    }
}
impl GetGridHighlightObjectsForTest {
    pub fn builder() -> GetGridHighlightObjectsForTestBuilder {
        GetGridHighlightObjectsForTestBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetGridHighlightObjectsForTestBuilder {
    node_ids: Option<Vec<super::super::dom::types::NodeId>>,
}
impl GetGridHighlightObjectsForTestBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        let v = self.node_ids.get_or_insert(Vec::new());
        v.push(node_id.into());
        self
    }
    pub fn node_ids<I, S>(mut self, node_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::super::dom::types::NodeId>,
    {
        let v = self.node_ids.get_or_insert(Vec::new());
        for val in node_ids {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<GetGridHighlightObjectsForTest, String> {
        Ok(GetGridHighlightObjectsForTest {
            method: GetGridHighlightObjectsForTestMethod::GetGridHighlightObjectsForTest,
            params: GetGridHighlightObjectsForTestParams {
                node_ids: self.node_ids.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(node_ids))
                })?,
            },
        })
    }
}
impl GetSourceOrderHighlightObjectForTest {
    pub fn builder() -> GetSourceOrderHighlightObjectForTestBuilder {
        GetSourceOrderHighlightObjectForTestBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetSourceOrderHighlightObjectForTestBuilder {
    node_id: Option<super::super::dom::types::NodeId>,
}
impl GetSourceOrderHighlightObjectForTestBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<GetSourceOrderHighlightObjectForTest, String> {
        Ok(GetSourceOrderHighlightObjectForTest {
            method:
                GetSourceOrderHighlightObjectForTestMethod::GetSourceOrderHighlightObjectForTest,
            params: GetSourceOrderHighlightObjectForTestParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            },
        })
    }
}
impl HighlightNode {
    pub fn builder() -> HighlightNodeBuilder {
        HighlightNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct HighlightNodeBuilder {
    highlight_config: Option<super::types::HighlightConfig>,
    node_id: Option<super::super::dom::types::NodeId>,
    backend_node_id: Option<super::super::dom::types::BackendNodeId>,
    object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
    selector: Option<String>,
}
impl HighlightNodeBuilder {
    pub fn highlight_config(
        mut self,
        highlight_config: impl Into<super::types::HighlightConfig>,
    ) -> Self {
        self.highlight_config = Some(highlight_config.into());
        self
    }
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<super::super::dom::types::BackendNodeId>,
    ) -> Self {
        self.backend_node_id = Some(backend_node_id.into());
        self
    }
    pub fn object_id(
        mut self,
        object_id: impl Into<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
    ) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = Some(selector.into());
        self
    }
    pub fn build(self) -> Result<HighlightNode, String> {
        Ok(HighlightNode {
            method: HighlightNodeMethod::HighlightNode,
            params: HighlightNodeParams {
                highlight_config: self.highlight_config.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(highlight_config)
                    )
                })?,
                node_id: self.node_id,
                backend_node_id: self.backend_node_id,
                object_id: self.object_id,
                selector: self.selector,
            },
        })
    }
}
impl HighlightQuad {
    pub fn builder() -> HighlightQuadBuilder {
        HighlightQuadBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct HighlightQuadBuilder {
    quad: Option<super::super::dom::types::Quad>,
    color: Option<super::super::dom::types::Rgba>,
    outline_color: Option<super::super::dom::types::Rgba>,
}
impl HighlightQuadBuilder {
    pub fn quad(mut self, quad: impl Into<super::super::dom::types::Quad>) -> Self {
        self.quad = Some(quad.into());
        self
    }
    pub fn color(mut self, color: impl Into<super::super::dom::types::Rgba>) -> Self {
        self.color = Some(color.into());
        self
    }
    pub fn outline_color(
        mut self,
        outline_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.outline_color = Some(outline_color.into());
        self
    }
    pub fn build(self) -> Result<HighlightQuad, String> {
        Ok(HighlightQuad {
            method: HighlightQuadMethod::HighlightQuad,
            params: HighlightQuadParams {
                quad: self
                    .quad
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(quad)))?,
                color: self.color,
                outline_color: self.outline_color,
            },
        })
    }
}
impl HighlightRect {
    pub fn builder() -> HighlightRectBuilder {
        HighlightRectBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct HighlightRectBuilder {
    x: Option<i64>,
    y: Option<i64>,
    width: Option<i64>,
    height: Option<i64>,
    color: Option<super::super::dom::types::Rgba>,
    outline_color: Option<super::super::dom::types::Rgba>,
}
impl HighlightRectBuilder {
    pub fn x(mut self, x: impl Into<i64>) -> Self {
        self.x = Some(x.into());
        self
    }
    pub fn y(mut self, y: impl Into<i64>) -> Self {
        self.y = Some(y.into());
        self
    }
    pub fn width(mut self, width: impl Into<i64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn height(mut self, height: impl Into<i64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn color(mut self, color: impl Into<super::super::dom::types::Rgba>) -> Self {
        self.color = Some(color.into());
        self
    }
    pub fn outline_color(
        mut self,
        outline_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.outline_color = Some(outline_color.into());
        self
    }
    pub fn build(self) -> Result<HighlightRect, String> {
        Ok(HighlightRect {
            method: HighlightRectMethod::HighlightRect,
            params: HighlightRectParams {
                x: self
                    .x
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
                y: self
                    .y
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
                width: self
                    .width
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(width)))?,
                height: self
                    .height
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(height)))?,
                color: self.color,
                outline_color: self.outline_color,
            },
        })
    }
}
impl HighlightSourceOrder {
    pub fn builder() -> HighlightSourceOrderBuilder {
        HighlightSourceOrderBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct HighlightSourceOrderBuilder {
    source_order_config: Option<super::types::SourceOrderConfig>,
    node_id: Option<super::super::dom::types::NodeId>,
    backend_node_id: Option<super::super::dom::types::BackendNodeId>,
    object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
}
impl HighlightSourceOrderBuilder {
    pub fn source_order_config(
        mut self,
        source_order_config: impl Into<super::types::SourceOrderConfig>,
    ) -> Self {
        self.source_order_config = Some(source_order_config.into());
        self
    }
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<super::super::dom::types::BackendNodeId>,
    ) -> Self {
        self.backend_node_id = Some(backend_node_id.into());
        self
    }
    pub fn object_id(
        mut self,
        object_id: impl Into<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
    ) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn build(self) -> Result<HighlightSourceOrder, String> {
        Ok(HighlightSourceOrder {
            method: HighlightSourceOrderMethod::HighlightSourceOrder,
            params: HighlightSourceOrderParams {
                source_order_config: self.source_order_config.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(source_order_config)
                    )
                })?,
                node_id: self.node_id,
                backend_node_id: self.backend_node_id,
                object_id: self.object_id,
            },
        })
    }
}
impl SetInspectMode {
    pub fn builder() -> SetInspectModeBuilder {
        SetInspectModeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetInspectModeBuilder {
    mode: Option<super::types::InspectMode>,
    highlight_config: Option<super::types::HighlightConfig>,
}
impl SetInspectModeBuilder {
    pub fn mode(mut self, mode: impl Into<super::types::InspectMode>) -> Self {
        self.mode = Some(mode.into());
        self
    }
    pub fn highlight_config(
        mut self,
        highlight_config: impl Into<super::types::HighlightConfig>,
    ) -> Self {
        self.highlight_config = Some(highlight_config.into());
        self
    }
    pub fn build(self) -> Result<SetInspectMode, String> {
        Ok(SetInspectMode {
            method: SetInspectModeMethod::SetInspectMode,
            params: SetInspectModeParams {
                mode: self
                    .mode
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(mode)))?,
                highlight_config: self.highlight_config,
            },
        })
    }
}
impl SetShowAdHighlights {
    pub fn builder() -> SetShowAdHighlightsBuilder {
        SetShowAdHighlightsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetShowAdHighlightsBuilder {
    show: Option<bool>,
}
impl SetShowAdHighlightsBuilder {
    pub fn show(mut self, show: impl Into<bool>) -> Self {
        self.show = Some(show.into());
        self
    }
    pub fn build(self) -> Result<SetShowAdHighlights, String> {
        Ok(SetShowAdHighlights {
            method: SetShowAdHighlightsMethod::SetShowAdHighlights,
            params: SetShowAdHighlightsParams {
                show: self
                    .show
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(show)))?,
            },
        })
    }
}
impl SetPausedInDebuggerMessage {
    pub fn builder() -> SetPausedInDebuggerMessageBuilder {
        SetPausedInDebuggerMessageBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetPausedInDebuggerMessageBuilder {
    message: Option<String>,
}
impl SetPausedInDebuggerMessageBuilder {
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
    pub fn build(self) -> SetPausedInDebuggerMessage {
        SetPausedInDebuggerMessage {
            method: SetPausedInDebuggerMessageMethod::SetPausedInDebuggerMessage,
            params: SetPausedInDebuggerMessageParams {
                message: self.message,
            },
        }
    }
}
impl SetShowDebugBorders {
    pub fn builder() -> SetShowDebugBordersBuilder {
        SetShowDebugBordersBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetShowDebugBordersBuilder {
    show: Option<bool>,
}
impl SetShowDebugBordersBuilder {
    pub fn show(mut self, show: impl Into<bool>) -> Self {
        self.show = Some(show.into());
        self
    }
    pub fn build(self) -> Result<SetShowDebugBorders, String> {
        Ok(SetShowDebugBorders {
            method: SetShowDebugBordersMethod::SetShowDebugBorders,
            params: SetShowDebugBordersParams {
                show: self
                    .show
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(show)))?,
            },
        })
    }
}
impl SetShowFpsCounter {
    pub fn builder() -> SetShowFpsCounterBuilder {
        SetShowFpsCounterBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetShowFpsCounterBuilder {
    show: Option<bool>,
}
impl SetShowFpsCounterBuilder {
    pub fn show(mut self, show: impl Into<bool>) -> Self {
        self.show = Some(show.into());
        self
    }
    pub fn build(self) -> Result<SetShowFpsCounter, String> {
        Ok(SetShowFpsCounter {
            method: SetShowFpsCounterMethod::SetShowFpsCounter,
            params: SetShowFpsCounterParams {
                show: self
                    .show
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(show)))?,
            },
        })
    }
}
impl SetShowGridOverlays {
    pub fn builder() -> SetShowGridOverlaysBuilder {
        SetShowGridOverlaysBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetShowGridOverlaysBuilder {
    grid_node_highlight_configs: Option<Vec<super::types::GridNodeHighlightConfig>>,
}
impl SetShowGridOverlaysBuilder {
    pub fn grid_node_highlight_config(
        mut self,
        grid_node_highlight_config: impl Into<super::types::GridNodeHighlightConfig>,
    ) -> Self {
        let v = self.grid_node_highlight_configs.get_or_insert(Vec::new());
        v.push(grid_node_highlight_config.into());
        self
    }
    pub fn grid_node_highlight_configs<I, S>(mut self, grid_node_highlight_configs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::GridNodeHighlightConfig>,
    {
        let v = self.grid_node_highlight_configs.get_or_insert(Vec::new());
        for val in grid_node_highlight_configs {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetShowGridOverlays, String> {
        Ok(SetShowGridOverlays {
            method: SetShowGridOverlaysMethod::SetShowGridOverlays,
            params: SetShowGridOverlaysParams {
                grid_node_highlight_configs: self.grid_node_highlight_configs.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(grid_node_highlight_configs)
                    )
                })?,
            },
        })
    }
}
impl SetShowFlexOverlays {
    pub fn builder() -> SetShowFlexOverlaysBuilder {
        SetShowFlexOverlaysBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetShowFlexOverlaysBuilder {
    flex_node_highlight_configs: Option<Vec<super::types::FlexNodeHighlightConfig>>,
}
impl SetShowFlexOverlaysBuilder {
    pub fn flex_node_highlight_config(
        mut self,
        flex_node_highlight_config: impl Into<super::types::FlexNodeHighlightConfig>,
    ) -> Self {
        let v = self.flex_node_highlight_configs.get_or_insert(Vec::new());
        v.push(flex_node_highlight_config.into());
        self
    }
    pub fn flex_node_highlight_configs<I, S>(mut self, flex_node_highlight_configs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::FlexNodeHighlightConfig>,
    {
        let v = self.flex_node_highlight_configs.get_or_insert(Vec::new());
        for val in flex_node_highlight_configs {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetShowFlexOverlays, String> {
        Ok(SetShowFlexOverlays {
            method: SetShowFlexOverlaysMethod::SetShowFlexOverlays,
            params: SetShowFlexOverlaysParams {
                flex_node_highlight_configs: self.flex_node_highlight_configs.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(flex_node_highlight_configs)
                    )
                })?,
            },
        })
    }
}
impl SetShowScrollSnapOverlays {
    pub fn builder() -> SetShowScrollSnapOverlaysBuilder {
        SetShowScrollSnapOverlaysBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetShowScrollSnapOverlaysBuilder {
    scroll_snap_highlight_configs: Option<Vec<super::types::ScrollSnapHighlightConfig>>,
}
impl SetShowScrollSnapOverlaysBuilder {
    pub fn scroll_snap_highlight_config(
        mut self,
        scroll_snap_highlight_config: impl Into<super::types::ScrollSnapHighlightConfig>,
    ) -> Self {
        let v = self.scroll_snap_highlight_configs.get_or_insert(Vec::new());
        v.push(scroll_snap_highlight_config.into());
        self
    }
    pub fn scroll_snap_highlight_configs<I, S>(mut self, scroll_snap_highlight_configs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::ScrollSnapHighlightConfig>,
    {
        let v = self.scroll_snap_highlight_configs.get_or_insert(Vec::new());
        for val in scroll_snap_highlight_configs {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetShowScrollSnapOverlays, String> {
        Ok(SetShowScrollSnapOverlays {
            method: SetShowScrollSnapOverlaysMethod::SetShowScrollSnapOverlays,
            params: SetShowScrollSnapOverlaysParams {
                scroll_snap_highlight_configs: self.scroll_snap_highlight_configs.ok_or_else(
                    || {
                        format!(
                            "Field `{}` is mandatory.",
                            std::stringify!(scroll_snap_highlight_configs)
                        )
                    },
                )?,
            },
        })
    }
}
impl SetShowContainerQueryOverlays {
    pub fn builder() -> SetShowContainerQueryOverlaysBuilder {
        SetShowContainerQueryOverlaysBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetShowContainerQueryOverlaysBuilder {
    container_query_highlight_configs: Option<Vec<super::types::ContainerQueryHighlightConfig>>,
}
impl SetShowContainerQueryOverlaysBuilder {
    pub fn container_query_highlight_config(
        mut self,
        container_query_highlight_config: impl Into<super::types::ContainerQueryHighlightConfig>,
    ) -> Self {
        let v = self
            .container_query_highlight_configs
            .get_or_insert(Vec::new());
        v.push(container_query_highlight_config.into());
        self
    }
    pub fn container_query_highlight_configs<I, S>(
        mut self,
        container_query_highlight_configs: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::ContainerQueryHighlightConfig>,
    {
        let v = self
            .container_query_highlight_configs
            .get_or_insert(Vec::new());
        for val in container_query_highlight_configs {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetShowContainerQueryOverlays, String> {
        Ok(SetShowContainerQueryOverlays {
            method: SetShowContainerQueryOverlaysMethod::SetShowContainerQueryOverlays,
            params: SetShowContainerQueryOverlaysParams {
                container_query_highlight_configs: self
                    .container_query_highlight_configs
                    .ok_or_else(|| {
                        format!(
                            "Field `{}` is mandatory.",
                            std::stringify!(container_query_highlight_configs)
                        )
                    })?,
            },
        })
    }
}
impl SetShowInspectedElementAnchor {
    pub fn builder() -> SetShowInspectedElementAnchorBuilder {
        SetShowInspectedElementAnchorBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetShowInspectedElementAnchorBuilder {
    inspected_element_anchor_config: Option<super::types::InspectedElementAnchorConfig>,
}
impl SetShowInspectedElementAnchorBuilder {
    pub fn inspected_element_anchor_config(
        mut self,
        inspected_element_anchor_config: impl Into<super::types::InspectedElementAnchorConfig>,
    ) -> Self {
        self.inspected_element_anchor_config = Some(inspected_element_anchor_config.into());
        self
    }
    pub fn build(self) -> Result<SetShowInspectedElementAnchor, String> {
        Ok(SetShowInspectedElementAnchor {
            method: SetShowInspectedElementAnchorMethod::SetShowInspectedElementAnchor,
            params: SetShowInspectedElementAnchorParams {
                inspected_element_anchor_config: self.inspected_element_anchor_config.ok_or_else(
                    || {
                        format!(
                            "Field `{}` is mandatory.",
                            std::stringify!(inspected_element_anchor_config)
                        )
                    },
                )?,
            },
        })
    }
}
impl SetShowPaintRects {
    pub fn builder() -> SetShowPaintRectsBuilder {
        SetShowPaintRectsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetShowPaintRectsBuilder {
    result: Option<bool>,
}
impl SetShowPaintRectsBuilder {
    pub fn result(mut self, result: impl Into<bool>) -> Self {
        self.result = Some(result.into());
        self
    }
    pub fn build(self) -> Result<SetShowPaintRects, String> {
        Ok(SetShowPaintRects {
            method: SetShowPaintRectsMethod::SetShowPaintRects,
            params: SetShowPaintRectsParams {
                result: self
                    .result
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(result)))?,
            },
        })
    }
}
impl SetShowLayoutShiftRegions {
    pub fn builder() -> SetShowLayoutShiftRegionsBuilder {
        SetShowLayoutShiftRegionsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetShowLayoutShiftRegionsBuilder {
    result: Option<bool>,
}
impl SetShowLayoutShiftRegionsBuilder {
    pub fn result(mut self, result: impl Into<bool>) -> Self {
        self.result = Some(result.into());
        self
    }
    pub fn build(self) -> Result<SetShowLayoutShiftRegions, String> {
        Ok(SetShowLayoutShiftRegions {
            method: SetShowLayoutShiftRegionsMethod::SetShowLayoutShiftRegions,
            params: SetShowLayoutShiftRegionsParams {
                result: self
                    .result
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(result)))?,
            },
        })
    }
}
impl SetShowScrollBottleneckRects {
    pub fn builder() -> SetShowScrollBottleneckRectsBuilder {
        SetShowScrollBottleneckRectsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetShowScrollBottleneckRectsBuilder {
    show: Option<bool>,
}
impl SetShowScrollBottleneckRectsBuilder {
    pub fn show(mut self, show: impl Into<bool>) -> Self {
        self.show = Some(show.into());
        self
    }
    pub fn build(self) -> Result<SetShowScrollBottleneckRects, String> {
        Ok(SetShowScrollBottleneckRects {
            method: SetShowScrollBottleneckRectsMethod::SetShowScrollBottleneckRects,
            params: SetShowScrollBottleneckRectsParams {
                show: self
                    .show
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(show)))?,
            },
        })
    }
}
impl SetShowViewportSizeOnResize {
    pub fn builder() -> SetShowViewportSizeOnResizeBuilder {
        SetShowViewportSizeOnResizeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetShowViewportSizeOnResizeBuilder {
    show: Option<bool>,
}
impl SetShowViewportSizeOnResizeBuilder {
    pub fn show(mut self, show: impl Into<bool>) -> Self {
        self.show = Some(show.into());
        self
    }
    pub fn build(self) -> Result<SetShowViewportSizeOnResize, String> {
        Ok(SetShowViewportSizeOnResize {
            method: SetShowViewportSizeOnResizeMethod::SetShowViewportSizeOnResize,
            params: SetShowViewportSizeOnResizeParams {
                show: self
                    .show
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(show)))?,
            },
        })
    }
}
impl SetShowHinge {
    pub fn builder() -> SetShowHingeBuilder {
        SetShowHingeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetShowHingeBuilder {
    hinge_config: Option<super::types::HingeConfig>,
}
impl SetShowHingeBuilder {
    pub fn hinge_config(mut self, hinge_config: impl Into<super::types::HingeConfig>) -> Self {
        self.hinge_config = Some(hinge_config.into());
        self
    }
    pub fn build(self) -> SetShowHinge {
        SetShowHinge {
            method: SetShowHingeMethod::SetShowHinge,
            params: SetShowHingeParams {
                hinge_config: self.hinge_config,
            },
        }
    }
}
impl SetShowIsolatedElements {
    pub fn builder() -> SetShowIsolatedElementsBuilder {
        SetShowIsolatedElementsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetShowIsolatedElementsBuilder {
    isolated_element_highlight_configs: Option<Vec<super::types::IsolatedElementHighlightConfig>>,
}
impl SetShowIsolatedElementsBuilder {
    pub fn isolated_element_highlight_config(
        mut self,
        isolated_element_highlight_config: impl Into<super::types::IsolatedElementHighlightConfig>,
    ) -> Self {
        let v = self
            .isolated_element_highlight_configs
            .get_or_insert(Vec::new());
        v.push(isolated_element_highlight_config.into());
        self
    }
    pub fn isolated_element_highlight_configs<I, S>(
        mut self,
        isolated_element_highlight_configs: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::IsolatedElementHighlightConfig>,
    {
        let v = self
            .isolated_element_highlight_configs
            .get_or_insert(Vec::new());
        for val in isolated_element_highlight_configs {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetShowIsolatedElements, String> {
        Ok(SetShowIsolatedElements {
            method: SetShowIsolatedElementsMethod::SetShowIsolatedElements,
            params: SetShowIsolatedElementsParams {
                isolated_element_highlight_configs: self
                    .isolated_element_highlight_configs
                    .ok_or_else(|| {
                        format!(
                            "Field `{}` is mandatory.",
                            std::stringify!(isolated_element_highlight_configs)
                        )
                    })?,
            },
        })
    }
}
impl SetShowWindowControlsOverlay {
    pub fn builder() -> SetShowWindowControlsOverlayBuilder {
        SetShowWindowControlsOverlayBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetShowWindowControlsOverlayBuilder {
    window_controls_overlay_config: Option<super::types::WindowControlsOverlayConfig>,
}
impl SetShowWindowControlsOverlayBuilder {
    pub fn window_controls_overlay_config(
        mut self,
        window_controls_overlay_config: impl Into<super::types::WindowControlsOverlayConfig>,
    ) -> Self {
        self.window_controls_overlay_config = Some(window_controls_overlay_config.into());
        self
    }
    pub fn build(self) -> SetShowWindowControlsOverlay {
        SetShowWindowControlsOverlay {
            method: SetShowWindowControlsOverlayMethod::SetShowWindowControlsOverlay,
            params: SetShowWindowControlsOverlayParams {
                window_controls_overlay_config: self.window_controls_overlay_config,
            },
        }
    }
}
