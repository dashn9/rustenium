use std::collections::HashMap;
use std::sync::{Arc, Mutex as StdMutex};
use std::time::Duration;

use crate::error::cdp::ScreenshotError;
use crate::input::{CdpKeyboard, CdpMouse, CdpTouchscreen, HumanMouse};

use rustenium_cdp_definitions::Command;
use rustenium_cdp_definitions::base::CommandResponse;
use rustenium_cdp_definitions::browser_protocol::dom::commands::{
    DescribeNode, GetDocument, QuerySelector, QuerySelectorAll,
};
use rustenium_cdp_definitions::browser_protocol::dom::results::{
    DescribeNodeResult, GetDocumentResult, QuerySelectorAllResult, QuerySelectorResult,
};
use rustenium_cdp_definitions::browser_protocol::dom::types::Node as DomNode;
use rustenium_cdp_definitions::browser_protocol::emulation::commands::SetDeviceMetricsOverride;
use rustenium_cdp_definitions::browser_protocol::page::command_builders::{
    EnableBuilder as PageEnableBuilder, GetLayoutMetricsBuilder,
};
use rustenium_cdp_definitions::browser_protocol::page::commands::Navigate;
use rustenium_cdp_definitions::browser_protocol::page::commands::{
    AddScriptToEvaluateOnNewDocument, CaptureScreenshot, RemoveScriptToEvaluateOnNewDocument,
};
use rustenium_cdp_definitions::browser_protocol::page::results::NavigateResult;
use rustenium_cdp_definitions::browser_protocol::page::results::{
    AddScriptToEvaluateOnNewDocumentResult, CaptureScreenshotResult, GetLayoutMetricsResult,
};
use rustenium_cdp_definitions::browser_protocol::page::types::ScriptIdentifier;
use rustenium_cdp_definitions::browser_protocol::target::command_builders::SetDiscoverTargetsBuilder;
use rustenium_cdp_definitions::browser_protocol::target::commands::CreateTarget;
use rustenium_cdp_definitions::browser_protocol::target::events::{TargetCreated, TargetDestroyed};
use rustenium_cdp_definitions::browser_protocol::target::results::CreateTargetResult;
use rustenium_cdp_definitions::browser_protocol::target::types::{TargetId, TargetInfo};
use rustenium_cdp_definitions::js_protocol::runtime::command_builders::EvaluateBuilder as RuntimeEvaluateBuilder;
use rustenium_cdp_definitions::js_protocol::runtime::results::EvaluateResult;
use rustenium_core::CdpEventManagement;
use rustenium_core::WebsocketConnectionTransport;
use rustenium_core::error::CdpCommandResultError;
use rustenium_core::error::CdpSessionSendError;
use rustenium_core::session::CdpSession;
use rustenium_core::transport::{ConnectionTransport, ConnectionTransportConfig};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex as TokioMutex;
use tokio::time::sleep;

#[derive(Clone)]
pub struct CdpAdapter<T: ConnectionTransport + Send + Sync> {
    pub session: Arc<TokioMutex<CdpSession<T>>>,
    pub page_targets: Arc<StdMutex<HashMap<TargetId, TargetInfo>>>,
    pub mouse: Arc<CdpMouse>,
    pub human_mouse: Arc<HumanMouse<CdpMouse>>,
    pub keyboard: Arc<CdpKeyboard<T>>,
    pub touchscreen: Arc<CdpTouchscreen>,
}

impl CdpAdapter<WebsocketConnectionTransport> {
    pub fn new(session: Arc<TokioMutex<CdpSession<WebsocketConnectionTransport>>>) -> Self {
        let modifiers = Arc::new(StdMutex::new(0i64));
        let mouse = CdpMouse::new(session.clone(), modifiers.clone());
        let human_mouse = Arc::new(HumanMouse::new(mouse.clone()));
        let mouse = Arc::new(mouse);
        let keyboard = Arc::new(CdpKeyboard::new(session.clone(), modifiers));
        let touchscreen = Arc::new(CdpTouchscreen::new(session.clone()));
        Self {
            session,
            page_targets: Arc::new(StdMutex::new(HashMap::new())),
            mouse,
            human_mouse,
            keyboard,
            touchscreen,
        }
    }
}

