use std::sync::Arc;

use rustenium_cdp_definitions::Command;
use rustenium_cdp_definitions::base::CommandResponse;
use rustenium_core::error::CdpCommandResultError;
use rustenium_cdp_definitions::browser_protocol::page::commands::Navigate;
use rustenium_cdp_definitions::browser_protocol::page::results::NavigateResult;
use rustenium_cdp_definitions::browser_protocol::target::command_builders::SetDiscoverTargetsBuilder;
use rustenium_cdp_definitions::browser_protocol::target::events::TargetCreated;
use rustenium_core::WebsocketConnectionTransport;
use rustenium_core::error::CdpSessionSendError;
use rustenium_core::session::CdpSession;
use rustenium_core::transport::{ConnectionTransport, ConnectionTransportConfig};
use rustenium_core::CdpEventManagement;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex as TokioMutex;

pub trait AdaptCdp<T: ConnectionTransport> {
    /// Fetches the first `webSocketDebuggerUrl` from the Chrome DevTools Protocol
    /// `/json` endpoint at `http://host:port/json`.
    fn fetch_ws_debugger_url(
        host: &str,
        port: u16,
    ) -> impl Future<Output = Result<String, String>> {
        async move {
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
    }

    fn start(
        connection_transport_config: &ConnectionTransportConfig,
    ) -> impl Future<Output = Arc<TokioMutex<CdpSession<WebsocketConnectionTransport>>>> {
        async {
            let session = CdpSession::<T>::ws_new(connection_transport_config).await;
            Arc::new(TokioMutex::new(session))
        }
    }
}

#[derive(Clone)]
pub struct CdpAdapter<T: ConnectionTransport + Send + Sync> {
    pub session: Arc<TokioMutex<CdpSession<T>>>,
}

impl<T: ConnectionTransport + Send + Sync + 'static> CdpAdapter<T> {
    pub fn new(session: Arc<TokioMutex<CdpSession<T>>>) -> Self {
        Self { session }
    }

    pub async fn listen_to_target_creation(&mut self) -> Result<(), CdpSessionSendError> {
        // Register handler for Target.targetCreated
        self.session.lock().await.add_event_handler(
            [TargetCreated::IDENTIFIER],
            move |event| async move {
                if let Ok(target) = event.try_into_event::<TargetCreated>() {
                    tracing::debug!(
                        "[CdpAdapter]: Target created: id={}, type={}, url={}",
                        target.params.target_info.target_id.as_ref(),
                        target.params.target_info.r#type,
                        target.params.target_info.url,
                    );
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

    pub async fn close(&mut self) {
        self.session.lock().await.close().await;
    }
}
