use super::commands::*;
impl Enable {
    pub fn builder() -> EnableBuilder {
        <EnableBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct EnableBuilder {
    presentation_url: Option<String>,
}
impl EnableBuilder {
    pub fn presentation_url(mut self, presentation_url: impl Into<String>) -> Self {
        self.presentation_url = Some(presentation_url.into());
        self
    }
    pub fn build(self) -> Enable {
        Enable {
            method: EnableMethod::Enable,
            params: EnableParams {
                presentation_url: self.presentation_url,
            },
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct DisableBuilder;
impl DisableBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Disable {
        Disable {
            method: DisableMethod::Disable,
            params: DisableParams {},
        }
    }
}
impl Disable {
    pub fn builder() -> DisableBuilder {
        DisableBuilder
    }
}
impl SetSinkToUse {
    pub fn builder() -> SetSinkToUseBuilder {
        <SetSinkToUseBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetSinkToUseBuilder {
    sink_name: Option<String>,
}
impl SetSinkToUseBuilder {
    pub fn sink_name(mut self, sink_name: impl Into<String>) -> Self {
        self.sink_name = Some(sink_name.into());
        self
    }
    pub fn build(self) -> Result<SetSinkToUse, String> {
        Ok(SetSinkToUse {
            method: SetSinkToUseMethod::SetSinkToUse,
            params: SetSinkToUseParams {
                sink_name: self.sink_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(sink_name))
                })?,
            },
        })
    }
}
impl StartDesktopMirroring {
    pub fn builder() -> StartDesktopMirroringBuilder {
        <StartDesktopMirroringBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StartDesktopMirroringBuilder {
    sink_name: Option<String>,
}
impl StartDesktopMirroringBuilder {
    pub fn sink_name(mut self, sink_name: impl Into<String>) -> Self {
        self.sink_name = Some(sink_name.into());
        self
    }
    pub fn build(self) -> Result<StartDesktopMirroring, String> {
        Ok(StartDesktopMirroring {
            method: StartDesktopMirroringMethod::StartDesktopMirroring,
            params: StartDesktopMirroringParams {
                sink_name: self.sink_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(sink_name))
                })?,
            },
        })
    }
}
impl StartTabMirroring {
    pub fn builder() -> StartTabMirroringBuilder {
        <StartTabMirroringBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StartTabMirroringBuilder {
    sink_name: Option<String>,
}
impl StartTabMirroringBuilder {
    pub fn sink_name(mut self, sink_name: impl Into<String>) -> Self {
        self.sink_name = Some(sink_name.into());
        self
    }
    pub fn build(self) -> Result<StartTabMirroring, String> {
        Ok(StartTabMirroring {
            method: StartTabMirroringMethod::StartTabMirroring,
            params: StartTabMirroringParams {
                sink_name: self.sink_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(sink_name))
                })?,
            },
        })
    }
}
impl StopCasting {
    pub fn builder() -> StopCastingBuilder {
        <StopCastingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StopCastingBuilder {
    sink_name: Option<String>,
}
impl StopCastingBuilder {
    pub fn sink_name(mut self, sink_name: impl Into<String>) -> Self {
        self.sink_name = Some(sink_name.into());
        self
    }
    pub fn build(self) -> Result<StopCasting, String> {
        Ok(StopCasting {
            method: StopCastingMethod::StopCasting,
            params: StopCastingParams {
                sink_name: self.sink_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(sink_name))
                })?,
            },
        })
    }
}
