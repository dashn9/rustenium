use std::collections::HashMap;
use rustenium_bidi_definitions::browsing_context::command_builders::CaptureScreenshotBuilder;
use rustenium_bidi_definitions::browsing_context::commands::CaptureScreenshotOrigin;
use rustenium_bidi_definitions::browsing_context::types::{BrowsingContext, ImageFormat, Locator};
use crate::error::{EvaluateResultError, ScreenshotError};
use crate::nodes::bidi::node::BidiNode;
use rustenium_bidi_definitions::script::types::{Handle, NodeRemoteValue, SharedId};
use rustenium_core::transport::ConnectionTransport;
use rustenium_core::BidiSession;
use crate::nodes::node::Node;
use crate::nodes::NodePosition;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ChromeNode<T: ConnectionTransport = rustenium_core::transport::WebsocketConnectionTransport> {
    bidi_node: BidiNode<T>,
    children: Vec<ChromeNode<T>>,
}

pub struct ScreenshotBuilder<'a, T: ConnectionTransport> {
    node: &'a ChromeNode<T>,
    origin: Option<CaptureScreenshotOrigin>,
    format: Option<ImageFormat>,
    save_path: Option<String>,
}

impl<'a, T: ConnectionTransport> ScreenshotBuilder<'a, T> {
    pub fn new(node: &'a ChromeNode<T>) -> Self {
        Self {
            node,
            origin: None,
            format: None,
            save_path: None,
        }
    }

    pub fn origin(mut self, origin: CaptureScreenshotOrigin) -> Self {
        self.origin = Some(origin);
        self
    }

    pub fn format(mut self, format: ImageFormat) -> Self {
        self.format = Some(format);
        self
    }

    pub fn save_path(mut self, path: impl Into<String>) -> Self {
        self.save_path = Some(path.into());
        self
    }

    pub async fn execute(self) -> Result<String, ScreenshotError> {
        let context = self.node.bidi_node.context_id.clone();

        let mut builder = CaptureScreenshotBuilder::default().context(context);
        if let Some(origin) = self.origin {
            builder = builder.origin(origin);
        }
        if let Some(format) = self.format {
            builder = builder.format(format);
        }
        let command = builder.build().unwrap();

        let base64_data = self.node.bidi_node.screenshot(command).await?;

        if let Some(path) = self.save_path {
            use std::path::Path;
            let path_obj = Path::new(&path);
            let final_path = if path_obj.is_dir() {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis())
                    .unwrap_or(0);
                path_obj.join(format!("screenshot_{}.png", timestamp))
            } else {
                if let Some(parent) = path_obj.parent() {
                    if !parent.as_os_str().is_empty() && !parent.exists() {
                        return Err(ScreenshotError::InvalidPath(format!(
                            "Parent directory does not exist: {}",
                            parent.display()
                        )));
                    }
                }
                path_obj.to_path_buf()
            };

            use base64::{Engine as _, engine::general_purpose};
            let decoded = general_purpose::STANDARD
                .decode(&base64_data)
                .map_err(|e| ScreenshotError::Base64DecodeError(e.to_string()))?;
            std::fs::write(&final_path, decoded)
                .map_err(|e| ScreenshotError::FileWriteError(e.to_string()))?;

            Ok(final_path.to_string_lossy().to_string())
        } else {
            Ok(base64_data)
        }
    }
}

impl<T: ConnectionTransport> ChromeNode<T> {
    pub fn from_bidi(
        _raw_bidi_node: NodeRemoteValue,
        locator: Locator,
        session: Arc<Mutex<BidiSession<T>>>,
        context: BrowsingContext,
    ) -> Self {
        let bidi_node = BidiNode::new(_raw_bidi_node, locator.clone(), session.clone(), context.clone());

        let children = bidi_node.children.iter()
            .map(|bidi_child| {
                ChromeNode::from_bidi(
                    bidi_child.get_raw_node_ref().clone(),
                    locator.clone(),
                    session.clone(),
                    context.clone(),
                )
            })
            .collect();

        Self { bidi_node, children }
    }

    pub fn screenshot(&self) -> ScreenshotBuilder<'_, T> {
        ScreenshotBuilder::new(self)
    }
}

impl<T: ConnectionTransport> Node for ChromeNode<T> {
    #[allow(refining_impl_trait)]
    fn get_children_nodes(&self) -> &Vec<ChromeNode<T>> {
        &self.children
    }

    fn get_bidi_locator(&self) -> &Locator {
        &self.bidi_node.locator
    }

    async fn get_inner_text(&self) -> String {
        self.bidi_node.get_inner_text().await.unwrap()
    }

    async fn get_text_content(&self) -> String {
        self.bidi_node.get_text_content().await.unwrap()
    }

    async fn get_inner_html(&self) -> String {
        self.bidi_node.get_inner_html().await.unwrap()
    }

    fn get_attribute(&self, attribute_name: &str) -> Option<serde_json::Value> {
        self.bidi_node.get_attribute(attribute_name)
    }

    fn get_attributes(&self) -> HashMap<String, serde_json::Value> {
        self.bidi_node.get_attributes()
    }

    async fn get_position(&mut self) -> Option<&NodePosition> {
        self.bidi_node.get_position().await.unwrap_or(None)
    }

    fn get_context_id(&self) -> &BrowsingContext {
        &self.bidi_node.context_id
    }

    fn get_shared_id(&self) -> Option<&SharedId> {
        self.bidi_node.get_raw_node_ref().shared_id.as_ref()
    }

    fn get_handle(&self) -> &Option<Handle> {
        &self.bidi_node.get_raw_node_ref().handle
    }

    fn set_position(&mut self, position: NodePosition) -> () {
        self.bidi_node.position = Some(position);
    }

    async fn scroll_into_view(&self) -> Result<(), EvaluateResultError> {
        self.bidi_node.scroll_into_view().await
    }

    async fn is_visible(&self) -> Result<bool, EvaluateResultError> {
        self.bidi_node.is_visible().await
    }

    async fn delete(&self) -> Result<(), EvaluateResultError> {
        self.bidi_node.delete().await
    }
}
