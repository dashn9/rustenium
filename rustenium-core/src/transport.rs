use std::{error::Error, future::Future};
use std::fmt::{Display};
use std::sync::Arc;
use fastwebsockets::{handshake, Frame, OpCode, WebSocket, WebSocketError, WebSocketRead, WebSocketWrite};
use hyper::{
    body::Bytes,
    header::{CONNECTION, UPGRADE},
    upgrade::Upgraded,
    Request,
};
use hyper_util::rt::TokioIo;
use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub enum ConnectionTransportProtocol {
    Http,
    Https,
    Ws,
    Wss,
}

impl Display for ConnectionTransportProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ConnectionTransportProtocol::Http => "http",
            ConnectionTransportProtocol::Https => "https",
            ConnectionTransportProtocol::Ws => "ws",
            ConnectionTransportProtocol::Wss => "wss",
        };
        write!(f, "{}", str)
    }
}
pub enum UrlFormat {
    HostPort,
    ProtocolHostPort,
    Full, // protocol://host:port/path
}

#[derive(Debug, Clone)]
pub struct ConnectionTransportConfig {
    pub protocol: ConnectionTransportProtocol,
    pub host: String,
    pub port: u16,
    pub path:  &'static str,
}

impl Default for ConnectionTransportConfig {
    fn default() -> Self {
        Self {
            protocol: ConnectionTransportProtocol::Ws,
            host: String::from("localhost"),
            port: 0,
            path: "session",
        }
    }
}

impl ConnectionTransportConfig {
    pub fn full_endpoint(&self) -> String {
        format!("{}://{}{}", self.protocol, self.host_port(), self.path())
    }
    
    pub fn host_port(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn path(&self) -> String {
        let path_str = self.path.trim_start_matches('/');
        format!("/{}", path_str)
    }

}

pub trait ConnectionTransport {
    fn send(&mut self, message: String) -> impl Future<Output=()> + Send;
    fn listen(&self, listener: UnboundedSender<String>) -> ();
    fn close(&self) -> ();
    fn on_close(&self) -> ();
}

pub struct WebsocketConnectionTransport {
    client_tx: Arc<Mutex<WebSocketWrite<WriteHalf<TokioIo<Upgraded>>>>>,
    client_rx: Arc<Mutex<WebSocketRead<ReadHalf<TokioIo<Upgraded>>>>>,
}

impl ConnectionTransport for WebsocketConnectionTransport {
    fn send(&mut self, message: String) -> impl Future<Output=()> + Send
    {
        async move {
            let frame = Frame::text(fastwebsockets::Payload::from(message.as_bytes()));
            self.client_tx.lock().await.write_frame(frame).await.unwrap();
        }
    }

    fn listen(&self, listener: UnboundedSender<String>) -> () {
        WebsocketConnectionTransport::listener_loop(self.client_rx.clone(), self.client_tx.clone(), listener).unwrap();
    }

    fn close(&self) -> () {
        let client_tx = self.client_tx.clone();
        tokio::spawn(async move {
            let mut tx = client_tx.lock().await;
            let _ = tx.write_frame(Frame::close(1000, b"")).await;
        });
    }

    fn on_close(&self) -> () {
        todo!()
    }
}

impl WebsocketConnectionTransport {
    pub async fn new(connection_config: &ConnectionTransportConfig) -> Result<Self, Box<dyn Error>> {
        let addr_host = connection_config.host_port();

        // Retry on connection refused (driver starting up)
        let retry_delay_ms = 400;
        let mut retries = 3;

        let stream = loop {
            match TcpStream::connect(&addr_host).await {
                Ok(stream) => break stream,
                Err(e) if e.kind() == std::io::ErrorKind::ConnectionRefused && retries > 0 => {
                    tracing::warn!("Connection refused, retrying... ({} attempts remaining)", retries);
                    retries -= 1;
                    tokio::time::sleep(tokio::time::Duration::from_millis(retry_delay_ms)).await;
                }
                Err(e) => return Err(Box::new(e)),
            }
        };

        let uri = connection_config.path();
        let req = Request::builder()
            .method("GET")
            .uri(uri)
            .header("Host", &addr_host)
            .header(UPGRADE, "websocket")
            .header(CONNECTION, "upgrade")
            .header(
                "Sec-WebSocket-Key",
                fastwebsockets::handshake::generate_key(),
            )
            .header("Sec-WebSocket-Version", "13")
            .body(http_body_util::Empty::<Bytes>::new()).unwrap();

        let (mut ws, _) = handshake::client(&SpawnExecutor, req, stream).await.unwrap();
        ws = Self::configure_client(ws);
        let (rx, tx) = ws.split(tokio::io::split);
        tracing::info!("Successfully connected to WebDriver");

        Ok(Self {
            client_rx: Arc::new(Mutex::new(rx)),
            client_tx: Arc::new(Mutex::new(tx))
        })
    }

    fn configure_client(mut ws: WebSocket<TokioIo<Upgraded>>) -> WebSocket<TokioIo<Upgraded>> {
        ws.set_writev(true);
        ws.set_auto_close(true);
        ws.set_auto_pong(true);

        ws
    }
    pub fn listener_loop(ws_rx: Arc<Mutex<WebSocketRead<ReadHalf<TokioIo<Upgraded>>>>>, ws_tx: Arc<Mutex<WebSocketWrite<WriteHalf<TokioIo<Upgraded>>>>>, tx: UnboundedSender<String>) -> Result<(), WebSocketError>
    {
        tokio::spawn(async move {
            loop {
                let mut ws_rx_half = ws_rx.lock().await;
                let frame = match ws_rx_half.read_frame(&mut |frame| async {
                    // Handles obligated send
                    let mut ws_write_half = ws_tx.lock().await;
                    return ws_write_half.write_frame(frame).await;
                }).await {
                    Ok(frame) => frame,
                    // Err(WebSocketError::IoError(e)) if e.kind() == std::io::ErrorKind::ConnectionAborted => {
                    //     tracing::warn!("WebSocket connection aborted: {}. Exiting listener loop.", e);
                    //     break;
                    // }
                    Err(WebSocketError::UnexpectedEOF) => {
                        tracing::warn!("WebSocket connection closed (unexpected EOF). Exiting listener loop.");
                        break;
                    }
                    Err(e) => {
                        panic!("Unexpected WebSocket error: {:?}", e);
                    }
                };

                match frame.opcode {
                    OpCode::Close => break,
                    OpCode::Text | OpCode::Binary => {
                        let incoming = Frame::new(true, frame.opcode, None, frame.payload);
                        assert!(incoming.fin);
                        let string_payload = String::from_utf8(incoming.payload.to_owned());
                        if let Ok(str_payload) = string_payload {
                            tx.send(str_payload).unwrap()
                        }
                    }
                    _ => {}
                }
            }
        });
        Ok(())
    }
} //

struct SpawnExecutor;

impl<Fut> hyper::rt::Executor<Fut> for SpawnExecutor
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    fn execute(&self, fut: Fut) {
        tokio::task::spawn(fut);
    }
}
