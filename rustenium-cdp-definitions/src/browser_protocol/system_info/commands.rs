use serde::{Deserialize, Serialize};
#[doc = "Returns information about the system.\n[getInfo](https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#method-getInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInfoParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetInfoMethod {
    #[serde(rename = "SystemInfo.getInfo")]
    GetInfo,
}
#[doc = "Returns information about the system.\n[getInfo](https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#method-getInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInfo {
    pub method: GetInfoMethod,
    pub params: GetInfoParams,
}
impl GetInfo {
    pub const IDENTIFIER: &'static str = "SystemInfo.getInfo";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetInfo {
    type Result = super::results::GetInfoResult;
}
#[doc = "Returns information about the feature state.\n[getFeatureState](https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#method-getFeatureState)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFeatureStateParams {
    #[serde(rename = "featureState")]
    pub feature_state: String,
}
impl GetFeatureStateParams {
    pub fn new(feature_state: impl Into<String>) -> Self {
        Self {
            feature_state: feature_state.into(),
        }
    }
}
impl<T: Into<String>> From<T> for GetFeatureStateParams {
    fn from(url: T) -> Self {
        GetFeatureStateParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetFeatureStateMethod {
    #[serde(rename = "SystemInfo.getFeatureState")]
    GetFeatureState,
}
#[doc = "Returns information about the feature state.\n[getFeatureState](https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#method-getFeatureState)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFeatureState {
    pub method: GetFeatureStateMethod,
    pub params: GetFeatureStateParams,
}
impl GetFeatureState {
    pub const IDENTIFIER: &'static str = "SystemInfo.getFeatureState";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetFeatureState {
    type Result = super::results::GetFeatureStateResult;
}
#[doc = "Returns information about all running processes.\n[getProcessInfo](https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#method-getProcessInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProcessInfoParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetProcessInfoMethod {
    #[serde(rename = "SystemInfo.getProcessInfo")]
    GetProcessInfo,
}
#[doc = "Returns information about all running processes.\n[getProcessInfo](https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#method-getProcessInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProcessInfo {
    pub method: GetProcessInfoMethod,
    pub params: GetProcessInfoParams,
}
impl GetProcessInfo {
    pub const IDENTIFIER: &'static str = "SystemInfo.getProcessInfo";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetProcessInfo {
    type Result = super::results::GetProcessInfoResult;
}
group_enum ! (SystemInfoCommands { GetInfo (GetInfo) , GetFeatureState (GetFeatureState) , GetProcessInfo (GetProcessInfo) } + identifiable);
