use super::types::*;
impl AxValueSource {
    pub fn builder() -> AxValueSourceBuilder {
        AxValueSourceBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AxValueSourceBuilder {
    r#type: Option<AxValueSourceType>,
    value: Option<AxValue>,
    attribute: Option<String>,
    attribute_value: Option<AxValue>,
    superseded: Option<bool>,
    native_source: Option<AxValueNativeSourceType>,
    native_source_value: Option<AxValue>,
    invalid: Option<bool>,
    invalid_reason: Option<String>,
}
impl AxValueSourceBuilder {
    pub fn r#type(mut self, r#type: impl Into<AxValueSourceType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<AxValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn attribute(mut self, attribute: impl Into<String>) -> Self {
        self.attribute = Some(attribute.into());
        self
    }
    pub fn attribute_value(mut self, attribute_value: impl Into<AxValue>) -> Self {
        self.attribute_value = Some(attribute_value.into());
        self
    }
    pub fn superseded(mut self, superseded: impl Into<bool>) -> Self {
        self.superseded = Some(superseded.into());
        self
    }
    pub fn native_source(mut self, native_source: impl Into<AxValueNativeSourceType>) -> Self {
        self.native_source = Some(native_source.into());
        self
    }
    pub fn native_source_value(mut self, native_source_value: impl Into<AxValue>) -> Self {
        self.native_source_value = Some(native_source_value.into());
        self
    }
    pub fn invalid(mut self, invalid: impl Into<bool>) -> Self {
        self.invalid = Some(invalid.into());
        self
    }
    pub fn invalid_reason(mut self, invalid_reason: impl Into<String>) -> Self {
        self.invalid_reason = Some(invalid_reason.into());
        self
    }
    pub fn build(self) -> Result<AxValueSource, String> {
        Ok(AxValueSource {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self.value,
            attribute: self.attribute,
            attribute_value: self.attribute_value,
            superseded: self.superseded,
            native_source: self.native_source,
            native_source_value: self.native_source_value,
            invalid: self.invalid,
            invalid_reason: self.invalid_reason,
        })
    }
}
impl AxRelatedNode {
    pub fn builder() -> AxRelatedNodeBuilder {
        AxRelatedNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AxRelatedNodeBuilder {
    backend_dom_node_id: Option<super::super::dom::types::BackendNodeId>,
    idref: Option<String>,
    text: Option<String>,
}
impl AxRelatedNodeBuilder {
    pub fn backend_dom_node_id(
        mut self,
        backend_dom_node_id: impl Into<super::super::dom::types::BackendNodeId>,
    ) -> Self {
        self.backend_dom_node_id = Some(backend_dom_node_id.into());
        self
    }
    pub fn idref(mut self, idref: impl Into<String>) -> Self {
        self.idref = Some(idref.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn build(self) -> Result<AxRelatedNode, String> {
        Ok(AxRelatedNode {
            backend_dom_node_id: self.backend_dom_node_id.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(backend_dom_node_id)
                )
            })?,
            idref: self.idref,
            text: self.text,
        })
    }
}
impl AxProperty {
    pub fn builder() -> AxPropertyBuilder {
        AxPropertyBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AxPropertyBuilder {
    name: Option<AxPropertyName>,
    value: Option<AxValue>,
}
impl AxPropertyBuilder {
    pub fn name(mut self, name: impl Into<AxPropertyName>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<AxValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<AxProperty, String> {
        Ok(AxProperty {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl AxValue {
    pub fn builder() -> AxValueBuilder {
        AxValueBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AxValueBuilder {
    r#type: Option<AxValueType>,
    value: Option<serde_json::Value>,
    related_nodes: Option<Vec<AxRelatedNode>>,
    sources: Option<Vec<AxValueSource>>,
}
impl AxValueBuilder {
    pub fn r#type(mut self, r#type: impl Into<AxValueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<serde_json::Value>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn related_node(mut self, related_node: impl Into<AxRelatedNode>) -> Self {
        let v = self.related_nodes.get_or_insert(Vec::new());
        v.push(related_node.into());
        self
    }
    pub fn related_nodes<I, S>(mut self, related_nodes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AxRelatedNode>,
    {
        let v = self.related_nodes.get_or_insert(Vec::new());
        for val in related_nodes {
            v.push(val.into());
        }
        self
    }
    pub fn source(mut self, source: impl Into<AxValueSource>) -> Self {
        let v = self.sources.get_or_insert(Vec::new());
        v.push(source.into());
        self
    }
    pub fn sources<I, S>(mut self, sources: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AxValueSource>,
    {
        let v = self.sources.get_or_insert(Vec::new());
        for val in sources {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<AxValue, String> {
        Ok(AxValue {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self.value,
            related_nodes: self.related_nodes,
            sources: self.sources,
        })
    }
}
impl AxNode {
    pub fn builder() -> AxNodeBuilder {
        AxNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AxNodeBuilder {
    node_id: Option<AxNodeId>,
    ignored: Option<bool>,
    ignored_reasons: Option<Vec<AxProperty>>,
    role: Option<AxValue>,
    chrome_role: Option<AxValue>,
    name: Option<AxValue>,
    description: Option<AxValue>,
    value: Option<AxValue>,
    properties: Option<Vec<AxProperty>>,
    parent_id: Option<AxNodeId>,
    child_ids: Option<Vec<AxNodeId>>,
    backend_dom_node_id: Option<super::super::dom::types::BackendNodeId>,
    frame_id: Option<super::super::page::types::FrameId>,
}
impl AxNodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<AxNodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn ignored(mut self, ignored: impl Into<bool>) -> Self {
        self.ignored = Some(ignored.into());
        self
    }
    pub fn ignored_reason(mut self, ignored_reason: impl Into<AxProperty>) -> Self {
        let v = self.ignored_reasons.get_or_insert(Vec::new());
        v.push(ignored_reason.into());
        self
    }
    pub fn ignored_reasons<I, S>(mut self, ignored_reasons: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AxProperty>,
    {
        let v = self.ignored_reasons.get_or_insert(Vec::new());
        for val in ignored_reasons {
            v.push(val.into());
        }
        self
    }
    pub fn role(mut self, role: impl Into<AxValue>) -> Self {
        self.role = Some(role.into());
        self
    }
    pub fn chrome_role(mut self, chrome_role: impl Into<AxValue>) -> Self {
        self.chrome_role = Some(chrome_role.into());
        self
    }
    pub fn name(mut self, name: impl Into<AxValue>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn description(mut self, description: impl Into<AxValue>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn value(mut self, value: impl Into<AxValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn propertie(mut self, propertie: impl Into<AxProperty>) -> Self {
        let v = self.properties.get_or_insert(Vec::new());
        v.push(propertie.into());
        self
    }
    pub fn properties<I, S>(mut self, properties: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AxProperty>,
    {
        let v = self.properties.get_or_insert(Vec::new());
        for val in properties {
            v.push(val.into());
        }
        self
    }
    pub fn parent_id(mut self, parent_id: impl Into<AxNodeId>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }
    pub fn child_id(mut self, child_id: impl Into<AxNodeId>) -> Self {
        let v = self.child_ids.get_or_insert(Vec::new());
        v.push(child_id.into());
        self
    }
    pub fn child_ids<I, S>(mut self, child_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AxNodeId>,
    {
        let v = self.child_ids.get_or_insert(Vec::new());
        for val in child_ids {
            v.push(val.into());
        }
        self
    }
    pub fn backend_dom_node_id(
        mut self,
        backend_dom_node_id: impl Into<super::super::dom::types::BackendNodeId>,
    ) -> Self {
        self.backend_dom_node_id = Some(backend_dom_node_id.into());
        self
    }
    pub fn frame_id(mut self, frame_id: impl Into<super::super::page::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn build(self) -> Result<AxNode, String> {
        Ok(AxNode {
            node_id: self
                .node_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            ignored: self
                .ignored
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(ignored)))?,
            ignored_reasons: self.ignored_reasons,
            role: self.role,
            chrome_role: self.chrome_role,
            name: self.name,
            description: self.description,
            value: self.value,
            properties: self.properties,
            parent_id: self.parent_id,
            child_ids: self.child_ids,
            backend_dom_node_id: self.backend_dom_node_id,
            frame_id: self.frame_id,
        })
    }
}
