use std::collections::HashMap;
use std::sync::{Arc, Mutex as StdMutex};

use rustenium_cdp_definitions::Command;
use rustenium_cdp_definitions::base::CommandResponse;
use rustenium_core::error::CdpCommandResultError;
use rustenium_cdp_definitions::browser_protocol::emulation::commands::SetDeviceMetricsOverride;
use rustenium_cdp_definitions::browser_protocol::page::commands::Navigate;
use rustenium_cdp_definitions::browser_protocol::page::results::NavigateResult;
use rustenium_cdp_definitions::browser_protocol::target::command_builders::SetDiscoverTargetsBuilder;
use rustenium_cdp_definitions::browser_protocol::target::commands::CreateTarget;
use rustenium_cdp_definitions::browser_protocol::target::events::{TargetCreated, TargetDestroyed};
use rustenium_cdp_definitions::browser_protocol::target::results::CreateTargetResult;
use rustenium_cdp_definitions::browser_protocol::target::types::{TargetId, TargetInfo};
use rustenium_core::WebsocketConnectionTransport;
use rustenium_core::error::CdpSessionSendError;
use rustenium_core::session::CdpSession;
use rustenium_core::transport::{ConnectionTransport, ConnectionTransportConfig};
use rustenium_core::CdpEventManagement;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex as TokioMutex;

#[derive(Clone)]
pub struct CdpAdapter<T: ConnectionTransport + Send + Sync> {
    pub session: Arc<TokioMutex<CdpSession<T>>>,
    pub page_targets: Arc<StdMutex<HashMap<TargetId, TargetInfo>>>,
}

impl<T: ConnectionTransport + Send + Sync + 'static> CdpAdapter<T> {
    pub fn new(session: Arc<TokioMutex<CdpSession<T>>>) -> Self {
        Self {
            session,
            page_targets: Arc::new(StdMutex::new(HashMap::new())),
        }
    }

    pub async fn listen_to_target_creation(&mut self) -> Result<(), CdpSessionSendError> {
        let page_targets = self.page_targets.clone();
        self.session.lock().await.add_event_handler(
            [TargetCreated::IDENTIFIER],
            move |event| {
                let page_targets = page_targets.clone();
                async move {
                    if let Ok(target) = event.try_into_event::<TargetCreated>() {
                        let info = target.params.target_info;
                        tracing::debug!("[CdpAdapter] Target created: id={}, type={}, url={}", info.target_id.as_ref(), info.r#type, info.url);
                        if info.r#type == "page" {
                            page_targets.lock().unwrap().insert(info.target_id.clone(), info);
                        }
                    }
                }
            },
        );

        let page_targets = self.page_targets.clone();
        self.session.lock().await.add_event_handler(
            [TargetDestroyed::IDENTIFIER],
            move |event| {
                let page_targets = page_targets.clone();
                async move {
                    if let Ok(destroyed) = event.try_into_event::<TargetDestroyed>() {
                        let id = &destroyed.params.target_id;
                        tracing::debug!("[CdpAdapter] Target destroyed: id={}", id.as_ref());
                        page_targets.lock().unwrap().remove(id);
                    }
                }
            },
        );

        // Enable target discovery so Chrome sends Target.targetCreated events
        let command = SetDiscoverTargetsBuilder::default().discover(true).build().unwrap();
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

    pub async fn emulate_device_metrics(
        &mut self,
        command: SetDeviceMetricsOverride,
    ) -> Result<(), crate::error::cdp::EmulateDeviceMetricsError> {
        self.send_command(command)
            .await
            .map_err(|err| {
                crate::error::cdp::EmulateDeviceMetricsError::CommandResultError(
                    CdpCommandResultError::SessionSendError(err),
                )
            })?;
        Ok(())
    }

    pub async fn close(&mut self) {
        self.session.lock().await.close().await;
    }
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
    let session = CdpSession::<WebsocketConnectionTransport>::ws_new(connection_transport_config).await;
    Arc::new(TokioMutex::new(session))
}
