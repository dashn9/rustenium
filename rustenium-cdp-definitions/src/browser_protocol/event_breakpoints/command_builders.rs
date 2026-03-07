use super::commands::*;
impl SetInstrumentationBreakpoint {
    pub fn builder() -> SetInstrumentationBreakpointBuilder {
        <SetInstrumentationBreakpointBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetInstrumentationBreakpointBuilder {
    event_name: Option<String>,
}
impl SetInstrumentationBreakpointBuilder {
    pub fn event_name(mut self, event_name: impl Into<String>) -> Self {
        self.event_name = Some(event_name.into());
        self
    }
    pub fn build(self) -> Result<SetInstrumentationBreakpoint, String> {
        Ok(SetInstrumentationBreakpoint {
            method: SetInstrumentationBreakpointMethod::SetInstrumentationBreakpoint,
            params: SetInstrumentationBreakpointParams {
                event_name: self.event_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(event_name))
                })?,
            },
        })
    }
}
impl RemoveInstrumentationBreakpoint {
    pub fn builder() -> RemoveInstrumentationBreakpointBuilder {
        <RemoveInstrumentationBreakpointBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveInstrumentationBreakpointBuilder {
    event_name: Option<String>,
}
impl RemoveInstrumentationBreakpointBuilder {
    pub fn event_name(mut self, event_name: impl Into<String>) -> Self {
        self.event_name = Some(event_name.into());
        self
    }
    pub fn build(self) -> Result<RemoveInstrumentationBreakpoint, String> {
        Ok(RemoveInstrumentationBreakpoint {
            method: RemoveInstrumentationBreakpointMethod::RemoveInstrumentationBreakpoint,
            params: RemoveInstrumentationBreakpointParams {
                event_name: self.event_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(event_name))
                })?,
            },
        })
    }
}
