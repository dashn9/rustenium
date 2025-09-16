use regex::Regex;
use crate::module::Module;
use crate::parser;

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
    /// Generated command methods
    pub command_methods: Vec<CommandMethods>,
    /// Generated command parameters
    pub command_params: Vec<CommandParams>,
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
/// * `cddl_strings` - All CDDL content for searching command details
/// 
/// # Returns
/// A CommandDefinition struct containing the parsed commands and metadata
pub fn parse_command_definition(name: String, content: String, cddl_strings: Vec<&str>, module: &mut Module) -> Result<CommandDefinition, Box<dyn std::error::Error>> {
    let command_enum_pattern = Regex::new(r"(\w+)\.(\w+)\s*//")?;
    let mut commands = Vec::new();

    // Create a mutable command definition to build up
    let mut command_def = CommandDefinition {
        name: name.clone(),
        content: content.clone(),
        commands: Vec::new(),
        command_methods: Vec::new(),
        command_params: Vec::new(),
        attributes: vec![
            "#[derive(Debug, Serialize, Deserialize)]".to_string(),
            "#[serde(untagged)]".to_string(),
        ],
    };

    // Extract command names from the enum definition
    for line in content.lines() {
        let line = line.trim();

        if let Some(captures) = command_enum_pattern.captures(line) {
            let module_name = captures[1].to_string(); // Extract module name (before the dot)
            let name = captures[2].to_string(); // Extract command name (after the dot)
            let attributes = extract_attributes(line)?;

            let rust_method_name = format!("{}{}",
                                           module_name.replace("_", "").to_string().chars().next().unwrap().to_uppercase().to_string() + &module_name[1..],
                                           name
            );
            let mut command = Command {
                name,
                module_name,
                attributes: vec![
                    "#[derive(Debug, Serialize, Deserialize)]".to_string(),
                ],
                properties: vec![
                    crate::parser::Property {
                        is_enum: false,
                        is_primitive: true,
                        is_optional: false,
                        name: "method".to_string(),
                        value: format!("{}Method", rust_method_name),
                        attributes: vec![r#"#[serde(rename = "method")]"#.to_string()],
                    },
                    crate::parser::Property {
                        is_enum: false,
                        is_primitive: false,
                        is_optional: false,
                        name: "params".to_string(),
                        value: "".to_string(), // Will be set to param_content later
                        attributes: vec![r#"#[serde(rename = "params")]"#.to_string()],
                    },
                ],
            };

            // Search and update the command in all CDDL content
            search_and_update_command(cddl_strings.clone(), &mut command, &mut command_def, module)?;

            commands.push(command);
        }
    }

    // Update the command definition with the processed commands
    command_def.commands = commands;
    Ok(command_def)
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

/// Represents a WebDriver BiDi command method with its serialization attributes
#[derive(Debug, Clone)]
pub struct CommandMethods {
    /// The Rust method name (e.g., "EmulationSetGeolocationOverride")
    pub name: String,
    /// Attributes for the method variants (e.g., serde rename)
    pub method_attributes: Vec<String>,
    /// Attributes for the enum itself (e.g., derive attributes)
    pub enum_attributes: Vec<String>,
}

/// Represents a single WebDriver BiDi command parsed from CDDL
/// Commands follow the pattern module.method (e.g., "browser.close", "network.enable")
#[derive(Debug, Clone)]
pub struct Command {
    /// The command name as found in CDDL (e.g., "close", "enable")
    pub name: String,
    /// The name of the module this command belongs to (e.g., "browser", "network", "session")
    pub module_name: String,
    /// Rust attributes to be applied to this command variant
    pub attributes: Vec<String>,
    /// Properties for the command struct 
    /// Expected to contain: method property (with rename attribute) and params property (with rename attribute)
    pub properties: Vec<crate::parser::Property>,
}

/// Searches for a specific command in CDDL content and updates the command with parsed data
/// 
/// This function takes a Command struct and searches for its corresponding definition
/// in the CDDL content using the pattern "module.command //". When found, it will
/// update the command with the parsed method and parameter information.
/// 
/// # Arguments
/// * `cddl_strings` - The array of CDDL content strings to search through
/// * `command` - Mutable reference to the Command struct to update
/// * `command_def` - Mutable reference to the CommandDefinition to update
/// 
/// # Returns
/// Ok(()) if successful, or an error if parsing fails
/// 
/// # TODO
/// Currently only searches for the command pattern. Need to implement:
/// - Parsing the actual method name from the CDDL definition
/// - Extracting and parsing parameter definitions
pub fn search_and_update_command(cddl_strings: Vec<&str>, command: &mut Command, command_def: &mut CommandDefinition, module: &mut Module) -> Result<(), Box<dyn std::error::Error>> {
    // Search for command definition using pattern: module_name.name = (
    let method_name = format!("{}.{}", command.module_name, command.name);
    let pattern = format!(r"^{}\s*=\s*\(", regex::escape(&method_name));
    let regex = Regex::new(&pattern)?;
    
    // Search through all CDDL content strings
    for cddl_content in cddl_strings.clone() {
        let lines: Vec<&str> = cddl_content.lines().collect();
    
    for (line_num, line) in lines.iter().enumerate() {
        if regex.is_match(line.trim()) {
            // Found the command definition, set the method
            
            // Create CommandMethods if it doesn't exist yet
            let rust_method_name = format!("{}{}",
                command.module_name.replace("_", "").to_string().chars().next().unwrap().to_uppercase().to_string() + &command.module_name[1..],
                command.name
            );
            
            let method_exists = command_def.command_methods.iter().any(|m| m.name == rust_method_name);
            if !method_exists {
                let method_attributes = vec![
                    format!(r#"#[serde(rename = "{}")]"#, method_name)
                ];
                let enum_attributes = vec![
                    "#[derive(Debug, Serialize, Deserialize)]".to_string()
                ];

                // TODO: Use The Method within the body of the cddl as it is the more idiomatic way
                
                command_def.command_methods.push(CommandMethods {
                    name: format!("{}Method", rust_method_name),
                    method_attributes,
                    enum_attributes,
                });
            }
            
            // Collect lines between the parentheses (skip the first line with opening paren)
            let mut paren_count = 0;
            let mut param_lines = Vec::new();
            
            for i in (line_num + 1)..lines.len() {
                let current_line = lines[i];
                
                for ch in current_line.chars() {
                    match ch {
                        '(' => {
                            paren_count += 1;
                        }
                        ')' => {
                            paren_count -= 1;
                            if paren_count < 0 {
                                // Found the closing paren, join the collected lines
                                let param_content = param_lines.join("\n").trim().to_string();
                                
                                // Parse the parameter content
                                let command_value = parser::parse_command_parameters(&param_lines, cddl_strings.clone(), module, command_def)?;
                                if let Some(params_prop) = command.properties.iter_mut().find(|p| p.name == "params") {
                                    params_prop.value = command_value;
                                }

                                return Ok(());
                            }
                        }
                        _ => {}
                    }
                }
                
                // Add the line (we start from line_num + 1, so first line is skipped)
                param_lines.push(current_line.trim());
            }
            
            return Ok(()); // Found and processed the command, return early
        }
    }
    }
    
    Ok(())
}