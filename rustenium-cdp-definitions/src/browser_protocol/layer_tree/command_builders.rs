use super::commands::*;
impl CompositingReasons {
    pub fn builder() -> CompositingReasonsBuilder {
        <CompositingReasonsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CompositingReasonsBuilder {
    layer_id: Option<super::types::LayerId>,
}
impl CompositingReasonsBuilder {
    pub fn layer_id(mut self, layer_id: impl Into<super::types::LayerId>) -> Self {
        self.layer_id = Some(layer_id.into());
        self
    }
    pub fn build(self) -> Result<CompositingReasons, String> {
        Ok(CompositingReasons {
            method: CompositingReasonsMethod::CompositingReasons,
            params: CompositingReasonsParams {
                layer_id: self.layer_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(layer_id))
                })?,
            },
        })
    }
}
impl LoadSnapshot {
    pub fn builder() -> LoadSnapshotBuilder {
        <LoadSnapshotBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct LoadSnapshotBuilder {
    tiles: Option<Vec<super::types::PictureTile>>,
}
impl LoadSnapshotBuilder {
    pub fn tile(mut self, tile: impl Into<super::types::PictureTile>) -> Self {
        let v = self.tiles.get_or_insert(Vec::new());
        v.push(tile.into());
        self
    }
    pub fn tiles<I, S>(mut self, tiles: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::PictureTile>,
    {
        let v = self.tiles.get_or_insert(Vec::new());
        for val in tiles {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<LoadSnapshot, String> {
        Ok(LoadSnapshot {
            method: LoadSnapshotMethod::LoadSnapshot,
            params: LoadSnapshotParams {
                tiles: self
                    .tiles
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(tiles)))?,
            },
        })
    }
}
impl MakeSnapshot {
    pub fn builder() -> MakeSnapshotBuilder {
        <MakeSnapshotBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct MakeSnapshotBuilder {
    layer_id: Option<super::types::LayerId>,
}
impl MakeSnapshotBuilder {
    pub fn layer_id(mut self, layer_id: impl Into<super::types::LayerId>) -> Self {
        self.layer_id = Some(layer_id.into());
        self
    }
    pub fn build(self) -> Result<MakeSnapshot, String> {
        Ok(MakeSnapshot {
            method: MakeSnapshotMethod::MakeSnapshot,
            params: MakeSnapshotParams {
                layer_id: self.layer_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(layer_id))
                })?,
            },
        })
    }
}
impl ProfileSnapshot {
    pub fn builder() -> ProfileSnapshotBuilder {
        <ProfileSnapshotBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ProfileSnapshotBuilder {
    snapshot_id: Option<super::types::SnapshotId>,
    min_repeat_count: Option<i64>,
    min_duration: Option<f64>,
    clip_rect: Option<crate::browser_protocol::dom::types::Rect>,
}
impl ProfileSnapshotBuilder {
    pub fn snapshot_id(mut self, snapshot_id: impl Into<super::types::SnapshotId>) -> Self {
        self.snapshot_id = Some(snapshot_id.into());
        self
    }
    pub fn min_repeat_count(mut self, min_repeat_count: impl Into<i64>) -> Self {
        self.min_repeat_count = Some(min_repeat_count.into());
        self
    }
    pub fn min_duration(mut self, min_duration: impl Into<f64>) -> Self {
        self.min_duration = Some(min_duration.into());
        self
    }
    pub fn clip_rect(
        mut self,
        clip_rect: impl Into<crate::browser_protocol::dom::types::Rect>,
    ) -> Self {
        self.clip_rect = Some(clip_rect.into());
        self
    }
    pub fn build(self) -> Result<ProfileSnapshot, String> {
        Ok(ProfileSnapshot {
            method: ProfileSnapshotMethod::ProfileSnapshot,
            params: ProfileSnapshotParams {
                snapshot_id: self.snapshot_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(snapshot_id))
                })?,
                min_repeat_count: self.min_repeat_count,
                min_duration: self.min_duration,
                clip_rect: self.clip_rect,
            },
        })
    }
}
impl ReleaseSnapshot {
    pub fn builder() -> ReleaseSnapshotBuilder {
        <ReleaseSnapshotBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ReleaseSnapshotBuilder {
    snapshot_id: Option<super::types::SnapshotId>,
}
impl ReleaseSnapshotBuilder {
    pub fn snapshot_id(mut self, snapshot_id: impl Into<super::types::SnapshotId>) -> Self {
        self.snapshot_id = Some(snapshot_id.into());
        self
    }
    pub fn build(self) -> Result<ReleaseSnapshot, String> {
        Ok(ReleaseSnapshot {
            method: ReleaseSnapshotMethod::ReleaseSnapshot,
            params: ReleaseSnapshotParams {
                snapshot_id: self.snapshot_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(snapshot_id))
                })?,
            },
        })
    }
}
impl ReplaySnapshot {
    pub fn builder() -> ReplaySnapshotBuilder {
        <ReplaySnapshotBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ReplaySnapshotBuilder {
    snapshot_id: Option<super::types::SnapshotId>,
    from_step: Option<i64>,
    to_step: Option<i64>,
    scale: Option<f64>,
}
impl ReplaySnapshotBuilder {
    pub fn snapshot_id(mut self, snapshot_id: impl Into<super::types::SnapshotId>) -> Self {
        self.snapshot_id = Some(snapshot_id.into());
        self
    }
    pub fn from_step(mut self, from_step: impl Into<i64>) -> Self {
        self.from_step = Some(from_step.into());
        self
    }
    pub fn to_step(mut self, to_step: impl Into<i64>) -> Self {
        self.to_step = Some(to_step.into());
        self
    }
    pub fn scale(mut self, scale: impl Into<f64>) -> Self {
        self.scale = Some(scale.into());
        self
    }
    pub fn build(self) -> Result<ReplaySnapshot, String> {
        Ok(ReplaySnapshot {
            method: ReplaySnapshotMethod::ReplaySnapshot,
            params: ReplaySnapshotParams {
                snapshot_id: self.snapshot_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(snapshot_id))
                })?,
                from_step: self.from_step,
                to_step: self.to_step,
                scale: self.scale,
            },
        })
    }
}
impl SnapshotCommandLog {
    pub fn builder() -> SnapshotCommandLogBuilder {
        <SnapshotCommandLogBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SnapshotCommandLogBuilder {
    snapshot_id: Option<super::types::SnapshotId>,
}
impl SnapshotCommandLogBuilder {
    pub fn snapshot_id(mut self, snapshot_id: impl Into<super::types::SnapshotId>) -> Self {
        self.snapshot_id = Some(snapshot_id.into());
        self
    }
    pub fn build(self) -> Result<SnapshotCommandLog, String> {
        Ok(SnapshotCommandLog {
            method: SnapshotCommandLogMethod::SnapshotCommandLog,
            params: SnapshotCommandLogParams {
                snapshot_id: self.snapshot_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(snapshot_id))
                })?,
            },
        })
    }
}
