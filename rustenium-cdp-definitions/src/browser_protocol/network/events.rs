use serde::{Deserialize, Serialize};
#[doc = "Fired when data chunk was received over the network.\n[dataReceived](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-dataReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataReceived {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[doc = "Data chunk length."]
    #[serde(rename = "dataLength")]
    pub data_length: i64,
    #[doc = "Actual bytes received (might be less than dataLength for compressed encodings)."]
    #[serde(rename = "encodedDataLength")]
    pub encoded_data_length: i64,
    #[doc = "Data that was received."]
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data: Option<super::super::super::Binary>,
}
impl DataReceived {
    pub const IDENTIFIER: &'static str = "Network.dataReceived";
}
#[doc = "Fired when EventSource message is received.\n[eventSourceMessageReceived](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-eventSourceMessageReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventSourceMessageReceived {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[doc = "Message type."]
    #[serde(rename = "eventName")]
    pub event_name: String,
    #[doc = "Message identifier."]
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[doc = "Message content."]
    #[serde(rename = "data")]
    pub data: String,
}
impl EventSourceMessageReceived {
    pub const IDENTIFIER: &'static str = "Network.eventSourceMessageReceived";
}
#[doc = "Fired when HTTP request has failed to load.\n[loadingFailed](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-loadingFailed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadingFailed {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[doc = "Resource type."]
    #[serde(rename = "type")]
    pub r#type: super::types::ResourceType,
    #[doc = "Error message. List of network errors: https://cs.chromium.org/chromium/src/net/base/net_error_list.h"]
    #[serde(rename = "errorText")]
    pub error_text: String,
    #[doc = "True if loading was canceled."]
    #[serde(rename = "canceled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub canceled: Option<bool>,
    #[doc = "The reason why loading was blocked, if any."]
    #[serde(rename = "blockedReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub blocked_reason: Option<super::types::BlockedReason>,
    #[doc = "The reason why loading was blocked by CORS, if any."]
    #[serde(rename = "corsErrorStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cors_error_status: Option<super::types::CorsErrorStatus>,
}
impl LoadingFailed {
    pub const IDENTIFIER: &'static str = "Network.loadingFailed";
}
#[doc = "Fired when HTTP request has finished loading.\n[loadingFinished](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-loadingFinished)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadingFinished {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[doc = "Total number of bytes received for this request."]
    #[serde(rename = "encodedDataLength")]
    pub encoded_data_length: f64,
}
impl LoadingFinished {
    pub const IDENTIFIER: &'static str = "Network.loadingFinished";
}
#[doc = "Fired if request ended up loading from cache.\n[requestServedFromCache](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-requestServedFromCache)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestServedFromCache {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
}
impl RequestServedFromCache {
    pub const IDENTIFIER: &'static str = "Network.requestServedFromCache";
}
#[doc = "Fired when page is about to send HTTP request.\n[requestWillBeSent](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-requestWillBeSent)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestWillBeSent {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Loader identifier. Empty string if the request is fetched from worker."]
    #[serde(rename = "loaderId")]
    pub loader_id: super::types::LoaderId,
    #[doc = "URL of the document this request is loaded for."]
    #[serde(rename = "documentURL")]
    pub document_url: String,
    #[doc = "Request data."]
    #[serde(rename = "request")]
    pub request: super::types::Request,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[doc = "Timestamp."]
    #[serde(rename = "wallTime")]
    pub wall_time: super::types::TimeSinceEpoch,
    #[doc = "Request initiator."]
    #[serde(rename = "initiator")]
    pub initiator: super::types::Initiator,
    #[doc = "In the case that redirectResponse is populated, this flag indicates whether\nrequestWillBeSentExtraInfo and responseReceivedExtraInfo events will be or were emitted\nfor the request which was just redirected."]
    #[serde(rename = "redirectHasExtraInfo")]
    pub redirect_has_extra_info: bool,
    #[doc = "Redirect response data."]
    #[serde(rename = "redirectResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub redirect_response: Option<super::types::Response>,
    #[doc = "Type of this resource."]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub r#type: Option<super::types::ResourceType>,
    #[doc = "Frame identifier."]
    #[serde(rename = "frameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_id: Option<super::super::page::types::FrameId>,
    #[doc = "Whether the request is initiated by a user gesture. Defaults to false."]
    #[serde(rename = "hasUserGesture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub has_user_gesture: Option<bool>,
    #[doc = "The render blocking behavior of the request."]
    #[serde(rename = "renderBlockingBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub render_blocking_behavior: Option<super::types::RenderBlockingBehavior>,
}
impl RequestWillBeSent {
    pub const IDENTIFIER: &'static str = "Network.requestWillBeSent";
}
#[doc = "Fired when resource loading priority is changed\n[resourceChangedPriority](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-resourceChangedPriority)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceChangedPriority {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "New priority"]
    #[serde(rename = "newPriority")]
    pub new_priority: super::types::ResourcePriority,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
}
impl ResourceChangedPriority {
    pub const IDENTIFIER: &'static str = "Network.resourceChangedPriority";
}
#[doc = "Fired when a signed exchange was received over the network\n[signedExchangeReceived](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-signedExchangeReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignedExchangeReceived {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Information about the signed exchange response."]
    #[serde(rename = "info")]
    pub info: super::types::SignedExchangeInfo,
}
impl SignedExchangeReceived {
    pub const IDENTIFIER: &'static str = "Network.signedExchangeReceived";
}
#[doc = "Fired when HTTP response is available.\n[responseReceived](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-responseReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseReceived {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Loader identifier. Empty string if the request is fetched from worker."]
    #[serde(rename = "loaderId")]
    pub loader_id: super::types::LoaderId,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[doc = "Resource type."]
    #[serde(rename = "type")]
    pub r#type: super::types::ResourceType,
    #[doc = "Response data."]
    #[serde(rename = "response")]
    pub response: super::types::Response,
    #[doc = "Indicates whether requestWillBeSentExtraInfo and responseReceivedExtraInfo events will be\nor were emitted for this request."]
    #[serde(rename = "hasExtraInfo")]
    pub has_extra_info: bool,
    #[doc = "Frame identifier."]
    #[serde(rename = "frameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_id: Option<super::super::page::types::FrameId>,
}
impl ResponseReceived {
    pub const IDENTIFIER: &'static str = "Network.responseReceived";
}
#[doc = "Fired when WebSocket is closed.\n[webSocketClosed](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-webSocketClosed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebSocketClosed {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
}
impl WebSocketClosed {
    pub const IDENTIFIER: &'static str = "Network.webSocketClosed";
}
#[doc = "Fired upon WebSocket creation.\n[webSocketCreated](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-webSocketCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebSocketCreated {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "WebSocket request URL."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Request initiator."]
    #[serde(rename = "initiator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub initiator: Option<super::types::Initiator>,
}
impl WebSocketCreated {
    pub const IDENTIFIER: &'static str = "Network.webSocketCreated";
}
#[doc = "Fired when WebSocket message error occurs.\n[webSocketFrameError](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-webSocketFrameError)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebSocketFrameError {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[doc = "WebSocket error message."]
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}
impl WebSocketFrameError {
    pub const IDENTIFIER: &'static str = "Network.webSocketFrameError";
}
#[doc = "Fired when WebSocket message is received.\n[webSocketFrameReceived](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-webSocketFrameReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebSocketFrameReceived {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[doc = "WebSocket response data."]
    #[serde(rename = "response")]
    pub response: super::types::WebSocketFrame,
}
impl WebSocketFrameReceived {
    pub const IDENTIFIER: &'static str = "Network.webSocketFrameReceived";
}
#[doc = "Fired when WebSocket message is sent.\n[webSocketFrameSent](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-webSocketFrameSent)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebSocketFrameSent {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[doc = "WebSocket response data."]
    #[serde(rename = "response")]
    pub response: super::types::WebSocketFrame,
}
impl WebSocketFrameSent {
    pub const IDENTIFIER: &'static str = "Network.webSocketFrameSent";
}
#[doc = "Fired when WebSocket handshake response becomes available.\n[webSocketHandshakeResponseReceived](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-webSocketHandshakeResponseReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebSocketHandshakeResponseReceived {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[doc = "WebSocket response data."]
    #[serde(rename = "response")]
    pub response: super::types::WebSocketResponse,
}
impl WebSocketHandshakeResponseReceived {
    pub const IDENTIFIER: &'static str = "Network.webSocketHandshakeResponseReceived";
}
#[doc = "Fired when WebSocket is about to initiate handshake.\n[webSocketWillSendHandshakeRequest](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-webSocketWillSendHandshakeRequest)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebSocketWillSendHandshakeRequest {
    #[doc = "Request identifier."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[doc = "UTC Timestamp."]
    #[serde(rename = "wallTime")]
    pub wall_time: super::types::TimeSinceEpoch,
    #[doc = "WebSocket request data."]
    #[serde(rename = "request")]
    pub request: super::types::WebSocketRequest,
}
impl WebSocketWillSendHandshakeRequest {
    pub const IDENTIFIER: &'static str = "Network.webSocketWillSendHandshakeRequest";
}
#[doc = "Fired upon WebTransport creation.\n[webTransportCreated](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-webTransportCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebTransportCreated {
    #[doc = "WebTransport identifier."]
    #[serde(rename = "transportId")]
    pub transport_id: super::types::RequestId,
    #[doc = "WebTransport request URL."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[doc = "Request initiator."]
    #[serde(rename = "initiator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub initiator: Option<super::types::Initiator>,
}
impl WebTransportCreated {
    pub const IDENTIFIER: &'static str = "Network.webTransportCreated";
}
#[doc = "Fired when WebTransport handshake is finished.\n[webTransportConnectionEstablished](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-webTransportConnectionEstablished)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebTransportConnectionEstablished {
    #[doc = "WebTransport identifier."]
    #[serde(rename = "transportId")]
    pub transport_id: super::types::RequestId,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
}
impl WebTransportConnectionEstablished {
    pub const IDENTIFIER: &'static str = "Network.webTransportConnectionEstablished";
}
#[doc = "Fired when WebTransport is disposed.\n[webTransportClosed](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-webTransportClosed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebTransportClosed {
    #[doc = "WebTransport identifier."]
    #[serde(rename = "transportId")]
    pub transport_id: super::types::RequestId,
    #[doc = "Timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
}
impl WebTransportClosed {
    pub const IDENTIFIER: &'static str = "Network.webTransportClosed";
}
#[doc = "Fired upon direct_socket.TCPSocket creation.\n[directTCPSocketCreated](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-directTCPSocketCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectTcpSocketCreated {
    #[serde(rename = "identifier")]
    pub identifier: super::types::RequestId,
    #[serde(rename = "remoteAddr")]
    pub remote_addr: String,
    #[doc = "Unsigned int 16."]
    #[serde(rename = "remotePort")]
    pub remote_port: i64,
    #[serde(rename = "options")]
    pub options: super::types::DirectTcpSocketOptions,
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[serde(rename = "initiator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub initiator: Option<super::types::Initiator>,
}
impl DirectTcpSocketCreated {
    pub const IDENTIFIER: &'static str = "Network.directTCPSocketCreated";
}
#[doc = "Fired when direct_socket.TCPSocket connection is opened.\n[directTCPSocketOpened](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-directTCPSocketOpened)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectTcpSocketOpened {
    #[serde(rename = "identifier")]
    pub identifier: super::types::RequestId,
    #[serde(rename = "remoteAddr")]
    pub remote_addr: String,
    #[doc = "Expected to be unsigned integer."]
    #[serde(rename = "remotePort")]
    pub remote_port: i64,
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[serde(rename = "localAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub local_addr: Option<String>,
    #[doc = "Expected to be unsigned integer."]
    #[serde(rename = "localPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub local_port: Option<i64>,
}
impl DirectTcpSocketOpened {
    pub const IDENTIFIER: &'static str = "Network.directTCPSocketOpened";
}
#[doc = "Fired when direct_socket.TCPSocket is aborted.\n[directTCPSocketAborted](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-directTCPSocketAborted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectTcpSocketAborted {
    #[serde(rename = "identifier")]
    pub identifier: super::types::RequestId,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
}
impl DirectTcpSocketAborted {
    pub const IDENTIFIER: &'static str = "Network.directTCPSocketAborted";
}
#[doc = "Fired when direct_socket.TCPSocket is closed.\n[directTCPSocketClosed](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-directTCPSocketClosed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectTcpSocketClosed {
    #[serde(rename = "identifier")]
    pub identifier: super::types::RequestId,
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
}
impl DirectTcpSocketClosed {
    pub const IDENTIFIER: &'static str = "Network.directTCPSocketClosed";
}
#[doc = "Fired when data is sent to tcp direct socket stream.\n[directTCPSocketChunkSent](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-directTCPSocketChunkSent)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectTcpSocketChunkSent {
    #[serde(rename = "identifier")]
    pub identifier: super::types::RequestId,
    #[serde(rename = "data")]
    pub data: super::super::super::Binary,
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
}
impl DirectTcpSocketChunkSent {
    pub const IDENTIFIER: &'static str = "Network.directTCPSocketChunkSent";
}
#[doc = "Fired when data is received from tcp direct socket stream.\n[directTCPSocketChunkReceived](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-directTCPSocketChunkReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectTcpSocketChunkReceived {
    #[serde(rename = "identifier")]
    pub identifier: super::types::RequestId,
    #[serde(rename = "data")]
    pub data: super::super::super::Binary,
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
}
impl DirectTcpSocketChunkReceived {
    pub const IDENTIFIER: &'static str = "Network.directTCPSocketChunkReceived";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectUdpSocketJoinedMulticastGroup {
    #[serde(rename = "identifier")]
    pub identifier: super::types::RequestId,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
}
impl DirectUdpSocketJoinedMulticastGroup {
    pub const IDENTIFIER: &'static str = "Network.directUDPSocketJoinedMulticastGroup";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectUdpSocketLeftMulticastGroup {
    #[serde(rename = "identifier")]
    pub identifier: super::types::RequestId,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
}
impl DirectUdpSocketLeftMulticastGroup {
    pub const IDENTIFIER: &'static str = "Network.directUDPSocketLeftMulticastGroup";
}
#[doc = "Fired upon direct_socket.UDPSocket creation.\n[directUDPSocketCreated](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-directUDPSocketCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectUdpSocketCreated {
    #[serde(rename = "identifier")]
    pub identifier: super::types::RequestId,
    #[serde(rename = "options")]
    pub options: super::types::DirectUdpSocketOptions,
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[serde(rename = "initiator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub initiator: Option<super::types::Initiator>,
}
impl DirectUdpSocketCreated {
    pub const IDENTIFIER: &'static str = "Network.directUDPSocketCreated";
}
#[doc = "Fired when direct_socket.UDPSocket connection is opened.\n[directUDPSocketOpened](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-directUDPSocketOpened)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectUdpSocketOpened {
    #[serde(rename = "identifier")]
    pub identifier: super::types::RequestId,
    #[serde(rename = "localAddr")]
    pub local_addr: String,
    #[doc = "Expected to be unsigned integer."]
    #[serde(rename = "localPort")]
    pub local_port: i64,
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
    #[serde(rename = "remoteAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub remote_addr: Option<String>,
    #[doc = "Expected to be unsigned integer."]
    #[serde(rename = "remotePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub remote_port: Option<i64>,
}
impl DirectUdpSocketOpened {
    pub const IDENTIFIER: &'static str = "Network.directUDPSocketOpened";
}
#[doc = "Fired when direct_socket.UDPSocket is aborted.\n[directUDPSocketAborted](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-directUDPSocketAborted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectUdpSocketAborted {
    #[serde(rename = "identifier")]
    pub identifier: super::types::RequestId,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
}
impl DirectUdpSocketAborted {
    pub const IDENTIFIER: &'static str = "Network.directUDPSocketAborted";
}
#[doc = "Fired when direct_socket.UDPSocket is closed.\n[directUDPSocketClosed](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-directUDPSocketClosed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectUdpSocketClosed {
    #[serde(rename = "identifier")]
    pub identifier: super::types::RequestId,
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
}
impl DirectUdpSocketClosed {
    pub const IDENTIFIER: &'static str = "Network.directUDPSocketClosed";
}
#[doc = "Fired when message is sent to udp direct socket stream.\n[directUDPSocketChunkSent](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-directUDPSocketChunkSent)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectUdpSocketChunkSent {
    #[serde(rename = "identifier")]
    pub identifier: super::types::RequestId,
    #[serde(rename = "message")]
    pub message: super::types::DirectUdpMessage,
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
}
impl DirectUdpSocketChunkSent {
    pub const IDENTIFIER: &'static str = "Network.directUDPSocketChunkSent";
}
#[doc = "Fired when message is received from udp direct socket stream.\n[directUDPSocketChunkReceived](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-directUDPSocketChunkReceived)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectUdpSocketChunkReceived {
    #[serde(rename = "identifier")]
    pub identifier: super::types::RequestId,
    #[serde(rename = "message")]
    pub message: super::types::DirectUdpMessage,
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::MonotonicTime,
}
impl DirectUdpSocketChunkReceived {
    pub const IDENTIFIER: &'static str = "Network.directUDPSocketChunkReceived";
}
#[doc = "Fired when additional information about a requestWillBeSent event is available from the\nnetwork stack. Not every requestWillBeSent event will have an additional\nrequestWillBeSentExtraInfo fired for it, and there is no guarantee whether requestWillBeSent\nor requestWillBeSentExtraInfo will be fired first for the same request.\n[requestWillBeSentExtraInfo](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-requestWillBeSentExtraInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestWillBeSentExtraInfo {
    #[doc = "Request identifier. Used to match this information to an existing requestWillBeSent event."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "A list of cookies potentially associated to the requested URL. This includes both cookies sent with\nthe request and the ones not sent; the latter are distinguished by having blockedReasons field set."]
    #[serde(rename = "associatedCookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub associated_cookies: Vec<super::types::AssociatedCookie>,
    #[doc = "Raw request headers as they will be sent over the wire."]
    #[serde(rename = "headers")]
    pub headers: super::types::Headers,
    #[doc = "Connection timing information for the request."]
    #[serde(rename = "connectTiming")]
    pub connect_timing: super::types::ConnectTiming,
    #[doc = "How the request site's device bound sessions were used during this request."]
    #[serde(rename = "deviceBoundSessionUsages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub device_bound_session_usages: Option<Vec<super::types::DeviceBoundSessionWithUsage>>,
    #[doc = "The client security state set for the request."]
    #[serde(rename = "clientSecurityState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub client_security_state: Option<super::types::ClientSecurityState>,
    #[doc = "Whether the site has partitioned cookies stored in a partition different than the current one."]
    #[serde(rename = "siteHasCookieInOtherPartition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub site_has_cookie_in_other_partition: Option<bool>,
    #[doc = "The network conditions id if this request was affected by network conditions configured via\nemulateNetworkConditionsByRule."]
    #[serde(rename = "appliedNetworkConditionsId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub applied_network_conditions_id: Option<String>,
}
impl RequestWillBeSentExtraInfo {
    pub const IDENTIFIER: &'static str = "Network.requestWillBeSentExtraInfo";
}
#[doc = "Fired when additional information about a responseReceived event is available from the network\nstack. Not every responseReceived event will have an additional responseReceivedExtraInfo for\nit, and responseReceivedExtraInfo may be fired before or after responseReceived.\n[responseReceivedExtraInfo](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-responseReceivedExtraInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseReceivedExtraInfo {
    #[doc = "Request identifier. Used to match this information to another responseReceived event."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "A list of cookies which were not stored from the response along with the corresponding\nreasons for blocking. The cookies here may not be valid due to syntax errors, which\nare represented by the invalid cookie line string instead of a proper cookie."]
    #[serde(rename = "blockedCookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub blocked_cookies: Vec<super::types::BlockedSetCookieWithReason>,
    #[doc = "Raw response headers as they were received over the wire.\nDuplicate headers in the response are represented as a single key with their values\nconcatentated using `\\n` as the separator.\nSee also `headersText` that contains verbatim text for HTTP/1.*."]
    #[serde(rename = "headers")]
    pub headers: super::types::Headers,
    #[doc = "The IP address space of the resource. The address space can only be determined once the transport\nestablished the connection, so we can't send it in `requestWillBeSentExtraInfo`."]
    #[serde(rename = "resourceIPAddressSpace")]
    pub resource_ip_address_space: super::types::IpAddressSpace,
    #[doc = "The status code of the response. This is useful in cases the request failed and no responseReceived\nevent is triggered, which is the case for, e.g., CORS errors. This is also the correct status code\nfor cached requests, where the status in responseReceived is a 200 and this will be 304."]
    #[serde(rename = "statusCode")]
    pub status_code: i64,
    #[doc = "Raw response header text as it was received over the wire. The raw text may not always be\navailable, such as in the case of HTTP/2 or QUIC."]
    #[serde(rename = "headersText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub headers_text: Option<String>,
    #[doc = "The cookie partition key that will be used to store partitioned cookies set in this response.\nOnly sent when partitioned cookies are enabled."]
    #[serde(rename = "cookiePartitionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cookie_partition_key: Option<super::types::CookiePartitionKey>,
    #[doc = "True if partitioned cookies are enabled, but the partition key is not serializable to string."]
    #[serde(rename = "cookiePartitionKeyOpaque")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cookie_partition_key_opaque: Option<bool>,
    #[doc = "A list of cookies which should have been blocked by 3PCD but are exempted and stored from\nthe response with the corresponding reason."]
    #[serde(rename = "exemptedCookies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exempted_cookies: Option<Vec<super::types::ExemptedSetCookieWithReason>>,
}
impl ResponseReceivedExtraInfo {
    pub const IDENTIFIER: &'static str = "Network.responseReceivedExtraInfo";
}
#[doc = "Fired when 103 Early Hints headers is received in addition to the common response.\nNot every responseReceived event will have an responseReceivedEarlyHints fired.\nOnly one responseReceivedEarlyHints may be fired for eached responseReceived event.\n[responseReceivedEarlyHints](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-responseReceivedEarlyHints)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseReceivedEarlyHints {
    #[doc = "Request identifier. Used to match this information to another responseReceived event."]
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Raw response headers as they were received over the wire.\nDuplicate headers in the response are represented as a single key with their values\nconcatentated using `\\n` as the separator.\nSee also `headersText` that contains verbatim text for HTTP/1.*."]
    #[serde(rename = "headers")]
    pub headers: super::types::Headers,
}
impl ResponseReceivedEarlyHints {
    pub const IDENTIFIER: &'static str = "Network.responseReceivedEarlyHints";
}
#[doc = "Fired exactly once for each Trust Token operation. Depending on\nthe type of the operation and whether the operation succeeded or\nfailed, the event is fired before the corresponding request was sent\nor after the response was received.\n[trustTokenOperationDone](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-trustTokenOperationDone)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrustTokenOperationDone {
    #[doc = "Detailed success or error status of the operation.\n'AlreadyExists' also signifies a successful operation, as the result\nof the operation already exists und thus, the operation was abort\npreemptively (e.g. a cache hit)."]
    #[serde(rename = "status")]
    pub status: TrustTokenOperationDoneStatus,
    #[serde(rename = "type")]
    pub r#type: super::types::TrustTokenOperationType,
    #[serde(rename = "requestId")]
    pub request_id: super::types::RequestId,
    #[doc = "Top level origin. The context in which the operation was attempted."]
    #[serde(rename = "topLevelOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub top_level_origin: Option<String>,
    #[doc = "Origin of the issuer in case of a \"Issuance\" or \"Redemption\" operation."]
    #[serde(rename = "issuerOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub issuer_origin: Option<String>,
    #[doc = "The number of obtained Trust Tokens on a successful \"Issuance\" operation."]
    #[serde(rename = "issuedTokenCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub issued_token_count: Option<i64>,
}
#[doc = "Detailed success or error status of the operation.\n'AlreadyExists' also signifies a successful operation, as the result\nof the operation already exists und thus, the operation was abort\npreemptively (e.g. a cache hit)."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrustTokenOperationDoneStatus {
    #[serde(rename = "Ok")]
    Ok,
    #[serde(rename = "InvalidArgument")]
    InvalidArgument,
    #[serde(rename = "MissingIssuerKeys")]
    MissingIssuerKeys,
    #[serde(rename = "FailedPrecondition")]
    FailedPrecondition,
    #[serde(rename = "ResourceExhausted")]
    ResourceExhausted,
    #[serde(rename = "AlreadyExists")]
    AlreadyExists,
    #[serde(rename = "ResourceLimited")]
    ResourceLimited,
    #[serde(rename = "Unauthorized")]
    Unauthorized,
    #[serde(rename = "BadResponse")]
    BadResponse,
    #[serde(rename = "InternalError")]
    InternalError,
    #[serde(rename = "UnknownError")]
    UnknownError,
    #[serde(rename = "FulfilledLocally")]
    FulfilledLocally,
    #[serde(rename = "SiteIssuerLimit")]
    SiteIssuerLimit,
}
impl TrustTokenOperationDone {
    pub const IDENTIFIER: &'static str = "Network.trustTokenOperationDone";
}
#[doc = "Fired once security policy has been updated.\n[policyUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-policyUpdated)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PolicyUpdated {}
impl PolicyUpdated {
    pub const IDENTIFIER: &'static str = "Network.policyUpdated";
}
#[doc = "Is sent whenever a new report is added.\nAnd after 'enableReportingApi' for all existing reports.\n[reportingApiReportAdded](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-reportingApiReportAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportingApiReportAdded {
    #[serde(rename = "report")]
    pub report: super::types::ReportingApiReport,
}
impl ReportingApiReportAdded {
    pub const IDENTIFIER: &'static str = "Network.reportingApiReportAdded";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportingApiReportUpdated {
    #[serde(rename = "report")]
    pub report: super::types::ReportingApiReport,
}
impl ReportingApiReportUpdated {
    pub const IDENTIFIER: &'static str = "Network.reportingApiReportUpdated";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportingApiEndpointsChangedForOrigin {
    #[doc = "Origin of the document(s) which configured the endpoints."]
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(rename = "endpoints")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<super::types::ReportingApiEndpoint>,
}
impl ReportingApiEndpointsChangedForOrigin {
    pub const IDENTIFIER: &'static str = "Network.reportingApiEndpointsChangedForOrigin";
}
#[doc = "Triggered when the initial set of device bound sessions is added.\n[deviceBoundSessionsAdded](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-deviceBoundSessionsAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceBoundSessionsAdded {
    #[doc = "The device bound sessions."]
    #[serde(rename = "sessions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sessions: Vec<super::types::DeviceBoundSession>,
}
impl DeviceBoundSessionsAdded {
    pub const IDENTIFIER: &'static str = "Network.deviceBoundSessionsAdded";
}
#[doc = "Triggered when a device bound session event occurs.\n[deviceBoundSessionEventOccurred](https://chromedevtools.github.io/devtools-protocol/tot/Network/#event-deviceBoundSessionEventOccurred)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceBoundSessionEventOccurred {
    #[doc = "A unique identifier for this session event."]
    #[serde(rename = "eventId")]
    pub event_id: super::types::DeviceBoundSessionEventId,
    #[doc = "The site this session event is associated with."]
    #[serde(rename = "site")]
    pub site: String,
    #[doc = "Whether this event was considered successful."]
    #[serde(rename = "succeeded")]
    pub succeeded: bool,
    #[doc = "The session ID this event is associated with. May not be populated for\nfailed events."]
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub session_id: Option<String>,
    #[doc = "The below are the different session event type details. Exactly one is populated."]
    #[serde(rename = "creationEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub creation_event_details: Option<super::types::CreationEventDetails>,
    #[serde(rename = "refreshEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub refresh_event_details: Option<super::types::RefreshEventDetails>,
    #[serde(rename = "terminationEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub termination_event_details: Option<super::types::TerminationEventDetails>,
    #[serde(rename = "challengeEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub challenge_event_details: Option<super::types::ChallengeEventDetails>,
}
impl DeviceBoundSessionEventOccurred {
    pub const IDENTIFIER: &'static str = "Network.deviceBoundSessionEventOccurred";
}
group_enum ! (Event { DataReceived (DataReceived) , EventSourceMessageReceived (EventSourceMessageReceived) , LoadingFailed (LoadingFailed) , LoadingFinished (LoadingFinished) , RequestServedFromCache (RequestServedFromCache) , RequestWillBeSent (RequestWillBeSent) , ResourceChangedPriority (ResourceChangedPriority) , SignedExchangeReceived (SignedExchangeReceived) , ResponseReceived (ResponseReceived) , WebSocketClosed (WebSocketClosed) , WebSocketCreated (WebSocketCreated) , WebSocketFrameError (WebSocketFrameError) , WebSocketFrameReceived (WebSocketFrameReceived) , WebSocketFrameSent (WebSocketFrameSent) , WebSocketHandshakeResponseReceived (WebSocketHandshakeResponseReceived) , WebSocketWillSendHandshakeRequest (WebSocketWillSendHandshakeRequest) , WebTransportCreated (WebTransportCreated) , WebTransportConnectionEstablished (WebTransportConnectionEstablished) , WebTransportClosed (WebTransportClosed) , DirectTcpSocketCreated (DirectTcpSocketCreated) , DirectTcpSocketOpened (DirectTcpSocketOpened) , DirectTcpSocketAborted (DirectTcpSocketAborted) , DirectTcpSocketClosed (DirectTcpSocketClosed) , DirectTcpSocketChunkSent (DirectTcpSocketChunkSent) , DirectTcpSocketChunkReceived (DirectTcpSocketChunkReceived) , DirectUdpSocketJoinedMulticastGroup (DirectUdpSocketJoinedMulticastGroup) , DirectUdpSocketLeftMulticastGroup (DirectUdpSocketLeftMulticastGroup) , DirectUdpSocketCreated (DirectUdpSocketCreated) , DirectUdpSocketOpened (DirectUdpSocketOpened) , DirectUdpSocketAborted (DirectUdpSocketAborted) , DirectUdpSocketClosed (DirectUdpSocketClosed) , DirectUdpSocketChunkSent (DirectUdpSocketChunkSent) , DirectUdpSocketChunkReceived (DirectUdpSocketChunkReceived) , RequestWillBeSentExtraInfo (RequestWillBeSentExtraInfo) , ResponseReceivedExtraInfo (ResponseReceivedExtraInfo) , ResponseReceivedEarlyHints (ResponseReceivedEarlyHints) , TrustTokenOperationDone (TrustTokenOperationDone) , PolicyUpdated (PolicyUpdated) , ReportingApiReportAdded (ReportingApiReportAdded) , ReportingApiReportUpdated (ReportingApiReportUpdated) , ReportingApiEndpointsChangedForOrigin (ReportingApiEndpointsChangedForOrigin) , DeviceBoundSessionsAdded (DeviceBoundSessionsAdded) , DeviceBoundSessionEventOccurred (DeviceBoundSessionEventOccurred) });
