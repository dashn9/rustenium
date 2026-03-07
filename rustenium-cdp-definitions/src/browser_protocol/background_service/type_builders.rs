use super::types::*;
impl EventMetadata {
    pub fn builder() -> EventMetadataBuilder {
        <EventMetadataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct EventMetadataBuilder {
    key: Option<String>,
    value: Option<String>,
}
impl EventMetadataBuilder {
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<EventMetadata, String> {
        Ok(EventMetadata {
            key: self
                .key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl BackgroundServiceEvent {
    pub fn builder() -> BackgroundServiceEventBuilder {
        <BackgroundServiceEventBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BackgroundServiceEventBuilder {
    timestamp: Option<crate::browser_protocol::network::types::TimeSinceEpoch>,
    origin: Option<String>,
    service_worker_registration_id:
        Option<crate::browser_protocol::service_worker::types::RegistrationId>,
    service: Option<ServiceName>,
    event_name: Option<String>,
    instance_id: Option<String>,
    event_metadata: Option<Vec<EventMetadata>>,
    storage_key: Option<String>,
}
impl BackgroundServiceEventBuilder {
    pub fn timestamp(
        mut self,
        timestamp: impl Into<crate::browser_protocol::network::types::TimeSinceEpoch>,
    ) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn service_worker_registration_id(
        mut self,
        service_worker_registration_id: impl Into<
            crate::browser_protocol::service_worker::types::RegistrationId,
        >,
    ) -> Self {
        self.service_worker_registration_id = Some(service_worker_registration_id.into());
        self
    }
    pub fn service(mut self, service: impl Into<ServiceName>) -> Self {
        self.service = Some(service.into());
        self
    }
    pub fn event_name(mut self, event_name: impl Into<String>) -> Self {
        self.event_name = Some(event_name.into());
        self
    }
    pub fn instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.instance_id = Some(instance_id.into());
        self
    }
    pub fn event_metadata(mut self, event_metadata: impl Into<EventMetadata>) -> Self {
        let v = self.event_metadata.get_or_insert(Vec::new());
        v.push(event_metadata.into());
        self
    }
    pub fn event_metadatas<I, S>(mut self, event_metadatas: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<EventMetadata>,
    {
        let v = self.event_metadata.get_or_insert(Vec::new());
        for val in event_metadatas {
            v.push(val.into());
        }
        self
    }
    pub fn storage_key(mut self, storage_key: impl Into<String>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn build(self) -> Result<BackgroundServiceEvent, String> {
        Ok(BackgroundServiceEvent {
            timestamp: self
                .timestamp
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(timestamp)))?,
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            service_worker_registration_id: self.service_worker_registration_id.ok_or_else(
                || {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(service_worker_registration_id)
                    )
                },
            )?,
            service: self
                .service
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(service)))?,
            event_name: self
                .event_name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(event_name)))?,
            instance_id: self
                .instance_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(instance_id)))?,
            event_metadata: self.event_metadata.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(event_metadata))
            })?,
            storage_key: self
                .storage_key
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(storage_key)))?,
        })
    }
}
