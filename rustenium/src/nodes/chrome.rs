use rustenium_cdp_definitions::browser_protocol::accessibility::types::{
    AxNode as CdpAxNode, AxProperty, AxValue,
};
use rustenium_cdp_definitions::browser_protocol::dom::types::{Node as DomNode, NodeId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use rustenium_bidi_definitions::browsing_context::types::{BrowsingContext, Locator};
use rustenium_bidi_definitions::script::types::{Handle, NodeRemoteValue, SharedId};
use rustenium_core::transport::ConnectionTransport;
use rustenium_core::{BidiSession, CdpSession};

use crate::error::node::{NodeActionError, NodeInputError, NodeMouseError, NodeScreenshotError};
use crate::input::{BidiKeyboard, BidiMouse, Keyboard, Mouse, MouseClickOptions, MouseMoveOptions};
use crate::nodes::NodePosition;
use crate::nodes::bidi::node::BidiNode;
use crate::nodes::cdp::CdpNode;
use crate::nodes::node::{Node, NodeScreenShotOptions, NodeType};

// Bidi and Cdp variants are intentionally near-equal in size — both hold a node
// plus the same Arc<M>/Arc<K> handles — so boxing one wouldn't measurably
// shrink the enum and would add a heap indirection on every node access.
#[allow(clippy::large_enum_variant)]
enum ChromeNodeInner<T: ConnectionTransport, M: Mouse + Send + Sync, K: Keyboard + Send + Sync> {
    Bidi {
        node: BidiNode<T>,
        mouse: Arc<M>,
        keyboard: Arc<K>,
    },
    Cdp {
        node: CdpNode<T>,
        mouse: Arc<M>,
        #[allow(dead_code)]
        keyboard: Arc<K>,
    },
}

pub struct ChromeNode<
    T: ConnectionTransport,
    M: Mouse + Send + Sync = BidiMouse<T>,
    K: Keyboard + Send + Sync = BidiKeyboard<T>,
> {
    inner: ChromeNodeInner<T, M, K>,
    children: Vec<ChromeNode<T, M, K>>,
}

impl<T: ConnectionTransport, M: Mouse + Send + Sync, K: Keyboard + Send + Sync> std::fmt::Debug
    for ChromeNode<T, M, K>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            ChromeNodeInner::Bidi { node, .. } => node.fmt(f),
            ChromeNodeInner::Cdp { node: n, .. } => n.fmt(f),
        }
    }
}

impl<T: ConnectionTransport, M: Mouse + Send + Sync + 'static, K: Keyboard + Send + Sync + 'static>
    ChromeNode<T, M, K>
{
    pub fn from_bidi(
        raw_bidi_node: NodeRemoteValue,
        locator: Locator,
        session: Arc<Mutex<BidiSession<T>>>,
        context: BrowsingContext,
        mouse: Arc<M>,
        keyboard: Arc<K>,
    ) -> Self {
        let bidi_node = BidiNode::new(
            raw_bidi_node,
            locator.clone(),
            session.clone(),
            context.clone(),
        );

        let children = bidi_node
            .children
            .iter()
            .map(|bidi_child| {
                ChromeNode::from_bidi(
                    bidi_child.get_raw_node_ref().clone(),
                    locator.clone(),
                    session.clone(),
                    context.clone(),
                    mouse.clone(),
                    keyboard.clone(),
                )
            })
            .collect();

        Self {
            inner: ChromeNodeInner::Bidi {
                node: bidi_node,
                mouse,
                keyboard,
            },
            children,
        }
    }

    pub fn from_cdp(
        raw_cdp_node: DomNode,
        session: Arc<Mutex<CdpSession<T>>>,
        mouse: Arc<M>,
        keyboard: Arc<K>,
    ) -> Self {
        let cdp_node = CdpNode::new(raw_cdp_node, session.clone());
        let children = cdp_node
            .children
            .iter()
            .map(|c| {
                ChromeNode::from_cdp(
                    c.raw_node.clone(),
                    session.clone(),
                    mouse.clone(),
                    keyboard.clone(),
                )
            })
            .collect();
        Self {
            inner: ChromeNodeInner::Cdp {
                node: cdp_node,
                mouse,
                keyboard,
            },
            children,
        }
    }

    pub fn node_id(&self) -> NodeId {
        match &self.inner {
            ChromeNodeInner::Bidi { .. } => {
                unimplemented!("get_node_id not available for BiDi-sourced nodes")
            }
            ChromeNodeInner::Cdp { node, .. } => *node.raw_node.node_id.clone(),
        }
    }
}

