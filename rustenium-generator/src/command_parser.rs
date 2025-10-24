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

/// Represents a complete result definition parsed from CDDL (Concise Data Definition Language)
/// This contains both the raw CDDL content and the parsed result structures
#[derive(Debug)]
pub struct ResultDefinition {
    /// The name of the result definition (e.g., "BrowserResult", "NetworkResult")
    pub name: String,
    /// The raw CDDL content for this result definition
    pub content: String,
    /// Parsed individual result types found within this definition
    pub results: Vec<BidiResult>,
    /// Rust attributes to be applied to the generated enum (e.g., #[derive(Debug, Serialize)])
    pub attributes: Vec<String>,
}

/// Represents a single WebDriver BiDi result type parsed from CDDL
/// Results follow the pattern module.ResultType (e.g., "drivers.CreateUserContextResult")
#[derive(Debug, Clone)]
pub struct BidiResult {
    /// The result type name as found in CDDL (e.g., "CreateUserContextResult", "GetTreeResult")
    pub name: String,
    /// The module name this result belongs to (e.g., "drivers", "session")
    pub module_name: String,
    /// Properties for the result struct
    pub properties: Vec<crate::parser::Property>,
    /// Rust attributes to be applied to this result variant
    pub attributes: Vec<String>,
    /// The raw CDDL content for this result
    pub content: String,
    /// Whether this is a type alias (true) or struct definition (false)
    pub is_alias: bool,
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
    let command_enum_pattern = Regex::new(r"(\w+)\.(\w+)\s*")?;
    let mut commands = Vec::new();

