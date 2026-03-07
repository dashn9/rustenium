use super::types::*;
impl BackendNode {
    pub fn builder() -> BackendNodeBuilder {
        <BackendNodeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BackendNodeBuilder {
    node_type: Option<i64>,
    node_name: Option<String>,
    backend_node_id: Option<BackendNodeId>,
}
impl BackendNodeBuilder {
    pub fn node_type(mut self, node_type: impl Into<i64>) -> Self {
        self.node_type = Some(node_type.into());
        self
    }
    pub fn node_name(mut self, node_name: impl Into<String>) -> Self {
        self.node_name = Some(node_name.into());
        self
    }
    pub fn backend_node_id(mut self, backend_node_id: impl Into<BackendNodeId>) -> Self {
        self.backend_node_id = Some(backend_node_id.into());
        self
    }
    pub fn build(self) -> Result<BackendNode, String> {
        Ok(BackendNode {
            node_type: Box::new(self.node_type.ok_or_else(|| {
                std::stringify!("Field `{}` is mandatory.", std::stringify!(node_type))
            })?),
            node_name: self
                .node_name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_name)))?,
            backend_node_id: Box::new(self.backend_node_id.ok_or_else(|| {
                std::stringify!("Field `{}` is mandatory.", std::stringify!(backend_node_id))
            })?),
        })
    }
}
impl Node {
    pub fn builder() -> NodeBuilder {
        <NodeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct NodeBuilder {
    node_id: Option<NodeId>,
    parent_id: Option<NodeId>,
    backend_node_id: Option<BackendNodeId>,
    node_type: Option<i64>,
    node_name: Option<String>,
    local_name: Option<String>,
    node_value: Option<String>,
    child_node_count: Option<i64>,
    children: Option<Vec<Node>>,
    attributes: Option<Vec<String>>,
    document_url: Option<String>,
    base_url: Option<String>,
    public_id: Option<String>,
    system_id: Option<String>,
    internal_subset: Option<String>,
    xml_version: Option<String>,
    name: Option<String>,
    value: Option<String>,
    pseudo_type: Option<PseudoType>,
    pseudo_identifier: Option<String>,
    shadow_root_type: Option<ShadowRootType>,
    frame_id: Option<crate::browser_protocol::page::types::FrameId>,
    content_document: Option<Node>,
    shadow_roots: Option<Vec<Node>>,
    template_content: Option<Node>,
    pseudo_elements: Option<Vec<Node>>,
    distributed_nodes: Option<Vec<BackendNode>>,
    is_svg: Option<bool>,
    compatibility_mode: Option<CompatibilityMode>,
    assigned_slot: Option<BackendNode>,
    is_scrollable: Option<bool>,
    affected_by_starting_styles: Option<bool>,
    adopted_style_sheets: Option<Vec<StyleSheetId>>,
    is_ad_related: Option<bool>,
}
impl NodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn parent_id(mut self, parent_id: impl Into<NodeId>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }
    pub fn backend_node_id(mut self, backend_node_id: impl Into<BackendNodeId>) -> Self {
        self.backend_node_id = Some(backend_node_id.into());
        self
    }
    pub fn node_type(mut self, node_type: impl Into<i64>) -> Self {
        self.node_type = Some(node_type.into());
        self
    }
    pub fn node_name(mut self, node_name: impl Into<String>) -> Self {
        self.node_name = Some(node_name.into());
        self
    }
    pub fn local_name(mut self, local_name: impl Into<String>) -> Self {
        self.local_name = Some(local_name.into());
        self
    }
    pub fn node_value(mut self, node_value: impl Into<String>) -> Self {
        self.node_value = Some(node_value.into());
        self
    }
    pub fn child_node_count(mut self, child_node_count: impl Into<i64>) -> Self {
        self.child_node_count = Some(child_node_count.into());
        self
    }
    pub fn children(mut self, children: impl Into<Node>) -> Self {
        let v = self.children.get_or_insert(Vec::new());
        v.push(children.into());
        self
    }
    pub fn childrens<I, S>(mut self, childrens: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Node>,
    {
        let v = self.children.get_or_insert(Vec::new());
        for val in childrens {
            v.push(val.into());
        }
        self
    }
    pub fn attribute(mut self, attribute: impl Into<String>) -> Self {
        let v = self.attributes.get_or_insert(Vec::new());
        v.push(attribute.into());
        self
    }
    pub fn attributes<I, S>(mut self, attributes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.attributes.get_or_insert(Vec::new());
        for val in attributes {
            v.push(val.into());
        }
        self
    }
    pub fn document_url(mut self, document_url: impl Into<String>) -> Self {
        self.document_url = Some(document_url.into());
        self
    }
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }
    pub fn public_id(mut self, public_id: impl Into<String>) -> Self {
        self.public_id = Some(public_id.into());
        self
    }
    pub fn system_id(mut self, system_id: impl Into<String>) -> Self {
        self.system_id = Some(system_id.into());
        self
    }
    pub fn internal_subset(mut self, internal_subset: impl Into<String>) -> Self {
        self.internal_subset = Some(internal_subset.into());
        self
    }
    pub fn xml_version(mut self, xml_version: impl Into<String>) -> Self {
        self.xml_version = Some(xml_version.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn pseudo_type(mut self, pseudo_type: impl Into<PseudoType>) -> Self {
        self.pseudo_type = Some(pseudo_type.into());
        self
    }
    pub fn pseudo_identifier(mut self, pseudo_identifier: impl Into<String>) -> Self {
        self.pseudo_identifier = Some(pseudo_identifier.into());
        self
    }
    pub fn shadow_root_type(mut self, shadow_root_type: impl Into<ShadowRootType>) -> Self {
        self.shadow_root_type = Some(shadow_root_type.into());
        self
    }
    pub fn frame_id(
        mut self,
        frame_id: impl Into<crate::browser_protocol::page::types::FrameId>,
    ) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn content_document(mut self, content_document: impl Into<Node>) -> Self {
        self.content_document = Some(content_document.into());
        self
    }
    pub fn shadow_root(mut self, shadow_root: impl Into<Node>) -> Self {
        let v = self.shadow_roots.get_or_insert(Vec::new());
        v.push(shadow_root.into());
        self
    }
    pub fn shadow_roots<I, S>(mut self, shadow_roots: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Node>,
    {
        let v = self.shadow_roots.get_or_insert(Vec::new());
        for val in shadow_roots {
            v.push(val.into());
        }
        self
    }
    pub fn template_content(mut self, template_content: impl Into<Node>) -> Self {
        self.template_content = Some(template_content.into());
        self
    }
    pub fn pseudo_element(mut self, pseudo_element: impl Into<Node>) -> Self {
        let v = self.pseudo_elements.get_or_insert(Vec::new());
        v.push(pseudo_element.into());
        self
    }
    pub fn pseudo_elements<I, S>(mut self, pseudo_elements: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Node>,
    {
        let v = self.pseudo_elements.get_or_insert(Vec::new());
        for val in pseudo_elements {
            v.push(val.into());
        }
        self
    }
    pub fn distributed_node(mut self, distributed_node: impl Into<BackendNode>) -> Self {
        let v = self.distributed_nodes.get_or_insert(Vec::new());
        v.push(distributed_node.into());
        self
    }
    pub fn distributed_nodes<I, S>(mut self, distributed_nodes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<BackendNode>,
    {
        let v = self.distributed_nodes.get_or_insert(Vec::new());
        for val in distributed_nodes {
            v.push(val.into());
        }
        self
    }
    pub fn is_svg(mut self, is_svg: impl Into<bool>) -> Self {
        self.is_svg = Some(is_svg.into());
        self
    }
    pub fn compatibility_mode(mut self, compatibility_mode: impl Into<CompatibilityMode>) -> Self {
        self.compatibility_mode = Some(compatibility_mode.into());
        self
    }
    pub fn assigned_slot(mut self, assigned_slot: impl Into<BackendNode>) -> Self {
        self.assigned_slot = Some(assigned_slot.into());
        self
    }
    pub fn is_scrollable(mut self, is_scrollable: impl Into<bool>) -> Self {
        self.is_scrollable = Some(is_scrollable.into());
        self
    }
    pub fn affected_by_starting_styles(
        mut self,
        affected_by_starting_styles: impl Into<bool>,
    ) -> Self {
        self.affected_by_starting_styles = Some(affected_by_starting_styles.into());
        self
    }
    pub fn adopted_style_sheet(mut self, adopted_style_sheet: impl Into<StyleSheetId>) -> Self {
        let v = self.adopted_style_sheets.get_or_insert(Vec::new());
        v.push(adopted_style_sheet.into());
        self
    }
    pub fn adopted_style_sheets<I, S>(mut self, adopted_style_sheets: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<StyleSheetId>,
    {
        let v = self.adopted_style_sheets.get_or_insert(Vec::new());
        for val in adopted_style_sheets {
            v.push(val.into());
        }
        self
    }
    pub fn is_ad_related(mut self, is_ad_related: impl Into<bool>) -> Self {
        self.is_ad_related = Some(is_ad_related.into());
        self
    }
    pub fn build(self) -> Result<Node, String> {
        Ok(Node {
            node_id: Box::new(self.node_id.ok_or_else(|| {
                std::stringify!("Field `{}` is mandatory.", std::stringify!(node_id))
            })?),
            parent_id: self.parent_id,
            backend_node_id: Box::new(self.backend_node_id.ok_or_else(|| {
                std::stringify!("Field `{}` is mandatory.", std::stringify!(backend_node_id))
            })?),
            node_type: Box::new(self.node_type.ok_or_else(|| {
                std::stringify!("Field `{}` is mandatory.", std::stringify!(node_type))
            })?),
            node_name: self
                .node_name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_name)))?,
            local_name: self
                .local_name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(local_name)))?,
            node_value: self
                .node_value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_value)))?,
            child_node_count: self.child_node_count,
            children: self.children,
            attributes: self.attributes,
            document_url: self.document_url,
            base_url: self.base_url,
            public_id: self.public_id,
            system_id: self.system_id,
            internal_subset: self.internal_subset,
            xml_version: self.xml_version,
            name: self.name,
            value: self.value,
            pseudo_type: self.pseudo_type.map(Box::new),
            pseudo_identifier: self.pseudo_identifier,
            shadow_root_type: self.shadow_root_type.map(Box::new),
            frame_id: self.frame_id,
            content_document: self.content_document.map(Box::new),
            shadow_roots: self.shadow_roots,
            template_content: self.template_content.map(Box::new),
            pseudo_elements: self.pseudo_elements,
            distributed_nodes: self.distributed_nodes,
            is_svg: self.is_svg,
            compatibility_mode: self.compatibility_mode,
            assigned_slot: self.assigned_slot,
            is_scrollable: self.is_scrollable,
            affected_by_starting_styles: self.affected_by_starting_styles,
            adopted_style_sheets: self.adopted_style_sheets,
            is_ad_related: self.is_ad_related,
        })
    }
}
impl DetachedElementInfo {
    pub fn builder() -> DetachedElementInfoBuilder {
        <DetachedElementInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DetachedElementInfoBuilder {
    tree_node: Option<Node>,
    retained_node_ids: Option<Vec<NodeId>>,
}
impl DetachedElementInfoBuilder {
    pub fn tree_node(mut self, tree_node: impl Into<Node>) -> Self {
        self.tree_node = Some(tree_node.into());
        self
    }
    pub fn retained_node_id(mut self, retained_node_id: impl Into<NodeId>) -> Self {
        let v = self.retained_node_ids.get_or_insert(Vec::new());
        v.push(retained_node_id.into());
        self
    }
    pub fn retained_node_ids<I, S>(mut self, retained_node_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<NodeId>,
    {
        let v = self.retained_node_ids.get_or_insert(Vec::new());
        for val in retained_node_ids {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<DetachedElementInfo, String> {
        Ok(DetachedElementInfo {
            tree_node: self
                .tree_node
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(tree_node)))?,
            retained_node_ids: self.retained_node_ids.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(retained_node_ids)
                )
            })?,
        })
    }
}
impl Rgba {
    pub fn builder() -> RgbaBuilder {
        <RgbaBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RgbaBuilder {
    r: Option<i64>,
    g: Option<i64>,
    b: Option<i64>,
    a: Option<f64>,
}
impl RgbaBuilder {
    pub fn r(mut self, r: impl Into<i64>) -> Self {
        self.r = Some(r.into());
        self
    }
    pub fn g(mut self, g: impl Into<i64>) -> Self {
        self.g = Some(g.into());
        self
    }
    pub fn b(mut self, b: impl Into<i64>) -> Self {
        self.b = Some(b.into());
        self
    }
    pub fn a(mut self, a: impl Into<f64>) -> Self {
        self.a = Some(a.into());
        self
    }
    pub fn build(self) -> Result<Rgba, String> {
        Ok(Rgba {
            r: self
                .r
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r)))?,
            g: self
                .g
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(g)))?,
            b: self
                .b
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(b)))?,
            a: self.a,
        })
    }
}
impl BoxModel {
    pub fn builder() -> BoxModelBuilder {
        <BoxModelBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BoxModelBuilder {
    content: Option<Quad>,
    padding: Option<Quad>,
    border: Option<Quad>,
    margin: Option<Quad>,
    width: Option<i64>,
    height: Option<i64>,
    shape_outside: Option<ShapeOutsideInfo>,
}
impl BoxModelBuilder {
    pub fn content(mut self, content: impl Into<Quad>) -> Self {
        self.content = Some(content.into());
        self
    }
    pub fn padding(mut self, padding: impl Into<Quad>) -> Self {
        self.padding = Some(padding.into());
        self
    }
    pub fn border(mut self, border: impl Into<Quad>) -> Self {
        self.border = Some(border.into());
        self
    }
    pub fn margin(mut self, margin: impl Into<Quad>) -> Self {
        self.margin = Some(margin.into());
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
    pub fn shape_outside(mut self, shape_outside: impl Into<ShapeOutsideInfo>) -> Self {
        self.shape_outside = Some(shape_outside.into());
        self
    }
    pub fn build(self) -> Result<BoxModel, String> {
        Ok(BoxModel {
            content: self
                .content
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(content)))?,
            padding: self
                .padding
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(padding)))?,
            border: self
                .border
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(border)))?,
            margin: self
                .margin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(margin)))?,
            width: self
                .width
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(width)))?,
            height: self
                .height
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(height)))?,
            shape_outside: self.shape_outside,
        })
    }
}
impl ShapeOutsideInfo {
    pub fn builder() -> ShapeOutsideInfoBuilder {
        <ShapeOutsideInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ShapeOutsideInfoBuilder {
    bounds: Option<Quad>,
    shape: Option<Vec<serde_json::Value>>,
    margin_shape: Option<Vec<serde_json::Value>>,
}
impl ShapeOutsideInfoBuilder {
    pub fn bounds(mut self, bounds: impl Into<Quad>) -> Self {
        self.bounds = Some(bounds.into());
        self
    }
    pub fn shape(mut self, shape: impl Into<serde_json::Value>) -> Self {
        let v = self.shape.get_or_insert(Vec::new());
        v.push(shape.into());
        self
    }
    pub fn shapes<I, S>(mut self, shapes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<serde_json::Value>,
    {
        let v = self.shape.get_or_insert(Vec::new());
        for val in shapes {
            v.push(val.into());
        }
        self
    }
    pub fn margin_shape(mut self, margin_shape: impl Into<serde_json::Value>) -> Self {
        let v = self.margin_shape.get_or_insert(Vec::new());
        v.push(margin_shape.into());
        self
    }
    pub fn margin_shapes<I, S>(mut self, margin_shapes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<serde_json::Value>,
    {
        let v = self.margin_shape.get_or_insert(Vec::new());
        for val in margin_shapes {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<ShapeOutsideInfo, String> {
        Ok(ShapeOutsideInfo {
            bounds: self
                .bounds
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(bounds)))?,
            shape: self
                .shape
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(shape)))?,
            margin_shape: self.margin_shape.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(margin_shape))
            })?,
        })
    }
}
impl Rect {
    pub fn builder() -> RectBuilder {
        <RectBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RectBuilder {
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
}
impl RectBuilder {
    pub fn x(mut self, x: impl Into<f64>) -> Self {
        self.x = Some(x.into());
        self
    }
    pub fn y(mut self, y: impl Into<f64>) -> Self {
        self.y = Some(y.into());
        self
    }
    pub fn width(mut self, width: impl Into<f64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn height(mut self, height: impl Into<f64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn build(self) -> Result<Rect, String> {
        Ok(Rect {
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
        })
    }
}
impl CssComputedStyleProperty {
    pub fn builder() -> CssComputedStylePropertyBuilder {
        <CssComputedStylePropertyBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssComputedStylePropertyBuilder {
    name: Option<String>,
    value: Option<String>,
}
impl CssComputedStylePropertyBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<CssComputedStyleProperty, String> {
        Ok(CssComputedStyleProperty {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
