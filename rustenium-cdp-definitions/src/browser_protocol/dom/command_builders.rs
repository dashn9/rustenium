use super::commands::*;
impl CollectClassNamesFromSubtree {
    pub fn builder() -> CollectClassNamesFromSubtreeBuilder {
        CollectClassNamesFromSubtreeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CollectClassNamesFromSubtreeBuilder {
    node_id: Option<super::types::NodeId>,
}
impl CollectClassNamesFromSubtreeBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<CollectClassNamesFromSubtree, String> {
        Ok(CollectClassNamesFromSubtree {
            method: CollectClassNamesFromSubtreeMethod::CollectClassNamesFromSubtree,
            params: CollectClassNamesFromSubtreeParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            },
        })
    }
}
impl CopyTo {
    pub fn builder() -> CopyToBuilder {
        CopyToBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CopyToBuilder {
    node_id: Option<super::types::NodeId>,
    target_node_id: Option<super::types::NodeId>,
    insert_before_node_id: Option<super::types::NodeId>,
}
impl CopyToBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn target_node_id(mut self, target_node_id: impl Into<super::types::NodeId>) -> Self {
        self.target_node_id = Some(target_node_id.into());
        self
    }
    pub fn insert_before_node_id(
        mut self,
        insert_before_node_id: impl Into<super::types::NodeId>,
    ) -> Self {
        self.insert_before_node_id = Some(insert_before_node_id.into());
        self
    }
    pub fn build(self) -> Result<CopyTo, String> {
        Ok(CopyTo {
            method: CopyToMethod::CopyTo,
            params: CopyToParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                target_node_id: self.target_node_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(target_node_id))
                })?,
                insert_before_node_id: self.insert_before_node_id,
            },
        })
    }
}
impl DescribeNode {
    pub fn builder() -> DescribeNodeBuilder {
        DescribeNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DescribeNodeBuilder {
    node_id: Option<super::types::NodeId>,
    backend_node_id: Option<super::types::BackendNodeId>,
    object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
    depth: Option<i64>,
    pierce: Option<bool>,
}
impl DescribeNodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<super::types::BackendNodeId>,
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
    pub fn depth(mut self, depth: impl Into<i64>) -> Self {
        self.depth = Some(depth.into());
        self
    }
    pub fn pierce(mut self, pierce: impl Into<bool>) -> Self {
        self.pierce = Some(pierce.into());
        self
    }
    pub fn build(self) -> DescribeNode {
        DescribeNode {
            method: DescribeNodeMethod::DescribeNode,
            params: DescribeNodeParams {
                node_id: self.node_id,
                backend_node_id: self.backend_node_id,
                object_id: self.object_id,
                depth: self.depth,
                pierce: self.pierce,
            },
        }
    }
}
impl ScrollIntoViewIfNeeded {
    pub fn builder() -> ScrollIntoViewIfNeededBuilder {
        ScrollIntoViewIfNeededBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ScrollIntoViewIfNeededBuilder {
    node_id: Option<super::types::NodeId>,
    backend_node_id: Option<super::types::BackendNodeId>,
    object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
    rect: Option<super::types::Rect>,
}
impl ScrollIntoViewIfNeededBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<super::types::BackendNodeId>,
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
    pub fn rect(mut self, rect: impl Into<super::types::Rect>) -> Self {
        self.rect = Some(rect.into());
        self
    }
    pub fn build(self) -> ScrollIntoViewIfNeeded {
        ScrollIntoViewIfNeeded {
            method: ScrollIntoViewIfNeededMethod::ScrollIntoViewIfNeeded,
            params: ScrollIntoViewIfNeededParams {
                node_id: self.node_id,
                backend_node_id: self.backend_node_id,
                object_id: self.object_id,
                rect: self.rect,
            },
        }
    }
}
impl DiscardSearchResults {
    pub fn builder() -> DiscardSearchResultsBuilder {
        DiscardSearchResultsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DiscardSearchResultsBuilder {
    search_id: Option<String>,
}
impl DiscardSearchResultsBuilder {
    pub fn search_id(mut self, search_id: impl Into<String>) -> Self {
        self.search_id = Some(search_id.into());
        self
    }
    pub fn build(self) -> Result<DiscardSearchResults, String> {
        Ok(DiscardSearchResults {
            method: DiscardSearchResultsMethod::DiscardSearchResults,
            params: DiscardSearchResultsParams {
                search_id: self.search_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(search_id))
                })?,
            },
        })
    }
}
impl Enable {
    pub fn builder() -> EnableBuilder {
        EnableBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct EnableBuilder {
    include_whitespace: Option<EnableIncludeWhitespace>,
}
impl EnableBuilder {
    pub fn include_whitespace(
        mut self,
        include_whitespace: impl Into<EnableIncludeWhitespace>,
    ) -> Self {
        self.include_whitespace = Some(include_whitespace.into());
        self
    }
    pub fn build(self) -> Enable {
        Enable {
            method: EnableMethod::Enable,
            params: EnableParams {
                include_whitespace: self.include_whitespace,
            },
        }
    }
}
impl Focus {
    pub fn builder() -> FocusBuilder {
        FocusBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FocusBuilder {
    node_id: Option<super::types::NodeId>,
    backend_node_id: Option<super::types::BackendNodeId>,
    object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
}
impl FocusBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<super::types::BackendNodeId>,
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
    pub fn build(self) -> Focus {
        Focus {
            method: FocusMethod::Focus,
            params: FocusParams {
                node_id: self.node_id,
                backend_node_id: self.backend_node_id,
                object_id: self.object_id,
            },
        }
    }
}
impl GetAttributes {
    pub fn builder() -> GetAttributesBuilder {
        GetAttributesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetAttributesBuilder {
    node_id: Option<super::types::NodeId>,
}
impl GetAttributesBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<GetAttributes, String> {
        Ok(GetAttributes {
            method: GetAttributesMethod::GetAttributes,
            params: GetAttributesParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            },
        })
    }
}
impl GetBoxModel {
    pub fn builder() -> GetBoxModelBuilder {
        GetBoxModelBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetBoxModelBuilder {
    node_id: Option<super::types::NodeId>,
    backend_node_id: Option<super::types::BackendNodeId>,
    object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
}
impl GetBoxModelBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<super::types::BackendNodeId>,
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
    pub fn build(self) -> GetBoxModel {
        GetBoxModel {
            method: GetBoxModelMethod::GetBoxModel,
            params: GetBoxModelParams {
                node_id: self.node_id,
                backend_node_id: self.backend_node_id,
                object_id: self.object_id,
            },
        }
    }
}
impl GetContentQuads {
    pub fn builder() -> GetContentQuadsBuilder {
        GetContentQuadsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetContentQuadsBuilder {
    node_id: Option<super::types::NodeId>,
    backend_node_id: Option<super::types::BackendNodeId>,
    object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
}
impl GetContentQuadsBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<super::types::BackendNodeId>,
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
    pub fn build(self) -> GetContentQuads {
        GetContentQuads {
            method: GetContentQuadsMethod::GetContentQuads,
            params: GetContentQuadsParams {
                node_id: self.node_id,
                backend_node_id: self.backend_node_id,
                object_id: self.object_id,
            },
        }
    }
}
impl GetDocument {
    pub fn builder() -> GetDocumentBuilder {
        GetDocumentBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetDocumentBuilder {
    depth: Option<i64>,
    pierce: Option<bool>,
}
impl GetDocumentBuilder {
    pub fn depth(mut self, depth: impl Into<i64>) -> Self {
        self.depth = Some(depth.into());
        self
    }
    pub fn pierce(mut self, pierce: impl Into<bool>) -> Self {
        self.pierce = Some(pierce.into());
        self
    }
    pub fn build(self) -> GetDocument {
        GetDocument {
            method: GetDocumentMethod::GetDocument,
            params: GetDocumentParams {
                depth: self.depth,
                pierce: self.pierce,
            },
        }
    }
}
impl GetNodesForSubtreeByStyle {
    pub fn builder() -> GetNodesForSubtreeByStyleBuilder {
        GetNodesForSubtreeByStyleBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetNodesForSubtreeByStyleBuilder {
    node_id: Option<super::types::NodeId>,
    computed_styles: Option<Vec<super::types::CssComputedStyleProperty>>,
    pierce: Option<bool>,
}
impl GetNodesForSubtreeByStyleBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn computed_style(
        mut self,
        computed_style: impl Into<super::types::CssComputedStyleProperty>,
    ) -> Self {
        let v = self.computed_styles.get_or_insert(Vec::new());
        v.push(computed_style.into());
        self
    }
    pub fn computed_styles<I, S>(mut self, computed_styles: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::CssComputedStyleProperty>,
    {
        let v = self.computed_styles.get_or_insert(Vec::new());
        for val in computed_styles {
            v.push(val.into());
        }
        self
    }
    pub fn pierce(mut self, pierce: impl Into<bool>) -> Self {
        self.pierce = Some(pierce.into());
        self
    }
    pub fn build(self) -> Result<GetNodesForSubtreeByStyle, String> {
        Ok(GetNodesForSubtreeByStyle {
            method: GetNodesForSubtreeByStyleMethod::GetNodesForSubtreeByStyle,
            params: GetNodesForSubtreeByStyleParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                computed_styles: self.computed_styles.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(computed_styles))
                })?,
                pierce: self.pierce,
            },
        })
    }
}
impl GetNodeForLocation {
    pub fn builder() -> GetNodeForLocationBuilder {
        GetNodeForLocationBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetNodeForLocationBuilder {
    x: Option<i64>,
    y: Option<i64>,
    include_user_agent_shadow_dom: Option<bool>,
    ignore_pointer_events_none: Option<bool>,
}
impl GetNodeForLocationBuilder {
    pub fn x(mut self, x: impl Into<i64>) -> Self {
        self.x = Some(x.into());
        self
    }
    pub fn y(mut self, y: impl Into<i64>) -> Self {
        self.y = Some(y.into());
        self
    }
    pub fn include_user_agent_shadow_dom(
        mut self,
        include_user_agent_shadow_dom: impl Into<bool>,
    ) -> Self {
        self.include_user_agent_shadow_dom = Some(include_user_agent_shadow_dom.into());
        self
    }
    pub fn ignore_pointer_events_none(
        mut self,
        ignore_pointer_events_none: impl Into<bool>,
    ) -> Self {
        self.ignore_pointer_events_none = Some(ignore_pointer_events_none.into());
        self
    }
    pub fn build(self) -> Result<GetNodeForLocation, String> {
        Ok(GetNodeForLocation {
            method: GetNodeForLocationMethod::GetNodeForLocation,
            params: GetNodeForLocationParams {
                x: self
                    .x
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
                y: self
                    .y
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
                include_user_agent_shadow_dom: self.include_user_agent_shadow_dom,
                ignore_pointer_events_none: self.ignore_pointer_events_none,
            },
        })
    }
}
impl GetOuterHtml {
    pub fn builder() -> GetOuterHtmlBuilder {
        GetOuterHtmlBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetOuterHtmlBuilder {
    node_id: Option<super::types::NodeId>,
    backend_node_id: Option<super::types::BackendNodeId>,
    object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
    include_shadow_dom: Option<bool>,
}
impl GetOuterHtmlBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<super::types::BackendNodeId>,
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
    pub fn include_shadow_dom(mut self, include_shadow_dom: impl Into<bool>) -> Self {
        self.include_shadow_dom = Some(include_shadow_dom.into());
        self
    }
    pub fn build(self) -> GetOuterHtml {
        GetOuterHtml {
            method: GetOuterHtmlMethod::GetOuterHtml,
            params: GetOuterHtmlParams {
                node_id: self.node_id,
                backend_node_id: self.backend_node_id,
                object_id: self.object_id,
                include_shadow_dom: self.include_shadow_dom,
            },
        }
    }
}
impl GetRelayoutBoundary {
    pub fn builder() -> GetRelayoutBoundaryBuilder {
        GetRelayoutBoundaryBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetRelayoutBoundaryBuilder {
    node_id: Option<super::types::NodeId>,
}
impl GetRelayoutBoundaryBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<GetRelayoutBoundary, String> {
        Ok(GetRelayoutBoundary {
            method: GetRelayoutBoundaryMethod::GetRelayoutBoundary,
            params: GetRelayoutBoundaryParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            },
        })
    }
}
impl GetSearchResults {
    pub fn builder() -> GetSearchResultsBuilder {
        GetSearchResultsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetSearchResultsBuilder {
    search_id: Option<String>,
    from_index: Option<i64>,
    to_index: Option<i64>,
}
impl GetSearchResultsBuilder {
    pub fn search_id(mut self, search_id: impl Into<String>) -> Self {
        self.search_id = Some(search_id.into());
        self
    }
    pub fn from_index(mut self, from_index: impl Into<i64>) -> Self {
        self.from_index = Some(from_index.into());
        self
    }
    pub fn to_index(mut self, to_index: impl Into<i64>) -> Self {
        self.to_index = Some(to_index.into());
        self
    }
    pub fn build(self) -> Result<GetSearchResults, String> {
        Ok(GetSearchResults {
            method: GetSearchResultsMethod::GetSearchResults,
            params: GetSearchResultsParams {
                search_id: self.search_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(search_id))
                })?,
                from_index: self.from_index.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(from_index))
                })?,
                to_index: self.to_index.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(to_index))
                })?,
            },
        })
    }
}
impl MoveTo {
    pub fn builder() -> MoveToBuilder {
        MoveToBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct MoveToBuilder {
    node_id: Option<super::types::NodeId>,
    target_node_id: Option<super::types::NodeId>,
    insert_before_node_id: Option<super::types::NodeId>,
}
impl MoveToBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn target_node_id(mut self, target_node_id: impl Into<super::types::NodeId>) -> Self {
        self.target_node_id = Some(target_node_id.into());
        self
    }
    pub fn insert_before_node_id(
        mut self,
        insert_before_node_id: impl Into<super::types::NodeId>,
    ) -> Self {
        self.insert_before_node_id = Some(insert_before_node_id.into());
        self
    }
    pub fn build(self) -> Result<MoveTo, String> {
        Ok(MoveTo {
            method: MoveToMethod::MoveTo,
            params: MoveToParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                target_node_id: self.target_node_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(target_node_id))
                })?,
                insert_before_node_id: self.insert_before_node_id,
            },
        })
    }
}
impl PerformSearch {
    pub fn builder() -> PerformSearchBuilder {
        PerformSearchBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PerformSearchBuilder {
    query: Option<String>,
    include_user_agent_shadow_dom: Option<bool>,
}
impl PerformSearchBuilder {
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }
    pub fn include_user_agent_shadow_dom(
        mut self,
        include_user_agent_shadow_dom: impl Into<bool>,
    ) -> Self {
        self.include_user_agent_shadow_dom = Some(include_user_agent_shadow_dom.into());
        self
    }
    pub fn build(self) -> Result<PerformSearch, String> {
        Ok(PerformSearch {
            method: PerformSearchMethod::PerformSearch,
            params: PerformSearchParams {
                query: self
                    .query
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(query)))?,
                include_user_agent_shadow_dom: self.include_user_agent_shadow_dom,
            },
        })
    }
}
impl PushNodeByPathToFrontend {
    pub fn builder() -> PushNodeByPathToFrontendBuilder {
        PushNodeByPathToFrontendBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PushNodeByPathToFrontendBuilder {
    path: Option<String>,
}
impl PushNodeByPathToFrontendBuilder {
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
    pub fn build(self) -> Result<PushNodeByPathToFrontend, String> {
        Ok(PushNodeByPathToFrontend {
            method: PushNodeByPathToFrontendMethod::PushNodeByPathToFrontend,
            params: PushNodeByPathToFrontendParams {
                path: self
                    .path
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(path)))?,
            },
        })
    }
}
impl PushNodesByBackendIdsToFrontend {
    pub fn builder() -> PushNodesByBackendIdsToFrontendBuilder {
        PushNodesByBackendIdsToFrontendBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PushNodesByBackendIdsToFrontendBuilder {
    backend_node_ids: Option<Vec<super::types::BackendNodeId>>,
}
impl PushNodesByBackendIdsToFrontendBuilder {
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<super::types::BackendNodeId>,
    ) -> Self {
        let v = self.backend_node_ids.get_or_insert(Vec::new());
        v.push(backend_node_id.into());
        self
    }
    pub fn backend_node_ids<I, S>(mut self, backend_node_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::BackendNodeId>,
    {
        let v = self.backend_node_ids.get_or_insert(Vec::new());
        for val in backend_node_ids {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<PushNodesByBackendIdsToFrontend, String> {
        Ok(PushNodesByBackendIdsToFrontend {
            method: PushNodesByBackendIdsToFrontendMethod::PushNodesByBackendIdsToFrontend,
            params: PushNodesByBackendIdsToFrontendParams {
                backend_node_ids: self.backend_node_ids.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(backend_node_ids)
                    )
                })?,
            },
        })
    }
}
impl QuerySelector {
    pub fn builder() -> QuerySelectorBuilder {
        QuerySelectorBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct QuerySelectorBuilder {
    node_id: Option<super::types::NodeId>,
    selector: Option<String>,
}
impl QuerySelectorBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = Some(selector.into());
        self
    }
    pub fn build(self) -> Result<QuerySelector, String> {
        Ok(QuerySelector {
            method: QuerySelectorMethod::QuerySelector,
            params: QuerySelectorParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                selector: self.selector.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(selector))
                })?,
            },
        })
    }
}
impl QuerySelectorAll {
    pub fn builder() -> QuerySelectorAllBuilder {
        QuerySelectorAllBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct QuerySelectorAllBuilder {
    node_id: Option<super::types::NodeId>,
    selector: Option<String>,
}
impl QuerySelectorAllBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = Some(selector.into());
        self
    }
    pub fn build(self) -> Result<QuerySelectorAll, String> {
        Ok(QuerySelectorAll {
            method: QuerySelectorAllMethod::QuerySelectorAll,
            params: QuerySelectorAllParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                selector: self.selector.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(selector))
                })?,
            },
        })
    }
}
impl GetElementByRelation {
    pub fn builder() -> GetElementByRelationBuilder {
        GetElementByRelationBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetElementByRelationBuilder {
    node_id: Option<super::types::NodeId>,
    relation: Option<GetElementByRelationRelation>,
}
impl GetElementByRelationBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn relation(mut self, relation: impl Into<GetElementByRelationRelation>) -> Self {
        self.relation = Some(relation.into());
        self
    }
    pub fn build(self) -> Result<GetElementByRelation, String> {
        Ok(GetElementByRelation {
            method: GetElementByRelationMethod::GetElementByRelation,
            params: GetElementByRelationParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                relation: self.relation.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(relation))
                })?,
            },
        })
    }
}
impl RemoveAttribute {
    pub fn builder() -> RemoveAttributeBuilder {
        RemoveAttributeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveAttributeBuilder {
    node_id: Option<super::types::NodeId>,
    name: Option<String>,
}
impl RemoveAttributeBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn build(self) -> Result<RemoveAttribute, String> {
        Ok(RemoveAttribute {
            method: RemoveAttributeMethod::RemoveAttribute,
            params: RemoveAttributeParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                name: self
                    .name
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            },
        })
    }
}
impl RemoveNode {
    pub fn builder() -> RemoveNodeBuilder {
        RemoveNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveNodeBuilder {
    node_id: Option<super::types::NodeId>,
}
impl RemoveNodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<RemoveNode, String> {
        Ok(RemoveNode {
            method: RemoveNodeMethod::RemoveNode,
            params: RemoveNodeParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            },
        })
    }
}
impl RequestChildNodes {
    pub fn builder() -> RequestChildNodesBuilder {
        RequestChildNodesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct RequestChildNodesBuilder {
    node_id: Option<super::types::NodeId>,
    depth: Option<i64>,
    pierce: Option<bool>,
}
impl RequestChildNodesBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn depth(mut self, depth: impl Into<i64>) -> Self {
        self.depth = Some(depth.into());
        self
    }
    pub fn pierce(mut self, pierce: impl Into<bool>) -> Self {
        self.pierce = Some(pierce.into());
        self
    }
    pub fn build(self) -> Result<RequestChildNodes, String> {
        Ok(RequestChildNodes {
            method: RequestChildNodesMethod::RequestChildNodes,
            params: RequestChildNodesParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                depth: self.depth,
                pierce: self.pierce,
            },
        })
    }
}
impl RequestNode {
    pub fn builder() -> RequestNodeBuilder {
        RequestNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct RequestNodeBuilder {
    object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
}
impl RequestNodeBuilder {
    pub fn object_id(
        mut self,
        object_id: impl Into<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
    ) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn build(self) -> Result<RequestNode, String> {
        Ok(RequestNode {
            method: RequestNodeMethod::RequestNode,
            params: RequestNodeParams {
                object_id: self.object_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(object_id))
                })?,
            },
        })
    }
}
impl ResolveNode {
    pub fn builder() -> ResolveNodeBuilder {
        ResolveNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ResolveNodeBuilder {
    node_id: Option<super::types::NodeId>,
    backend_node_id: Option<super::types::BackendNodeId>,
    object_group: Option<String>,
    execution_context_id:
        Option<super::super::super::js_protocol::runtime::types::ExecutionContextId>,
}
impl ResolveNodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<super::types::BackendNodeId>,
    ) -> Self {
        self.backend_node_id = Some(backend_node_id.into());
        self
    }
    pub fn object_group(mut self, object_group: impl Into<String>) -> Self {
        self.object_group = Some(object_group.into());
        self
    }
    pub fn execution_context_id(
        mut self,
        execution_context_id: impl Into<
            super::super::super::js_protocol::runtime::types::ExecutionContextId,
        >,
    ) -> Self {
        self.execution_context_id = Some(execution_context_id.into());
        self
    }
    pub fn build(self) -> ResolveNode {
        ResolveNode {
            method: ResolveNodeMethod::ResolveNode,
            params: ResolveNodeParams {
                node_id: self.node_id,
                backend_node_id: self.backend_node_id,
                object_group: self.object_group,
                execution_context_id: self.execution_context_id,
            },
        }
    }
}
impl SetAttributeValue {
    pub fn builder() -> SetAttributeValueBuilder {
        SetAttributeValueBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetAttributeValueBuilder {
    node_id: Option<super::types::NodeId>,
    name: Option<String>,
    value: Option<String>,
}
impl SetAttributeValueBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
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
    pub fn build(self) -> Result<SetAttributeValue, String> {
        Ok(SetAttributeValue {
            method: SetAttributeValueMethod::SetAttributeValue,
            params: SetAttributeValueParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                name: self
                    .name
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
                value: self
                    .value
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            },
        })
    }
}
impl SetAttributesAsText {
    pub fn builder() -> SetAttributesAsTextBuilder {
        SetAttributesAsTextBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetAttributesAsTextBuilder {
    node_id: Option<super::types::NodeId>,
    text: Option<String>,
    name: Option<String>,
}
impl SetAttributesAsTextBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn build(self) -> Result<SetAttributesAsText, String> {
        Ok(SetAttributesAsText {
            method: SetAttributesAsTextMethod::SetAttributesAsText,
            params: SetAttributesAsTextParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                text: self
                    .text
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
                name: self.name,
            },
        })
    }
}
impl SetFileInputFiles {
    pub fn builder() -> SetFileInputFilesBuilder {
        SetFileInputFilesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetFileInputFilesBuilder {
    files: Option<Vec<String>>,
    node_id: Option<super::types::NodeId>,
    backend_node_id: Option<super::types::BackendNodeId>,
    object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
}
impl SetFileInputFilesBuilder {
    pub fn file(mut self, file: impl Into<String>) -> Self {
        let v = self.files.get_or_insert(Vec::new());
        v.push(file.into());
        self
    }
    pub fn files<I, S>(mut self, files: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.files.get_or_insert(Vec::new());
        for val in files {
            v.push(val.into());
        }
        self
    }
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<super::types::BackendNodeId>,
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
    pub fn build(self) -> Result<SetFileInputFiles, String> {
        Ok(SetFileInputFiles {
            method: SetFileInputFilesMethod::SetFileInputFiles,
            params: SetFileInputFilesParams {
                files: self
                    .files
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(files)))?,
                node_id: self.node_id,
                backend_node_id: self.backend_node_id,
                object_id: self.object_id,
            },
        })
    }
}
impl SetNodeStackTracesEnabled {
    pub fn builder() -> SetNodeStackTracesEnabledBuilder {
        SetNodeStackTracesEnabledBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetNodeStackTracesEnabledBuilder {
    enable: Option<bool>,
}
impl SetNodeStackTracesEnabledBuilder {
    pub fn enable(mut self, enable: impl Into<bool>) -> Self {
        self.enable = Some(enable.into());
        self
    }
    pub fn build(self) -> Result<SetNodeStackTracesEnabled, String> {
        Ok(SetNodeStackTracesEnabled {
            method: SetNodeStackTracesEnabledMethod::SetNodeStackTracesEnabled,
            params: SetNodeStackTracesEnabledParams {
                enable: self
                    .enable
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enable)))?,
            },
        })
    }
}
impl GetNodeStackTraces {
    pub fn builder() -> GetNodeStackTracesBuilder {
        GetNodeStackTracesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetNodeStackTracesBuilder {
    node_id: Option<super::types::NodeId>,
}
impl GetNodeStackTracesBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<GetNodeStackTraces, String> {
        Ok(GetNodeStackTraces {
            method: GetNodeStackTracesMethod::GetNodeStackTraces,
            params: GetNodeStackTracesParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            },
        })
    }
}
impl GetFileInfo {
    pub fn builder() -> GetFileInfoBuilder {
        GetFileInfoBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetFileInfoBuilder {
    object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
}
impl GetFileInfoBuilder {
    pub fn object_id(
        mut self,
        object_id: impl Into<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
    ) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn build(self) -> Result<GetFileInfo, String> {
        Ok(GetFileInfo {
            method: GetFileInfoMethod::GetFileInfo,
            params: GetFileInfoParams {
                object_id: self.object_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(object_id))
                })?,
            },
        })
    }
}
impl SetInspectedNode {
    pub fn builder() -> SetInspectedNodeBuilder {
        SetInspectedNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetInspectedNodeBuilder {
    node_id: Option<super::types::NodeId>,
}
impl SetInspectedNodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<SetInspectedNode, String> {
        Ok(SetInspectedNode {
            method: SetInspectedNodeMethod::SetInspectedNode,
            params: SetInspectedNodeParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            },
        })
    }
}
impl SetNodeName {
    pub fn builder() -> SetNodeNameBuilder {
        SetNodeNameBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetNodeNameBuilder {
    node_id: Option<super::types::NodeId>,
    name: Option<String>,
}
impl SetNodeNameBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn build(self) -> Result<SetNodeName, String> {
        Ok(SetNodeName {
            method: SetNodeNameMethod::SetNodeName,
            params: SetNodeNameParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                name: self
                    .name
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            },
        })
    }
}
impl SetNodeValue {
    pub fn builder() -> SetNodeValueBuilder {
        SetNodeValueBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetNodeValueBuilder {
    node_id: Option<super::types::NodeId>,
    value: Option<String>,
}
impl SetNodeValueBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<SetNodeValue, String> {
        Ok(SetNodeValue {
            method: SetNodeValueMethod::SetNodeValue,
            params: SetNodeValueParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                value: self
                    .value
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            },
        })
    }
}
impl SetOuterHtml {
    pub fn builder() -> SetOuterHtmlBuilder {
        SetOuterHtmlBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetOuterHtmlBuilder {
    node_id: Option<super::types::NodeId>,
    outer_html: Option<String>,
}
impl SetOuterHtmlBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn outer_html(mut self, outer_html: impl Into<String>) -> Self {
        self.outer_html = Some(outer_html.into());
        self
    }
    pub fn build(self) -> Result<SetOuterHtml, String> {
        Ok(SetOuterHtml {
            method: SetOuterHtmlMethod::SetOuterHtml,
            params: SetOuterHtmlParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                outer_html: self.outer_html.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(outer_html))
                })?,
            },
        })
    }
}
impl GetFrameOwner {
    pub fn builder() -> GetFrameOwnerBuilder {
        GetFrameOwnerBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetFrameOwnerBuilder {
    frame_id: Option<super::super::page::types::FrameId>,
}
impl GetFrameOwnerBuilder {
    pub fn frame_id(mut self, frame_id: impl Into<super::super::page::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn build(self) -> Result<GetFrameOwner, String> {
        Ok(GetFrameOwner {
            method: GetFrameOwnerMethod::GetFrameOwner,
            params: GetFrameOwnerParams {
                frame_id: self.frame_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(frame_id))
                })?,
            },
        })
    }
}
impl GetContainerForNode {
    pub fn builder() -> GetContainerForNodeBuilder {
        GetContainerForNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetContainerForNodeBuilder {
    node_id: Option<super::types::NodeId>,
    container_name: Option<String>,
    physical_axes: Option<super::types::PhysicalAxes>,
    logical_axes: Option<super::types::LogicalAxes>,
    queries_scroll_state: Option<bool>,
    queries_anchored: Option<bool>,
}
impl GetContainerForNodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn container_name(mut self, container_name: impl Into<String>) -> Self {
        self.container_name = Some(container_name.into());
        self
    }
    pub fn physical_axes(mut self, physical_axes: impl Into<super::types::PhysicalAxes>) -> Self {
        self.physical_axes = Some(physical_axes.into());
        self
    }
    pub fn logical_axes(mut self, logical_axes: impl Into<super::types::LogicalAxes>) -> Self {
        self.logical_axes = Some(logical_axes.into());
        self
    }
    pub fn queries_scroll_state(mut self, queries_scroll_state: impl Into<bool>) -> Self {
        self.queries_scroll_state = Some(queries_scroll_state.into());
        self
    }
    pub fn queries_anchored(mut self, queries_anchored: impl Into<bool>) -> Self {
        self.queries_anchored = Some(queries_anchored.into());
        self
    }
    pub fn build(self) -> Result<GetContainerForNode, String> {
        Ok(GetContainerForNode {
            method: GetContainerForNodeMethod::GetContainerForNode,
            params: GetContainerForNodeParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                container_name: self.container_name,
                physical_axes: self.physical_axes,
                logical_axes: self.logical_axes,
                queries_scroll_state: self.queries_scroll_state,
                queries_anchored: self.queries_anchored,
            },
        })
    }
}
impl GetQueryingDescendantsForContainer {
    pub fn builder() -> GetQueryingDescendantsForContainerBuilder {
        GetQueryingDescendantsForContainerBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetQueryingDescendantsForContainerBuilder {
    node_id: Option<super::types::NodeId>,
}
impl GetQueryingDescendantsForContainerBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<GetQueryingDescendantsForContainer, String> {
        Ok(GetQueryingDescendantsForContainer {
            method: GetQueryingDescendantsForContainerMethod::GetQueryingDescendantsForContainer,
            params: GetQueryingDescendantsForContainerParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            },
        })
    }
}
impl GetAnchorElement {
    pub fn builder() -> GetAnchorElementBuilder {
        GetAnchorElementBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetAnchorElementBuilder {
    node_id: Option<super::types::NodeId>,
    anchor_specifier: Option<String>,
}
impl GetAnchorElementBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn anchor_specifier(mut self, anchor_specifier: impl Into<String>) -> Self {
        self.anchor_specifier = Some(anchor_specifier.into());
        self
    }
    pub fn build(self) -> Result<GetAnchorElement, String> {
        Ok(GetAnchorElement {
            method: GetAnchorElementMethod::GetAnchorElement,
            params: GetAnchorElementParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                anchor_specifier: self.anchor_specifier,
            },
        })
    }
}
impl ForceShowPopover {
    pub fn builder() -> ForceShowPopoverBuilder {
        ForceShowPopoverBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ForceShowPopoverBuilder {
    node_id: Option<super::types::NodeId>,
    enable: Option<bool>,
}
impl ForceShowPopoverBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn enable(mut self, enable: impl Into<bool>) -> Self {
        self.enable = Some(enable.into());
        self
    }
    pub fn build(self) -> Result<ForceShowPopover, String> {
        Ok(ForceShowPopover {
            method: ForceShowPopoverMethod::ForceShowPopover,
            params: ForceShowPopoverParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                enable: self
                    .enable
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enable)))?,
            },
        })
    }
}
