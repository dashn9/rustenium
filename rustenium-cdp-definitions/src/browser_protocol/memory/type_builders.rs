use super::types::*;
impl SamplingProfileNode {
    pub fn builder() -> SamplingProfileNodeBuilder {
        <SamplingProfileNodeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SamplingProfileNodeBuilder {
    size: Option<f64>,
    total: Option<f64>,
    stack: Option<Vec<String>>,
}
impl SamplingProfileNodeBuilder {
    pub fn size(mut self, size: impl Into<f64>) -> Self {
        self.size = Some(size.into());
        self
    }
    pub fn total(mut self, total: impl Into<f64>) -> Self {
        self.total = Some(total.into());
        self
    }
    pub fn stack(mut self, stack: impl Into<String>) -> Self {
        let v = self.stack.get_or_insert(Vec::new());
        v.push(stack.into());
        self
    }
    pub fn stacks<I, S>(mut self, stacks: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.stack.get_or_insert(Vec::new());
        for val in stacks {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SamplingProfileNode, String> {
        Ok(SamplingProfileNode {
            size: self
                .size
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(size)))?,
            total: self
                .total
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(total)))?,
            stack: self
                .stack
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(stack)))?,
        })
    }
}
impl SamplingProfile {
    pub fn builder() -> SamplingProfileBuilder {
        <SamplingProfileBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SamplingProfileBuilder {
    samples: Option<Vec<SamplingProfileNode>>,
    modules: Option<Vec<Module>>,
}
impl SamplingProfileBuilder {
    pub fn sample(mut self, sample: impl Into<SamplingProfileNode>) -> Self {
        let v = self.samples.get_or_insert(Vec::new());
        v.push(sample.into());
        self
    }
    pub fn samples<I, S>(mut self, samples: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<SamplingProfileNode>,
    {
        let v = self.samples.get_or_insert(Vec::new());
        for val in samples {
            v.push(val.into());
        }
        self
    }
    pub fn module(mut self, module: impl Into<Module>) -> Self {
        let v = self.modules.get_or_insert(Vec::new());
        v.push(module.into());
        self
    }
    pub fn modules<I, S>(mut self, modules: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Module>,
    {
        let v = self.modules.get_or_insert(Vec::new());
        for val in modules {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SamplingProfile, String> {
        Ok(SamplingProfile {
            samples: self
                .samples
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(samples)))?,
            modules: self
                .modules
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(modules)))?,
        })
    }
}
impl Module {
    pub fn builder() -> ModuleBuilder {
        <ModuleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ModuleBuilder {
    name: Option<String>,
    uuid: Option<String>,
    base_address: Option<String>,
    size: Option<f64>,
}
impl ModuleBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }
    pub fn base_address(mut self, base_address: impl Into<String>) -> Self {
        self.base_address = Some(base_address.into());
        self
    }
    pub fn size(mut self, size: impl Into<f64>) -> Self {
        self.size = Some(size.into());
        self
    }
    pub fn build(self) -> Result<Module, String> {
        Ok(Module {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            uuid: self
                .uuid
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(uuid)))?,
            base_address: self.base_address.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(base_address))
            })?,
            size: self
                .size
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(size)))?,
        })
    }
}
impl DomCounter {
    pub fn builder() -> DomCounterBuilder {
        <DomCounterBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DomCounterBuilder {
    name: Option<String>,
    count: Option<i64>,
}
impl DomCounterBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn count(mut self, count: impl Into<i64>) -> Self {
        self.count = Some(count.into());
        self
    }
    pub fn build(self) -> Result<DomCounter, String> {
        Ok(DomCounter {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            count: self
                .count
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(count)))?,
        })
    }
}
