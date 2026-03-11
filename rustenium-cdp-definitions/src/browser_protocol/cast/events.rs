use serde::{Deserialize, Serialize};
#[doc = "This is fired whenever the list of available sinks changes. A sink is a\ndevice or a software surface that you can cast to.\n[sinksUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#event-sinksUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SinksUpdatedParams {
    #[serde(rename = "sinks")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sinks: Vec<super::types::Sink>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SinksUpdatedMethod {
    #[serde(rename = "Cast.sinksUpdated")]
    SinksUpdated,
}
#[doc = "This is fired whenever the list of available sinks changes. A sink is a\ndevice or a software surface that you can cast to.\n[sinksUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#event-sinksUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SinksUpdated {
    pub method: SinksUpdatedMethod,
    pub params: SinksUpdatedParams,
}
impl SinksUpdated {
    pub const IDENTIFIER: &'static str = "Cast.sinksUpdated";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "This is fired whenever the outstanding issue/error message changes.\n|issueMessage| is empty if there is no issue.\n[issueUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#event-issueUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueUpdatedParams {
    #[serde(rename = "issueMessage")]
    pub issue_message: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IssueUpdatedMethod {
    #[serde(rename = "Cast.issueUpdated")]
    IssueUpdated,
}
#[doc = "This is fired whenever the outstanding issue/error message changes.\n|issueMessage| is empty if there is no issue.\n[issueUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#event-issueUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueUpdated {
    pub method: IssueUpdatedMethod,
    pub params: IssueUpdatedParams,
}
impl IssueUpdated {
    pub const IDENTIFIER: &'static str = "Cast.issueUpdated";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (CastEvents { SinksUpdated (SinksUpdated) , IssueUpdated (IssueUpdated) } + identifiable);
