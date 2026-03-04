use super::types::*;
impl SourceOrderConfig {
    pub fn builder() -> SourceOrderConfigBuilder {
        SourceOrderConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SourceOrderConfigBuilder {
    parent_outline_color: Option<super::super::dom::types::Rgba>,
    child_outline_color: Option<super::super::dom::types::Rgba>,
}
impl SourceOrderConfigBuilder {
    pub fn parent_outline_color(
        mut self,
        parent_outline_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.parent_outline_color = Some(parent_outline_color.into());
        self
    }
    pub fn child_outline_color(
        mut self,
        child_outline_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.child_outline_color = Some(child_outline_color.into());
        self
    }
    pub fn build(self) -> Result<SourceOrderConfig, String> {
        Ok(SourceOrderConfig {
            parent_outline_color: self.parent_outline_color.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(parent_outline_color)
                )
            })?,
            child_outline_color: self.child_outline_color.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(child_outline_color)
                )
            })?,
        })
    }
}
impl GridHighlightConfig {
    pub fn builder() -> GridHighlightConfigBuilder {
        GridHighlightConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GridHighlightConfigBuilder {
    show_grid_extension_lines: Option<bool>,
    show_positive_line_numbers: Option<bool>,
    show_negative_line_numbers: Option<bool>,
    show_area_names: Option<bool>,
    show_line_names: Option<bool>,
    show_track_sizes: Option<bool>,
    grid_border_color: Option<super::super::dom::types::Rgba>,
    row_line_color: Option<super::super::dom::types::Rgba>,
    column_line_color: Option<super::super::dom::types::Rgba>,
    grid_border_dash: Option<bool>,
    row_line_dash: Option<bool>,
    column_line_dash: Option<bool>,
    row_gap_color: Option<super::super::dom::types::Rgba>,
    row_hatch_color: Option<super::super::dom::types::Rgba>,
    column_gap_color: Option<super::super::dom::types::Rgba>,
    column_hatch_color: Option<super::super::dom::types::Rgba>,
    area_border_color: Option<super::super::dom::types::Rgba>,
    grid_background_color: Option<super::super::dom::types::Rgba>,
}
impl GridHighlightConfigBuilder {
    pub fn show_grid_extension_lines(mut self, show_grid_extension_lines: impl Into<bool>) -> Self {
        self.show_grid_extension_lines = Some(show_grid_extension_lines.into());
        self
    }
    pub fn show_positive_line_numbers(
        mut self,
        show_positive_line_numbers: impl Into<bool>,
    ) -> Self {
        self.show_positive_line_numbers = Some(show_positive_line_numbers.into());
        self
    }
    pub fn show_negative_line_numbers(
        mut self,
        show_negative_line_numbers: impl Into<bool>,
    ) -> Self {
        self.show_negative_line_numbers = Some(show_negative_line_numbers.into());
        self
    }
    pub fn show_area_names(mut self, show_area_names: impl Into<bool>) -> Self {
        self.show_area_names = Some(show_area_names.into());
        self
    }
    pub fn show_line_names(mut self, show_line_names: impl Into<bool>) -> Self {
        self.show_line_names = Some(show_line_names.into());
        self
    }
    pub fn show_track_sizes(mut self, show_track_sizes: impl Into<bool>) -> Self {
        self.show_track_sizes = Some(show_track_sizes.into());
        self
    }
    pub fn grid_border_color(
        mut self,
        grid_border_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.grid_border_color = Some(grid_border_color.into());
        self
    }
    pub fn row_line_color(
        mut self,
        row_line_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.row_line_color = Some(row_line_color.into());
        self
    }
    pub fn column_line_color(
        mut self,
        column_line_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.column_line_color = Some(column_line_color.into());
        self
    }
    pub fn grid_border_dash(mut self, grid_border_dash: impl Into<bool>) -> Self {
        self.grid_border_dash = Some(grid_border_dash.into());
        self
    }
    pub fn row_line_dash(mut self, row_line_dash: impl Into<bool>) -> Self {
        self.row_line_dash = Some(row_line_dash.into());
        self
    }
    pub fn column_line_dash(mut self, column_line_dash: impl Into<bool>) -> Self {
        self.column_line_dash = Some(column_line_dash.into());
        self
    }
    pub fn row_gap_color(
        mut self,
        row_gap_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.row_gap_color = Some(row_gap_color.into());
        self
    }
    pub fn row_hatch_color(
        mut self,
        row_hatch_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.row_hatch_color = Some(row_hatch_color.into());
        self
    }
    pub fn column_gap_color(
        mut self,
        column_gap_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.column_gap_color = Some(column_gap_color.into());
        self
    }
    pub fn column_hatch_color(
        mut self,
        column_hatch_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.column_hatch_color = Some(column_hatch_color.into());
        self
    }
    pub fn area_border_color(
        mut self,
        area_border_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.area_border_color = Some(area_border_color.into());
        self
    }
    pub fn grid_background_color(
        mut self,
        grid_background_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.grid_background_color = Some(grid_background_color.into());
        self
    }
    pub fn build(self) -> GridHighlightConfig {
        GridHighlightConfig {
            show_grid_extension_lines: self.show_grid_extension_lines,
            show_positive_line_numbers: self.show_positive_line_numbers,
            show_negative_line_numbers: self.show_negative_line_numbers,
            show_area_names: self.show_area_names,
            show_line_names: self.show_line_names,
            show_track_sizes: self.show_track_sizes,
            grid_border_color: self.grid_border_color,
            row_line_color: self.row_line_color,
            column_line_color: self.column_line_color,
            grid_border_dash: self.grid_border_dash,
            row_line_dash: self.row_line_dash,
            column_line_dash: self.column_line_dash,
            row_gap_color: self.row_gap_color,
            row_hatch_color: self.row_hatch_color,
            column_gap_color: self.column_gap_color,
            column_hatch_color: self.column_hatch_color,
            area_border_color: self.area_border_color,
            grid_background_color: self.grid_background_color,
        }
    }
}
impl FlexContainerHighlightConfig {
    pub fn builder() -> FlexContainerHighlightConfigBuilder {
        FlexContainerHighlightConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FlexContainerHighlightConfigBuilder {
    container_border: Option<LineStyle>,
    line_separator: Option<LineStyle>,
    item_separator: Option<LineStyle>,
    main_distributed_space: Option<BoxStyle>,
    cross_distributed_space: Option<BoxStyle>,
    row_gap_space: Option<BoxStyle>,
    column_gap_space: Option<BoxStyle>,
    cross_alignment: Option<LineStyle>,
}
impl FlexContainerHighlightConfigBuilder {
    pub fn container_border(mut self, container_border: impl Into<LineStyle>) -> Self {
        self.container_border = Some(container_border.into());
        self
    }
    pub fn line_separator(mut self, line_separator: impl Into<LineStyle>) -> Self {
        self.line_separator = Some(line_separator.into());
        self
    }
    pub fn item_separator(mut self, item_separator: impl Into<LineStyle>) -> Self {
        self.item_separator = Some(item_separator.into());
        self
    }
    pub fn main_distributed_space(mut self, main_distributed_space: impl Into<BoxStyle>) -> Self {
        self.main_distributed_space = Some(main_distributed_space.into());
        self
    }
    pub fn cross_distributed_space(mut self, cross_distributed_space: impl Into<BoxStyle>) -> Self {
        self.cross_distributed_space = Some(cross_distributed_space.into());
        self
    }
    pub fn row_gap_space(mut self, row_gap_space: impl Into<BoxStyle>) -> Self {
        self.row_gap_space = Some(row_gap_space.into());
        self
    }
    pub fn column_gap_space(mut self, column_gap_space: impl Into<BoxStyle>) -> Self {
        self.column_gap_space = Some(column_gap_space.into());
        self
    }
    pub fn cross_alignment(mut self, cross_alignment: impl Into<LineStyle>) -> Self {
        self.cross_alignment = Some(cross_alignment.into());
        self
    }
    pub fn build(self) -> FlexContainerHighlightConfig {
        FlexContainerHighlightConfig {
            container_border: self.container_border,
            line_separator: self.line_separator,
            item_separator: self.item_separator,
            main_distributed_space: self.main_distributed_space,
            cross_distributed_space: self.cross_distributed_space,
            row_gap_space: self.row_gap_space,
            column_gap_space: self.column_gap_space,
            cross_alignment: self.cross_alignment,
        }
    }
}
impl FlexItemHighlightConfig {
    pub fn builder() -> FlexItemHighlightConfigBuilder {
        FlexItemHighlightConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FlexItemHighlightConfigBuilder {
    base_size_box: Option<BoxStyle>,
    base_size_border: Option<LineStyle>,
    flexibility_arrow: Option<LineStyle>,
}
impl FlexItemHighlightConfigBuilder {
    pub fn base_size_box(mut self, base_size_box: impl Into<BoxStyle>) -> Self {
        self.base_size_box = Some(base_size_box.into());
        self
    }
    pub fn base_size_border(mut self, base_size_border: impl Into<LineStyle>) -> Self {
        self.base_size_border = Some(base_size_border.into());
        self
    }
    pub fn flexibility_arrow(mut self, flexibility_arrow: impl Into<LineStyle>) -> Self {
        self.flexibility_arrow = Some(flexibility_arrow.into());
        self
    }
    pub fn build(self) -> FlexItemHighlightConfig {
        FlexItemHighlightConfig {
            base_size_box: self.base_size_box,
            base_size_border: self.base_size_border,
            flexibility_arrow: self.flexibility_arrow,
        }
    }
}
impl LineStyle {
    pub fn builder() -> LineStyleBuilder {
        LineStyleBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct LineStyleBuilder {
    color: Option<super::super::dom::types::Rgba>,
    pattern: Option<LineStylePattern>,
}
impl LineStyleBuilder {
    pub fn color(mut self, color: impl Into<super::super::dom::types::Rgba>) -> Self {
        self.color = Some(color.into());
        self
    }
    pub fn pattern(mut self, pattern: impl Into<LineStylePattern>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }
    pub fn build(self) -> LineStyle {
        LineStyle {
            color: self.color,
            pattern: self.pattern,
        }
    }
}
impl BoxStyle {
    pub fn builder() -> BoxStyleBuilder {
        BoxStyleBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct BoxStyleBuilder {
    fill_color: Option<super::super::dom::types::Rgba>,
    hatch_color: Option<super::super::dom::types::Rgba>,
}
impl BoxStyleBuilder {
    pub fn fill_color(mut self, fill_color: impl Into<super::super::dom::types::Rgba>) -> Self {
        self.fill_color = Some(fill_color.into());
        self
    }
    pub fn hatch_color(mut self, hatch_color: impl Into<super::super::dom::types::Rgba>) -> Self {
        self.hatch_color = Some(hatch_color.into());
        self
    }
    pub fn build(self) -> BoxStyle {
        BoxStyle {
            fill_color: self.fill_color,
            hatch_color: self.hatch_color,
        }
    }
}
impl HighlightConfig {
    pub fn builder() -> HighlightConfigBuilder {
        HighlightConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct HighlightConfigBuilder {
    show_info: Option<bool>,
    show_styles: Option<bool>,
    show_rulers: Option<bool>,
    show_accessibility_info: Option<bool>,
    show_extension_lines: Option<bool>,
    content_color: Option<super::super::dom::types::Rgba>,
    padding_color: Option<super::super::dom::types::Rgba>,
    border_color: Option<super::super::dom::types::Rgba>,
    margin_color: Option<super::super::dom::types::Rgba>,
    event_target_color: Option<super::super::dom::types::Rgba>,
    shape_color: Option<super::super::dom::types::Rgba>,
    shape_margin_color: Option<super::super::dom::types::Rgba>,
    css_grid_color: Option<super::super::dom::types::Rgba>,
    color_format: Option<ColorFormat>,
    grid_highlight_config: Option<GridHighlightConfig>,
    flex_container_highlight_config: Option<FlexContainerHighlightConfig>,
    flex_item_highlight_config: Option<FlexItemHighlightConfig>,
    contrast_algorithm: Option<ContrastAlgorithm>,
    container_query_container_highlight_config: Option<ContainerQueryContainerHighlightConfig>,
}
impl HighlightConfigBuilder {
    pub fn show_info(mut self, show_info: impl Into<bool>) -> Self {
        self.show_info = Some(show_info.into());
        self
    }
    pub fn show_styles(mut self, show_styles: impl Into<bool>) -> Self {
        self.show_styles = Some(show_styles.into());
        self
    }
    pub fn show_rulers(mut self, show_rulers: impl Into<bool>) -> Self {
        self.show_rulers = Some(show_rulers.into());
        self
    }
    pub fn show_accessibility_info(mut self, show_accessibility_info: impl Into<bool>) -> Self {
        self.show_accessibility_info = Some(show_accessibility_info.into());
        self
    }
    pub fn show_extension_lines(mut self, show_extension_lines: impl Into<bool>) -> Self {
        self.show_extension_lines = Some(show_extension_lines.into());
        self
    }
    pub fn content_color(
        mut self,
        content_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.content_color = Some(content_color.into());
        self
    }
    pub fn padding_color(
        mut self,
        padding_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.padding_color = Some(padding_color.into());
        self
    }
    pub fn border_color(mut self, border_color: impl Into<super::super::dom::types::Rgba>) -> Self {
        self.border_color = Some(border_color.into());
        self
    }
    pub fn margin_color(mut self, margin_color: impl Into<super::super::dom::types::Rgba>) -> Self {
        self.margin_color = Some(margin_color.into());
        self
    }
    pub fn event_target_color(
        mut self,
        event_target_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.event_target_color = Some(event_target_color.into());
        self
    }
    pub fn shape_color(mut self, shape_color: impl Into<super::super::dom::types::Rgba>) -> Self {
        self.shape_color = Some(shape_color.into());
        self
    }
    pub fn shape_margin_color(
        mut self,
        shape_margin_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.shape_margin_color = Some(shape_margin_color.into());
        self
    }
    pub fn css_grid_color(
        mut self,
        css_grid_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.css_grid_color = Some(css_grid_color.into());
        self
    }
    pub fn color_format(mut self, color_format: impl Into<ColorFormat>) -> Self {
        self.color_format = Some(color_format.into());
        self
    }
    pub fn grid_highlight_config(
        mut self,
        grid_highlight_config: impl Into<GridHighlightConfig>,
    ) -> Self {
        self.grid_highlight_config = Some(grid_highlight_config.into());
        self
    }
    pub fn flex_container_highlight_config(
        mut self,
        flex_container_highlight_config: impl Into<FlexContainerHighlightConfig>,
    ) -> Self {
        self.flex_container_highlight_config = Some(flex_container_highlight_config.into());
        self
    }
    pub fn flex_item_highlight_config(
        mut self,
        flex_item_highlight_config: impl Into<FlexItemHighlightConfig>,
    ) -> Self {
        self.flex_item_highlight_config = Some(flex_item_highlight_config.into());
        self
    }
    pub fn contrast_algorithm(mut self, contrast_algorithm: impl Into<ContrastAlgorithm>) -> Self {
        self.contrast_algorithm = Some(contrast_algorithm.into());
        self
    }
    pub fn container_query_container_highlight_config(
        mut self,
        container_query_container_highlight_config: impl Into<ContainerQueryContainerHighlightConfig>,
    ) -> Self {
        self.container_query_container_highlight_config =
            Some(container_query_container_highlight_config.into());
        self
    }
    pub fn build(self) -> HighlightConfig {
        HighlightConfig {
            show_info: self.show_info,
            show_styles: self.show_styles,
            show_rulers: self.show_rulers,
            show_accessibility_info: self.show_accessibility_info,
            show_extension_lines: self.show_extension_lines,
            content_color: self.content_color,
            padding_color: self.padding_color,
            border_color: self.border_color,
            margin_color: self.margin_color,
            event_target_color: self.event_target_color,
            shape_color: self.shape_color,
            shape_margin_color: self.shape_margin_color,
            css_grid_color: self.css_grid_color,
            color_format: self.color_format,
            grid_highlight_config: self.grid_highlight_config,
            flex_container_highlight_config: self.flex_container_highlight_config,
            flex_item_highlight_config: self.flex_item_highlight_config,
            contrast_algorithm: self.contrast_algorithm,
            container_query_container_highlight_config: self
                .container_query_container_highlight_config,
        }
    }
}
impl GridNodeHighlightConfig {
    pub fn builder() -> GridNodeHighlightConfigBuilder {
        GridNodeHighlightConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GridNodeHighlightConfigBuilder {
    grid_highlight_config: Option<GridHighlightConfig>,
    node_id: Option<super::super::dom::types::NodeId>,
}
impl GridNodeHighlightConfigBuilder {
    pub fn grid_highlight_config(
        mut self,
        grid_highlight_config: impl Into<GridHighlightConfig>,
    ) -> Self {
        self.grid_highlight_config = Some(grid_highlight_config.into());
        self
    }
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<GridNodeHighlightConfig, String> {
        Ok(GridNodeHighlightConfig {
            grid_highlight_config: self.grid_highlight_config.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(grid_highlight_config)
                )
            })?,
            node_id: self
                .node_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
        })
    }
}
impl FlexNodeHighlightConfig {
    pub fn builder() -> FlexNodeHighlightConfigBuilder {
        FlexNodeHighlightConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FlexNodeHighlightConfigBuilder {
    flex_container_highlight_config: Option<FlexContainerHighlightConfig>,
    node_id: Option<super::super::dom::types::NodeId>,
}
impl FlexNodeHighlightConfigBuilder {
    pub fn flex_container_highlight_config(
        mut self,
        flex_container_highlight_config: impl Into<FlexContainerHighlightConfig>,
    ) -> Self {
        self.flex_container_highlight_config = Some(flex_container_highlight_config.into());
        self
    }
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<FlexNodeHighlightConfig, String> {
        Ok(FlexNodeHighlightConfig {
            flex_container_highlight_config: self.flex_container_highlight_config.ok_or_else(
                || {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(flex_container_highlight_config)
                    )
                },
            )?,
            node_id: self
                .node_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
        })
    }
}
impl ScrollSnapContainerHighlightConfig {
    pub fn builder() -> ScrollSnapContainerHighlightConfigBuilder {
        ScrollSnapContainerHighlightConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ScrollSnapContainerHighlightConfigBuilder {
    snapport_border: Option<LineStyle>,
    snap_area_border: Option<LineStyle>,
    scroll_margin_color: Option<super::super::dom::types::Rgba>,
    scroll_padding_color: Option<super::super::dom::types::Rgba>,
}
impl ScrollSnapContainerHighlightConfigBuilder {
    pub fn snapport_border(mut self, snapport_border: impl Into<LineStyle>) -> Self {
        self.snapport_border = Some(snapport_border.into());
        self
    }
    pub fn snap_area_border(mut self, snap_area_border: impl Into<LineStyle>) -> Self {
        self.snap_area_border = Some(snap_area_border.into());
        self
    }
    pub fn scroll_margin_color(
        mut self,
        scroll_margin_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.scroll_margin_color = Some(scroll_margin_color.into());
        self
    }
    pub fn scroll_padding_color(
        mut self,
        scroll_padding_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.scroll_padding_color = Some(scroll_padding_color.into());
        self
    }
    pub fn build(self) -> ScrollSnapContainerHighlightConfig {
        ScrollSnapContainerHighlightConfig {
            snapport_border: self.snapport_border,
            snap_area_border: self.snap_area_border,
            scroll_margin_color: self.scroll_margin_color,
            scroll_padding_color: self.scroll_padding_color,
        }
    }
}
impl ScrollSnapHighlightConfig {
    pub fn builder() -> ScrollSnapHighlightConfigBuilder {
        ScrollSnapHighlightConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ScrollSnapHighlightConfigBuilder {
    scroll_snap_container_highlight_config: Option<ScrollSnapContainerHighlightConfig>,
    node_id: Option<super::super::dom::types::NodeId>,
}
impl ScrollSnapHighlightConfigBuilder {
    pub fn scroll_snap_container_highlight_config(
        mut self,
        scroll_snap_container_highlight_config: impl Into<ScrollSnapContainerHighlightConfig>,
    ) -> Self {
        self.scroll_snap_container_highlight_config =
            Some(scroll_snap_container_highlight_config.into());
        self
    }
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<ScrollSnapHighlightConfig, String> {
        Ok(ScrollSnapHighlightConfig {
            scroll_snap_container_highlight_config: self
                .scroll_snap_container_highlight_config
                .ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(scroll_snap_container_highlight_config)
                    )
                })?,
            node_id: self
                .node_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
        })
    }
}
impl HingeConfig {
    pub fn builder() -> HingeConfigBuilder {
        HingeConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct HingeConfigBuilder {
    rect: Option<super::super::dom::types::Rect>,
    content_color: Option<super::super::dom::types::Rgba>,
    outline_color: Option<super::super::dom::types::Rgba>,
}
impl HingeConfigBuilder {
    pub fn rect(mut self, rect: impl Into<super::super::dom::types::Rect>) -> Self {
        self.rect = Some(rect.into());
        self
    }
    pub fn content_color(
        mut self,
        content_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.content_color = Some(content_color.into());
        self
    }
    pub fn outline_color(
        mut self,
        outline_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.outline_color = Some(outline_color.into());
        self
    }
    pub fn build(self) -> Result<HingeConfig, String> {
        Ok(HingeConfig {
            rect: self
                .rect
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(rect)))?,
            content_color: self.content_color,
            outline_color: self.outline_color,
        })
    }
}
impl WindowControlsOverlayConfig {
    pub fn builder() -> WindowControlsOverlayConfigBuilder {
        WindowControlsOverlayConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct WindowControlsOverlayConfigBuilder {
    show_css: Option<bool>,
    selected_platform: Option<String>,
    theme_color: Option<String>,
}
impl WindowControlsOverlayConfigBuilder {
    pub fn show_css(mut self, show_css: impl Into<bool>) -> Self {
        self.show_css = Some(show_css.into());
        self
    }
    pub fn selected_platform(mut self, selected_platform: impl Into<String>) -> Self {
        self.selected_platform = Some(selected_platform.into());
        self
    }
    pub fn theme_color(mut self, theme_color: impl Into<String>) -> Self {
        self.theme_color = Some(theme_color.into());
        self
    }
    pub fn build(self) -> Result<WindowControlsOverlayConfig, String> {
        Ok(WindowControlsOverlayConfig {
            show_css: self
                .show_css
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(show_css)))?,
            selected_platform: self.selected_platform.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(selected_platform)
                )
            })?,
            theme_color: self
                .theme_color
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(theme_color)))?,
        })
    }
}
impl ContainerQueryHighlightConfig {
    pub fn builder() -> ContainerQueryHighlightConfigBuilder {
        ContainerQueryHighlightConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ContainerQueryHighlightConfigBuilder {
    container_query_container_highlight_config: Option<ContainerQueryContainerHighlightConfig>,
    node_id: Option<super::super::dom::types::NodeId>,
}
impl ContainerQueryHighlightConfigBuilder {
    pub fn container_query_container_highlight_config(
        mut self,
        container_query_container_highlight_config: impl Into<ContainerQueryContainerHighlightConfig>,
    ) -> Self {
        self.container_query_container_highlight_config =
            Some(container_query_container_highlight_config.into());
        self
    }
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<ContainerQueryHighlightConfig, String> {
        Ok(ContainerQueryHighlightConfig {
            container_query_container_highlight_config: self
                .container_query_container_highlight_config
                .ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(container_query_container_highlight_config)
                    )
                })?,
            node_id: self
                .node_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
        })
    }
}
impl ContainerQueryContainerHighlightConfig {
    pub fn builder() -> ContainerQueryContainerHighlightConfigBuilder {
        ContainerQueryContainerHighlightConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ContainerQueryContainerHighlightConfigBuilder {
    container_border: Option<LineStyle>,
    descendant_border: Option<LineStyle>,
}
impl ContainerQueryContainerHighlightConfigBuilder {
    pub fn container_border(mut self, container_border: impl Into<LineStyle>) -> Self {
        self.container_border = Some(container_border.into());
        self
    }
    pub fn descendant_border(mut self, descendant_border: impl Into<LineStyle>) -> Self {
        self.descendant_border = Some(descendant_border.into());
        self
    }
    pub fn build(self) -> ContainerQueryContainerHighlightConfig {
        ContainerQueryContainerHighlightConfig {
            container_border: self.container_border,
            descendant_border: self.descendant_border,
        }
    }
}
impl IsolatedElementHighlightConfig {
    pub fn builder() -> IsolatedElementHighlightConfigBuilder {
        IsolatedElementHighlightConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct IsolatedElementHighlightConfigBuilder {
    isolation_mode_highlight_config: Option<IsolationModeHighlightConfig>,
    node_id: Option<super::super::dom::types::NodeId>,
}
impl IsolatedElementHighlightConfigBuilder {
    pub fn isolation_mode_highlight_config(
        mut self,
        isolation_mode_highlight_config: impl Into<IsolationModeHighlightConfig>,
    ) -> Self {
        self.isolation_mode_highlight_config = Some(isolation_mode_highlight_config.into());
        self
    }
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<IsolatedElementHighlightConfig, String> {
        Ok(IsolatedElementHighlightConfig {
            isolation_mode_highlight_config: self.isolation_mode_highlight_config.ok_or_else(
                || {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(isolation_mode_highlight_config)
                    )
                },
            )?,
            node_id: self
                .node_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
        })
    }
}
impl IsolationModeHighlightConfig {
    pub fn builder() -> IsolationModeHighlightConfigBuilder {
        IsolationModeHighlightConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct IsolationModeHighlightConfigBuilder {
    resizer_color: Option<super::super::dom::types::Rgba>,
    resizer_handle_color: Option<super::super::dom::types::Rgba>,
    mask_color: Option<super::super::dom::types::Rgba>,
}
impl IsolationModeHighlightConfigBuilder {
    pub fn resizer_color(
        mut self,
        resizer_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.resizer_color = Some(resizer_color.into());
        self
    }
    pub fn resizer_handle_color(
        mut self,
        resizer_handle_color: impl Into<super::super::dom::types::Rgba>,
    ) -> Self {
        self.resizer_handle_color = Some(resizer_handle_color.into());
        self
    }
    pub fn mask_color(mut self, mask_color: impl Into<super::super::dom::types::Rgba>) -> Self {
        self.mask_color = Some(mask_color.into());
        self
    }
    pub fn build(self) -> IsolationModeHighlightConfig {
        IsolationModeHighlightConfig {
            resizer_color: self.resizer_color,
            resizer_handle_color: self.resizer_handle_color,
            mask_color: self.mask_color,
        }
    }
}
impl InspectedElementAnchorConfig {
    pub fn builder() -> InspectedElementAnchorConfigBuilder {
        InspectedElementAnchorConfigBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct InspectedElementAnchorConfigBuilder {
    node_id: Option<super::super::dom::types::NodeId>,
    backend_node_id: Option<super::super::dom::types::BackendNodeId>,
}
impl InspectedElementAnchorConfigBuilder {
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
    pub fn build(self) -> InspectedElementAnchorConfig {
        InspectedElementAnchorConfig {
            node_id: self.node_id,
            backend_node_id: self.backend_node_id,
        }
    }
}
