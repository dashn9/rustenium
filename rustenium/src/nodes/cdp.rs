use crate::error::node::{NodeActionError, NodeInputError, NodeMouseError, NodeScreenshotError};
use crate::input::{Mouse, MouseClickOptions, MouseMoveOptions};
use crate::nodes::NodePosition;
use crate::nodes::node::NodeType;
use rustenium_bidi_definitions::browsing_context::types::BrowsingContext;
use rustenium_cdp_definitions::browser_protocol::dom::commands::{
    Focus, GetBoxModel, GetOuterHtml, RemoveNode, ResolveNode, ScrollIntoViewIfNeeded,
};
use rustenium_cdp_definitions::browser_protocol::dom::results::{
    GetBoxModelResult, GetOuterHtmlResult, ResolveNodeResult,
};
use rustenium_cdp_definitions::browser_protocol::dom::types::{BackendNodeId, Node as DomNode, NodeId};
use rustenium_cdp_definitions::browser_protocol::input::commands::InsertText;
use rustenium_cdp_definitions::browser_protocol::page::commands::CaptureScreenshot;
use rustenium_cdp_definitions::browser_protocol::page::results::CaptureScreenshotResult;
use rustenium_cdp_definitions::browser_protocol::page::types::Viewport;
use rustenium_cdp_definitions::js_protocol::runtime::commands::CallFunctionOn;
use rustenium_cdp_definitions::js_protocol::runtime::results::CallFunctionOnResult;
use rustenium_core::error::CdpCommandResultError;
use rustenium_core::session::CdpSession;
use rustenium_core::transport::ConnectionTransport;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

/// A DOM node fetched via CDP `DOM.describeNode`.
/// Preserves all CDP data and reconstructs the child tree.
#[derive(Clone)]
pub struct CdpNode<T: ConnectionTransport = rustenium_core::transport::WebsocketConnectionTransport>
{
    pub(crate) raw_node: DomNode,
    pub attributes: HashMap<String, String>,
    pub children: Vec<CdpNode<T>>,
    pub position: Option<NodePosition>,
    pub(crate) session: Arc<TokioMutex<CdpSession<T>>>,
}

impl<T: ConnectionTransport> std::fmt::Debug for CdpNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "node_id: {:?} | type: {}", self.node_id(), self.node_name())?;
        if !self.local_name().is_empty() {
            write!(f, " | local_name: {}", self.local_name())?;
        }
        if !self.node_value().is_empty() {
            write!(f, " | value: {}", self.node_value())?;
        }
        for (k, v) in &self.attributes {
            write!(f, "\n  {}: {}", k, v)?;
        }
        for child in &self.children {
            let s = format!("{:?}", child);
            for line in s.lines() {
                write!(f, "\n  {}", line)?;
            }
        }
        Ok(())
    }
}

impl<T: ConnectionTransport> CdpNode<T> {
    /// Build a `CdpNode` from a CDP `DomNode` and a shared CDP session.
    pub fn new(
        raw_node: DomNode,
        session: Arc<TokioMutex<CdpSession<T>>>,
    ) -> Self {
        let attributes = parse_flat_attributes(raw_node.attributes.clone().unwrap_or_default());
        let children = raw_node
            .children
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|c| CdpNode::new(c, session.clone()))
            .collect();

        CdpNode {
            raw_node,
            attributes,
            children,
            position: None,
            session,
        }
    }
}

impl<T: ConnectionTransport> CdpNode<T> {
    // ── Raw-node accessors ────────────────────────────────────────────────

    /// The CDP `nodeId` for this node.
    pub fn node_id(&self) -> NodeId {
        *self.raw_node.node_id.clone()
    }

    /// The CDP `backendNodeId` for this node.
    pub fn backend_node_id(&self) -> BackendNodeId {
        *self.raw_node.backend_node_id.clone()
    }

    /// The raw integer node type (1 = Element, 3 = Text, …).
    pub fn node_type_raw(&self) -> i64 {
        *self.raw_node.node_type
    }

    /// The `nodeName` as reported by CDP (e.g. `"DIV"`, `"#text"`).
    pub fn node_name(&self) -> &str {
        &self.raw_node.node_name
    }

    /// The `localName` (tag name in lowercase for elements, empty for others).
    pub fn local_name(&self) -> &str {
        &self.raw_node.local_name
    }

    /// The `nodeValue` (text content for text nodes, empty for elements).
    pub fn node_value(&self) -> &str {
        &self.raw_node.node_value
    }

