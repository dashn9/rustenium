use super::types::*;
impl RuleSet {
    pub fn builder() -> RuleSetBuilder {
        RuleSetBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct RuleSetBuilder {
    id: Option<RuleSetId>,
    loader_id: Option<super::super::network::types::LoaderId>,
    source_text: Option<String>,
    backend_node_id: Option<super::super::dom::types::BackendNodeId>,
    url: Option<String>,
    request_id: Option<super::super::network::types::RequestId>,
    error_type: Option<RuleSetErrorType>,
    tag: Option<String>,
}
impl RuleSetBuilder {
    pub fn id(mut self, id: impl Into<RuleSetId>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn loader_id(
        mut self,
        loader_id: impl Into<super::super::network::types::LoaderId>,
    ) -> Self {
        self.loader_id = Some(loader_id.into());
        self
    }
    pub fn source_text(mut self, source_text: impl Into<String>) -> Self {
        self.source_text = Some(source_text.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<super::super::dom::types::BackendNodeId>,
    ) -> Self {
        self.backend_node_id = Some(backend_node_id.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn request_id(
        mut self,
        request_id: impl Into<super::super::network::types::RequestId>,
    ) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn error_type(mut self, error_type: impl Into<RuleSetErrorType>) -> Self {
        self.error_type = Some(error_type.into());
        self
    }
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
    pub fn build(self) -> Result<RuleSet, String> {
        Ok(RuleSet {
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            loader_id: self
                .loader_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(loader_id)))?,
            source_text: self
                .source_text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(source_text)))?,
            backend_node_id: self.backend_node_id,
            url: self.url,
            request_id: self.request_id,
            error_type: self.error_type,
            tag: self.tag,
        })
    }
}
impl PreloadingAttemptKey {
    pub fn builder() -> PreloadingAttemptKeyBuilder {
        PreloadingAttemptKeyBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PreloadingAttemptKeyBuilder {
    loader_id: Option<super::super::network::types::LoaderId>,
    action: Option<SpeculationAction>,
    url: Option<String>,
    target_hint: Option<SpeculationTargetHint>,
}
impl PreloadingAttemptKeyBuilder {
    pub fn loader_id(
        mut self,
        loader_id: impl Into<super::super::network::types::LoaderId>,
    ) -> Self {
        self.loader_id = Some(loader_id.into());
        self
    }
    pub fn action(mut self, action: impl Into<SpeculationAction>) -> Self {
        self.action = Some(action.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn target_hint(mut self, target_hint: impl Into<SpeculationTargetHint>) -> Self {
        self.target_hint = Some(target_hint.into());
        self
    }
    pub fn build(self) -> Result<PreloadingAttemptKey, String> {
        Ok(PreloadingAttemptKey {
            loader_id: self
                .loader_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(loader_id)))?,
            action: self
                .action
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(action)))?,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            target_hint: self.target_hint,
        })
    }
}
impl PreloadingAttemptSource {
    pub fn builder() -> PreloadingAttemptSourceBuilder {
        PreloadingAttemptSourceBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PreloadingAttemptSourceBuilder {
    key: Option<PreloadingAttemptKey>,
    rule_set_ids: Option<Vec<RuleSetId>>,
    node_ids: Option<Vec<super::super::dom::types::BackendNodeId>>,
}
impl PreloadingAttemptSourceBuilder {
    pub fn key(mut self, key: impl Into<PreloadingAttemptKey>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn rule_set_id(mut self, rule_set_id: impl Into<RuleSetId>) -> Self {
        let v = self.rule_set_ids.get_or_insert(Vec::new());
        v.push(rule_set_id.into());
        self
    }
    pub fn rule_set_ids<I, S>(mut self, rule_set_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<RuleSetId>,
    {
        let v = self.rule_set_ids.get_or_insert(Vec::new());
        for val in rule_set_ids {
            v.push(val.into());
        }
        self
    }
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::BackendNodeId>) -> Self {
        let v = self.node_ids.get_or_insert(Vec::new());
        v.push(node_id.into());
        self
    }
    pub fn node_ids<I, S>(mut self, node_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::super::dom::types::BackendNodeId>,
    {
        let v = self.node_ids.get_or_insert(Vec::new());
        for val in node_ids {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<PreloadingAttemptSource, String> {
        Ok(PreloadingAttemptSource {
            key: self
                .key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key)))?,
            rule_set_ids: self.rule_set_ids.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(rule_set_ids))
            })?,
            node_ids: self
                .node_ids
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_ids)))?,
        })
    }
}
impl PrerenderMismatchedHeaders {
    pub fn builder() -> PrerenderMismatchedHeadersBuilder {
        PrerenderMismatchedHeadersBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PrerenderMismatchedHeadersBuilder {
    header_name: Option<String>,
    initial_value: Option<String>,
    activation_value: Option<String>,
}
impl PrerenderMismatchedHeadersBuilder {
    pub fn header_name(mut self, header_name: impl Into<String>) -> Self {
        self.header_name = Some(header_name.into());
        self
    }
    pub fn initial_value(mut self, initial_value: impl Into<String>) -> Self {
        self.initial_value = Some(initial_value.into());
        self
    }
    pub fn activation_value(mut self, activation_value: impl Into<String>) -> Self {
        self.activation_value = Some(activation_value.into());
        self
    }
    pub fn build(self) -> Result<PrerenderMismatchedHeaders, String> {
        Ok(PrerenderMismatchedHeaders {
            header_name: self
                .header_name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(header_name)))?,
            initial_value: self.initial_value,
            activation_value: self.activation_value,
        })
    }
}
