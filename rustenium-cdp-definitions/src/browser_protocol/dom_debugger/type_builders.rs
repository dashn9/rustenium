use super::types::*;
impl EventListener {
    pub fn builder() -> EventListenerBuilder {
        EventListenerBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct EventListenerBuilder {
    r#type: Option<String>,
    use_capture: Option<bool>,
    passive: Option<bool>,
    once: Option<bool>,
    script_id: Option<super::super::super::js_protocol::runtime::types::ScriptId>,
    line_number: Option<i64>,
    column_number: Option<i64>,
    handler: Option<super::super::super::js_protocol::runtime::types::RemoteObject>,
    original_handler: Option<super::super::super::js_protocol::runtime::types::RemoteObject>,
    backend_node_id: Option<super::super::dom::types::BackendNodeId>,
}
impl EventListenerBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn use_capture(mut self, use_capture: impl Into<bool>) -> Self {
        self.use_capture = Some(use_capture.into());
        self
    }
    pub fn passive(mut self, passive: impl Into<bool>) -> Self {
        self.passive = Some(passive.into());
        self
    }
    pub fn once(mut self, once: impl Into<bool>) -> Self {
        self.once = Some(once.into());
        self
    }
    pub fn script_id(
        mut self,
        script_id: impl Into<super::super::super::js_protocol::runtime::types::ScriptId>,
    ) -> Self {
        self.script_id = Some(script_id.into());
        self
    }
    pub fn line_number(mut self, line_number: impl Into<i64>) -> Self {
        self.line_number = Some(line_number.into());
        self
    }
    pub fn column_number(mut self, column_number: impl Into<i64>) -> Self {
        self.column_number = Some(column_number.into());
        self
    }
    pub fn handler(
        mut self,
        handler: impl Into<super::super::super::js_protocol::runtime::types::RemoteObject>,
    ) -> Self {
        self.handler = Some(handler.into());
        self
    }
    pub fn original_handler(
        mut self,
        original_handler: impl Into<super::super::super::js_protocol::runtime::types::RemoteObject>,
    ) -> Self {
        self.original_handler = Some(original_handler.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<super::super::dom::types::BackendNodeId>,
    ) -> Self {
        self.backend_node_id = Some(backend_node_id.into());
        self
    }
    pub fn build(self) -> Result<EventListener, String> {
        Ok(EventListener {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            use_capture: self
                .use_capture
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(use_capture)))?,
            passive: self
                .passive
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(passive)))?,
            once: self
                .once
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(once)))?,
            script_id: self
                .script_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(script_id)))?,
            line_number: self
                .line_number
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(line_number)))?,
            column_number: self.column_number.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(column_number))
            })?,
            handler: self.handler,
            original_handler: self.original_handler,
            backend_node_id: self.backend_node_id,
        })
    }
}
