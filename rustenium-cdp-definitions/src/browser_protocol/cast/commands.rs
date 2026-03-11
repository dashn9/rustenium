use serde::{Deserialize, Serialize};
#[doc = "Starts observing for sinks that can be used for tab mirroring, and if set,\nsinks compatible with |presentationUrl| as well. When sinks are found, a\n|sinksUpdated| event is fired.\nAlso starts observing for issue messages. When an issue is added or removed,\nan |issueUpdated| event is fired.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {
    #[serde(rename = "presentationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub presentation_url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Cast.enable")]
    Enable,
}
#[doc = "Starts observing for sinks that can be used for tab mirroring, and if set,\nsinks compatible with |presentationUrl| as well. When sinks are found, a\n|sinksUpdated| event is fired.\nAlso starts observing for issue messages. When an issue is added or removed,\nan |issueUpdated| event is fired.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "Cast.enable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Stops observing for sinks and issues.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Cast.disable")]
    Disable,
}
#[doc = "Stops observing for sinks and issues.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "Cast.disable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Sets a sink to be used when the web page requests the browser to choose a\nsink via Presentation API, Remote Playback API, or Cast SDK.\n[setSinkToUse](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#method-setSinkToUse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSinkToUseParams {
    #[serde(rename = "sinkName")]
    pub sink_name: String,
}
impl SetSinkToUseParams {
    pub fn new(sink_name: impl Into<String>) -> Self {
        Self {
            sink_name: sink_name.into(),
        }
    }
}
impl<T: Into<String>> From<T> for SetSinkToUseParams {
    fn from(url: T) -> Self {
        SetSinkToUseParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetSinkToUseMethod {
    #[serde(rename = "Cast.setSinkToUse")]
    SetSinkToUse,
}
#[doc = "Sets a sink to be used when the web page requests the browser to choose a\nsink via Presentation API, Remote Playback API, or Cast SDK.\n[setSinkToUse](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#method-setSinkToUse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSinkToUse {
    pub method: SetSinkToUseMethod,
    pub params: SetSinkToUseParams,
}
impl SetSinkToUse {
    pub const IDENTIFIER: &'static str = "Cast.setSinkToUse";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetSinkToUse {
    type Result = super::results::SetSinkToUseResult;
}
#[doc = "Starts mirroring the desktop to the sink.\n[startDesktopMirroring](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#method-startDesktopMirroring)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartDesktopMirroringParams {
    #[serde(rename = "sinkName")]
    pub sink_name: String,
}
impl StartDesktopMirroringParams {
    pub fn new(sink_name: impl Into<String>) -> Self {
        Self {
            sink_name: sink_name.into(),
        }
    }
}
impl<T: Into<String>> From<T> for StartDesktopMirroringParams {
    fn from(url: T) -> Self {
        StartDesktopMirroringParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartDesktopMirroringMethod {
    #[serde(rename = "Cast.startDesktopMirroring")]
    StartDesktopMirroring,
}
#[doc = "Starts mirroring the desktop to the sink.\n[startDesktopMirroring](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#method-startDesktopMirroring)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartDesktopMirroring {
    pub method: StartDesktopMirroringMethod,
    pub params: StartDesktopMirroringParams,
}
impl StartDesktopMirroring {
    pub const IDENTIFIER: &'static str = "Cast.startDesktopMirroring";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StartDesktopMirroring {
    type Result = super::results::StartDesktopMirroringResult;
}
#[doc = "Starts mirroring the tab to the sink.\n[startTabMirroring](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#method-startTabMirroring)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartTabMirroringParams {
    #[serde(rename = "sinkName")]
    pub sink_name: String,
}
impl StartTabMirroringParams {
    pub fn new(sink_name: impl Into<String>) -> Self {
        Self {
            sink_name: sink_name.into(),
        }
    }
}
impl<T: Into<String>> From<T> for StartTabMirroringParams {
    fn from(url: T) -> Self {
        StartTabMirroringParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartTabMirroringMethod {
    #[serde(rename = "Cast.startTabMirroring")]
    StartTabMirroring,
}
#[doc = "Starts mirroring the tab to the sink.\n[startTabMirroring](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#method-startTabMirroring)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartTabMirroring {
    pub method: StartTabMirroringMethod,
    pub params: StartTabMirroringParams,
}
impl StartTabMirroring {
    pub const IDENTIFIER: &'static str = "Cast.startTabMirroring";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StartTabMirroring {
    type Result = super::results::StartTabMirroringResult;
}
#[doc = "Stops the active Cast session on the sink.\n[stopCasting](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#method-stopCasting)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopCastingParams {
    #[serde(rename = "sinkName")]
    pub sink_name: String,
}
impl StopCastingParams {
    pub fn new(sink_name: impl Into<String>) -> Self {
        Self {
            sink_name: sink_name.into(),
        }
    }
}
impl<T: Into<String>> From<T> for StopCastingParams {
    fn from(url: T) -> Self {
        StopCastingParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopCastingMethod {
    #[serde(rename = "Cast.stopCasting")]
    StopCasting,
}
#[doc = "Stops the active Cast session on the sink.\n[stopCasting](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#method-stopCasting)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopCasting {
    pub method: StopCastingMethod,
    pub params: StopCastingParams,
}
impl StopCasting {
    pub const IDENTIFIER: &'static str = "Cast.stopCasting";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StopCasting {
    type Result = super::results::StopCastingResult;
}
group_enum ! (CastCommands { Enable (Enable) , Disable (Disable) , SetSinkToUse (SetSinkToUse) , StartDesktopMirroring (StartDesktopMirroring) , StartTabMirroring (StartTabMirroring) , StopCasting (StopCasting) });