impl<T: ConnectionTransport + Send + Sync> CdpAdapter<T> {
    pub async fn listen_to_target_creation(&mut self) -> Result<(), CdpSessionSendError> {
        let page_targets = self.page_targets.clone();
        self.session
            .lock()
            .await
            .add_event_handler([TargetCreated::IDENTIFIER], move |event| {
                let page_targets = page_targets.clone();
                async move {
                    if let Ok(target) = event.try_into_event::<TargetCreated>() {
                        let info = target.params.target_info;
                        tracing::debug!(
                            "[CdpAdapter] Target created: id={}, type={}, url={}",
                            info.target_id.as_ref(),
                            info.r#type,
                            info.url
                        );
                        if info.r#type == "page" {
                            page_targets
                                .lock()
                                .unwrap()
                                .insert(info.target_id.clone(), info);
                        }
                    }
                }
            });

        let page_targets = self.page_targets.clone();
        self.session
            .lock()
            .await
            .add_event_handler([TargetDestroyed::IDENTIFIER], move |event| {
                let page_targets = page_targets.clone();
                async move {
                    if let Ok(destroyed) = event.try_into_event::<TargetDestroyed>() {
                        let id = &destroyed.params.target_id;
                        tracing::debug!("[CdpAdapter] Target destroyed: id={}", id.as_ref());
                        page_targets.lock().unwrap().remove(id);
                    }
                }
            });

        // Enable target discovery so Chrome sends Target.targetCreated events
        let command = SetDiscoverTargetsBuilder::default()
            .discover(true)
            .build()
            .unwrap();
        self.send_command(command).await?;

