// Generated commands for module

use serde::{Serialize, Deserialize};
use crate::network::types::Cookie;
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StorageCommand {
    DeleteCookies(DeleteCookies),
    GetCookies(GetCookies),
    SetCookie(SetCookie),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageDeleteCookiesMethod {
    #[serde(rename = "storage.deleteCookies")]
    StorageDeleteCookies,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageGetCookiesMethod {
    #[serde(rename = "storage.getCookies")]
    StorageGetCookies,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageSetCookieMethod {
    #[serde(rename = "storage.setCookie")]
    StorageSetCookie,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCookiesParameters {
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<CookieFilter>,
    #[serde(rename = "partition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<PartitionDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCookiesParameters {
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<CookieFilter>,
    #[serde(rename = "partition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<PartitionDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCookieParameters {
    #[serde(rename = "cookie")]
    pub cookie: PartialCookie,
    #[serde(rename = "partition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<PartitionDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCookies {
    #[serde(rename = "method")]
    pub method: StorageDeleteCookiesMethod,
    #[serde(rename = "params")]
    pub params: DeleteCookiesParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCookies {
    #[serde(rename = "method")]
    pub method: StorageGetCookiesMethod,
    #[serde(rename = "params")]
    pub params: GetCookiesParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCookie {
    #[serde(rename = "method")]
    pub method: StorageSetCookieMethod,
    #[serde(rename = "params")]
    pub params: SetCookieParameters,
}

// Generated results

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StorageResult {
    DeleteCookiesResult(DeleteCookiesResult),
    GetCookiesResult(GetCookiesResult),
    SetCookieResult(SetCookieResult),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCookiesResult {
    #[serde(rename = "partitionKey")]
    pub partition_key: PartitionKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCookiesResult {
    #[serde(rename = "cookies")]
    pub cookies: Vec<Cookie>,
    #[serde(rename = "partitionKey")]
    pub partition_key: PartitionKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCookieResult {
    #[serde(rename = "partitionKey")]
    pub partition_key: PartitionKey,
}

