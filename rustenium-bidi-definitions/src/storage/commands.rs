use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCookiesParams {
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub filter: Option<super::types::CookieFilter>,
    #[serde(rename = "partition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub partition: Option<super::types::PartitionDescriptor>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetCookiesMethod {
    #[serde(rename = "storage.getCookies")]
    GetCookies,
}
impl GetCookiesMethod {
    pub const IDENTIFIER: &'static str = "storage.getCookies";
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetCookies {
    pub method: GetCookiesMethod,
    pub params: GetCookiesParams,
}
impl crate::CommandResult for GetCookies {
    type Result = super::results::GetCookiesResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCookieParams {
    #[serde(rename = "cookie")]
    pub cookie: super::types::PartialCookie,
    #[serde(rename = "partition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub partition: Option<super::types::PartitionDescriptor>,
}
impl SetCookieParams {
    pub fn new(cookie: impl Into<super::types::PartialCookie>) -> Self {
        Self {
            cookie: cookie.into(),
            partition: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetCookieMethod {
    #[serde(rename = "storage.setCookie")]
    SetCookie,
}
impl SetCookieMethod {
    pub const IDENTIFIER: &'static str = "storage.setCookie";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetCookie {
    pub method: SetCookieMethod,
    pub params: SetCookieParams,
}
impl crate::CommandResult for SetCookie {
    type Result = super::results::SetCookieResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteCookiesParams {
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub filter: Option<super::types::CookieFilter>,
    #[serde(rename = "partition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub partition: Option<super::types::PartitionDescriptor>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeleteCookiesMethod {
    #[serde(rename = "storage.deleteCookies")]
    DeleteCookies,
}
impl DeleteCookiesMethod {
    pub const IDENTIFIER: &'static str = "storage.deleteCookies";
}
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteCookies {
    pub method: DeleteCookiesMethod,
    pub params: DeleteCookiesParams,
}
impl crate::CommandResult for DeleteCookies {
    type Result = super::results::DeleteCookiesResult;
}
group_enum ! (StorageCommands { GetCookies (GetCookies) , SetCookie (SetCookie) , DeleteCookies (DeleteCookies) });