    /// The `parentId` of this node, if any.
    pub fn parent_id(&self) -> Option<NodeId> {
        self.raw_node.parent_id
    }

    /// Number of child nodes as reported by CDP (may differ from the
    /// populated `children` vec if the tree was not fully fetched).
    pub fn child_node_count(&self) -> Option<i64> {
        self.raw_node.child_node_count
    }

    // ── CDP commands ──────────────────────────────────────────────────────

    async fn send<C>(
        &self,
        cmd: C,
    ) -> Result<rustenium_cdp_definitions::base::CommandResponse, CdpCommandResultError>
    where
        C: Into<rustenium_cdp_definitions::Command>,
    {
        self.session
            .lock()
            .await
            .send(cmd)
            .await
            .map_err(CdpCommandResultError::SessionSendError)
    }

    pub async fn get_position(&mut self) -> Option<&NodePosition> {
        if self.position.is_none() {
            let cmd = GetBoxModel::builder()
                .backend_node_id(self.backend_node_id())
                .build();
            if let Ok(resp) = self.send(cmd).await {
                if let Ok(result) = GetBoxModelResult::try_from(resp.result) {
                    let q = result.model.content.inner();
                    if q.len() >= 8 {
                        let xs = [q[0], q[2], q[4], q[6]];
                        let ys = [q[1], q[3], q[5], q[7]];
                        let x = xs.iter().copied().fold(f64::INFINITY, f64::min);
                        let y = ys.iter().copied().fold(f64::INFINITY, f64::min);
                        self.position = Some(NodePosition {
                            x,
                            y,
                            scroll_x: 0.0,
                            scroll_y: 0.0,
                            width: result.model.width as f64,
                            height: result.model.height as f64,
                        });
                    }
                }
            }
        }
        self.position.as_ref()
    }

    pub async fn scroll_into_view(&self) -> Result<(), NodeActionError> {
        let cmd = ScrollIntoViewIfNeeded::builder()
            .backend_node_id(self.backend_node_id())
            .build();
        self.send(cmd)
            .await
            .map(|_| ())
            .map_err(NodeActionError::Cdp)
    }

    pub async fn is_visible(&self) -> Result<bool, NodeActionError> {
        let cmd = GetBoxModel::builder()
            .backend_node_id(self.backend_node_id())
            .build();
        let resp = self.send(cmd).await.map_err(NodeActionError::Cdp)?;
        match GetBoxModelResult::try_from(resp.result) {
            Ok(r) => Ok(r.model.width > 0 && r.model.height > 0),
            Err(_) => Ok(false),
        }
    }

    pub async fn delete(&self) -> Result<(), NodeActionError> {
        let cmd = RemoveNode::builder()
            .node_id(self.node_id())
            .build()
            .map_err(NodeActionError::Other)?;
        self.send(cmd)
            .await
            .map(|_| ())
            .map_err(NodeActionError::Cdp)
    }

    pub async fn focus(&self) -> Result<(), NodeActionError> {
        let cmd = Focus::builder()
            .backend_node_id(self.backend_node_id())
            .build();
        self.send(cmd)
            .await
            .map(|_| ())
            .map_err(NodeActionError::Cdp)
    }

    // ── Convenience getters ───────────────────────────────────────────────

    pub fn get_local_name(&self) -> &str {
        self.local_name()
    }

    pub fn get_node_type(&self) -> Option<NodeType> {
        NodeType::from_u16(self.node_type_raw() as u16)
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(String::as_str)
    }

    pub fn get_attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    pub fn get_node_name(&self) -> &str {
        self.node_name()
    }

    pub fn get_node_value(&self) -> &str {
        self.node_value()
    }

    pub fn get_children(&self) -> &Vec<CdpNode<T>> {
        &self.children
    }

    pub fn is_element(&self) -> bool {
        self.node_type_raw() == 1
    }

    // ── HTML / text content ───────────────────────────────────────────────

    /// Returns the outer HTML of this node via `DOM.getOuterHTML`.
    pub async fn get_html(&self) -> String {
        let cmd = GetOuterHtml::builder()
            .backend_node_id(self.backend_node_id())
            .build();
        self.send(cmd)
            .await
            .ok()
            .and_then(|r| GetOuterHtmlResult::try_from(r.result).ok())
            .map(|r| r.outer_html)
            .unwrap_or_default()
    }

