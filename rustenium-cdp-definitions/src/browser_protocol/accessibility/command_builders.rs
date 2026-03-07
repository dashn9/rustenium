use super::commands::*;
impl GetPartialAxTree {
    pub fn builder() -> GetPartialAxTreeBuilder {
        <GetPartialAxTreeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetPartialAxTreeBuilder {
    node_id: Option<crate::browser_protocol::dom::types::NodeId>,
    backend_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
    object_id: Option<crate::js_protocol::runtime::types::RemoteObjectId>,
    fetch_relatives: Option<bool>,
}
impl GetPartialAxTreeBuilder {
    pub fn node_id(
        mut self,
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
    ) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<crate::browser_protocol::dom::types::BackendNodeId>,
    ) -> Self {
        self.backend_node_id = Some(backend_node_id.into());
        self
    }
    pub fn object_id(
        mut self,
        object_id: impl Into<crate::js_protocol::runtime::types::RemoteObjectId>,
    ) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn fetch_relatives(mut self, fetch_relatives: impl Into<bool>) -> Self {
        self.fetch_relatives = Some(fetch_relatives.into());
        self
    }
    pub fn build(self) -> GetPartialAxTree {
        GetPartialAxTree {
            method: GetPartialAxTreeMethod::GetPartialAxTree,
            params: GetPartialAxTreeParams {
                node_id: self.node_id,
                backend_node_id: self.backend_node_id,
                object_id: self.object_id,
                fetch_relatives: self.fetch_relatives,
            },
        }
    }
}
impl GetFullAxTree {
    pub fn builder() -> GetFullAxTreeBuilder {
        <GetFullAxTreeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetFullAxTreeBuilder {
    depth: Option<i64>,
    frame_id: Option<crate::browser_protocol::page::types::FrameId>,
}
impl GetFullAxTreeBuilder {
    pub fn depth(mut self, depth: impl Into<i64>) -> Self {
        self.depth = Some(depth.into());
        self
    }
    pub fn frame_id(
        mut self,
        frame_id: impl Into<crate::browser_protocol::page::types::FrameId>,
    ) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn build(self) -> GetFullAxTree {
        GetFullAxTree {
            method: GetFullAxTreeMethod::GetFullAxTree,
            params: GetFullAxTreeParams {
                depth: self.depth,
                frame_id: self.frame_id,
            },
        }
    }
}
impl GetRootAxNode {
    pub fn builder() -> GetRootAxNodeBuilder {
        <GetRootAxNodeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetRootAxNodeBuilder {
    frame_id: Option<crate::browser_protocol::page::types::FrameId>,
}
impl GetRootAxNodeBuilder {
    pub fn frame_id(
        mut self,
        frame_id: impl Into<crate::browser_protocol::page::types::FrameId>,
    ) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn build(self) -> GetRootAxNode {
        GetRootAxNode {
            method: GetRootAxNodeMethod::GetRootAxNode,
            params: GetRootAxNodeParams {
                frame_id: self.frame_id,
            },
        }
    }
}
impl GetAxNodeAndAncestors {
    pub fn builder() -> GetAxNodeAndAncestorsBuilder {
        <GetAxNodeAndAncestorsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetAxNodeAndAncestorsBuilder {
    node_id: Option<crate::browser_protocol::dom::types::NodeId>,
    backend_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
    object_id: Option<crate::js_protocol::runtime::types::RemoteObjectId>,
}
impl GetAxNodeAndAncestorsBuilder {
    pub fn node_id(
        mut self,
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
    ) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<crate::browser_protocol::dom::types::BackendNodeId>,
    ) -> Self {
        self.backend_node_id = Some(backend_node_id.into());
        self
    }
    pub fn object_id(
        mut self,
        object_id: impl Into<crate::js_protocol::runtime::types::RemoteObjectId>,
    ) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn build(self) -> GetAxNodeAndAncestors {
        GetAxNodeAndAncestors {
            method: GetAxNodeAndAncestorsMethod::GetAxNodeAndAncestors,
            params: GetAxNodeAndAncestorsParams {
                node_id: self.node_id,
                backend_node_id: self.backend_node_id,
                object_id: self.object_id,
            },
        }
    }
}
impl GetChildAxNodes {
    pub fn builder() -> GetChildAxNodesBuilder {
        <GetChildAxNodesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetChildAxNodesBuilder {
    id: Option<super::types::AxNodeId>,
    frame_id: Option<crate::browser_protocol::page::types::FrameId>,
}
impl GetChildAxNodesBuilder {
    pub fn id(mut self, id: impl Into<super::types::AxNodeId>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn frame_id(
        mut self,
        frame_id: impl Into<crate::browser_protocol::page::types::FrameId>,
    ) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn build(self) -> Result<GetChildAxNodes, String> {
        Ok(GetChildAxNodes {
            method: GetChildAxNodesMethod::GetChildAxNodes,
            params: GetChildAxNodesParams {
                id: self
                    .id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
                frame_id: self.frame_id,
            },
        })
    }
}
impl QueryAxTree {
    pub fn builder() -> QueryAxTreeBuilder {
        <QueryAxTreeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct QueryAxTreeBuilder {
    node_id: Option<crate::browser_protocol::dom::types::NodeId>,
    backend_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
    object_id: Option<crate::js_protocol::runtime::types::RemoteObjectId>,
    accessible_name: Option<String>,
    role: Option<String>,
}
impl QueryAxTreeBuilder {
    pub fn node_id(
        mut self,
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
    ) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<crate::browser_protocol::dom::types::BackendNodeId>,
    ) -> Self {
        self.backend_node_id = Some(backend_node_id.into());
        self
    }
    pub fn object_id(
        mut self,
        object_id: impl Into<crate::js_protocol::runtime::types::RemoteObjectId>,
    ) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn accessible_name(mut self, accessible_name: impl Into<String>) -> Self {
        self.accessible_name = Some(accessible_name.into());
        self
    }
    pub fn role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }
    pub fn build(self) -> QueryAxTree {
        QueryAxTree {
            method: QueryAxTreeMethod::QueryAxTree,
            params: QueryAxTreeParams {
                node_id: self.node_id,
                backend_node_id: self.backend_node_id,
                object_id: self.object_id,
                accessible_name: self.accessible_name,
                role: self.role,
            },
        }
    }
}
