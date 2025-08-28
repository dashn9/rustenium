use regex::Regex;

/// Represents a complete command definition parsed from CDDL (Concise Data Definition Language)
/// This contains both the raw CDDL content and the parsed command structures
#[derive(Debug)]
pub struct CommandDefinition {
    /// The name of the command definition (e.g., "BrowserCommand", "NetworkCommand")
    pub name: String,
    /// The raw CDDL content for this command definition
    pub content: String,
    /// Parsed individual commands found within this definition
    pub commands: Vec<Command>,
    /// Rust attributes to be applied to the generated enum (e.g., #[derive(Debug, Serialize)])
    pub attributes: Vec<String>,
}


/// Parses a CDDL command definition and extracts individual commands
/// 
/// This function takes the raw CDDL content for a command definition (e.g., BrowserCommand)
/// and parses it to extract individual commands that follow the pattern "module.command //"
/// 
/// # Arguments
/// * `name` - The name of the command definition (e.g., "BrowserCommand")
/// * `content` - The raw CDDL content containing command definitions
/// 
/// # Returns
/// A CommandDefinition struct containing the parsed commands and metadata
pub fn parse_command_definition(name: String, content: String) -> Result<CommandDefinition, Box<dyn std::error::Error>> {
    let command_enum_pattern = Regex::new(r"(\w+)\.(\w+)\s*//")?;
    let mut commands = Vec::new();

    // Extract command names from the enum definition
    for line in content.lines() {
        let line = line.trim();

        if let Some(captures) = command_enum_pattern.captures(line) {
            let module_name = captures[1].to_string(); // Extract module name (before the dot)
            let name = captures[2].to_string(); // Extract command name (after the dot)
            let attributes = extract_attributes(line)?;

            commands.push(Command {
                name,
                module_name,
                method: String::new(), // Empty string for now
                params: CommandParams {},
                attributes,
            });
        }
    }

    // Extract attributes for the overall definition
    let definition_attributes = vec![
        "#[derive(Debug, Serialize, Deserialize)]".to_string(),
        "#[serde(untagged)]".to_string(),
    ];

    Ok(CommandDefinition {
        name: name.clone(),
        content,
        commands,
        attributes: definition_attributes,
    })
}

/// Extracts Rust attributes from a CDDL command line
/// 
/// This function parses CDDL comments or annotations to extract Rust attributes
/// that should be applied to the generated command variants (e.g., #[serde(rename = "...")])
/// 
/// # Arguments
/// * `line` - The CDDL line to parse for attributes
/// 
/// # Returns
/// A vector of Rust attribute strings to be applied to the command
/// 
/// # Note
/// Currently returns an empty vector - can be expanded based on specific CDDL format needs
fn extract_attributes(line: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut attributes = Vec::new();
    
    // Extract attributes from comments or other patterns as needed
    // For now, return empty vector - can be expanded based on your specific format
    
    Ok(attributes)
}

/// Parameters for a WebDriver BiDi command
/// Currently empty but will be populated with actual parameter types from CDDL parsing
#[derive(Debug, Clone)]
pub struct CommandParams {
    // Empty for now - will contain parsed parameter definitions
}

/// Represents a single WebDriver BiDi command parsed from CDDL
/// Commands follow the pattern module.method (e.g., "browser.close", "network.enable")
#[derive(Debug, Clone)]
pub struct Command {
    /// The command name as found in CDDL (e.g., "close", "enable")
    pub name: String,
    /// The name of the module this command belongs to (e.g., "browser", "network", "session")
    pub module_name: String,
    /// The actual WebDriver BiDi method name - currently empty, will be populated later
    pub method: String,
    /// The parsed parameters for this command
    pub params: CommandParams,
    /// Rust attributes to be applied to this command variant
    pub attributes: Vec<String>,
}

/// Searches for a specific command in CDDL content and updates the command with parsed data
/// 
/// This function takes a Command struct and searches for its corresponding definition
/// in the CDDL content using the pattern "module.command //". When found, it will
/// update the command with the parsed method and parameter information.
/// 
/// # Arguments
/// * `cddl_content` - The raw CDDL content to search through
/// * `command` - Mutable reference to the Command struct to update
/// 
/// # Returns
/// Ok(()) if successful, or an error if parsing fails
/// 
/// # TODO
/// Currently only searches for the command pattern. Need to implement:
/// - Parsing the actual method name from the CDDL definition
/// - Extracting and parsing parameter definitions
pub fn search_and_update_command(cddl_content: &str, command: &mut Command) -> Result<(), Box<dyn std::error::Error>> {
    // Search for individual command using module.command pattern
    let pattern = format!(r"{}\s*\.\s*{}\s*//", command.module_name.to_lowercase(), command.name.to_lowercase());
    let regex = Regex::new(&pattern)?;
    
    for line in cddl_content.lines() {
        if regex.is_match(line.trim()) {
            // TODO: Parse the method and params from the CDDL definition
            // For now, just mark that we found it
            break;
        }
    }
    
    // TODO: Need to parse the method and the params from the CDDL definition
    
    Ok(())
}