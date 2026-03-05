use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSinkToUseResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartDesktopMirroringResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartTabMirroringResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopCastingResult {}
