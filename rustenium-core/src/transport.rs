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
pub struct ConnectionTransportConfig<'a> {
    pub protocol: ConnectionTransportProtocol,
    pub host: &'a str,
    pub port: u16,
    pub path:  &'a str,
}

impl <'a>Default for ConnectionTransportConfig<'a> {
    fn default() -> Self {
        Self {
            protocol: ConnectionTransportProtocol::Ws,
            host: "localhost",
            port: 0,
            path: "session",
        }
    }
}

impl<'a> ConnectionTransportConfig<'a> {
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

pub trait ConnectionTransport<'a> {
    async fn send(&mut self, message: String) -> ();
    fn listen(&self, listener: UnboundedSender<String>) -> ();
    fn close(&self) -> ();
    fn on_close(&self) -> ();
}

pub struct WebsocketConnectionTransport<'a> {
    pub config: &'a ConnectionTransportConfig<'a>,
    client_tx: Arc<Mutex<WebSocketWrite<WriteHalf<TokioIo<Upgraded>>>>>,
    client_rx: Arc<Mutex<WebSocketRead<ReadHalf<TokioIo<Upgraded>>>>>,
}

impl <'a>ConnectionTransport<'a> for WebsocketConnectionTransport<'a> {
    async fn send(&mut self, message: String) -> () {
        let frame = Frame::text(fastwebsockets::Payload::from(message.as_bytes()));
        self.client_tx.lock().await.write_frame(frame).await.unwrap();
    }

    fn listen(&self, listener: UnboundedSender<String>) -> () {
        WebsocketConnectionTransport::listener_loop(self.client_rx.clone(), self.client_tx.clone(), listener).unwrap();
    }

    fn close(&self) -> () {
        todo!()
    }

    fn on_close(&self) -> () {
        todo!()
    }
}

impl <'a> WebsocketConnectionTransport<'a> {
    pub async fn new(connection_config: &'a ConnectionTransportConfig<'a>) -> Result<Self, Box<dyn Error>> {
        let addr_host = connection_config.host_port();
        let stream = TcpStream::connect(&addr_host).await.unwrap();
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
        println!("Successfully connected to browser");

        Ok(Self {
            config: connection_config,
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
                let frame = ws_rx_half.read_frame(&mut |frame| async {
                    // Handles obligated send
                    let mut ws_write_half = ws_tx.lock().await;
                    return ws_write_half.write_frame(frame).await;
                }).await.unwrap();

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
