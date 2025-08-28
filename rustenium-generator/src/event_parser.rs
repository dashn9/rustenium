/// Represents a WebDriver BiDi event definition parsed from CDDL
/// Events are notifications sent from the browser to the client
#[derive(Debug)]
pub struct EventDefinition {
    /// The name of the event definition (e.g., "BrowsingContextEvent")
    pub name: String,
    /// The raw CDDL content for this event definition
    pub content: String,
}