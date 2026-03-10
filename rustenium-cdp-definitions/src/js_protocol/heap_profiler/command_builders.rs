use super::commands::*;
impl AddInspectedHeapObject {
    pub fn builder() -> AddInspectedHeapObjectBuilder {
        <AddInspectedHeapObjectBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AddInspectedHeapObjectBuilder {
    heap_object_id: Option<super::types::HeapSnapshotObjectId>,
}
impl AddInspectedHeapObjectBuilder {
    pub fn heap_object_id(
        mut self,
        heap_object_id: impl Into<super::types::HeapSnapshotObjectId>,
    ) -> Self {
        self.heap_object_id = Some(heap_object_id.into());
        self
    }
    pub fn build(self) -> Result<AddInspectedHeapObject, String> {
        Ok(AddInspectedHeapObject {
            method: AddInspectedHeapObjectMethod::AddInspectedHeapObject,
            params: AddInspectedHeapObjectParams {
                heap_object_id: self.heap_object_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(heap_object_id))
                })?,
            },
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct CollectGarbageBuilder;
impl CollectGarbageBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> CollectGarbage {
        CollectGarbage {
            method: CollectGarbageMethod::CollectGarbage,
            params: CollectGarbageParams {},
        }
    }
}
impl CollectGarbage {
    pub fn builder() -> CollectGarbageBuilder {
        CollectGarbageBuilder
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
#[derive(Debug, Clone, Default)]
pub struct EnableBuilder;
impl EnableBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Enable {
        Enable {
            method: EnableMethod::Enable,
            params: EnableParams {},
        }
    }
}
impl Enable {
    pub fn builder() -> EnableBuilder {
        EnableBuilder
    }
}
impl GetHeapObjectId {
    pub fn builder() -> GetHeapObjectIdBuilder {
        <GetHeapObjectIdBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetHeapObjectIdBuilder {
    object_id: Option<crate::js_protocol::runtime::types::RemoteObjectId>,
}
impl GetHeapObjectIdBuilder {
    pub fn object_id(
        mut self,
        object_id: impl Into<crate::js_protocol::runtime::types::RemoteObjectId>,
    ) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn build(self) -> Result<GetHeapObjectId, String> {
        Ok(GetHeapObjectId {
            method: GetHeapObjectIdMethod::GetHeapObjectId,
            params: GetHeapObjectIdParams {
                object_id: self.object_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(object_id))
                })?,
            },
        })
    }
}
impl GetObjectByHeapObjectId {
    pub fn builder() -> GetObjectByHeapObjectIdBuilder {
        <GetObjectByHeapObjectIdBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetObjectByHeapObjectIdBuilder {
    object_id: Option<super::types::HeapSnapshotObjectId>,
    object_group: Option<String>,
}
impl GetObjectByHeapObjectIdBuilder {
    pub fn object_id(mut self, object_id: impl Into<super::types::HeapSnapshotObjectId>) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn object_group(mut self, object_group: impl Into<String>) -> Self {
        self.object_group = Some(object_group.into());
        self
    }
    pub fn build(self) -> Result<GetObjectByHeapObjectId, String> {
        Ok(GetObjectByHeapObjectId {
            method: GetObjectByHeapObjectIdMethod::GetObjectByHeapObjectId,
            params: GetObjectByHeapObjectIdParams {
                object_id: self.object_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(object_id))
                })?,
                object_group: self.object_group,
            },
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct GetSamplingProfileBuilder;
impl GetSamplingProfileBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> GetSamplingProfile {
        GetSamplingProfile {
            method: GetSamplingProfileMethod::GetSamplingProfile,
            params: GetSamplingProfileParams {},
        }
    }
}
impl GetSamplingProfile {
    pub fn builder() -> GetSamplingProfileBuilder {
        GetSamplingProfileBuilder
    }
}
impl StartSampling {
    pub fn builder() -> StartSamplingBuilder {
        <StartSamplingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StartSamplingBuilder {
    sampling_interval: Option<f64>,
    stack_depth: Option<f64>,
    include_objects_collected_by_major_gc: Option<bool>,
    include_objects_collected_by_minor_gc: Option<bool>,
}
impl StartSamplingBuilder {
    pub fn sampling_interval(mut self, sampling_interval: impl Into<f64>) -> Self {
        self.sampling_interval = Some(sampling_interval.into());
        self
    }
    pub fn stack_depth(mut self, stack_depth: impl Into<f64>) -> Self {
        self.stack_depth = Some(stack_depth.into());
        self
    }
    pub fn include_objects_collected_by_major_gc(
        mut self,
        include_objects_collected_by_major_gc: impl Into<bool>,
    ) -> Self {
        self.include_objects_collected_by_major_gc =
            Some(include_objects_collected_by_major_gc.into());
        self
    }
    pub fn include_objects_collected_by_minor_gc(
        mut self,
        include_objects_collected_by_minor_gc: impl Into<bool>,
    ) -> Self {
        self.include_objects_collected_by_minor_gc =
            Some(include_objects_collected_by_minor_gc.into());
        self
    }
    pub fn build(self) -> StartSampling {
        StartSampling {
            method: StartSamplingMethod::StartSampling,
            params: StartSamplingParams {
                sampling_interval: self.sampling_interval,
                stack_depth: self.stack_depth,
                include_objects_collected_by_major_gc: self.include_objects_collected_by_major_gc,
                include_objects_collected_by_minor_gc: self.include_objects_collected_by_minor_gc,
            },
        }
    }
}
impl StartTrackingHeapObjects {
    pub fn builder() -> StartTrackingHeapObjectsBuilder {
        <StartTrackingHeapObjectsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StartTrackingHeapObjectsBuilder {
    track_allocations: Option<bool>,
}
impl StartTrackingHeapObjectsBuilder {
    pub fn track_allocations(mut self, track_allocations: impl Into<bool>) -> Self {
        self.track_allocations = Some(track_allocations.into());
        self
    }
    pub fn build(self) -> StartTrackingHeapObjects {
        StartTrackingHeapObjects {
            method: StartTrackingHeapObjectsMethod::StartTrackingHeapObjects,
            params: StartTrackingHeapObjectsParams {
                track_allocations: self.track_allocations,
            },
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct StopSamplingBuilder;
impl StopSamplingBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> StopSampling {
        StopSampling {
            method: StopSamplingMethod::StopSampling,
            params: StopSamplingParams {},
        }
    }
}
impl StopSampling {
    pub fn builder() -> StopSamplingBuilder {
        StopSamplingBuilder
    }
}
impl StopTrackingHeapObjects {
    pub fn builder() -> StopTrackingHeapObjectsBuilder {
        <StopTrackingHeapObjectsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StopTrackingHeapObjectsBuilder {
    report_progress: Option<bool>,
    capture_numeric_value: Option<bool>,
    expose_internals: Option<bool>,
}
impl StopTrackingHeapObjectsBuilder {
    pub fn report_progress(mut self, report_progress: impl Into<bool>) -> Self {
        self.report_progress = Some(report_progress.into());
        self
    }
    pub fn capture_numeric_value(mut self, capture_numeric_value: impl Into<bool>) -> Self {
        self.capture_numeric_value = Some(capture_numeric_value.into());
        self
    }
    pub fn expose_internals(mut self, expose_internals: impl Into<bool>) -> Self {
        self.expose_internals = Some(expose_internals.into());
        self
    }
    pub fn build(self) -> StopTrackingHeapObjects {
        StopTrackingHeapObjects {
            method: StopTrackingHeapObjectsMethod::StopTrackingHeapObjects,
            params: StopTrackingHeapObjectsParams {
                report_progress: self.report_progress,
                capture_numeric_value: self.capture_numeric_value,
                expose_internals: self.expose_internals,
            },
        }
    }
}
impl TakeHeapSnapshot {
    pub fn builder() -> TakeHeapSnapshotBuilder {
        <TakeHeapSnapshotBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct TakeHeapSnapshotBuilder {
    report_progress: Option<bool>,
    capture_numeric_value: Option<bool>,
    expose_internals: Option<bool>,
}
impl TakeHeapSnapshotBuilder {
    pub fn report_progress(mut self, report_progress: impl Into<bool>) -> Self {
        self.report_progress = Some(report_progress.into());
        self
    }
    pub fn capture_numeric_value(mut self, capture_numeric_value: impl Into<bool>) -> Self {
        self.capture_numeric_value = Some(capture_numeric_value.into());
        self
    }
    pub fn expose_internals(mut self, expose_internals: impl Into<bool>) -> Self {
        self.expose_internals = Some(expose_internals.into());
        self
    }
    pub fn build(self) -> TakeHeapSnapshot {
        TakeHeapSnapshot {
            method: TakeHeapSnapshotMethod::TakeHeapSnapshot,
            params: TakeHeapSnapshotParams {
                report_progress: self.report_progress,
                capture_numeric_value: self.capture_numeric_value,
                expose_internals: self.expose_internals,
            },
        }
    }
}
