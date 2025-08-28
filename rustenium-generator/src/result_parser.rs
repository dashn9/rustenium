/// Represents a WebDriver BiDi result definition parsed from CDDL
/// Results are the response data returned from successful command execution
#[derive(Debug)]
pub struct ResultDefinition {
    /// The name of the result definition (e.g., "BrowsingContextResult")
    pub name: String,
    /// The raw CDDL content for this result definition
    pub content: String,
}