impl<T: ConnectionTransport, M: Mouse + Send + Sync + 'static, K: Keyboard + Send + Sync + 'static>
    Node for ChromeNode<T, M, K>
{
    #[allow(refining_impl_trait)]
    fn get_children_nodes(&self) -> &Vec<ChromeNode<T, M, K>> {
        &self.children
    }

    fn get_bidi_locator(&self) -> &Locator {
        match &self.inner {
            ChromeNodeInner::Bidi { node, .. } => &node.locator,
            ChromeNodeInner::Cdp { .. } => {
                unimplemented!("get_bidi_locator not available for CDP-sourced nodes")
            }
        }
    }

    fn get_local_name(&self) -> Option<&str> {
        match &self.inner {
            ChromeNodeInner::Bidi { node, .. } => node.get_local_name(),
            ChromeNodeInner::Cdp { node: n, .. } => Some(n.get_local_name()),
        }
    }

    fn get_node_type(&self) -> Option<NodeType> {
        match &self.inner {
            ChromeNodeInner::Bidi { node, .. } => node.get_node_type(),
            ChromeNodeInner::Cdp { node: n, .. } => n.get_node_type(),
        }
    }

    async fn get_inner_text(&self) -> String {
        match &self.inner {
            ChromeNodeInner::Bidi { node, .. } => node.get_inner_text().await.unwrap(),
            ChromeNodeInner::Cdp { node: n, .. } => {
                n.get_inner_text().await.unwrap_or(String::new())
            }
        }
    }

    async fn get_text_content(&self) -> String {
        match &self.inner {
            ChromeNodeInner::Bidi { node, .. } => node.get_text_content().await.unwrap(),
            ChromeNodeInner::Cdp { node: n, .. } => {
                n.get_text_content().await.unwrap_or(String::new())
            }
        }
    }

    async fn get_html(&self) -> String {
        match &self.inner {
            ChromeNodeInner::Bidi { node, .. } => node.get_inner_html().await.unwrap(),
            ChromeNodeInner::Cdp { node: n, .. } => n.get_html().await,
        }
    }

    fn get_attribute(&self, attribute_name: &str) -> Option<serde_json::Value> {
        match &self.inner {
            ChromeNodeInner::Bidi { node, .. } => node.get_attribute(attribute_name),
            ChromeNodeInner::Cdp { node: n, .. } => n
                .get_attribute(attribute_name)
                .map(|v| serde_json::Value::String(v.to_string())),
        }
    }

    fn get_attributes(&self) -> HashMap<String, serde_json::Value> {
        match &self.inner {
            ChromeNodeInner::Bidi { node, .. } => node.get_attributes(),
            ChromeNodeInner::Cdp { node: n, .. } => n
                .get_attributes()
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect(),
        }
    }

    async fn get_position(&mut self) -> Option<&NodePosition> {
        match &mut self.inner {
            ChromeNodeInner::Bidi { node, .. } => node.get_position().await.unwrap_or(None),
            ChromeNodeInner::Cdp { node, .. } => node.get_position().await.unwrap_or(None),
        }
    }

    fn get_context_id(&self) -> &BrowsingContext {
        match &self.inner {
            ChromeNodeInner::Bidi { node, .. } => &node.context_id,
            ChromeNodeInner::Cdp { .. } => {
                unimplemented!("get_context_id not available for CDP-sourced nodes")
            }
        }
    }

    fn get_shared_id(&self) -> Option<&SharedId> {
        match &self.inner {
            ChromeNodeInner::Bidi { node, .. } => node.get_raw_node_ref().shared_id.as_ref(),
            ChromeNodeInner::Cdp { .. } => {
                unimplemented!("get_shared_id not available for CDP-sourced nodes")
            }
        }
    }

    fn get_handle(&self) -> &Option<Handle> {
        match &self.inner {
            ChromeNodeInner::Bidi { node, .. } => &node.get_raw_node_ref().handle,
            ChromeNodeInner::Cdp { .. } => {
                unimplemented!("get_handle not available for CDP-sourced nodes")
            }
        }
    }

    fn set_position(&mut self, position: NodePosition) {
        match &mut self.inner {
            ChromeNodeInner::Bidi { node, .. } => node.position = Some(position),
            ChromeNodeInner::Cdp { .. } => {
                unimplemented!("set_position not available for CDP-sourced nodes")
            }
        }
    }

    async fn scroll_into_view(&self) -> Result<(), NodeActionError> {
        match &self.inner {
            ChromeNodeInner::Bidi { node, .. } => {
                node.scroll_into_view().await.map_err(NodeActionError::from)
            }
            ChromeNodeInner::Cdp { node, .. } => node.scroll_into_view().await,
        }
    }

    async fn is_visible(&self) -> Result<bool, NodeActionError> {
        match &self.inner {
            ChromeNodeInner::Bidi { node, .. } => {
                node.is_visible().await.map_err(NodeActionError::from)
            }
            ChromeNodeInner::Cdp { .. } => {
                unimplemented!("is_visible not available for CDP-sourced nodes")
            }
        }
    }

    async fn delete(&self) -> Result<(), NodeActionError> {
        match &self.inner {
            ChromeNodeInner::Bidi { node, .. } => {
                node.delete().await.map_err(NodeActionError::from)
            }
            ChromeNodeInner::Cdp { .. } => {
                unimplemented!("delete not available for CDP-sourced nodes")
            }
        }
    }

    async fn mouse_move(&mut self) -> Result<(), NodeMouseError> {
        match &mut self.inner {
            ChromeNodeInner::Bidi { node, mouse, .. } => {
                let mouse = mouse.clone();
                node.mouse_move(mouse.as_ref(), MouseMoveOptions::default())
                    .await
                    .map_err(NodeMouseError::from)
            }
            ChromeNodeInner::Cdp { node, mouse, .. } => {
                let mouse = mouse.clone();
                node.mouse_move(mouse.as_ref(), MouseMoveOptions::default())
                    .await
            }
        }
    }

    async fn mouse_move_with_options(
        &mut self,
        options: MouseMoveOptions,
    ) -> Result<(), NodeMouseError> {
        match &mut self.inner {
            ChromeNodeInner::Bidi { node, mouse, .. } => {
                let mouse = mouse.clone();
                node.mouse_move(mouse.as_ref(), options)
                    .await
                    .map_err(NodeMouseError::from)
            }
            ChromeNodeInner::Cdp { node, mouse, .. } => {
                let mouse = mouse.clone();
                node.mouse_move(mouse.as_ref(), options).await
            }
        }
    }

    async fn mouse_click(&mut self) -> Result<(), NodeMouseError> {
        match &mut self.inner {
            ChromeNodeInner::Bidi { node, mouse, .. } => {
                let mouse = mouse.clone();
                node.mouse_click(mouse.as_ref(), MouseClickOptions::default())
                    .await
                    .map_err(NodeMouseError::from)
            }
            ChromeNodeInner::Cdp { node, mouse, .. } => {
                let mouse = mouse.clone();
                node.mouse_click(mouse.as_ref(), MouseClickOptions::default())
                    .await
            }
        }
    }

    async fn mouse_click_with_options(
        &mut self,
        options: MouseClickOptions,
    ) -> Result<(), NodeMouseError> {
        match &mut self.inner {
            ChromeNodeInner::Bidi { node, mouse, .. } => {
                let mouse = mouse.clone();
                node.mouse_click(mouse.as_ref(), options)
                    .await
                    .map_err(NodeMouseError::from)
            }
            ChromeNodeInner::Cdp { node, mouse, .. } => {
                let mouse = mouse.clone();
                node.mouse_click(mouse.as_ref(), options).await
            }
        }
    }

    async fn screenshot(&mut self) -> Result<String, NodeScreenshotError> {
        match &mut self.inner {
            ChromeNodeInner::Bidi { node, .. } => node
                .screenshot(NodeScreenShotOptions::default())
                .await
                .map_err(NodeScreenshotError::from),
            ChromeNodeInner::Cdp { node, .. } => {
                node.screenshot(NodeScreenShotOptions::default()).await
            }
        }
    }

    async fn screenshot_with_options(
        &mut self,
        options: NodeScreenShotOptions,
    ) -> Result<String, NodeScreenshotError> {
        match &mut self.inner {
            ChromeNodeInner::Bidi { node, .. } => node
                .screenshot(options)
                .await
                .map_err(NodeScreenshotError::from),
            ChromeNodeInner::Cdp { node, .. } => {
                let screenshot = node.screenshot(options).await?;
                Ok(screenshot)
            }
        }
    }

    async fn type_text(&mut self, text: String) -> Result<(), NodeInputError> {
        match &mut self.inner {
            ChromeNodeInner::Bidi { node, keyboard, .. } => {
                let keyboard = keyboard.clone();
                node.type_text(keyboard.as_ref(), text)
                    .await
                    .map_err(NodeInputError::from)
            }
            ChromeNodeInner::Cdp { .. } => {
                unimplemented!("type_text not available for CDP-sourced nodes")
            }
        }
    }
}

