use serde::{Deserialize, Serialize};
#[doc = "Issued when attached to target because of auto-attach or `attachToTarget` command.\n[attachedToTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#event-attachedToTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachedToTargetParams {
    #[doc = "Identifier assigned to the session used to send/receive messages."]
    #[serde(rename = "sessionId")]
    pub session_id: super::types::SessionId,
    #[serde(rename = "targetInfo")]
    pub target_info: super::types::TargetInfo,
    #[serde(rename = "waitingForDebugger")]
    pub waiting_for_debugger: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttachedToTargetMethod {
    #[serde(rename = "Target.attachedToTarget")]
    AttachedToTarget,
}
#[doc = "Issued when attached to target because of auto-attach or `attachToTarget` command.\n[attachedToTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#event-attachedToTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachedToTarget {
    pub method: AttachedToTargetMethod,
    pub params: AttachedToTargetParams,
}
impl AttachedToTarget {
    pub const IDENTIFIER: &'static str = "Target.attachedToTarget";
}
#[doc = "Issued when detached from target for any reason (including `detachFromTarget` command). Can be\nissued multiple times per target if multiple sessions have been attached to it.\n[detachedFromTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#event-detachedFromTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetachedFromTargetParams {
    #[doc = "Detached session identifier."]
    #[serde(rename = "sessionId")]
    pub session_id: super::types::SessionId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DetachedFromTargetMethod {
    #[serde(rename = "Target.detachedFromTarget")]
    DetachedFromTarget,
}
#[doc = "Issued when detached from target for any reason (including `detachFromTarget` command). Can be\nissued multiple times per target if multiple sessions have been attached to it.\n[detachedFromTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#event-detachedFromTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetachedFromTarget {
    pub method: DetachedFromTargetMethod,
    pub params: DetachedFromTargetParams,
}
impl DetachedFromTarget {
    pub const IDENTIFIER: &'static str = "Target.detachedFromTarget";
}
#[doc = "Notifies about a new protocol message received from the session (as reported in\n`attachedToTarget` event).\n[receivedMessageFromTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#event-receivedMessageFromTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceivedMessageFromTargetParams {
    #[doc = "Identifier of a session which sends a message."]
    #[serde(rename = "sessionId")]
    pub session_id: super::types::SessionId,
    #[serde(rename = "message")]
    pub message: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReceivedMessageFromTargetMethod {
    #[serde(rename = "Target.receivedMessageFromTarget")]
    ReceivedMessageFromTarget,
}
#[doc = "Notifies about a new protocol message received from the session (as reported in\n`attachedToTarget` event).\n[receivedMessageFromTarget](https://chromedevtools.github.io/devtools-protocol/tot/Target/#event-receivedMessageFromTarget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceivedMessageFromTarget {
    pub method: ReceivedMessageFromTargetMethod,
    pub params: ReceivedMessageFromTargetParams,
}
impl ReceivedMessageFromTarget {
    pub const IDENTIFIER: &'static str = "Target.receivedMessageFromTarget";
}
#[doc = "Issued when a possible inspection target is created.\n[targetCreated](https://chromedevtools.github.io/devtools-protocol/tot/Target/#event-targetCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetCreatedParams {
    #[serde(rename = "targetInfo")]
    pub target_info: super::types::TargetInfo,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TargetCreatedMethod {
    #[serde(rename = "Target.targetCreated")]
    TargetCreated,
}
#[doc = "Issued when a possible inspection target is created.\n[targetCreated](https://chromedevtools.github.io/devtools-protocol/tot/Target/#event-targetCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetCreated {
    pub method: TargetCreatedMethod,
    pub params: TargetCreatedParams,
}
impl TargetCreated {
    pub const IDENTIFIER: &'static str = "Target.targetCreated";
}
#[doc = "Issued when a target is destroyed.\n[targetDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/Target/#event-targetDestroyed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetDestroyedParams {
    #[serde(rename = "targetId")]
    pub target_id: super::types::TargetId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TargetDestroyedMethod {
    #[serde(rename = "Target.targetDestroyed")]
    TargetDestroyed,
}
#[doc = "Issued when a target is destroyed.\n[targetDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/Target/#event-targetDestroyed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetDestroyed {
    pub method: TargetDestroyedMethod,
    pub params: TargetDestroyedParams,
}
impl TargetDestroyed {
    pub const IDENTIFIER: &'static str = "Target.targetDestroyed";
}
#[doc = "Issued when a target has crashed.\n[targetCrashed](https://chromedevtools.github.io/devtools-protocol/tot/Target/#event-targetCrashed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetCrashedParams {
    #[serde(rename = "targetId")]
    pub target_id: super::types::TargetId,
    #[doc = "Termination status type."]
    #[serde(rename = "status")]
    pub status: String,
    #[doc = "Termination error code."]
    #[serde(rename = "errorCode")]
    pub error_code: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TargetCrashedMethod {
    #[serde(rename = "Target.targetCrashed")]
    TargetCrashed,
}
#[doc = "Issued when a target has crashed.\n[targetCrashed](https://chromedevtools.github.io/devtools-protocol/tot/Target/#event-targetCrashed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetCrashed {
    pub method: TargetCrashedMethod,
    pub params: TargetCrashedParams,
}
impl TargetCrashed {
    pub const IDENTIFIER: &'static str = "Target.targetCrashed";
}
#[doc = "Issued when some information about a target has changed. This only happens between\n`targetCreated` and `targetDestroyed`.\n[targetInfoChanged](https://chromedevtools.github.io/devtools-protocol/tot/Target/#event-targetInfoChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetInfoChangedParams {
    #[serde(rename = "targetInfo")]
    pub target_info: super::types::TargetInfo,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TargetInfoChangedMethod {
    #[serde(rename = "Target.targetInfoChanged")]
    TargetInfoChanged,
}
#[doc = "Issued when some information about a target has changed. This only happens between\n`targetCreated` and `targetDestroyed`.\n[targetInfoChanged](https://chromedevtools.github.io/devtools-protocol/tot/Target/#event-targetInfoChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetInfoChanged {
    pub method: TargetInfoChangedMethod,
    pub params: TargetInfoChangedParams,
}
impl TargetInfoChanged {
    pub const IDENTIFIER: &'static str = "Target.targetInfoChanged";
}
group_enum ! (TargetEvents { AttachedToTarget (AttachedToTarget) , DetachedFromTarget (DetachedFromTarget) , ReceivedMessageFromTarget (ReceivedMessageFromTarget) , TargetCreated (TargetCreated) , TargetDestroyed (TargetDestroyed) , TargetCrashed (TargetCrashed) , TargetInfoChanged (TargetInfoChanged) });
