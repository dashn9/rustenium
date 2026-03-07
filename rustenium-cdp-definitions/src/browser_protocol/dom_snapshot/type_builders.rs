use super::types::*;
impl DomNode {
    pub fn builder() -> DomNodeBuilder {
        <DomNodeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DomNodeBuilder {
    node_type: Option<i64>,
    node_name: Option<String>,
    node_value: Option<String>,
    text_value: Option<String>,
    input_value: Option<String>,
    input_checked: Option<bool>,
    option_selected: Option<bool>,
    backend_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
    child_node_indexes: Option<Vec<i64>>,
    attributes: Option<Vec<NameValue>>,
    pseudo_element_indexes: Option<Vec<i64>>,
    layout_node_index: Option<i64>,
    document_url: Option<String>,
    base_url: Option<String>,
    content_language: Option<String>,
    document_encoding: Option<String>,
    public_id: Option<String>,
    system_id: Option<String>,
    frame_id: Option<crate::browser_protocol::page::types::FrameId>,
    content_document_index: Option<i64>,
    pseudo_type: Option<crate::browser_protocol::dom::types::PseudoType>,
    shadow_root_type: Option<crate::browser_protocol::dom::types::ShadowRootType>,
    is_clickable: Option<bool>,
    event_listeners: Option<Vec<crate::browser_protocol::dom_debugger::types::EventListener>>,
    current_source_url: Option<String>,
    origin_url: Option<String>,
    scroll_offset_x: Option<f64>,
    scroll_offset_y: Option<f64>,
}
impl DomNodeBuilder {
    pub fn node_type(mut self, node_type: impl Into<i64>) -> Self {
        self.node_type = Some(node_type.into());
        self
    }
    pub fn node_name(mut self, node_name: impl Into<String>) -> Self {
        self.node_name = Some(node_name.into());
        self
    }
    pub fn node_value(mut self, node_value: impl Into<String>) -> Self {
        self.node_value = Some(node_value.into());
        self
    }
    pub fn text_value(mut self, text_value: impl Into<String>) -> Self {
        self.text_value = Some(text_value.into());
        self
    }
    pub fn input_value(mut self, input_value: impl Into<String>) -> Self {
        self.input_value = Some(input_value.into());
        self
    }
    pub fn input_checked(mut self, input_checked: impl Into<bool>) -> Self {
        self.input_checked = Some(input_checked.into());
        self
    }
    pub fn option_selected(mut self, option_selected: impl Into<bool>) -> Self {
        self.option_selected = Some(option_selected.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<crate::browser_protocol::dom::types::BackendNodeId>,
    ) -> Self {
        self.backend_node_id = Some(backend_node_id.into());
        self
    }
    pub fn child_node_indexe(mut self, child_node_indexe: impl Into<i64>) -> Self {
        let v = self.child_node_indexes.get_or_insert(Vec::new());
        v.push(child_node_indexe.into());
        self
    }
    pub fn child_node_indexes<I, S>(mut self, child_node_indexes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.child_node_indexes.get_or_insert(Vec::new());
        for val in child_node_indexes {
            v.push(val.into());
        }
        self
    }
    pub fn attribute(mut self, attribute: impl Into<NameValue>) -> Self {
        let v = self.attributes.get_or_insert(Vec::new());
        v.push(attribute.into());
        self
    }
    pub fn attributes<I, S>(mut self, attributes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<NameValue>,
    {
        let v = self.attributes.get_or_insert(Vec::new());
        for val in attributes {
            v.push(val.into());
        }
        self
    }
    pub fn pseudo_element_indexe(mut self, pseudo_element_indexe: impl Into<i64>) -> Self {
        let v = self.pseudo_element_indexes.get_or_insert(Vec::new());
        v.push(pseudo_element_indexe.into());
        self
    }
    pub fn pseudo_element_indexes<I, S>(mut self, pseudo_element_indexes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.pseudo_element_indexes.get_or_insert(Vec::new());
        for val in pseudo_element_indexes {
            v.push(val.into());
        }
        self
    }
    pub fn layout_node_index(mut self, layout_node_index: impl Into<i64>) -> Self {
        self.layout_node_index = Some(layout_node_index.into());
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
    pub fn content_language(mut self, content_language: impl Into<String>) -> Self {
        self.content_language = Some(content_language.into());
        self
    }
    pub fn document_encoding(mut self, document_encoding: impl Into<String>) -> Self {
        self.document_encoding = Some(document_encoding.into());
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
    pub fn frame_id(
        mut self,
        frame_id: impl Into<crate::browser_protocol::page::types::FrameId>,
    ) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn content_document_index(mut self, content_document_index: impl Into<i64>) -> Self {
        self.content_document_index = Some(content_document_index.into());
        self
    }
    pub fn pseudo_type(
        mut self,
        pseudo_type: impl Into<crate::browser_protocol::dom::types::PseudoType>,
    ) -> Self {
        self.pseudo_type = Some(pseudo_type.into());
        self
    }
    pub fn shadow_root_type(
        mut self,
        shadow_root_type: impl Into<crate::browser_protocol::dom::types::ShadowRootType>,
    ) -> Self {
        self.shadow_root_type = Some(shadow_root_type.into());
        self
    }
    pub fn is_clickable(mut self, is_clickable: impl Into<bool>) -> Self {
        self.is_clickable = Some(is_clickable.into());
        self
    }
    pub fn event_listener(
        mut self,
        event_listener: impl Into<crate::browser_protocol::dom_debugger::types::EventListener>,
    ) -> Self {
        let v = self.event_listeners.get_or_insert(Vec::new());
        v.push(event_listener.into());
        self
    }
    pub fn event_listeners<I, S>(mut self, event_listeners: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser_protocol::dom_debugger::types::EventListener>,
    {
        let v = self.event_listeners.get_or_insert(Vec::new());
        for val in event_listeners {
            v.push(val.into());
        }
        self
    }
    pub fn current_source_url(mut self, current_source_url: impl Into<String>) -> Self {
        self.current_source_url = Some(current_source_url.into());
        self
    }
    pub fn origin_url(mut self, origin_url: impl Into<String>) -> Self {
        self.origin_url = Some(origin_url.into());
        self
    }
    pub fn scroll_offset_x(mut self, scroll_offset_x: impl Into<f64>) -> Self {
        self.scroll_offset_x = Some(scroll_offset_x.into());
        self
    }
    pub fn scroll_offset_y(mut self, scroll_offset_y: impl Into<f64>) -> Self {
        self.scroll_offset_y = Some(scroll_offset_y.into());
        self
    }
    pub fn build(self) -> Result<DomNode, String> {
        Ok(DomNode {
            node_type: self
                .node_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_type)))?,
            node_name: self
                .node_name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_name)))?,
            node_value: self
                .node_value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_value)))?,
            text_value: self.text_value,
            input_value: self.input_value,
            input_checked: self.input_checked,
            option_selected: self.option_selected,
            backend_node_id: self.backend_node_id.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(backend_node_id))
            })?,
            child_node_indexes: self.child_node_indexes,
            attributes: self.attributes,
            pseudo_element_indexes: self.pseudo_element_indexes,
            layout_node_index: self.layout_node_index,
            document_url: self.document_url,
            base_url: self.base_url,
            content_language: self.content_language,
            document_encoding: self.document_encoding,
            public_id: self.public_id,
            system_id: self.system_id,
            frame_id: self.frame_id,
            content_document_index: self.content_document_index,
            pseudo_type: self.pseudo_type,
            shadow_root_type: self.shadow_root_type,
            is_clickable: self.is_clickable,
            event_listeners: self.event_listeners,
            current_source_url: self.current_source_url,
            origin_url: self.origin_url,
            scroll_offset_x: self.scroll_offset_x,
            scroll_offset_y: self.scroll_offset_y,
        })
    }
}
impl InlineTextBox {
    pub fn builder() -> InlineTextBoxBuilder {
        <InlineTextBoxBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct InlineTextBoxBuilder {
    bounding_box: Option<crate::browser_protocol::dom::types::Rect>,
    start_character_index: Option<i64>,
    num_characters: Option<i64>,
}
impl InlineTextBoxBuilder {
    pub fn bounding_box(
        mut self,
        bounding_box: impl Into<crate::browser_protocol::dom::types::Rect>,
    ) -> Self {
        self.bounding_box = Some(bounding_box.into());
        self
    }
    pub fn start_character_index(mut self, start_character_index: impl Into<i64>) -> Self {
        self.start_character_index = Some(start_character_index.into());
        self
    }
    pub fn num_characters(mut self, num_characters: impl Into<i64>) -> Self {
        self.num_characters = Some(num_characters.into());
        self
    }
    pub fn build(self) -> Result<InlineTextBox, String> {
        Ok(InlineTextBox {
            bounding_box: self.bounding_box.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(bounding_box))
            })?,
            start_character_index: self.start_character_index.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(start_character_index)
                )
            })?,
            num_characters: self.num_characters.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(num_characters))
            })?,
        })
    }
}
impl LayoutTreeNode {
    pub fn builder() -> LayoutTreeNodeBuilder {
        <LayoutTreeNodeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct LayoutTreeNodeBuilder {
    dom_node_index: Option<i64>,
    bounding_box: Option<crate::browser_protocol::dom::types::Rect>,
    layout_text: Option<String>,
    inline_text_nodes: Option<Vec<InlineTextBox>>,
    style_index: Option<i64>,
    paint_order: Option<i64>,
    is_stacking_context: Option<bool>,
}
impl LayoutTreeNodeBuilder {
    pub fn dom_node_index(mut self, dom_node_index: impl Into<i64>) -> Self {
        self.dom_node_index = Some(dom_node_index.into());
        self
    }
    pub fn bounding_box(
        mut self,
        bounding_box: impl Into<crate::browser_protocol::dom::types::Rect>,
    ) -> Self {
        self.bounding_box = Some(bounding_box.into());
        self
    }
    pub fn layout_text(mut self, layout_text: impl Into<String>) -> Self {
        self.layout_text = Some(layout_text.into());
        self
    }
    pub fn inline_text_node(mut self, inline_text_node: impl Into<InlineTextBox>) -> Self {
        let v = self.inline_text_nodes.get_or_insert(Vec::new());
        v.push(inline_text_node.into());
        self
    }
    pub fn inline_text_nodes<I, S>(mut self, inline_text_nodes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<InlineTextBox>,
    {
        let v = self.inline_text_nodes.get_or_insert(Vec::new());
        for val in inline_text_nodes {
            v.push(val.into());
        }
        self
    }
    pub fn style_index(mut self, style_index: impl Into<i64>) -> Self {
        self.style_index = Some(style_index.into());
        self
    }
    pub fn paint_order(mut self, paint_order: impl Into<i64>) -> Self {
        self.paint_order = Some(paint_order.into());
        self
    }
    pub fn is_stacking_context(mut self, is_stacking_context: impl Into<bool>) -> Self {
        self.is_stacking_context = Some(is_stacking_context.into());
        self
    }
    pub fn build(self) -> Result<LayoutTreeNode, String> {
        Ok(LayoutTreeNode {
            dom_node_index: self.dom_node_index.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(dom_node_index))
            })?,
            bounding_box: self.bounding_box.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(bounding_box))
            })?,
            layout_text: self.layout_text,
            inline_text_nodes: self.inline_text_nodes,
            style_index: self.style_index,
            paint_order: self.paint_order,
            is_stacking_context: self.is_stacking_context,
        })
    }
}
impl ComputedStyle {
    pub fn builder() -> ComputedStyleBuilder {
        <ComputedStyleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ComputedStyleBuilder {
    properties: Option<Vec<NameValue>>,
}
impl ComputedStyleBuilder {
    pub fn propertie(mut self, propertie: impl Into<NameValue>) -> Self {
        let v = self.properties.get_or_insert(Vec::new());
        v.push(propertie.into());
        self
    }
    pub fn properties<I, S>(mut self, properties: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<NameValue>,
    {
        let v = self.properties.get_or_insert(Vec::new());
        for val in properties {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<ComputedStyle, String> {
        Ok(ComputedStyle {
            properties: self
                .properties
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(properties)))?,
        })
    }
}
impl NameValue {
    pub fn builder() -> NameValueBuilder {
        <NameValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct NameValueBuilder {
    name: Option<String>,
    value: Option<String>,
}
impl NameValueBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<NameValue, String> {
        Ok(NameValue {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl RareStringData {
    pub fn builder() -> RareStringDataBuilder {
        <RareStringDataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RareStringDataBuilder {
    index: Option<Vec<i64>>,
    value: Option<Vec<StringIndex>>,
}
impl RareStringDataBuilder {
    pub fn index(mut self, index: impl Into<i64>) -> Self {
        let v = self.index.get_or_insert(Vec::new());
        v.push(index.into());
        self
    }
    pub fn indexs<I, S>(mut self, indexs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.index.get_or_insert(Vec::new());
        for val in indexs {
            v.push(val.into());
        }
        self
    }
    pub fn value(mut self, value: impl Into<StringIndex>) -> Self {
        let v = self.value.get_or_insert(Vec::new());
        v.push(value.into());
        self
    }
    pub fn values<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<StringIndex>,
    {
        let v = self.value.get_or_insert(Vec::new());
        for val in values {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<RareStringData, String> {
        Ok(RareStringData {
            index: self
                .index
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(index)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl RareBooleanData {
    pub fn builder() -> RareBooleanDataBuilder {
        <RareBooleanDataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RareBooleanDataBuilder {
    index: Option<Vec<i64>>,
}
impl RareBooleanDataBuilder {
    pub fn index(mut self, index: impl Into<i64>) -> Self {
        let v = self.index.get_or_insert(Vec::new());
        v.push(index.into());
        self
    }
    pub fn indexs<I, S>(mut self, indexs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.index.get_or_insert(Vec::new());
        for val in indexs {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<RareBooleanData, String> {
        Ok(RareBooleanData {
            index: self
                .index
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(index)))?,
        })
    }
}
impl RareIntegerData {
    pub fn builder() -> RareIntegerDataBuilder {
        <RareIntegerDataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RareIntegerDataBuilder {
    index: Option<Vec<i64>>,
    value: Option<Vec<i64>>,
}
impl RareIntegerDataBuilder {
    pub fn index(mut self, index: impl Into<i64>) -> Self {
        let v = self.index.get_or_insert(Vec::new());
        v.push(index.into());
        self
    }
    pub fn indexs<I, S>(mut self, indexs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.index.get_or_insert(Vec::new());
        for val in indexs {
            v.push(val.into());
        }
        self
    }
    pub fn value(mut self, value: impl Into<i64>) -> Self {
        let v = self.value.get_or_insert(Vec::new());
        v.push(value.into());
        self
    }
    pub fn values<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.value.get_or_insert(Vec::new());
        for val in values {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<RareIntegerData, String> {
        Ok(RareIntegerData {
            index: self
                .index
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(index)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl DocumentSnapshot {
    pub fn builder() -> DocumentSnapshotBuilder {
        <DocumentSnapshotBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DocumentSnapshotBuilder {
    document_url: Option<StringIndex>,
    title: Option<StringIndex>,
    base_url: Option<StringIndex>,
    content_language: Option<StringIndex>,
    encoding_name: Option<StringIndex>,
    public_id: Option<StringIndex>,
    system_id: Option<StringIndex>,
    frame_id: Option<StringIndex>,
    nodes: Option<NodeTreeSnapshot>,
    layout: Option<LayoutTreeSnapshot>,
    text_boxes: Option<TextBoxSnapshot>,
    scroll_offset_x: Option<f64>,
    scroll_offset_y: Option<f64>,
    content_width: Option<f64>,
    content_height: Option<f64>,
}
impl DocumentSnapshotBuilder {
    pub fn document_url(mut self, document_url: impl Into<StringIndex>) -> Self {
        self.document_url = Some(document_url.into());
        self
    }
    pub fn title(mut self, title: impl Into<StringIndex>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn base_url(mut self, base_url: impl Into<StringIndex>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }
    pub fn content_language(mut self, content_language: impl Into<StringIndex>) -> Self {
        self.content_language = Some(content_language.into());
        self
    }
    pub fn encoding_name(mut self, encoding_name: impl Into<StringIndex>) -> Self {
        self.encoding_name = Some(encoding_name.into());
        self
    }
    pub fn public_id(mut self, public_id: impl Into<StringIndex>) -> Self {
        self.public_id = Some(public_id.into());
        self
    }
    pub fn system_id(mut self, system_id: impl Into<StringIndex>) -> Self {
        self.system_id = Some(system_id.into());
        self
    }
    pub fn frame_id(mut self, frame_id: impl Into<StringIndex>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn nodes(mut self, nodes: impl Into<NodeTreeSnapshot>) -> Self {
        self.nodes = Some(nodes.into());
        self
    }
    pub fn layout(mut self, layout: impl Into<LayoutTreeSnapshot>) -> Self {
        self.layout = Some(layout.into());
        self
    }
    pub fn text_boxes(mut self, text_boxes: impl Into<TextBoxSnapshot>) -> Self {
        self.text_boxes = Some(text_boxes.into());
        self
    }
    pub fn scroll_offset_x(mut self, scroll_offset_x: impl Into<f64>) -> Self {
        self.scroll_offset_x = Some(scroll_offset_x.into());
        self
    }
    pub fn scroll_offset_y(mut self, scroll_offset_y: impl Into<f64>) -> Self {
        self.scroll_offset_y = Some(scroll_offset_y.into());
        self
    }
    pub fn content_width(mut self, content_width: impl Into<f64>) -> Self {
        self.content_width = Some(content_width.into());
        self
    }
    pub fn content_height(mut self, content_height: impl Into<f64>) -> Self {
        self.content_height = Some(content_height.into());
        self
    }
    pub fn build(self) -> Result<DocumentSnapshot, String> {
        Ok(DocumentSnapshot {
            document_url: self.document_url.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(document_url))
            })?,
            title: self
                .title
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(title)))?,
            base_url: self
                .base_url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(base_url)))?,
            content_language: self.content_language.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(content_language)
                )
            })?,
            encoding_name: self.encoding_name.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(encoding_name))
            })?,
            public_id: self
                .public_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(public_id)))?,
            system_id: self
                .system_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(system_id)))?,
            frame_id: self
                .frame_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(frame_id)))?,
            nodes: self
                .nodes
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(nodes)))?,
            layout: self
                .layout
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(layout)))?,
            text_boxes: self
                .text_boxes
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text_boxes)))?,
            scroll_offset_x: self.scroll_offset_x,
            scroll_offset_y: self.scroll_offset_y,
            content_width: self.content_width,
            content_height: self.content_height,
        })
    }
}
impl NodeTreeSnapshot {
    pub fn builder() -> NodeTreeSnapshotBuilder {
        <NodeTreeSnapshotBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct NodeTreeSnapshotBuilder {
    parent_index: Option<Vec<i64>>,
    node_type: Option<Vec<i64>>,
    shadow_root_type: Option<RareStringData>,
    node_name: Option<Vec<StringIndex>>,
    node_value: Option<Vec<StringIndex>>,
    backend_node_id: Option<Vec<crate::browser_protocol::dom::types::BackendNodeId>>,
    attributes: Option<Vec<ArrayOfStrings>>,
    text_value: Option<RareStringData>,
    input_value: Option<RareStringData>,
    input_checked: Option<RareBooleanData>,
    option_selected: Option<RareBooleanData>,
    content_document_index: Option<RareIntegerData>,
    pseudo_type: Option<RareStringData>,
    pseudo_identifier: Option<RareStringData>,
    is_clickable: Option<RareBooleanData>,
    current_source_url: Option<RareStringData>,
    origin_url: Option<RareStringData>,
}
impl NodeTreeSnapshotBuilder {
    pub fn parent_index(mut self, parent_index: impl Into<i64>) -> Self {
        let v = self.parent_index.get_or_insert(Vec::new());
        v.push(parent_index.into());
        self
    }
    pub fn parent_indexs<I, S>(mut self, parent_indexs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.parent_index.get_or_insert(Vec::new());
        for val in parent_indexs {
            v.push(val.into());
        }
        self
    }
    pub fn node_type(mut self, node_type: impl Into<i64>) -> Self {
        let v = self.node_type.get_or_insert(Vec::new());
        v.push(node_type.into());
        self
    }
    pub fn node_types<I, S>(mut self, node_types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.node_type.get_or_insert(Vec::new());
        for val in node_types {
            v.push(val.into());
        }
        self
    }
    pub fn shadow_root_type(mut self, shadow_root_type: impl Into<RareStringData>) -> Self {
        self.shadow_root_type = Some(shadow_root_type.into());
        self
    }
    pub fn node_name(mut self, node_name: impl Into<StringIndex>) -> Self {
        let v = self.node_name.get_or_insert(Vec::new());
        v.push(node_name.into());
        self
    }
    pub fn node_names<I, S>(mut self, node_names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<StringIndex>,
    {
        let v = self.node_name.get_or_insert(Vec::new());
        for val in node_names {
            v.push(val.into());
        }
        self
    }
    pub fn node_value(mut self, node_value: impl Into<StringIndex>) -> Self {
        let v = self.node_value.get_or_insert(Vec::new());
        v.push(node_value.into());
        self
    }
    pub fn node_values<I, S>(mut self, node_values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<StringIndex>,
    {
        let v = self.node_value.get_or_insert(Vec::new());
        for val in node_values {
            v.push(val.into());
        }
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<crate::browser_protocol::dom::types::BackendNodeId>,
    ) -> Self {
        let v = self.backend_node_id.get_or_insert(Vec::new());
        v.push(backend_node_id.into());
        self
    }
    pub fn backend_node_ids<I, S>(mut self, backend_node_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser_protocol::dom::types::BackendNodeId>,
    {
        let v = self.backend_node_id.get_or_insert(Vec::new());
        for val in backend_node_ids {
            v.push(val.into());
        }
        self
    }
    pub fn attribute(mut self, attribute: impl Into<ArrayOfStrings>) -> Self {
        let v = self.attributes.get_or_insert(Vec::new());
        v.push(attribute.into());
        self
    }
    pub fn attributes<I, S>(mut self, attributes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<ArrayOfStrings>,
    {
        let v = self.attributes.get_or_insert(Vec::new());
        for val in attributes {
            v.push(val.into());
        }
        self
    }
    pub fn text_value(mut self, text_value: impl Into<RareStringData>) -> Self {
        self.text_value = Some(text_value.into());
        self
    }
    pub fn input_value(mut self, input_value: impl Into<RareStringData>) -> Self {
        self.input_value = Some(input_value.into());
        self
    }
    pub fn input_checked(mut self, input_checked: impl Into<RareBooleanData>) -> Self {
        self.input_checked = Some(input_checked.into());
        self
    }
    pub fn option_selected(mut self, option_selected: impl Into<RareBooleanData>) -> Self {
        self.option_selected = Some(option_selected.into());
        self
    }
    pub fn content_document_index(
        mut self,
        content_document_index: impl Into<RareIntegerData>,
    ) -> Self {
        self.content_document_index = Some(content_document_index.into());
        self
    }
    pub fn pseudo_type(mut self, pseudo_type: impl Into<RareStringData>) -> Self {
        self.pseudo_type = Some(pseudo_type.into());
        self
    }
    pub fn pseudo_identifier(mut self, pseudo_identifier: impl Into<RareStringData>) -> Self {
        self.pseudo_identifier = Some(pseudo_identifier.into());
        self
    }
    pub fn is_clickable(mut self, is_clickable: impl Into<RareBooleanData>) -> Self {
        self.is_clickable = Some(is_clickable.into());
        self
    }
    pub fn current_source_url(mut self, current_source_url: impl Into<RareStringData>) -> Self {
        self.current_source_url = Some(current_source_url.into());
        self
    }
    pub fn origin_url(mut self, origin_url: impl Into<RareStringData>) -> Self {
        self.origin_url = Some(origin_url.into());
        self
    }
    pub fn build(self) -> NodeTreeSnapshot {
        NodeTreeSnapshot {
            parent_index: self.parent_index,
            node_type: self.node_type,
            shadow_root_type: self.shadow_root_type,
            node_name: self.node_name,
            node_value: self.node_value,
            backend_node_id: self.backend_node_id,
            attributes: self.attributes,
            text_value: self.text_value,
            input_value: self.input_value,
            input_checked: self.input_checked,
            option_selected: self.option_selected,
            content_document_index: self.content_document_index,
            pseudo_type: self.pseudo_type,
            pseudo_identifier: self.pseudo_identifier,
            is_clickable: self.is_clickable,
            current_source_url: self.current_source_url,
            origin_url: self.origin_url,
        }
    }
}
impl LayoutTreeSnapshot {
    pub fn builder() -> LayoutTreeSnapshotBuilder {
        <LayoutTreeSnapshotBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct LayoutTreeSnapshotBuilder {
    node_index: Option<Vec<i64>>,
    styles: Option<Vec<ArrayOfStrings>>,
    bounds: Option<Vec<Rectangle>>,
    text: Option<Vec<StringIndex>>,
    stacking_contexts: Option<RareBooleanData>,
    paint_orders: Option<Vec<i64>>,
    offset_rects: Option<Vec<Rectangle>>,
    scroll_rects: Option<Vec<Rectangle>>,
    client_rects: Option<Vec<Rectangle>>,
    blended_background_colors: Option<Vec<StringIndex>>,
    text_color_opacities: Option<Vec<f64>>,
}
impl LayoutTreeSnapshotBuilder {
    pub fn node_index(mut self, node_index: impl Into<i64>) -> Self {
        let v = self.node_index.get_or_insert(Vec::new());
        v.push(node_index.into());
        self
    }
    pub fn node_indexs<I, S>(mut self, node_indexs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.node_index.get_or_insert(Vec::new());
        for val in node_indexs {
            v.push(val.into());
        }
        self
    }
    pub fn style(mut self, style: impl Into<ArrayOfStrings>) -> Self {
        let v = self.styles.get_or_insert(Vec::new());
        v.push(style.into());
        self
    }
    pub fn styles<I, S>(mut self, styles: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<ArrayOfStrings>,
    {
        let v = self.styles.get_or_insert(Vec::new());
        for val in styles {
            v.push(val.into());
        }
        self
    }
    pub fn bound(mut self, bound: impl Into<Rectangle>) -> Self {
        let v = self.bounds.get_or_insert(Vec::new());
        v.push(bound.into());
        self
    }
    pub fn bounds<I, S>(mut self, bounds: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Rectangle>,
    {
        let v = self.bounds.get_or_insert(Vec::new());
        for val in bounds {
            v.push(val.into());
        }
        self
    }
    pub fn text(mut self, text: impl Into<StringIndex>) -> Self {
        let v = self.text.get_or_insert(Vec::new());
        v.push(text.into());
        self
    }
    pub fn texts<I, S>(mut self, texts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<StringIndex>,
    {
        let v = self.text.get_or_insert(Vec::new());
        for val in texts {
            v.push(val.into());
        }
        self
    }
    pub fn stacking_contexts(mut self, stacking_contexts: impl Into<RareBooleanData>) -> Self {
        self.stacking_contexts = Some(stacking_contexts.into());
        self
    }
    pub fn paint_order(mut self, paint_order: impl Into<i64>) -> Self {
        let v = self.paint_orders.get_or_insert(Vec::new());
        v.push(paint_order.into());
        self
    }
    pub fn paint_orders<I, S>(mut self, paint_orders: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.paint_orders.get_or_insert(Vec::new());
        for val in paint_orders {
            v.push(val.into());
        }
        self
    }
    pub fn offset_rect(mut self, offset_rect: impl Into<Rectangle>) -> Self {
        let v = self.offset_rects.get_or_insert(Vec::new());
        v.push(offset_rect.into());
        self
    }
    pub fn offset_rects<I, S>(mut self, offset_rects: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Rectangle>,
    {
        let v = self.offset_rects.get_or_insert(Vec::new());
        for val in offset_rects {
            v.push(val.into());
        }
        self
    }
    pub fn scroll_rect(mut self, scroll_rect: impl Into<Rectangle>) -> Self {
        let v = self.scroll_rects.get_or_insert(Vec::new());
        v.push(scroll_rect.into());
        self
    }
    pub fn scroll_rects<I, S>(mut self, scroll_rects: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Rectangle>,
    {
        let v = self.scroll_rects.get_or_insert(Vec::new());
        for val in scroll_rects {
            v.push(val.into());
        }
        self
    }
    pub fn client_rect(mut self, client_rect: impl Into<Rectangle>) -> Self {
        let v = self.client_rects.get_or_insert(Vec::new());
        v.push(client_rect.into());
        self
    }
    pub fn client_rects<I, S>(mut self, client_rects: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Rectangle>,
    {
        let v = self.client_rects.get_or_insert(Vec::new());
        for val in client_rects {
            v.push(val.into());
        }
        self
    }
    pub fn blended_background_color(
        mut self,
        blended_background_color: impl Into<StringIndex>,
    ) -> Self {
        let v = self.blended_background_colors.get_or_insert(Vec::new());
        v.push(blended_background_color.into());
        self
    }
    pub fn blended_background_colors<I, S>(mut self, blended_background_colors: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<StringIndex>,
    {
        let v = self.blended_background_colors.get_or_insert(Vec::new());
        for val in blended_background_colors {
            v.push(val.into());
        }
        self
    }
    pub fn text_color_opacitie(mut self, text_color_opacitie: impl Into<f64>) -> Self {
        let v = self.text_color_opacities.get_or_insert(Vec::new());
        v.push(text_color_opacitie.into());
        self
    }
    pub fn text_color_opacities<I, S>(mut self, text_color_opacities: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        let v = self.text_color_opacities.get_or_insert(Vec::new());
        for val in text_color_opacities {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<LayoutTreeSnapshot, String> {
        Ok(LayoutTreeSnapshot {
            node_index: self
                .node_index
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_index)))?,
            styles: self
                .styles
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(styles)))?,
            bounds: self
                .bounds
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(bounds)))?,
            text: self
                .text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
            stacking_contexts: self.stacking_contexts.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(stacking_contexts)
                )
            })?,
            paint_orders: self.paint_orders,
            offset_rects: self.offset_rects,
            scroll_rects: self.scroll_rects,
            client_rects: self.client_rects,
            blended_background_colors: self.blended_background_colors,
            text_color_opacities: self.text_color_opacities,
        })
    }
}
impl TextBoxSnapshot {
    pub fn builder() -> TextBoxSnapshotBuilder {
        <TextBoxSnapshotBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct TextBoxSnapshotBuilder {
    layout_index: Option<Vec<i64>>,
    bounds: Option<Vec<Rectangle>>,
    start: Option<Vec<i64>>,
    length: Option<Vec<i64>>,
}
impl TextBoxSnapshotBuilder {
    pub fn layout_index(mut self, layout_index: impl Into<i64>) -> Self {
        let v = self.layout_index.get_or_insert(Vec::new());
        v.push(layout_index.into());
        self
    }
    pub fn layout_indexs<I, S>(mut self, layout_indexs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.layout_index.get_or_insert(Vec::new());
        for val in layout_indexs {
            v.push(val.into());
        }
        self
    }
    pub fn bound(mut self, bound: impl Into<Rectangle>) -> Self {
        let v = self.bounds.get_or_insert(Vec::new());
        v.push(bound.into());
        self
    }
    pub fn bounds<I, S>(mut self, bounds: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Rectangle>,
    {
        let v = self.bounds.get_or_insert(Vec::new());
        for val in bounds {
            v.push(val.into());
        }
        self
    }
    pub fn start(mut self, start: impl Into<i64>) -> Self {
        let v = self.start.get_or_insert(Vec::new());
        v.push(start.into());
        self
    }
    pub fn starts<I, S>(mut self, starts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.start.get_or_insert(Vec::new());
        for val in starts {
            v.push(val.into());
        }
        self
    }
    pub fn length(mut self, length: impl Into<i64>) -> Self {
        let v = self.length.get_or_insert(Vec::new());
        v.push(length.into());
        self
    }
    pub fn lengths<I, S>(mut self, lengths: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.length.get_or_insert(Vec::new());
        for val in lengths {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<TextBoxSnapshot, String> {
        Ok(TextBoxSnapshot {
            layout_index: self.layout_index.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(layout_index))
            })?,
            bounds: self
                .bounds
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(bounds)))?,
            start: self
                .start
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(start)))?,
            length: self
                .length
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(length)))?,
        })
    }
}
