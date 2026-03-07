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
impl IssueAddedMethod {
    pub const IDENTIFIER: &'static str = "Audits.issueAdded";
}
#[derive(Debug, Clone, PartialEq)]
pub struct IssueAdded {
    pub method: IssueAddedMethod,
    pub params: IssueAddedParams,
}
group_enum ! (AuditsEvents { IssueAdded (IssueAdded) });
