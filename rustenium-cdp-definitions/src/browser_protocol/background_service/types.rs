use serde::{Deserialize, Serialize};
#[doc = "The Background Service that will be associated with the commands/events.\nEvery Background Service operates independently, but they share the same\nAPI."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceName {
    #[serde(rename = "backgroundFetch")]
    BackgroundFetch,
    #[serde(rename = "backgroundSync")]
    BackgroundSync,
    #[serde(rename = "pushMessaging")]
    PushMessaging,
    #[serde(rename = "notifications")]
    Notifications,
    #[serde(rename = "paymentHandler")]
    PaymentHandler,
    #[serde(rename = "periodicBackgroundSync")]
    PeriodicBackgroundSync,
}
#[doc = "A key-value pair for additional event information to pass along.\n[EventMetadata](https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#type-EventMetadata)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventMetadata {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl EventMetadata {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}
impl EventMetadata {
    pub const IDENTIFIER: &'static str = "BackgroundService.EventMetadata";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundServiceEvent {
    #[doc = "Timestamp of the event (in seconds)."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::super::network::types::TimeSinceEpoch,
    #[doc = "The origin this event belongs to."]
    #[serde(rename = "origin")]
    pub origin: String,
    #[doc = "The Service Worker ID that initiated the event."]
    #[serde(rename = "serviceWorkerRegistrationId")]
    pub service_worker_registration_id: super::super::service_worker::types::RegistrationId,
    #[doc = "The Background Service this event belongs to."]
    #[serde(rename = "service")]
    pub service: ServiceName,
    #[doc = "A description of the event."]
    #[serde(rename = "eventName")]
    pub event_name: String,
    #[doc = "An identifier that groups related events together."]
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[doc = "A list of event-specific information."]
    #[serde(rename = "eventMetadata")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub event_metadata: Vec<EventMetadata>,
    #[doc = "Storage key this event belongs to."]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
}
impl BackgroundServiceEvent {
    pub const IDENTIFIER: &'static str = "BackgroundService.BackgroundServiceEvent";
}
group_enum ! (Type { ServiceName (ServiceName) , EventMetadata (EventMetadata) , BackgroundServiceEvent (BackgroundServiceEvent) });