    // Create a mutable command definition to build up
    let mut command_def = CommandDefinition {
        name: name.clone(),
        content: content.clone(),
        commands: Vec::new(),
        command_methods: Vec::new(),
        command_params: Vec::new(),
        attributes: vec![
            "#[derive(Debug, Clone, Serialize, Deserialize)]".to_string(),
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
                    "#[derive(Debug, Clone, Serialize, Deserialize)]".to_string(),
                ],
                properties: vec![
                    crate::parser::Property {
                        is_enum: false,
                        is_primitive: true,
                        is_optional: false,
                        name: "method".to_string(),
                        value: format!("{}Method", rust_method_name),
                        attributes: vec![r#"#[serde(rename = "method")]"#.to_string()],
                        validation_info: None,
                    },
                    crate::parser::Property {
                        is_enum: false,
                        is_primitive: false,
                        is_optional: false,
                        name: "params".to_string(),
                        value: "".to_string(), // Will be set to param_content later
                        attributes: vec![r#"#[serde(rename = "params")]"#.to_string()],
                        validation_info: None,
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
#[derive(Debug, Clone)]
pub struct CommandParams {
    /// The parameter struct name (e.g., "BrowserCloseParameters")
    pub name: String,
    /// Properties of the parameter struct
    pub properties: Vec<crate::parser::Property>,
    /// Rust attributes to be applied to this parameter struct
    pub attributes: Vec<String>,
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
/// Commands follow the pattern module.method (e.g., "drivers.close", "network.enable")
#[derive(Debug, Clone)]
pub struct Command {
    /// The command name as found in CDDL (e.g., "close", "enable")
    pub name: String,
    /// The name of the module this command belongs to (e.g., "drivers", "network", "session")
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
            // Collect lines between the parentheses
            let mut paren_count = 0;
            let mut param_lines: Vec<&str> = Vec::new();

            for i in (line_num + 1)..lines.len() {
                let current_line = lines[i];

                for ch in current_line.chars() {
                    match ch {
                        '(' => paren_count += 1,
                        ')' => {
                            paren_count -= 1;
                            if paren_count < 0 {
                                // Extract the actual method value from CDDL
                                let mut actual_method_value = method_name.clone();
                                for line in &param_lines {
                                    let line_trimmed = line.trim();
                                    if line_trimmed.starts_with("method:") {
                                        if let Some(method_str) = line_trimmed.strip_prefix("method:").map(|s| s.trim().trim_end_matches(',')) {
                                            actual_method_value = method_str.trim_matches('"').to_string();
                                        }
                                        break;
                                    }
                                }

                                // Create CommandMethods with the actual method value
                                let rust_method_name = format!("{}{}",
                                    command.module_name.replace("_", "").chars().next().unwrap().to_uppercase().to_string() + &command.module_name[1..],
                                    command.name
                                );

                                if !command_def.command_methods.iter().any(|m| m.name == rust_method_name) {
                                    command_def.command_methods.push(CommandMethods {
                                        name: format!("{}Method", rust_method_name),
                                        method_attributes: vec![format!(r#"#[serde(rename = "{}")]"#, actual_method_value)],
                                        enum_attributes: vec!["#[derive(Debug, Clone, Serialize, Deserialize)]".to_string()],
                                    });
                                }

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

                param_lines.push(current_line.trim());
            }

            return Ok(());
        }
    }
    }

    Ok(())
}

/// Parses a CDDL result definition and extracts individual result types
///
/// This function takes the raw CDDL content for a result definition (e.g., BrowserResult)
/// and parses it to extract individual result types that follow the pattern "module.ResultType /"
///
/// # Arguments
/// * `name` - The name of the result definition (e.g., "BrowserResult")
/// * `content` - The raw CDDL content containing result definitions
/// * `cddl_strings` - All CDDL content for searching result details
/// * `module` - The module to populate with parsed results
///
/// # Returns
/// A ResultDefinition struct containing the parsed results and metadata
pub fn parse_result_definition(name: String, content: String, cddl_strings: Vec<&str>, module: &mut Module) -> Result<ResultDefinition, Box<dyn std::error::Error>> {
    let result_enum_pattern = Regex::new(r"(\w+)\.(\w+)\s*")?;
    let mut results = Vec::new();

    // Create a mutable result definition to build up
    let mut result_def = ResultDefinition {
        name: name.clone(),
        content: content.clone(),
        results: Vec::new(),
        attributes: vec![
            "#[derive(Debug, Clone, Serialize, Deserialize)]".to_string(),
            "#[serde(untagged)]".to_string(),
        ],
    };

    // Extract result names from the enum definition
    for line in content.lines() {
        let line = line.trim();

        if let Some(captures) = result_enum_pattern.captures(line) {
            let module_name = captures[1].to_string(); // Extract module name (before the dot)
            let name = captures[2].to_string(); // Extract result name (after the dot)

            let mut bidi_result = BidiResult {
                name,
                module_name,
                properties: Vec::new(),
                attributes: vec![
                    "#[derive(Debug, Clone, Serialize, Deserialize)]".to_string(),
                ],
                content: String::new(),
                is_alias: false,
            };

            // Search and update the result in all CDDL content
            search_and_update_result(cddl_strings.clone(), &mut bidi_result, &mut result_def, module)?;
            results.push(bidi_result);
        }
    }

    // Update the result definition with the processed results
    result_def.results = results;
    Ok(result_def)
}

/// Searches for a specific result in CDDL content and updates the result with parsed data
///
/// This function takes a BidiResult struct and searches for its corresponding definition
/// in the CDDL content using the pattern "ResultName = {". When found, it will
/// update the result with the parsed structure information.
///
/// # Arguments
/// * `cddl_strings` - The array of CDDL content strings to search through
/// * `bidi_result` - Mutable reference to the BidiResult struct to update
/// * `result_def` - Mutable reference to the ResultDefinition to update
/// * `module` - The module context for parsing
///
/// # Returns
/// Ok(()) if successful, or an error if parsing fails
pub fn search_and_update_result(cddl_strings: Vec<&str>, bidi_result: &mut BidiResult, result_def: &mut ResultDefinition, module: &mut Module) -> Result<(), Box<dyn std::error::Error>> {
    // Search for result definition using pattern: result_name = { or result_name = TypeAlias
    let result_name = format!("{}.{}", bidi_result.module_name, bidi_result.name);
    let struct_pattern = format!(r"^{}\s*=\s*\{{", regex::escape(&result_name));
    let alias_pattern = format!(r"^{}\s*=\s*(.+)", regex::escape(&result_name));
    let struct_regex = Regex::new(&struct_pattern)?;
    let alias_regex = Regex::new(&alias_pattern)?;

    // Search through all CDDL content strings
    for cddl_content in cddl_strings.clone() {
        let lines: Vec<&str> = cddl_content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line_trimmed = line.trim();

            // Check for struct definition pattern: result_name = {
            if struct_regex.is_match(line_trimmed) {
                // Found the result definition

                // Collect lines between the braces (skip the first line with opening brace)
                let mut brace_count = 0;
                let mut result_lines = Vec::new();

                for i in line_num..lines.len() {
                    let current_line = lines[i];

                    for ch in current_line.chars() {
                        match ch {
                            '{' => {
                                brace_count += 1;
                            }
                            '}' => {
                                brace_count -= 1;
                                if brace_count == 0 {
                                    // Found the closing brace, join the collected lines
                                    let result_content = result_lines.join("\n").trim().to_string();
                                    bidi_result.content = result_content.clone();

                                    // Parse the result content to extract properties
                                    let (result_properties, _) = parser::process_cddl_to_struct(&result_content, cddl_strings.clone(), module, Some(&bidi_result.name))?;
                                    bidi_result.properties = result_properties;

                                    return Ok(());
                                }
                            }
                            _ => {}
                        }
                    }

                    if i > line_num { // Skip the first line with opening brace
                        result_lines.push(current_line.trim());
                    }
                }

                return Ok(()); // Found and processed the result, return early
            }

            // Check for type alias pattern: result_name = TypeAlias
            else if let Some(captures) = alias_regex.captures(line_trimmed) {
                let mut is_enum = false;
                let mut alias_type = captures[1].trim().to_string();

                // If it's just "(", use the next line instead
                // this check should be propagated across commands and event also, however a need for it only exist in script result precisely scriptEvaluateResult
                if alias_type == "(" {
                    is_enum = true;
                    let mut offset = 0;
                    alias_type = lines[line_num + 1].trim().to_string();
                    while alias_type.ends_with("/") && line_num + 1 < lines.len() {
                        offset += 1;
                        alias_type = format!("{} {}", alias_type, lines[line_num + 1 + offset].trim());
                    }
                }

                bidi_result.content = alias_type.clone();
                bidi_result.is_alias = true;

                // Still run it through process_cddl_to_struct to handle the alias type
                let (mut result_properties, _) = parser::process_cddl_to_struct(&alias_type, cddl_strings.clone(), module, Some(&bidi_result.name))?;
                if is_enum {
                    if let Some(idx) = module.types.iter().position(|t| &t.name == &result_properties[0].value) {
                        let bidi_type = module.types.remove(idx);
                        result_properties = bidi_type.properties;
                    }
                }
                bidi_result.properties = result_properties;
                return Ok(());
            }
        }
    }

    Ok(())
}