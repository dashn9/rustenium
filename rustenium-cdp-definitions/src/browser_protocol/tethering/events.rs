use serde::{Deserialize, Serialize};
#[doc = "Informs that port was successfully bound and got a specified connection id.\n[accepted](https://chromedevtools.github.io/devtools-protocol/tot/Tethering/#event-accepted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcceptedParams {
    #[doc = "Port number that was successfully bound."]
    #[serde(rename = "port")]
    pub port: i64,
    #[doc = "Connection id to be used."]
    #[serde(rename = "connectionId")]
    pub connection_id: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AcceptedMethod {
    #[serde(rename = "Tethering.accepted")]
    Accepted,
}
#[doc = "Informs that port was successfully bound and got a specified connection id.\n[accepted](https://chromedevtools.github.io/devtools-protocol/tot/Tethering/#event-accepted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Accepted {
    pub method: AcceptedMethod,
    pub params: AcceptedParams,
}
impl Accepted {
    pub const IDENTIFIER: &'static str = "Tethering.accepted";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (TetheringEvents { Accepted (Accepted) } + identifiable);
