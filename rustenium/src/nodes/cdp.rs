use crate::error::node::{
    CdpNodeActionError, CdpNodeInputError, CdpNodeScreenshotError, NodeActionError, NodeInputError,
    NodeMouseError, NodeScreenshotError,
};
use crate::input::{Mouse, MouseClickOptions, MouseMoveOptions};
use crate::nodes::NodePosition;
use crate::nodes::node::{NodeScreenShotOptions, NodeType};
use rustenium_bidi_definitions::browsing_context::types::BrowsingContext;
use rustenium_cdp_definitions::browser_protocol::dom::commands::{
    Focus, GetBoxModel, GetOuterHtml, RemoveNode, ResolveNode, ScrollIntoViewIfNeeded,
};
use rustenium_cdp_definitions::browser_protocol::dom::results::{
    GetBoxModelResult, GetOuterHtmlResult, ResolveNodeResult,
};
use rustenium_cdp_definitions::browser_protocol::dom::types::{
    BackendNodeId, Node as DomNode, NodeId,
};
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
        write!(
            f,
            "node_id: {:?} | type: {}",
            self.node_id(),
            self.node_name()
        )?;
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
    pub fn new(raw_node: DomNode, session: Arc<TokioMutex<CdpSession<T>>>) -> Self {
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

    pub async fn update_position(&mut self) -> Result<bool, NodeActionError> {
        let script = "function() {
    if (!this) { return null; }
    const rect = this.getBoundingClientRect();
    const scroll_x = window.pageXOffset || document.documentElement.scrollLeft;
    const scroll_y = window.pageYOffset || document.documentElement.scrollTop;
    return JSON.stringify({
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
        scroll_x: rect.x + scroll_x,
        scroll_y: rect.y + scroll_y
    });
}";
        let json = self.eval(script).await?;
        let position: Option<NodePosition> = serde_json::from_str(&json).ok();
        if let Some(pos) = position {
            self.position = Some(pos);
            return Ok(true);
        }
        Ok(false)
    }

    pub async fn get_position(&mut self) -> Result<Option<&NodePosition>, NodeActionError> {
        if self.position.is_none() {
            self.update_position().await?;
        }
        Ok(self.position.as_ref())
    }

    pub async fn scroll_into_view(&self) -> Result<(), NodeActionError> {
        let cmd = ScrollIntoViewIfNeeded::builder()
            .backend_node_id(self.backend_node_id())
            .build();
        self.send(cmd)
            .await
            .map(|_| ())
            .map_err(NodeActionError::from)
    }

    pub async fn is_visible(&self) -> Result<bool, NodeActionError> {
        let cmd = GetBoxModel::builder()
            .backend_node_id(self.backend_node_id())
            .build();
        let resp = self.send(cmd).await.map_err(NodeActionError::from)?;
        match GetBoxModelResult::try_from(resp.result) {
            Ok(r) => Ok(r.model.width > 0 && r.model.height > 0),
            Err(_) => Ok(false),
        }
    }

    pub async fn delete(&self) -> Result<(), NodeActionError> {
        let cmd = RemoveNode::builder()
            .node_id(self.node_id())
            .build()
            .unwrap();
        self.send(cmd)
            .await
            .map(|_| ())
            .map_err(NodeActionError::from)
    }

    pub async fn focus(&self) -> Result<(), NodeActionError> {
        let cmd = Focus::builder()
            .backend_node_id(self.backend_node_id())
            .build();
        self.send(cmd)
            .await
            .map(|_| ())
            .map_err(NodeActionError::from)
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
        let resolve_resp = self
            .send(resolve_cmd)
            .await
            .map_err(NodeActionError::from)?;
        let object_id = ResolveNodeResult::try_from(resolve_resp.result)
            .map_err(|error| NodeActionError::Cdp(CdpNodeActionError::Decode(error.to_string())))?
            .object
            .object_id
            .ok_or(NodeActionError::Cdp(CdpNodeActionError::MissingObjectId))?;

        let call_cmd = CallFunctionOn::builder()
            .function_declaration(fn_body)
            .object_id(object_id)
            .return_by_value(true)
            .build()
            .unwrap();

        let call_resp = self.send(call_cmd).await.map_err(NodeActionError::from)?;
        Ok(CallFunctionOnResult::try_from(call_resp.result)
            .map_err(|error| NodeActionError::Cdp(CdpNodeActionError::Decode(error.to_string())))?
            .result
            .value
            .and_then(|v| v.as_str().map(ToOwned::to_owned))
            .unwrap_or_default())
    }

    /// Returns the `innerText` of this node.
    pub async fn get_inner_text(&self) -> Result<String, NodeActionError> {
        self.eval("function(){return this.innerText}").await
    }

    /// Returns the `textContent` of this node.
    pub async fn get_text_content(&self) -> Result<String, NodeActionError> {
        self.eval("function(){return this.textContent}").await
    }

    // ── Screenshot ────────────────────────────────────────────────────────

    pub async fn screenshot(
        &mut self,
        options: NodeScreenShotOptions,
    ) -> Result<String, NodeScreenshotError> {
        self.get_position().await.ok();
        let pos = self
            .position
            .as_ref()
            .ok_or(NodeScreenshotError::Cdp(CdpNodeScreenshotError::NoPosition))?;
        let clip = Viewport {
            x: pos.x,
            y: pos.y,
            width: pos.width,
            height: pos.height,
            scale: 1.0,
        };
        let mut cmd = CaptureScreenshot::builder().clip(clip);
        if let Some(format) = options.cdp_format {
            cmd = cmd.format(format);
        }
        if let Some(quality) = options.quality {
            cmd = cmd.quality((quality.clamp(0.0, 1.0) * 100.0).round() as i64);
        }
        if let Some(from_surface) = options.from_surface {
            cmd = cmd.from_surface(from_surface);
        }
        if let Some(capture_beyond_viewport) = options.capture_beyond_viewport {
            cmd = cmd.capture_beyond_viewport(capture_beyond_viewport);
        }
        if let Some(optimize_for_speed) = options.optimize_for_speed {
            cmd = cmd.optimize_for_speed(optimize_for_speed);
        }
        let cmd = cmd.build();
        let resp = self.send(cmd).await.map_err(NodeScreenshotError::from)?;
        let result = CaptureScreenshotResult::try_from(resp.result).map_err(|error| {
            NodeScreenshotError::Cdp(CdpNodeScreenshotError::Decode(error.to_string()))
        })?;
        let screenshot = String::from(result.data);

        if let Some(path) = options.save_path {
            let path = std::path::PathBuf::from(path);
            let final_path = if path.is_dir() {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|duration| duration.as_millis())
                    .unwrap_or(0);
                path.join(format!("screenshot_{timestamp}.png"))
            } else {
                if let Some(parent) = path.parent()
                    && !parent.as_os_str().is_empty()
                    && !parent.exists()
                {
                    return Err(NodeScreenshotError::InvalidPath(format!(
                        "Parent directory does not exist: {}",
                        parent.display()
                    )));
                }
                path
            };

            use base64::{Engine as _, engine::general_purpose};
            let decoded = general_purpose::STANDARD
                .decode(&screenshot)
                .map_err(|error| NodeScreenshotError::Base64DecodeError(error.to_string()))?;
            std::fs::write(&final_path, decoded)
                .map_err(|error| NodeScreenshotError::FileWriteError(error.to_string()))?;

            Ok(final_path.to_string_lossy().to_string())
        } else {
            Ok(screenshot)
        }
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
            .map_err(NodeMouseError::from)?;
        self.get_position().await.ok();
        let pos = self
            .position
            .as_ref()
            .ok_or(NodeMouseError::InvalidPosition)?;
        let center = crate::input::Point {
            x: pos.x + pos.width / 2.0,
            y: pos.y + pos.height / 2.0,
        };
        mouse
            .move_to(center, &BrowsingContext::from(String::new()), options)
            .await
            .map_err(|error| NodeMouseError::Driver(error.to_string()))
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
            .map_err(|error| NodeMouseError::Driver(error.to_string()))
    }

    // ── Keyboard input ────────────────────────────────────────────────────

    /// Focus this node and insert `text` via `Input.insertText`.
    pub async fn type_text(&self, text: impl Into<String>) -> Result<(), NodeInputError> {
        self.focus().await.map_err(NodeInputError::from)?;
        let cmd = InsertText::builder().text(text).build().unwrap();
        self.send(cmd)
            .await
            .map(|_| ())
            .map_err(|error| NodeInputError::Cdp(CdpNodeInputError::Command(error)))
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
