use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueAdded {
    #[serde(rename = "issue")]
    pub issue: super::types::InspectorIssue,
}
impl IssueAdded {
    pub const IDENTIFIER: &'static str = "Audits.issueAdded";
}
group_enum ! (AuditsEvents { IssueAdded (IssueAdded) });