/// A node in the browser's accessibility tree, built from CDP `Accessibility.getFullAXTree`.
/// All raw CDP data is preserved; convenience methods expose the most common values directly.
#[derive(Clone, PartialEq)]
pub struct AXNode {
    pub node_id: String,
    pub ignored: bool,
    /// Why this node is ignored (populated only when `ignored` is true).
    pub ignored_reasons: Vec<AxProperty>,
    /// Computed ARIA role (e.g. `"button"`, `"heading"`).
    pub role: Option<AxValue>,
    /// Chrome's internal role (may differ from the ARIA role).
    pub chrome_role: Option<AxValue>,
    /// Accessible name.
    pub name: Option<AxValue>,
    /// Accessible description.
    pub description: Option<AxValue>,
    /// Current value (for inputs, sliders, etc.).
    pub value: Option<AxValue>,
    /// All other ARIA/state properties (checked, expanded, disabled, …).
    pub properties: Vec<AxProperty>,
    pub parent_id: Option<String>,
    /// Reconstructed children — populated by `AXNode::build_tree`.
    pub children: Vec<AXNode>,
    pub backend_dom_node_id: Option<i64>,
    pub frame_id: Option<String>,
}

impl std::fmt::Debug for AXNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "node_id: {}", self.node_id)?;
        if let Some(role) = self.role_str() {
            write!(f, " | role: {}", role)?;
        }
        if let Some(chrome_role) = self
            .chrome_role
            .as_ref()
            .and_then(|v| v.value.as_ref())
            .and_then(|v| v.as_str())
        {
            write!(f, " | chrome_role: {}", chrome_role)?;
        }
        if let Some(name) = self.name_str() {
            write!(f, " | name: {}", name)?;
        }
        if let Some(desc) = self.description_str() {
            write!(f, " | description: {}", desc)?;
        }
        if let Some(val) = self.value_str() {
            write!(f, " | value: {}", val)?;
        }
        if let Some(id) = self.backend_dom_node_id {
            write!(f, " | backend_node_id: {}", id)?;
        }
        if let Some(ref fid) = self.frame_id {
            write!(f, " | frame_id: {}", fid)?;
        }
        if let Some(ref pid) = self.parent_id {
            write!(f, " | parent_id: {}", pid)?;
        }
        if self.ignored {
            write!(f, " | ignored")?;
            for r in &self.ignored_reasons {
                let key = format!("{:?}", r.name).to_lowercase();
                let val = r
                    .value
                    .value
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or_default();
                write!(f, "\n  ignored_reason {}: {}", key, val)?;
            }
        }
        for p in &self.properties {
            let key = format!("{:?}", p.name).to_lowercase();
            let val = p
                .value
                .value
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default();
            write!(f, "\n  {}: {}", key, val)?;
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

impl AXNode {
    pub fn role_str(&self) -> Option<&str> {
        self.role.as_ref()?.value.as_ref()?.as_str()
    }

    pub fn name_str(&self) -> Option<&str> {
        self.name.as_ref()?.value.as_ref()?.as_str()
    }

    pub fn description_str(&self) -> Option<&str> {
        self.description.as_ref()?.value.as_ref()?.as_str()
    }

    pub fn value_str(&self) -> Option<&str> {
        self.value.as_ref()?.value.as_ref()?.as_str()
    }

    pub fn is_ignored(&self) -> bool {
        self.ignored
    }

    /// Find a property by its raw CDP name (e.g. `"checked"`, `"expanded"`).
    pub fn get_property(&self, name: &str) -> Option<&AxValue> {
        self.properties
            .iter()
            .find_map(|p| (format!("{:?}", p.name).to_lowercase() == name).then_some(&p.value))
    }

    fn is_empty_container(&self) -> bool {
        let blank = |s: Option<&str>| s.is_none_or(|v| v.trim().is_empty());
        blank(self.name_str())
            && blank(self.value_str())
            && blank(self.description_str())
            && self
                .role_str()
                .is_none_or(|r| matches!(r, "none" | "generic" | "group"))
    }

    /// Build a proper tree from the flat list returned by CDP `getFullAXTree`.
    /// If `squash` is true, empty container nodes (no name/value/description, generic role)
    /// are removed in the same pass and their children are promoted in place — O(n) total.
    pub fn build_tree(flat: Vec<CdpAxNode>, squash: bool) -> Vec<AXNode> {
        use std::collections::{HashMap, HashSet};

        let child_map: HashMap<String, Vec<String>> = flat
            .iter()
            .map(|n| {
                let id: String = n.node_id.clone().into();
                let children = n
                    .child_ids
                    .clone()
                    .unwrap_or_default()
                    .into_iter()
                    .map(Into::into)
                    .collect();
                (id, children)
            })
            .collect();

        let child_set: HashSet<&String> = child_map.values().flatten().collect();

        let mut by_id: HashMap<String, AXNode> = flat
            .into_iter()
            .map(|n| {
                let id = n.node_id.clone().into();
                (id, AXNode::from_flat(n))
            })
            .collect();

        // Squashes redundant empty ancestors: an empty node is removed only when its
        // parent is also empty, so at most one empty container survives per chain.
        fn attach(
            id: &str,
            parent_id: Option<&str>,
            by_id: &mut HashMap<String, AXNode>,
            child_map: &HashMap<String, Vec<String>>,
            squash: bool,
            parent_is_empty: bool,
        ) -> Vec<AXNode> {
            let child_ids = child_map.get(id).cloned().unwrap_or_default();

            // Peek at whether this node is empty before recursing so children know.
            let this_is_empty = squash && by_id.get(id).is_some_and(|n| n.is_empty_container());

            let children: Vec<AXNode> = child_ids
                .iter()
                .flat_map(|cid| attach(cid, Some(id), by_id, child_map, squash, this_is_empty))
                .collect();

            if let Some(mut node) = by_id.remove(id) {
                node.parent_id = parent_id.map(str::to_string);
                if squash && node.is_empty_container() && parent_is_empty {
                    let grandparent = parent_id.map(str::to_string);
                    return children
                        .into_iter()
                        .map(|mut c| {
                            c.parent_id = grandparent.clone();
                            c
                        })
                        .collect();
                }
                node.children = children;
                vec![node]
            } else {
                children
            }
        }

        let roots: Vec<String> = child_map
            .keys()
            .filter(|id| !child_set.contains(id))
            .cloned()
            .collect();

        roots
            .iter()
            .flat_map(|root| attach(root, None, &mut by_id, &child_map, squash, false))
            .collect()
    }

    fn from_flat(n: CdpAxNode) -> Self {
        AXNode {
            node_id: n.node_id.into(),
            ignored: n.ignored,
            ignored_reasons: n.ignored_reasons.unwrap_or_default(),
            role: n.role,
            chrome_role: n.chrome_role,
            name: n.name,
            description: n.description,
            value: n.value,
            properties: n.properties.unwrap_or_default(),
            parent_id: n.parent_id.map(Into::into),
            children: vec![],
            backend_dom_node_id: n.backend_dom_node_id.map(|id| *id.inner()),
            frame_id: n.frame_id.map(|id| id.inner().clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustenium_cdp_definitions::browser_protocol::accessibility::types::{
        AxNodeId, AxValue, AxValueType,
    };

    fn make_node(
        id: &str,
        role: Option<&str>,
        name: Option<&str>,
        children: Vec<&str>,
    ) -> CdpAxNode {
        CdpAxNode {
            node_id: AxNodeId::new(id),
            ignored: false,
            ignored_reasons: None,
            role: role.map(|r| AxValue {
                r#type: AxValueType::ComputedString,
                value: Some(serde_json::Value::String(r.to_string())),
                related_nodes: None,
                sources: None,
            }),
            chrome_role: None,
            name: name.map(|n| AxValue {
                r#type: AxValueType::ComputedString,
                value: Some(serde_json::Value::String(n.to_string())),
                related_nodes: None,
                sources: None,
            }),
            description: None,
            value: None,
            properties: None,
            parent_id: None,
            child_ids: if children.is_empty() {
                None
            } else {
                Some(children.into_iter().map(AxNodeId::new).collect())
            },
            backend_dom_node_id: None,
            frame_id: None,
        }
    }

    #[test]
    fn build_tree_preserves_hierarchy() {
        // root(button) -> child(generic) -> leaf(link "Click")
        let flat = vec![
            make_node("1", Some("button"), Some("Submit"), vec!["2"]),
            make_node("2", Some("generic"), None, vec!["3"]),
            make_node("3", Some("link"), Some("Click"), vec![]),
        ];
        let tree = AXNode::build_tree(flat, false);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].node_id, "1");
        assert_eq!(tree[0].children.len(), 1);
        assert_eq!(tree[0].children[0].node_id, "2");
        assert_eq!(tree[0].children[0].children[0].node_id, "3");
    }

    #[test]
    fn build_tree_squash_keeps_first_empty_removes_deeper_ones() {
        // root(real) -> empty1 -> empty2 -> leaf(real)
        // empty1 is kept (its parent is real), empty2 is squashed (its parent is also empty)
        let flat = vec![
            make_node("root", Some("main"), Some("Page"), vec!["e1"]),
            make_node("e1", Some("generic"), None, vec!["e2"]),
            make_node("e2", Some("generic"), None, vec!["leaf"]),
            make_node("leaf", Some("link"), Some("Click"), vec![]),
        ];
        let tree = AXNode::build_tree(flat, true);
        assert_eq!(tree[0].node_id, "root");
        assert_eq!(tree[0].children.len(), 1);
        assert_eq!(tree[0].children[0].node_id, "e1"); // outermost empty kept
        assert_eq!(tree[0].children[0].children.len(), 1);
        assert_eq!(tree[0].children[0].children[0].node_id, "leaf"); // leaf promoted past e2
        assert_eq!(
            tree[0].children[0].children[0].parent_id.as_deref(),
            Some("e1")
        );
    }

    #[test]
    fn build_tree_squash_deep_chain_collapses_to_one_empty() {
        // root -> e1 -> e2 -> e3 -> e4 -> leaf
        // only e1 survives
        let flat = vec![
            make_node("root", Some("main"), Some("Page"), vec!["e1"]),
            make_node("e1", Some("none"), None, vec!["e2"]),
            make_node("e2", Some("group"), None, vec!["e3"]),
            make_node("e3", Some("generic"), None, vec!["e4"]),
            make_node("e4", Some("none"), None, vec!["leaf"]),
            make_node("leaf", Some("button"), Some("Go"), vec![]),
        ];
        let tree = AXNode::build_tree(flat, true);
        assert_eq!(tree[0].children[0].node_id, "e1");
        assert_eq!(tree[0].children[0].children.len(), 1);
        assert_eq!(tree[0].children[0].children[0].node_id, "leaf");
        assert_eq!(
            tree[0].children[0].children[0].parent_id.as_deref(),
            Some("e1")
        );
    }

    #[test]
    fn build_tree_squash_keeps_named_nodes() {
        // All nodes have names — squash should change nothing
        let flat = vec![
            make_node("1", Some("button"), Some("A"), vec!["2"]),
            make_node("2", Some("link"), Some("B"), vec![]),
        ];
        let tree = AXNode::build_tree(flat, true);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].children.len(), 1);
        assert_eq!(tree[0].children[0].node_id, "2");
    }

    #[test]
    fn build_tree_squash_single_empty_under_real_is_kept() {
        // root(real) -> generic(empty) -> [a, b]
        // single empty parent is kept — it groups the siblings
        let flat = vec![
            make_node("root", Some("main"), Some("Page"), vec!["mid"]),
            make_node("mid", Some("none"), None, vec!["a", "b"]),
            make_node("a", Some("button"), Some("Yes"), vec![]),
            make_node("b", Some("button"), Some("No"), vec![]),
        ];
        let tree = AXNode::build_tree(flat, true);
        assert_eq!(tree[0].children.len(), 1);
        assert_eq!(tree[0].children[0].node_id, "mid"); // empty parent kept
        assert_eq!(tree[0].children[0].children.len(), 2);
    }

    #[test]
    fn build_tree_squash_empty_root_is_kept() {
        // If the root itself is empty it is still kept (no parent above it)
        let flat = vec![
            make_node("root", Some("none"), None, vec!["a", "b"]),
            make_node("a", Some("button"), Some("Yes"), vec![]),
            make_node("b", Some("button"), Some("No"), vec![]),
        ];
        let tree = AXNode::build_tree(flat, true);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].node_id, "root");
        assert_eq!(tree[0].children.len(), 2);
    }
}
