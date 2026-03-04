use serde::{Deserialize, Serialize};
#[macro_use]
mod macros;
pub mod browser_protocol;
pub mod js_protocol;
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Binary(String);
impl AsRef<str> for Binary {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl AsRef<[u8]> for Binary {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
impl From<Binary> for String {
    fn from(b: Binary) -> String {
        b.0
    }
}
impl From<String> for Binary {
    fn from(expr: String) -> Self {
        Self(expr)
    }
}
group_enum ! (Type { JsProtocol (js_protocol :: Type) , BrowserProtocol (browser_protocol :: Type) });
group_enum ! (Command { JsProtocol (js_protocol :: Command) , BrowserProtocol (browser_protocol :: Command) });
group_enum ! (Event { JsProtocol (js_protocol :: Event) , BrowserProtocol (browser_protocol :: Event) } + other);
#[derive(Debug, PartialEq, Clone)]
pub struct EventMessage {
    pub method: String,
    pub session_id: Option<String>,
    pub params: Event,
}
