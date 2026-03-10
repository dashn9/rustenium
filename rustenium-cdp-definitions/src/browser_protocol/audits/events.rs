use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueAddedParams {
    #[serde(rename = "issue")]
    pub issue: super::types::InspectorIssue,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IssueAddedMethod {
    #[serde(rename = "Audits.issueAdded")]
    IssueAdded,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueAdded {
    pub method: IssueAddedMethod,
    pub params: IssueAddedParams,
}
impl IssueAdded {
    pub const IDENTIFIER: &'static str = "Audits.issueAdded";
}
group_enum ! (AuditsEvents { IssueAdded (IssueAdded) });
