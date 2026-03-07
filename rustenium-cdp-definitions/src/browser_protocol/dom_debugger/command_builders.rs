use super::commands::*;
impl GetEventListeners {
    pub fn builder() -> GetEventListenersBuilder {
        <GetEventListenersBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetEventListenersBuilder {
    object_id: Option<crate::js_protocol::runtime::types::RemoteObjectId>,
    depth: Option<i64>,
    pierce: Option<bool>,
}
impl GetEventListenersBuilder {
    pub fn object_id(
        mut self,
        object_id: impl Into<crate::js_protocol::runtime::types::RemoteObjectId>,
    ) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn depth(mut self, depth: impl Into<i64>) -> Self {
        self.depth = Some(depth.into());
        self
    }
    pub fn pierce(mut self, pierce: impl Into<bool>) -> Self {
        self.pierce = Some(pierce.into());
        self
    }
    pub fn build(self) -> Result<GetEventListeners, String> {
        Ok(GetEventListeners {
            method: GetEventListenersMethod::GetEventListeners,
            params: GetEventListenersParams {
                object_id: self.object_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(object_id))
                })?,
                depth: self.depth,
                pierce: self.pierce,
            },
        })
    }
}
impl RemoveDomBreakpoint {
    pub fn builder() -> RemoveDomBreakpointBuilder {
        <RemoveDomBreakpointBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveDomBreakpointBuilder {
    node_id: Option<crate::browser_protocol::dom::types::NodeId>,
    r#type: Option<super::types::DomBreakpointType>,
}
impl RemoveDomBreakpointBuilder {
    pub fn node_id(
        mut self,
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
    ) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<super::types::DomBreakpointType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<RemoveDomBreakpoint, String> {
        Ok(RemoveDomBreakpoint {
            method: RemoveDomBreakpointMethod::RemoveDomBreakpoint,
            params: RemoveDomBreakpointParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                r#type: self
                    .r#type
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            },
        })
    }
}
impl RemoveEventListenerBreakpoint {
    pub fn builder() -> RemoveEventListenerBreakpointBuilder {
        <RemoveEventListenerBreakpointBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveEventListenerBreakpointBuilder {
    event_name: Option<String>,
    target_name: Option<String>,
}
impl RemoveEventListenerBreakpointBuilder {
    pub fn event_name(mut self, event_name: impl Into<String>) -> Self {
        self.event_name = Some(event_name.into());
        self
    }
    pub fn target_name(mut self, target_name: impl Into<String>) -> Self {
        self.target_name = Some(target_name.into());
        self
    }
    pub fn build(self) -> Result<RemoveEventListenerBreakpoint, String> {
        Ok(RemoveEventListenerBreakpoint {
            method: RemoveEventListenerBreakpointMethod::RemoveEventListenerBreakpoint,
            params: RemoveEventListenerBreakpointParams {
                event_name: self.event_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(event_name))
                })?,
                target_name: self.target_name,
            },
        })
    }
}
impl RemoveXhrBreakpoint {
    pub fn builder() -> RemoveXhrBreakpointBuilder {
        <RemoveXhrBreakpointBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveXhrBreakpointBuilder {
    url: Option<String>,
}
impl RemoveXhrBreakpointBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn build(self) -> Result<RemoveXhrBreakpoint, String> {
        Ok(RemoveXhrBreakpoint {
            method: RemoveXhrBreakpointMethod::RemoveXhrBreakpoint,
            params: RemoveXhrBreakpointParams {
                url: self
                    .url
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            },
        })
    }
}
impl SetBreakOnCspViolation {
    pub fn builder() -> SetBreakOnCspViolationBuilder {
        <SetBreakOnCspViolationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetBreakOnCspViolationBuilder {
    violation_types: Option<Vec<super::types::CspViolationType>>,
}
impl SetBreakOnCspViolationBuilder {
    pub fn violation_type(
        mut self,
        violation_type: impl Into<super::types::CspViolationType>,
    ) -> Self {
        let v = self.violation_types.get_or_insert(Vec::new());
        v.push(violation_type.into());
        self
    }
    pub fn violation_types<I, S>(mut self, violation_types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::CspViolationType>,
    {
        let v = self.violation_types.get_or_insert(Vec::new());
        for val in violation_types {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetBreakOnCspViolation, String> {
        Ok(SetBreakOnCspViolation {
            method: SetBreakOnCspViolationMethod::SetBreakOnCspViolation,
            params: SetBreakOnCspViolationParams {
                violation_types: self.violation_types.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(violation_types))
                })?,
            },
        })
    }
}
impl SetDomBreakpoint {
    pub fn builder() -> SetDomBreakpointBuilder {
        <SetDomBreakpointBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetDomBreakpointBuilder {
    node_id: Option<crate::browser_protocol::dom::types::NodeId>,
    r#type: Option<super::types::DomBreakpointType>,
}
impl SetDomBreakpointBuilder {
    pub fn node_id(
        mut self,
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
    ) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<super::types::DomBreakpointType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<SetDomBreakpoint, String> {
        Ok(SetDomBreakpoint {
            method: SetDomBreakpointMethod::SetDomBreakpoint,
            params: SetDomBreakpointParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                r#type: self
                    .r#type
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            },
        })
    }
}
impl SetEventListenerBreakpoint {
    pub fn builder() -> SetEventListenerBreakpointBuilder {
        <SetEventListenerBreakpointBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetEventListenerBreakpointBuilder {
    event_name: Option<String>,
    target_name: Option<String>,
}
impl SetEventListenerBreakpointBuilder {
    pub fn event_name(mut self, event_name: impl Into<String>) -> Self {
        self.event_name = Some(event_name.into());
        self
    }
    pub fn target_name(mut self, target_name: impl Into<String>) -> Self {
        self.target_name = Some(target_name.into());
        self
    }
    pub fn build(self) -> Result<SetEventListenerBreakpoint, String> {
        Ok(SetEventListenerBreakpoint {
            method: SetEventListenerBreakpointMethod::SetEventListenerBreakpoint,
            params: SetEventListenerBreakpointParams {
                event_name: self.event_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(event_name))
                })?,
                target_name: self.target_name,
            },
        })
    }
}
impl SetXhrBreakpoint {
    pub fn builder() -> SetXhrBreakpointBuilder {
        <SetXhrBreakpointBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetXhrBreakpointBuilder {
    url: Option<String>,
}
impl SetXhrBreakpointBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn build(self) -> Result<SetXhrBreakpoint, String> {
        Ok(SetXhrBreakpoint {
            method: SetXhrBreakpointMethod::SetXhrBreakpoint,
            params: SetXhrBreakpointParams {
                url: self
                    .url
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            },
        })
    }
}
