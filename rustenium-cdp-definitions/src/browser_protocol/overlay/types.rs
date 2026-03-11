use serde::{Deserialize, Serialize};
#[doc = "Configuration data for drawing the source order of an elements children.\n[SourceOrderConfig](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#type-SourceOrderConfig)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceOrderConfig {
    #[doc = "the color to outline the given element in."]
    #[serde(rename = "parentOutlineColor")]
    pub parent_outline_color: crate::browser_protocol::dom::types::Rgba,
    #[doc = "the color to outline the child elements in."]
    #[serde(rename = "childOutlineColor")]
    pub child_outline_color: crate::browser_protocol::dom::types::Rgba,
}
impl SourceOrderConfig {
    pub fn new(
        parent_outline_color: impl Into<crate::browser_protocol::dom::types::Rgba>,
        child_outline_color: impl Into<crate::browser_protocol::dom::types::Rgba>,
    ) -> Self {
        Self {
            parent_outline_color: parent_outline_color.into(),
            child_outline_color: child_outline_color.into(),
        }
    }
}
impl SourceOrderConfig {
    pub const IDENTIFIER: &'static str = "Overlay.SourceOrderConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Configuration data for the highlighting of Grid elements.\n[GridHighlightConfig](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#type-GridHighlightConfig)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GridHighlightConfig {
    #[doc = "Whether the extension lines from grid cells to the rulers should be shown (default: false)."]
    #[serde(rename = "showGridExtensionLines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub show_grid_extension_lines: Option<bool>,
    #[doc = "Show Positive line number labels (default: false)."]
    #[serde(rename = "showPositiveLineNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub show_positive_line_numbers: Option<bool>,
    #[doc = "Show Negative line number labels (default: false)."]
    #[serde(rename = "showNegativeLineNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub show_negative_line_numbers: Option<bool>,
    #[doc = "Show area name labels (default: false)."]
    #[serde(rename = "showAreaNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub show_area_names: Option<bool>,
    #[doc = "Show line name labels (default: false)."]
    #[serde(rename = "showLineNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub show_line_names: Option<bool>,
    #[doc = "Show track size labels (default: false)."]
    #[serde(rename = "showTrackSizes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub show_track_sizes: Option<bool>,
    #[doc = "The grid container border highlight color (default: transparent)."]
    #[serde(rename = "gridBorderColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub grid_border_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The row line color (default: transparent)."]
    #[serde(rename = "rowLineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub row_line_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The column line color (default: transparent)."]
    #[serde(rename = "columnLineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub column_line_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "Whether the grid border is dashed (default: false)."]
    #[serde(rename = "gridBorderDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub grid_border_dash: Option<bool>,
    #[doc = "Whether row lines are dashed (default: false)."]
    #[serde(rename = "rowLineDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub row_line_dash: Option<bool>,
    #[doc = "Whether column lines are dashed (default: false)."]
    #[serde(rename = "columnLineDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub column_line_dash: Option<bool>,
    #[doc = "The row gap highlight fill color (default: transparent)."]
    #[serde(rename = "rowGapColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub row_gap_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The row gap hatching fill color (default: transparent)."]
    #[serde(rename = "rowHatchColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub row_hatch_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The column gap highlight fill color (default: transparent)."]
    #[serde(rename = "columnGapColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub column_gap_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The column gap hatching fill color (default: transparent)."]
    #[serde(rename = "columnHatchColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub column_hatch_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The named grid areas border color (Default: transparent)."]
    #[serde(rename = "areaBorderColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub area_border_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The grid container background color (Default: transparent)."]
    #[serde(rename = "gridBackgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub grid_background_color: Option<crate::browser_protocol::dom::types::Rgba>,
}
impl GridHighlightConfig {
    pub const IDENTIFIER: &'static str = "Overlay.GridHighlightConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Configuration data for the highlighting of Flex container elements.\n[FlexContainerHighlightConfig](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#type-FlexContainerHighlightConfig)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FlexContainerHighlightConfig {
    #[doc = "The style of the container border"]
    #[serde(rename = "containerBorder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub container_border: Option<LineStyle>,
    #[doc = "The style of the separator between lines"]
    #[serde(rename = "lineSeparator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub line_separator: Option<LineStyle>,
    #[doc = "The style of the separator between items"]
    #[serde(rename = "itemSeparator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub item_separator: Option<LineStyle>,
    #[doc = "Style of content-distribution space on the main axis (justify-content)."]
    #[serde(rename = "mainDistributedSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub main_distributed_space: Option<BoxStyle>,
    #[doc = "Style of content-distribution space on the cross axis (align-content)."]
    #[serde(rename = "crossDistributedSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cross_distributed_space: Option<BoxStyle>,
    #[doc = "Style of empty space caused by row gaps (gap/row-gap)."]
    #[serde(rename = "rowGapSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub row_gap_space: Option<BoxStyle>,
    #[doc = "Style of empty space caused by columns gaps (gap/column-gap)."]
    #[serde(rename = "columnGapSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub column_gap_space: Option<BoxStyle>,
    #[doc = "Style of the self-alignment line (align-items)."]
    #[serde(rename = "crossAlignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cross_alignment: Option<LineStyle>,
}
impl FlexContainerHighlightConfig {
    pub const IDENTIFIER: &'static str = "Overlay.FlexContainerHighlightConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Configuration data for the highlighting of Flex item elements.\n[FlexItemHighlightConfig](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#type-FlexItemHighlightConfig)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FlexItemHighlightConfig {
    #[doc = "Style of the box representing the item's base size"]
    #[serde(rename = "baseSizeBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub base_size_box: Option<BoxStyle>,
    #[doc = "Style of the border around the box representing the item's base size"]
    #[serde(rename = "baseSizeBorder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub base_size_border: Option<LineStyle>,
    #[doc = "Style of the arrow representing if the item grew or shrank"]
    #[serde(rename = "flexibilityArrow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub flexibility_arrow: Option<LineStyle>,
}
impl FlexItemHighlightConfig {
    pub const IDENTIFIER: &'static str = "Overlay.FlexItemHighlightConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Style information for drawing a line.\n[LineStyle](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#type-LineStyle)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LineStyle {
    #[doc = "The color of the line (default: transparent)"]
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The line pattern (default: solid)"]
    #[serde(rename = "pattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pattern: Option<LineStylePattern>,
}
#[doc = "The line pattern (default: solid)"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LineStylePattern {
    #[serde(rename = "dashed")]
    Dashed,
    #[serde(rename = "dotted")]
    Dotted,
}
impl LineStyle {
    pub const IDENTIFIER: &'static str = "Overlay.LineStyle";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Style information for drawing a box.\n[BoxStyle](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#type-BoxStyle)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BoxStyle {
    #[doc = "The background color for the box (default: transparent)"]
    #[serde(rename = "fillColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fill_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The hatching color for the box (default: transparent)"]
    #[serde(rename = "hatchColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub hatch_color: Option<crate::browser_protocol::dom::types::Rgba>,
}
impl BoxStyle {
    pub const IDENTIFIER: &'static str = "Overlay.BoxStyle";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContrastAlgorithm {
    #[serde(rename = "aa")]
    Aa,
    #[serde(rename = "aaa")]
    Aaa,
    #[serde(rename = "apca")]
    Apca,
}
#[doc = "Configuration data for the highlighting of page elements.\n[HighlightConfig](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#type-HighlightConfig)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HighlightConfig {
    #[doc = "Whether the node info tooltip should be shown (default: false)."]
    #[serde(rename = "showInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub show_info: Option<bool>,
    #[doc = "Whether the node styles in the tooltip (default: false)."]
    #[serde(rename = "showStyles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub show_styles: Option<bool>,
    #[doc = "Whether the rulers should be shown (default: false)."]
    #[serde(rename = "showRulers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub show_rulers: Option<bool>,
    #[doc = "Whether the a11y info should be shown (default: true)."]
    #[serde(rename = "showAccessibilityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub show_accessibility_info: Option<bool>,
    #[doc = "Whether the extension lines from node to the rulers should be shown (default: false)."]
    #[serde(rename = "showExtensionLines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub show_extension_lines: Option<bool>,
    #[doc = "The content box highlight fill color (default: transparent)."]
    #[serde(rename = "contentColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub content_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The padding highlight fill color (default: transparent)."]
    #[serde(rename = "paddingColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub padding_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The border highlight fill color (default: transparent)."]
    #[serde(rename = "borderColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub border_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The margin highlight fill color (default: transparent)."]
    #[serde(rename = "marginColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub margin_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The event target element highlight fill color (default: transparent)."]
    #[serde(rename = "eventTargetColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub event_target_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The shape outside fill color (default: transparent)."]
    #[serde(rename = "shapeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub shape_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The shape margin fill color (default: transparent)."]
    #[serde(rename = "shapeMarginColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub shape_margin_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The grid layout color (default: transparent)."]
    #[serde(rename = "cssGridColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub css_grid_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The color format used to format color styles (default: hex)."]
    #[serde(rename = "colorFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub color_format: Option<ColorFormat>,
    #[doc = "The grid layout highlight configuration (default: all transparent)."]
    #[serde(rename = "gridHighlightConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub grid_highlight_config: Option<GridHighlightConfig>,
    #[doc = "The flex container highlight configuration (default: all transparent)."]
    #[serde(rename = "flexContainerHighlightConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub flex_container_highlight_config: Option<FlexContainerHighlightConfig>,
    #[doc = "The flex item highlight configuration (default: all transparent)."]
    #[serde(rename = "flexItemHighlightConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub flex_item_highlight_config: Option<FlexItemHighlightConfig>,
    #[doc = "The contrast algorithm to use for the contrast ratio (default: aa)."]
    #[serde(rename = "contrastAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contrast_algorithm: Option<ContrastAlgorithm>,
    #[doc = "The container query container highlight configuration (default: all transparent)."]
    #[serde(rename = "containerQueryContainerHighlightConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub container_query_container_highlight_config: Option<ContainerQueryContainerHighlightConfig>,
}
impl HighlightConfig {
    pub const IDENTIFIER: &'static str = "Overlay.HighlightConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorFormat {
    #[serde(rename = "rgb")]
    Rgb,
    #[serde(rename = "hsl")]
    Hsl,
    #[serde(rename = "hwb")]
    Hwb,
    #[serde(rename = "hex")]
    Hex,
}
#[doc = "Configurations for Persistent Grid Highlight\n[GridNodeHighlightConfig](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#type-GridNodeHighlightConfig)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridNodeHighlightConfig {
    #[doc = "A descriptor for the highlight appearance."]
    #[serde(rename = "gridHighlightConfig")]
    pub grid_highlight_config: GridHighlightConfig,
    #[doc = "Identifier of the node to highlight."]
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
}
impl GridNodeHighlightConfig {
    pub fn new(
        grid_highlight_config: impl Into<GridHighlightConfig>,
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
    ) -> Self {
        Self {
            grid_highlight_config: grid_highlight_config.into(),
            node_id: node_id.into(),
        }
    }
}
impl GridNodeHighlightConfig {
    pub const IDENTIFIER: &'static str = "Overlay.GridNodeHighlightConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlexNodeHighlightConfig {
    #[doc = "A descriptor for the highlight appearance of flex containers."]
    #[serde(rename = "flexContainerHighlightConfig")]
    pub flex_container_highlight_config: FlexContainerHighlightConfig,
    #[doc = "Identifier of the node to highlight."]
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
}
impl FlexNodeHighlightConfig {
    pub fn new(
        flex_container_highlight_config: impl Into<FlexContainerHighlightConfig>,
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
    ) -> Self {
        Self {
            flex_container_highlight_config: flex_container_highlight_config.into(),
            node_id: node_id.into(),
        }
    }
}
impl FlexNodeHighlightConfig {
    pub const IDENTIFIER: &'static str = "Overlay.FlexNodeHighlightConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScrollSnapContainerHighlightConfig {
    #[doc = "The style of the snapport border (default: transparent)"]
    #[serde(rename = "snapportBorder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub snapport_border: Option<LineStyle>,
    #[doc = "The style of the snap area border (default: transparent)"]
    #[serde(rename = "snapAreaBorder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub snap_area_border: Option<LineStyle>,
    #[doc = "The margin highlight fill color (default: transparent)."]
    #[serde(rename = "scrollMarginColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scroll_margin_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The padding highlight fill color (default: transparent)."]
    #[serde(rename = "scrollPaddingColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scroll_padding_color: Option<crate::browser_protocol::dom::types::Rgba>,
}
impl ScrollSnapContainerHighlightConfig {
    pub const IDENTIFIER: &'static str = "Overlay.ScrollSnapContainerHighlightConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScrollSnapHighlightConfig {
    #[doc = "A descriptor for the highlight appearance of scroll snap containers."]
    #[serde(rename = "scrollSnapContainerHighlightConfig")]
    pub scroll_snap_container_highlight_config: ScrollSnapContainerHighlightConfig,
    #[doc = "Identifier of the node to highlight."]
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
}
impl ScrollSnapHighlightConfig {
    pub fn new(
        scroll_snap_container_highlight_config: impl Into<ScrollSnapContainerHighlightConfig>,
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
    ) -> Self {
        Self {
            scroll_snap_container_highlight_config: scroll_snap_container_highlight_config.into(),
            node_id: node_id.into(),
        }
    }
}
impl ScrollSnapHighlightConfig {
    pub const IDENTIFIER: &'static str = "Overlay.ScrollSnapHighlightConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Configuration for dual screen hinge\n[HingeConfig](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#type-HingeConfig)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HingeConfig {
    #[doc = "A rectangle represent hinge"]
    #[serde(rename = "rect")]
    pub rect: crate::browser_protocol::dom::types::Rect,
    #[doc = "The content box highlight fill color (default: a dark color)."]
    #[serde(rename = "contentColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub content_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The content box highlight outline color (default: transparent)."]
    #[serde(rename = "outlineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub outline_color: Option<crate::browser_protocol::dom::types::Rgba>,
}
impl HingeConfig {
    pub fn new(rect: impl Into<crate::browser_protocol::dom::types::Rect>) -> Self {
        Self {
            rect: rect.into(),
            content_color: None,
            outline_color: None,
        }
    }
}
impl HingeConfig {
    pub const IDENTIFIER: &'static str = "Overlay.HingeConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Configuration for Window Controls Overlay\n[WindowControlsOverlayConfig](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#type-WindowControlsOverlayConfig)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowControlsOverlayConfig {
    #[doc = "Whether the title bar CSS should be shown when emulating the Window Controls Overlay."]
    #[serde(rename = "showCSS")]
    pub show_css: bool,
    #[doc = "Selected platforms to show the overlay."]
    #[serde(rename = "selectedPlatform")]
    pub selected_platform: String,
    #[doc = "The theme color defined in app manifest."]
    #[serde(rename = "themeColor")]
    pub theme_color: String,
}
impl WindowControlsOverlayConfig {
    pub fn new(
        show_css: impl Into<bool>,
        selected_platform: impl Into<String>,
        theme_color: impl Into<String>,
    ) -> Self {
        Self {
            show_css: show_css.into(),
            selected_platform: selected_platform.into(),
            theme_color: theme_color.into(),
        }
    }
}
impl WindowControlsOverlayConfig {
    pub const IDENTIFIER: &'static str = "Overlay.WindowControlsOverlayConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerQueryHighlightConfig {
    #[doc = "A descriptor for the highlight appearance of container query containers."]
    #[serde(rename = "containerQueryContainerHighlightConfig")]
    pub container_query_container_highlight_config: ContainerQueryContainerHighlightConfig,
    #[doc = "Identifier of the container node to highlight."]
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
}
impl ContainerQueryHighlightConfig {
    pub fn new(
        container_query_container_highlight_config: impl Into<ContainerQueryContainerHighlightConfig>,
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
    ) -> Self {
        Self {
            container_query_container_highlight_config: container_query_container_highlight_config
                .into(),
            node_id: node_id.into(),
        }
    }
}
impl ContainerQueryHighlightConfig {
    pub const IDENTIFIER: &'static str = "Overlay.ContainerQueryHighlightConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ContainerQueryContainerHighlightConfig {
    #[doc = "The style of the container border."]
    #[serde(rename = "containerBorder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub container_border: Option<LineStyle>,
    #[doc = "The style of the descendants' borders."]
    #[serde(rename = "descendantBorder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub descendant_border: Option<LineStyle>,
}
impl ContainerQueryContainerHighlightConfig {
    pub const IDENTIFIER: &'static str = "Overlay.ContainerQueryContainerHighlightConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsolatedElementHighlightConfig {
    #[doc = "A descriptor for the highlight appearance of an element in isolation mode."]
    #[serde(rename = "isolationModeHighlightConfig")]
    pub isolation_mode_highlight_config: IsolationModeHighlightConfig,
    #[doc = "Identifier of the isolated element to highlight."]
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
}
impl IsolatedElementHighlightConfig {
    pub fn new(
        isolation_mode_highlight_config: impl Into<IsolationModeHighlightConfig>,
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
    ) -> Self {
        Self {
            isolation_mode_highlight_config: isolation_mode_highlight_config.into(),
            node_id: node_id.into(),
        }
    }
}
impl IsolatedElementHighlightConfig {
    pub const IDENTIFIER: &'static str = "Overlay.IsolatedElementHighlightConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IsolationModeHighlightConfig {
    #[doc = "The fill color of the resizers (default: transparent)."]
    #[serde(rename = "resizerColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub resizer_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The fill color for resizer handles (default: transparent)."]
    #[serde(rename = "resizerHandleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub resizer_handle_color: Option<crate::browser_protocol::dom::types::Rgba>,
    #[doc = "The fill color for the mask covering non-isolated elements (default: transparent)."]
    #[serde(rename = "maskColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mask_color: Option<crate::browser_protocol::dom::types::Rgba>,
}
impl IsolationModeHighlightConfig {
    pub const IDENTIFIER: &'static str = "Overlay.IsolationModeHighlightConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InspectMode {
    #[serde(rename = "searchForNode")]
    SearchForNode,
    #[serde(rename = "searchForUAShadowDOM")]
    SearchForUaShadowDom,
    #[serde(rename = "captureAreaScreenshot")]
    CaptureAreaScreenshot,
    #[serde(rename = "none")]
    None,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InspectedElementAnchorConfig {
    #[doc = "Identifier of the node to highlight."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<crate::browser_protocol::dom::types::NodeId>,
    #[doc = "Identifier of the backend node to highlight."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
}
impl InspectedElementAnchorConfig {
    pub const IDENTIFIER: &'static str = "Overlay.InspectedElementAnchorConfig";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (OverlayTypes { SourceOrderConfig (SourceOrderConfig) , GridHighlightConfig (GridHighlightConfig) , FlexContainerHighlightConfig (FlexContainerHighlightConfig) , FlexItemHighlightConfig (FlexItemHighlightConfig) , LineStyle (LineStyle) , BoxStyle (BoxStyle) , ContrastAlgorithm (ContrastAlgorithm) , HighlightConfig (HighlightConfig) , ColorFormat (ColorFormat) , GridNodeHighlightConfig (GridNodeHighlightConfig) , FlexNodeHighlightConfig (FlexNodeHighlightConfig) , ScrollSnapContainerHighlightConfig (ScrollSnapContainerHighlightConfig) , ScrollSnapHighlightConfig (ScrollSnapHighlightConfig) , HingeConfig (HingeConfig) , WindowControlsOverlayConfig (WindowControlsOverlayConfig) , ContainerQueryHighlightConfig (ContainerQueryHighlightConfig) , ContainerQueryContainerHighlightConfig (ContainerQueryContainerHighlightConfig) , IsolatedElementHighlightConfig (IsolatedElementHighlightConfig) , IsolationModeHighlightConfig (IsolationModeHighlightConfig) , InspectMode (InspectMode) , InspectedElementAnchorConfig (InspectedElementAnchorConfig) });
