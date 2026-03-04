use serde::{Deserialize, Serialize};
#[doc = "This is fired whenever the list of available sinks changes. A sink is a\ndevice or a software surface that you can cast to.\n[sinksUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#event-sinksUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SinksUpdated {
    #[serde(rename = "sinks")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sinks: Vec<super::types::Sink>,
}
impl SinksUpdated {
    pub const IDENTIFIER: &'static str = "Cast.sinksUpdated";
}
#[doc = "This is fired whenever the outstanding issue/error message changes.\n|issueMessage| is empty if there is no issue.\n[issueUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Cast/#event-issueUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueUpdated {
    #[serde(rename = "issueMessage")]
    pub issue_message: String,
}
impl IssueUpdated {
    pub const IDENTIFIER: &'static str = "Cast.issueUpdated";
}
group_enum ! (Event { SinksUpdated (SinksUpdated) , IssueUpdated (IssueUpdated) });
