use serde::{Deserialize, Serialize};
#[doc = "Provides the reasons why the given layer was composited.\n[compositingReasons](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-compositingReasons)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompositingReasonsParams {
    #[doc = "The id of the layer for which we want to get the reasons it was composited."]
    #[serde(rename = "layerId")]
    pub layer_id: super::types::LayerId,
}
impl CompositingReasonsParams {
    pub fn new(layer_id: impl Into<super::types::LayerId>) -> Self {
        Self {
            layer_id: layer_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompositingReasonsMethod {
    #[serde(rename = "LayerTree.compositingReasons")]
    CompositingReasons,
}
impl CompositingReasonsMethod {
    pub const IDENTIFIER: &'static str = "LayerTree.compositingReasons";
}
#[doc = "Provides the reasons why the given layer was composited.\n[compositingReasons](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-compositingReasons)"]
#[derive(Debug, Clone, PartialEq)]
pub struct CompositingReasons {
    pub method: CompositingReasonsMethod,
    pub params: CompositingReasonsParams,
}
impl crate::CommandResult for CompositingReasons {
    type Result = super::results::CompositingReasonsResult;
}
#[doc = "Disables compositing tree inspection.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "LayerTree.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "LayerTree.disable";
}
#[doc = "Disables compositing tree inspection.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enables compositing tree inspection.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "LayerTree.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "LayerTree.enable";
}
#[doc = "Enables compositing tree inspection.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Returns the snapshot identifier.\n[loadSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-loadSnapshot)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadSnapshotParams {
    #[doc = "An array of tiles composing the snapshot."]
    #[serde(rename = "tiles")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tiles: Vec<super::types::PictureTile>,
}
impl LoadSnapshotParams {
    pub fn new(tiles: Vec<super::types::PictureTile>) -> Self {
        Self { tiles }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoadSnapshotMethod {
    #[serde(rename = "LayerTree.loadSnapshot")]
    LoadSnapshot,
}
impl LoadSnapshotMethod {
    pub const IDENTIFIER: &'static str = "LayerTree.loadSnapshot";
}
#[doc = "Returns the snapshot identifier.\n[loadSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-loadSnapshot)"]
#[derive(Debug, Clone, PartialEq)]
pub struct LoadSnapshot {
    pub method: LoadSnapshotMethod,
    pub params: LoadSnapshotParams,
}
impl crate::CommandResult for LoadSnapshot {
    type Result = super::results::LoadSnapshotResult;
}
#[doc = "Returns the layer snapshot identifier.\n[makeSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-makeSnapshot)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakeSnapshotParams {
    #[doc = "The id of the layer."]
    #[serde(rename = "layerId")]
    pub layer_id: super::types::LayerId,
}
impl MakeSnapshotParams {
    pub fn new(layer_id: impl Into<super::types::LayerId>) -> Self {
        Self {
            layer_id: layer_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MakeSnapshotMethod {
    #[serde(rename = "LayerTree.makeSnapshot")]
    MakeSnapshot,
}
impl MakeSnapshotMethod {
    pub const IDENTIFIER: &'static str = "LayerTree.makeSnapshot";
}
#[doc = "Returns the layer snapshot identifier.\n[makeSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-makeSnapshot)"]
#[derive(Debug, Clone, PartialEq)]
pub struct MakeSnapshot {
    pub method: MakeSnapshotMethod,
    pub params: MakeSnapshotParams,
}
impl crate::CommandResult for MakeSnapshot {
    type Result = super::results::MakeSnapshotResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileSnapshotParams {
    #[doc = "The id of the layer snapshot."]
    #[serde(rename = "snapshotId")]
    pub snapshot_id: super::types::SnapshotId,
    #[doc = "The maximum number of times to replay the snapshot (1, if not specified)."]
    #[serde(rename = "minRepeatCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_repeat_count: Option<i64>,
    #[doc = "The minimum duration (in seconds) to replay the snapshot."]
    #[serde(rename = "minDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_duration: Option<f64>,
    #[doc = "The clip rectangle to apply when replaying the snapshot."]
    #[serde(rename = "clipRect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub clip_rect: Option<crate::browser_protocol::dom::types::Rect>,
}
impl ProfileSnapshotParams {
    pub fn new(snapshot_id: impl Into<super::types::SnapshotId>) -> Self {
        Self {
            snapshot_id: snapshot_id.into(),
            min_repeat_count: None,
            min_duration: None,
            clip_rect: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProfileSnapshotMethod {
    #[serde(rename = "LayerTree.profileSnapshot")]
    ProfileSnapshot,
}
impl ProfileSnapshotMethod {
    pub const IDENTIFIER: &'static str = "LayerTree.profileSnapshot";
}
#[derive(Debug, Clone, PartialEq)]
pub struct ProfileSnapshot {
    pub method: ProfileSnapshotMethod,
    pub params: ProfileSnapshotParams,
}
impl crate::CommandResult for ProfileSnapshot {
    type Result = super::results::ProfileSnapshotResult;
}
#[doc = "Releases layer snapshot captured by the back-end.\n[releaseSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-releaseSnapshot)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseSnapshotParams {
    #[doc = "The id of the layer snapshot."]
    #[serde(rename = "snapshotId")]
    pub snapshot_id: super::types::SnapshotId,
}
impl ReleaseSnapshotParams {
    pub fn new(snapshot_id: impl Into<super::types::SnapshotId>) -> Self {
        Self {
            snapshot_id: snapshot_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReleaseSnapshotMethod {
    #[serde(rename = "LayerTree.releaseSnapshot")]
    ReleaseSnapshot,
}
impl ReleaseSnapshotMethod {
    pub const IDENTIFIER: &'static str = "LayerTree.releaseSnapshot";
}
#[doc = "Releases layer snapshot captured by the back-end.\n[releaseSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-releaseSnapshot)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ReleaseSnapshot {
    pub method: ReleaseSnapshotMethod,
    pub params: ReleaseSnapshotParams,
}
impl crate::CommandResult for ReleaseSnapshot {
    type Result = super::results::ReleaseSnapshotResult;
}
#[doc = "Replays the layer snapshot and returns the resulting bitmap.\n[replaySnapshot](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-replaySnapshot)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplaySnapshotParams {
    #[doc = "The id of the layer snapshot."]
    #[serde(rename = "snapshotId")]
    pub snapshot_id: super::types::SnapshotId,
    #[doc = "The first step to replay from (replay from the very start if not specified)."]
    #[serde(rename = "fromStep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub from_step: Option<i64>,
    #[doc = "The last step to replay to (replay till the end if not specified)."]
    #[serde(rename = "toStep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub to_step: Option<i64>,
    #[doc = "The scale to apply while replaying (defaults to 1)."]
    #[serde(rename = "scale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scale: Option<f64>,
}
impl ReplaySnapshotParams {
    pub fn new(snapshot_id: impl Into<super::types::SnapshotId>) -> Self {
        Self {
            snapshot_id: snapshot_id.into(),
            from_step: None,
            to_step: None,
            scale: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReplaySnapshotMethod {
    #[serde(rename = "LayerTree.replaySnapshot")]
    ReplaySnapshot,
}
impl ReplaySnapshotMethod {
    pub const IDENTIFIER: &'static str = "LayerTree.replaySnapshot";
}
#[doc = "Replays the layer snapshot and returns the resulting bitmap.\n[replaySnapshot](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-replaySnapshot)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ReplaySnapshot {
    pub method: ReplaySnapshotMethod,
    pub params: ReplaySnapshotParams,
}
impl crate::CommandResult for ReplaySnapshot {
    type Result = super::results::ReplaySnapshotResult;
}
#[doc = "Replays the layer snapshot and returns canvas log.\n[snapshotCommandLog](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-snapshotCommandLog)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotCommandLogParams {
    #[doc = "The id of the layer snapshot."]
    #[serde(rename = "snapshotId")]
    pub snapshot_id: super::types::SnapshotId,
}
impl SnapshotCommandLogParams {
    pub fn new(snapshot_id: impl Into<super::types::SnapshotId>) -> Self {
        Self {
            snapshot_id: snapshot_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SnapshotCommandLogMethod {
    #[serde(rename = "LayerTree.snapshotCommandLog")]
    SnapshotCommandLog,
}
impl SnapshotCommandLogMethod {
    pub const IDENTIFIER: &'static str = "LayerTree.snapshotCommandLog";
}
#[doc = "Replays the layer snapshot and returns canvas log.\n[snapshotCommandLog](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#method-snapshotCommandLog)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotCommandLog {
    pub method: SnapshotCommandLogMethod,
    pub params: SnapshotCommandLogParams,
}
impl crate::CommandResult for SnapshotCommandLog {
    type Result = super::results::SnapshotCommandLogResult;
}
group_enum ! (LayerTreeCommands { CompositingReasons (CompositingReasons) , Disable (Disable) , Enable (Enable) , LoadSnapshot (LoadSnapshot) , MakeSnapshot (MakeSnapshot) , ProfileSnapshot (ProfileSnapshot) , ReleaseSnapshot (ReleaseSnapshot) , ReplaySnapshot (ReplaySnapshot) , SnapshotCommandLog (SnapshotCommandLog) });