        Ok(())
    }

    /// Enables the Page domain on the attached target. Required for
    /// `Page.addScriptToEvaluateOnNewDocument` to actually inject scripts.
    pub async fn enable_page_domain(&mut self) -> Result<(), CdpSessionSendError> {
        let command = PageEnableBuilder::default().build();
        self.send_command(command).await?;
        Ok(())
    }

    pub async fn send_command(
        &mut self,
        command: impl Into<Command>,
    ) -> Result<CommandResponse, CdpSessionSendError> {
        self.session.lock().await.send(command).await
    }

    pub async fn navigate(
        &mut self,
        command: Navigate,
    ) -> Result<NavigateResult, crate::error::cdp::NavigateError> {
        let result_value = self
            .send_command(command)
            .await
            .map_err(|err| {
                crate::error::cdp::NavigateError::CommandResultError(
                    CdpCommandResultError::SessionSendError(err),
                )
            })?
            .result;

        NavigateResult::try_from(result_value.clone()).map_err(|_| {
            crate::error::cdp::NavigateError::CommandResultError(
                CdpCommandResultError::InvalidResultTypeError(result_value),
            )
        })
    }

    pub async fn create_target(
        &mut self,
        command: CreateTarget,
    ) -> Result<TargetId, crate::error::cdp::CreateTargetError> {
        let result_value = self
            .send_command(command)
            .await
            .map_err(|err| {
                crate::error::cdp::CreateTargetError::CommandResultError(
                    CdpCommandResultError::SessionSendError(err),
                )
            })?
            .result;

        let result = CreateTargetResult::try_from(result_value.clone()).map_err(|_| {
            crate::error::cdp::CreateTargetError::CommandResultError(
                CdpCommandResultError::InvalidResultTypeError(result_value),
            )
        })?;

        Ok(result.target_id)
    }

    pub async fn fetch_node(
        &mut self,
        command: DescribeNode,
    ) -> Result<DomNode, crate::error::cdp::NodesFetchError> {
        let result_value = self
            .send_command(command)
            .await
            .map_err(|err| {
                crate::error::cdp::NodesFetchError::CommandResultError(
                    CdpCommandResultError::SessionSendError(err),
                )
            })?
            .result;

        let result = DescribeNodeResult::try_from(result_value)
            .map_err(|e| crate::error::cdp::NodesFetchError::ParseError(e.to_string()))?;

        Ok(*result.node)
    }

    pub async fn emulate_device_metrics(
        &mut self,
        command: SetDeviceMetricsOverride,
    ) -> Result<(), crate::error::cdp::EmulateDeviceMetricsError> {
        self.send_command(command).await.map_err(|err| {
            crate::error::cdp::EmulateDeviceMetricsError::CommandResultError(
                CdpCommandResultError::SessionSendError(err),
            )
        })?;
        Ok(())
    }

    /// Returns the root document `NodeId` via `DOM.getDocument`.
    async fn get_root_node_id(
        &mut self,
    ) -> Result<
        rustenium_cdp_definitions::browser_protocol::dom::types::NodeId,
        crate::error::cdp::LocateError,
    > {
        let result_value = self
            .send_command(GetDocument::builder().depth(0).build())
            .await
            .map_err(|e| {
                crate::error::cdp::LocateError::CommandResultError(
                    CdpCommandResultError::SessionSendError(e),
                )
            })?
            .result;
        let doc = GetDocumentResult::try_from(result_value)
            .map_err(|e| crate::error::cdp::LocateError::ParseError(e.to_string()))?;
        Ok(*doc.root.node_id)
    }

    /// Describe a node by `NodeId`, returning the full `DomNode` subtree.
    async fn describe_by_id(
        &mut self,
        node_id: rustenium_cdp_definitions::browser_protocol::dom::types::NodeId,
    ) -> Result<DomNode, crate::error::cdp::LocateError> {
        let result_value = self
            .send_command(DescribeNode::builder().node_id(node_id).depth(-1).build())
            .await
            .map_err(|e| {
                crate::error::cdp::LocateError::CommandResultError(
                    CdpCommandResultError::SessionSendError(e),
                )
            })?
            .result;
        DescribeNodeResult::try_from(result_value)
            .map(|r| *r.node)
            .map_err(|e| crate::error::cdp::LocateError::ParseError(e.to_string()))
    }

    /// Find the first element matching `selector` (CSS). Returns `None` if not found.
    pub async fn locate(
        &mut self,
        selector: &str,
    ) -> Result<Option<DomNode>, crate::error::cdp::LocateError> {
        let root_id = self.get_root_node_id().await?;
        let cmd = QuerySelector::builder()
            .node_id(root_id)
            .selector(selector)
            .build()
            .map_err(crate::error::cdp::LocateError::ParseError)?;
        let result_value = self
            .send_command(cmd)
            .await
            .map_err(|e| {
                crate::error::cdp::LocateError::CommandResultError(
                    CdpCommandResultError::SessionSendError(e),
                )
            })?
            .result;
        let qs = QuerySelectorResult::try_from(result_value)
            .map_err(|e| crate::error::cdp::LocateError::ParseError(e.to_string()))?;
        let node_id = *qs.node_id;
        // nodeId of 0 means no match
        if *node_id.inner() == 0 {
            return Ok(None);
        }
        Ok(Some(self.describe_by_id(node_id).await?))
    }

    /// Find all elements matching `selector` (CSS).
    pub async fn locate_all(
        &mut self,
        selector: &str,
    ) -> Result<Vec<DomNode>, crate::error::cdp::LocateError> {
        let root_id = self.get_root_node_id().await?;
        let cmd = QuerySelectorAll::builder()
            .node_id(root_id)
            .selector(selector)
            .build()
            .map_err(crate::error::cdp::LocateError::ParseError)?;
        let result_value = self
            .send_command(cmd)
            .await
            .map_err(|e| {
                crate::error::cdp::LocateError::CommandResultError(
                    CdpCommandResultError::SessionSendError(e),
                )
            })?
            .result;
        let ids = QuerySelectorAllResult::try_from(result_value)
            .map_err(|e| crate::error::cdp::LocateError::ParseError(e.to_string()))?
            .node_ids;
        let mut nodes = Vec::with_capacity(ids.len());
        for id in ids {
            nodes.push(self.describe_by_id(id).await?);
        }
        Ok(nodes)
    }

    /// Poll until the first element matching `selector` appears, or `timeout` elapses.
    pub async fn wait_for(
        &mut self,
        selector: &str,
        timeout: Duration,
    ) -> Result<DomNode, crate::error::cdp::LocateError> {
        let interval = Duration::from_millis(100);
        let start = tokio::time::Instant::now();
        loop {
            if let Some(node) = self.locate(selector).await? {
                return Ok(node);
            }
            if start.elapsed() >= timeout {
                return Err(crate::error::cdp::LocateError::Timeout(
                    selector.to_string(),
                ));
            }
            sleep(interval).await;
        }
    }

    /// Poll until at least one element matching `selector` appears, or `timeout` elapses.
    pub async fn wait_for_all(
        &mut self,
        selector: &str,
        timeout: Duration,
    ) -> Result<Vec<DomNode>, crate::error::cdp::LocateError> {
        let interval = Duration::from_millis(100);
        let start = tokio::time::Instant::now();
        loop {
            let nodes = self.locate_all(selector).await?;
            if !nodes.is_empty() {
                return Ok(nodes);
            }
            if start.elapsed() >= timeout {
                return Err(crate::error::cdp::LocateError::Timeout(
                    selector.to_string(),
                ));
            }
            sleep(interval).await;
        }
    }

    pub async fn layout_metrics(
        &mut self,
    ) -> Result<GetLayoutMetricsResult, CdpCommandResultError> {
        let result_value = self
            .send_command(GetLayoutMetricsBuilder.build())
            .await
            .map_err(CdpCommandResultError::SessionSendError)?
            .result;

        GetLayoutMetricsResult::try_from(result_value.clone())
            .map_err(|_| CdpCommandResultError::InvalidResultTypeError(result_value))
    }

    pub async fn screenshot(
        &mut self,
        screenshot: CaptureScreenshot,
        save_path: Option<String>,
    ) -> Result<String, crate::error::cdp::ScreenshotError> {
        let result_value = self
            .send_command(screenshot)
            .await
            .map_err(|e| {
                crate::error::cdp::ScreenshotError::CommandResultError(
                    CdpCommandResultError::SessionSendError(e),
                )
            })?
            .result;
        let result = CaptureScreenshotResult::try_from(result_value)
            .map_err(|e| crate::error::cdp::ScreenshotError::ParseError(e.to_string()))?;
        let base64_data = String::from(result.data);

        if let Some(path) = save_path {
            use std::path::Path;

            let path_obj = Path::new(path.as_str());

            let final_path = if path_obj.is_dir() {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis())
                    .unwrap_or(0);
                let filename = format!("screenshot_{}.png", timestamp);
                path_obj.join(filename)
            } else {
                if let Some(parent) = path_obj.parent()
                    && !parent.as_os_str().is_empty()
                    && !parent.exists()
                {
                    return Err(ScreenshotError::InvalidPath(format!(
                        "Parent directory does not exist: {}",
                        parent.display()
                    )));
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

    pub async fn evaluate_script(
        &mut self,
        expression: &str,
        await_promise: bool,
    ) -> Result<EvaluateResult, crate::error::cdp::EvaluateScriptError> {
        let command = RuntimeEvaluateBuilder::default()
            .expression(expression)
            .await_promise(await_promise)
            .build()
            .unwrap();
        let result_value = self
            .send_command(command)
            .await
            .map_err(|e| {
                crate::error::cdp::EvaluateScriptError::CommandResultError(
                    CdpCommandResultError::SessionSendError(e),
                )
            })?
            .result;
        EvaluateResult::try_from(result_value)
            .map_err(|e| crate::error::cdp::EvaluateScriptError::ParseError(e.to_string()))
    }

    pub async fn add_preload_script(
        &mut self,
        command: AddScriptToEvaluateOnNewDocument,
    ) -> Result<ScriptIdentifier, crate::error::cdp::PreloadScriptError> {
        let result_value = self
            .send_command(command)
            .await
            .map_err(|e| {
                crate::error::cdp::PreloadScriptError::CommandResultError(
                    CdpCommandResultError::SessionSendError(e),
                )
            })?
            .result;
        let result = AddScriptToEvaluateOnNewDocumentResult::try_from(result_value)
            .map_err(|e| crate::error::cdp::PreloadScriptError::ParseError(e.to_string()))?;
        Ok(result.identifier)
    }

    pub async fn remove_preload_script(
        &mut self,
        command: RemoveScriptToEvaluateOnNewDocument,
    ) -> Result<(), crate::error::cdp::PreloadScriptError> {
        self.send_command(command).await.map_err(|e| {
            crate::error::cdp::PreloadScriptError::CommandResultError(
                CdpCommandResultError::SessionSendError(e),
            )
        })?;
        Ok(())
    }

    pub async fn close(&mut self) {
        self.session.lock().await.close().await;
    }
}

pub async fn fetch_ws_debugger_url_with_retry(
    host: &str,
    chrome_port: u16,
) -> Result<String, String> {
    let mut last_err = None;

    for attempt in 0..3 {
        match fetch_ws_debugger_url(host, chrome_port).await {
            Ok(url) => return Ok(url),
            Err(e) => {
                last_err = Some(e);
                if attempt < 2 {
                    sleep(Duration::from_millis(500)).await;
                }
            }
        }
    }

    Err(last_err.expect("retry loop should always set an error"))
}

/// Fetches the first `webSocketDebuggerUrl` from the Chrome DevTools Protocol
/// `/json` endpoint at `http://host:port/json`.
pub async fn fetch_ws_debugger_url(host: &str, port: u16) -> Result<String, String> {
    let addr = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(&addr)
        .await
        .map_err(|e| format!("connect to {}: {}", addr, e))?;

    let request = format!(
        "GET /json HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        addr
    );
    stream
        .write_all(request.as_bytes())
        .await
        .map_err(|e| format!("write request: {}", e))?;

    let mut reader = BufReader::new(stream);

    // Read headers to find Content-Length
    let mut headers = String::new();
    let mut content_length: Option<usize> = None;
    loop {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .await
            .map_err(|e| format!("read header: {}", e))?;
        if line == "\r\n" || line.is_empty() {
            break;
        }
        if let Some(val) = line
            .strip_prefix("Content-Length:")
            .or_else(|| line.strip_prefix("content-length:"))
        {
            content_length = val.trim().parse().ok();
        }
        headers.push_str(&line);
    }

    // Read body using Content-Length, or fall back to reading until EOF with a timeout
    let body = if let Some(len) = content_length {
        let mut buf = vec![0u8; len];
        tokio::io::AsyncReadExt::read_exact(&mut reader, &mut buf)
            .await
            .map_err(|e| format!("read body: {}", e))?;
        String::from_utf8(buf).map_err(|e| format!("invalid utf8: {}", e))?
    } else {
        let mut buf = String::new();
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            tokio::io::AsyncReadExt::read_to_string(&mut reader, &mut buf),
        )
        .await
        .map_err(|_| "timeout reading response body".to_string())?
        .map_err(|e| format!("read body: {}", e))?;
        buf
    };

    let targets: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| format!("parse JSON: {}", e))?;

    targets[0]["webSocketDebuggerUrl"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "webSocketDebuggerUrl not found in /json response".to_string())
}

pub async fn start_cdp_session(
    connection_transport_config: &ConnectionTransportConfig,
) -> Arc<TokioMutex<CdpSession<WebsocketConnectionTransport>>> {
    let session =
        CdpSession::<WebsocketConnectionTransport>::ws_new(connection_transport_config).await;
    Arc::new(TokioMutex::new(session))
}
