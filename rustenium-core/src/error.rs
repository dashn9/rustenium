#[derive(Debug, Clone)]
pub struct BrowsingContextCreationInvalidResultError;

impl std::fmt::Display for BrowsingContextCreationInvalidResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid response for browser context creation")
    }
}