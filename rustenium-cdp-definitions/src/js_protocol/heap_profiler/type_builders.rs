use super::types::*;
impl SamplingHeapProfileNode {
    pub fn builder() -> SamplingHeapProfileNodeBuilder {
        SamplingHeapProfileNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SamplingHeapProfileNodeBuilder {
    call_frame: Option<super::super::runtime::types::CallFrame>,
    self_size: Option<f64>,
    id: Option<i64>,
    children: Option<Vec<SamplingHeapProfileNode>>,
}
impl SamplingHeapProfileNodeBuilder {
    pub fn call_frame(
        mut self,
        call_frame: impl Into<super::super::runtime::types::CallFrame>,
    ) -> Self {
        self.call_frame = Some(call_frame.into());
        self
    }
    pub fn self_size(mut self, self_size: impl Into<f64>) -> Self {
        self.self_size = Some(self_size.into());
        self
    }
    pub fn id(mut self, id: impl Into<i64>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn children(mut self, children: impl Into<SamplingHeapProfileNode>) -> Self {
        let v = self.children.get_or_insert(Vec::new());
        v.push(children.into());
        self
    }
    pub fn childrens<I, S>(mut self, childrens: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<SamplingHeapProfileNode>,
    {
        let v = self.children.get_or_insert(Vec::new());
        for val in childrens {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SamplingHeapProfileNode, String> {
        Ok(SamplingHeapProfileNode {
            call_frame: self
                .call_frame
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(call_frame)))?,
            self_size: self
                .self_size
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(self_size)))?,
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            children: self
                .children
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(children)))?,
        })
    }
}
impl SamplingHeapProfileSample {
    pub fn builder() -> SamplingHeapProfileSampleBuilder {
        SamplingHeapProfileSampleBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SamplingHeapProfileSampleBuilder {
    size: Option<f64>,
    node_id: Option<i64>,
    ordinal: Option<f64>,
}
impl SamplingHeapProfileSampleBuilder {
    pub fn size(mut self, size: impl Into<f64>) -> Self {
        self.size = Some(size.into());
        self
    }
    pub fn node_id(mut self, node_id: impl Into<i64>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn ordinal(mut self, ordinal: impl Into<f64>) -> Self {
        self.ordinal = Some(ordinal.into());
        self
    }
    pub fn build(self) -> Result<SamplingHeapProfileSample, String> {
        Ok(SamplingHeapProfileSample {
            size: self
                .size
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(size)))?,
            node_id: self
                .node_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            ordinal: self
                .ordinal
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(ordinal)))?,
        })
    }
}
impl SamplingHeapProfile {
    pub fn builder() -> SamplingHeapProfileBuilder {
        SamplingHeapProfileBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SamplingHeapProfileBuilder {
    head: Option<SamplingHeapProfileNode>,
    samples: Option<Vec<SamplingHeapProfileSample>>,
}
impl SamplingHeapProfileBuilder {
    pub fn head(mut self, head: impl Into<SamplingHeapProfileNode>) -> Self {
        self.head = Some(head.into());
        self
    }
    pub fn sample(mut self, sample: impl Into<SamplingHeapProfileSample>) -> Self {
        let v = self.samples.get_or_insert(Vec::new());
        v.push(sample.into());
        self
    }
    pub fn samples<I, S>(mut self, samples: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<SamplingHeapProfileSample>,
    {
        let v = self.samples.get_or_insert(Vec::new());
        for val in samples {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SamplingHeapProfile, String> {
        Ok(SamplingHeapProfile {
            head: self
                .head
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(head)))?,
            samples: self
                .samples
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(samples)))?,
        })
    }
}