    /// Resolve this node to a remote JS object, then evaluate `fn_body` on it.
    async fn eval(&self, fn_body: &str) -> Result<String, NodeActionError> {
        let resolve_cmd = ResolveNode::builder()
            .backend_node_id(self.backend_node_id())
            .build();
        let resolve_resp = self.send(resolve_cmd).await.map_err(NodeActionError::Cdp)?;
        let object_id = ResolveNodeResult::try_from(resolve_resp.result)
            .map_err(|e| NodeActionError::Other(e.to_string()))?
            .object
            .object_id
            .ok_or_else(|| NodeActionError::Other("no object_id from resolveNode".into()))?;

        let call_cmd = CallFunctionOn::builder()
            .function_declaration(fn_body)
            .object_id(object_id)
            .return_by_value(true)
            .build().unwrap();

        let call_resp = self.send(call_cmd).await.map_err(NodeActionError::Cdp)?;
        Ok(CallFunctionOnResult::try_from(call_resp.result)
            .map_err(|e| NodeActionError::Other(e.to_string()))?
            .result
            .value
            .and_then(|v| v.as_str().map(ToOwned::to_owned))
            .unwrap_or_default())
    }

    /// Returns the `innerText` of this node.
    pub async fn get_inner_text(&self) -> Result<String, NodeActionError> {
        self.eval("function(){return this.innerText}")
            .await
    }

    /// Returns the `textContent` of this node.
    pub async fn get_text_content(&self) -> Result<String, NodeActionError> {
        self.eval("function(){return this.textContent}")
            .await
    }

    // ── Screenshot ────────────────────────────────────────────────────────

    /// Captures a screenshot of this node's bounding box, returning base64 PNG.
    pub async fn screenshot(&mut self) -> Result<String, NodeScreenshotError> {
        self.get_position().await;
        let pos = self
            .position
            .as_ref()
            .ok_or_else(|| NodeScreenshotError::Other("could not determine node position".into()))?;
        let clip = Viewport { x: pos.x, y: pos.y, width: pos.width, height: pos.height, scale: 1.0 };
        let cmd = CaptureScreenshot::builder().clip(clip).build();
        let resp = self
            .send(cmd)
            .await
            .map_err(|e| NodeScreenshotError::Other(e.to_string()))?;
        let result = CaptureScreenshotResult::try_from(resp.result)
            .map_err(|e| NodeScreenshotError::Other(e.to_string()))?;
        Ok(String::from(result.data))
    }

    // ── Mouse input ───────────────────────────────────────────────────────

    /// Move the mouse to the centre of this node using the provided mouse driver.
    pub async fn mouse_move<M: Mouse>(
        &mut self,
        mouse: &M,
        options: MouseMoveOptions,
    ) -> Result<(), NodeMouseError> {
        self.scroll_into_view()
            .await
            .map_err(|e| NodeMouseError::Other(e.to_string()))?;
        self.get_position().await;
        let pos = self.position.as_ref().ok_or(NodeMouseError::InvalidPosition)?;
        let center = crate::input::Point {
            x: pos.x + pos.width / 2.0,
            y: pos.y + pos.height / 2.0,
        };
        mouse
            .move_to(center, &BrowsingContext::from(String::new()), options)
            .await
            .map_err(|e| NodeMouseError::Other(e.to_string()))
    }

    /// Click the centre of this node using the provided mouse driver.
    pub async fn mouse_click<M: Mouse>(
        &mut self,
        mouse: &M,
        options: MouseClickOptions,
    ) -> Result<(), NodeMouseError> {
        self.mouse_move(mouse, MouseMoveOptions::default()).await?;
        mouse
            .click(None, &BrowsingContext::from(String::new()), options)
            .await
            .map_err(|e| NodeMouseError::Other(e.to_string()))
    }

    // ── Keyboard input ────────────────────────────────────────────────────

    /// Focus this node and insert `text` via `Input.insertText`.
    pub async fn type_text(&self, text: impl Into<String>) -> Result<(), NodeInputError> {
        self.focus()
            .await
            .map_err(|e| NodeInputError::Other(e.to_string()))?;
        let cmd = InsertText::builder()
            .text(text)
            .build()
            .map_err(NodeInputError::CdpBuild)?;
        self.send(cmd)
            .await
            .map(|_| ())
            .map_err(NodeInputError::Cdp)
    }
}

/// CDP attributes come as a flat `[name, value, name, value, ...]` array.
fn parse_flat_attributes(flat: Vec<String>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut iter = flat.into_iter();
    while let (Some(k), Some(v)) = (iter.next(), iter.next()) {
        map.insert(k, v);
    }
    map
}
