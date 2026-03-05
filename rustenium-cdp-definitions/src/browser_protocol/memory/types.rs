use serde::{Deserialize, Serialize};
#[doc = "Memory pressure level."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PressureLevel {
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "critical")]
    Critical,
}
#[doc = "Heap profile sample.\n[SamplingProfileNode](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#type-SamplingProfileNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SamplingProfileNode {
    #[doc = "Size of the sampled allocation."]
    #[serde(rename = "size")]
    pub size: f64,
    #[doc = "Total bytes attributed to this sample."]
    #[serde(rename = "total")]
    pub total: f64,
    #[doc = "Execution stack at the point of allocation."]
    #[serde(rename = "stack")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stack: Vec<String>,
}
impl SamplingProfileNode {
    pub fn new(size: impl Into<f64>, total: impl Into<f64>, stack: Vec<String>) -> Self {
        Self {
            size: size.into(),
            total: total.into(),
            stack,
        }
    }
}
impl SamplingProfileNode {
    pub const IDENTIFIER: &'static str = "Memory.SamplingProfileNode";
}
#[doc = "Array of heap profile samples.\n[SamplingProfile](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#type-SamplingProfile)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SamplingProfile {
    #[serde(rename = "samples")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub samples: Vec<SamplingProfileNode>,
    #[serde(rename = "modules")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub modules: Vec<Module>,
}
impl SamplingProfile {
    pub fn new(samples: Vec<SamplingProfileNode>, modules: Vec<Module>) -> Self {
        Self { samples, modules }
    }
}
impl SamplingProfile {
    pub const IDENTIFIER: &'static str = "Memory.SamplingProfile";
}
#[doc = "Executable module information\n[Module](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#type-Module)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    #[doc = "Name of the module."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "UUID of the module."]
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[doc = "Base address where the module is loaded into memory. Encoded as a decimal\nor hexadecimal (0x prefixed) string."]
    #[serde(rename = "baseAddress")]
    pub base_address: String,
    #[doc = "Size of the module in bytes."]
    #[serde(rename = "size")]
    pub size: f64,
}
impl Module {
    pub fn new(
        name: impl Into<String>,
        uuid: impl Into<String>,
        base_address: impl Into<String>,
        size: impl Into<f64>,
    ) -> Self {
        Self {
            name: name.into(),
            uuid: uuid.into(),
            base_address: base_address.into(),
            size: size.into(),
        }
    }
}
impl Module {
    pub const IDENTIFIER: &'static str = "Memory.Module";
}
#[doc = "DOM object counter data.\n[DOMCounter](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#type-DOMCounter)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomCounter {
    #[doc = "Object name. Note: object names should be presumed volatile and clients should not expect\nthe returned names to be consistent across runs."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Object count."]
    #[serde(rename = "count")]
    pub count: i64,
}
impl DomCounter {
    pub fn new(name: impl Into<String>, count: impl Into<i64>) -> Self {
        Self {
            name: name.into(),
            count: count.into(),
        }
    }
}
impl DomCounter {
    pub const IDENTIFIER: &'static str = "Memory.DOMCounter";
}
group_enum ! (MemoryTypes { PressureLevel (PressureLevel) , SamplingProfileNode (SamplingProfileNode) , SamplingProfile (SamplingProfile) , Module (Module) , DomCounter (DomCounter) });
