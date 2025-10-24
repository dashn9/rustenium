use regex::Regex;
use crate::module::Module;
use crate::parser;

/// Represents a complete event definition parsed from CDDL (Concise Data Definition Language)
/// This contains both the raw CDDL content and the parsed event structures
#[derive(Debug)]
pub struct EventDefinition {
    /// The name of the event definition (e.g., "BrowserEvent", "NetworkEvent")
    pub name: String,
    /// The raw CDDL content for this event definition
    pub content: String,
    /// Parsed individual events found within this definition
    pub events: Vec<Event>,
    /// Generated event methods
    pub event_methods: Vec<EventMethods>,
    /// Generated event parameters
    pub event_params: Vec<EventParams>,
    /// Rust attributes to be applied to the generated enum (e.g., #[derive(Debug, Serialize)])
    pub attributes: Vec<String>,
}

/// Parameters for a WebDriver BiDi event
#[derive(Debug, Clone)]
pub struct EventParams {
    /// The parameter struct name (e.g., "BrowserContextCreatedParameters")
    pub name: String,
    /// Properties of the parameter struct
    pub properties: Vec<crate::parser::Property>,
    /// Rust attributes to be applied to this parameter struct
    pub attributes: Vec<String>,
}

/// Represents a WebDriver BiDi event method with its serialization attributes
#[derive(Debug, Clone)]
pub struct EventMethods {
    /// The Rust method name (e.g., "BrowserContextCreated")
    pub name: String,
    /// Attributes for the method variants (e.g., serde rename)
    pub method_attributes: Vec<String>,
    /// Attributes for the enum itself (e.g., derive attributes)
    pub enum_attributes: Vec<String>,
}

/// Represents a single WebDriver BiDi event parsed from CDDL
/// Events follow the pattern module.event (e.g., "drivers.contextCreated", "network.responseReceived")
#[derive(Debug, Clone)]
pub struct Event {
    /// The event name as found in CDDL (e.g., "contextCreated", "responseReceived")
    pub name: String,
    /// The name of the module this event belongs to (e.g., "drivers", "network", "session")
    pub module_name: String,
    /// Rust attributes to be applied to this event variant
    pub attributes: Vec<String>,
    /// Properties for the event struct
    /// Expected to contain: method property (with rename attribute) and params property (with rename attribute)
    pub properties: Vec<crate::parser::Property>,
}

/// Parses a CDDL event definition and extracts individual event types
///
/// This function takes the raw CDDL content for an event definition (e.g., BrowserEvent)
/// and parses it to extract individual event types that follow the pattern "module.EventType /"
///
/// # Arguments
/// * `name` - The name of the event definition (e.g., "BrowserEvent")
/// * `content` - The raw CDDL content containing event definitions
/// * `cddl_strings` - All CDDL content for searching event details
/// * `module` - The module to populate with parsed events
///
/// # Returns
/// An EventDefinition struct containing the parsed events and metadata
pub fn parse_event_definition(name: String, content: String, cddl_strings: Vec<&str>, module: &mut Module) -> Result<EventDefinition, Box<dyn std::error::Error>> {
    let event_enum_pattern = Regex::new(r"(\w+)\.(\w+)\s*")?;
    let mut events = Vec::new();

    // Create a mutable event definition to build up
    let mut event_def = EventDefinition {
        name: name.clone(),
        content: content.clone(),
        events: Vec::new(),
        event_methods: Vec::new(),
        event_params: Vec::new(),
        attributes: vec![
            "#[derive(Debug, Clone, Serialize, Deserialize)]".to_string(),
            "#[serde(untagged)]".to_string(),
        ],
    };

    // Extract event names from the enum definition
    for line in content.lines() {
        let line = line.trim();

        if let Some(captures) = event_enum_pattern.captures(line) {
            let module_name = captures[1].to_string(); // Extract module name (before the dot)
            let name = captures[2].to_string(); // Extract event name (after the dot)

            let rust_method_name = format!("{}{}",
                                           module_name.replace("_", "").to_string().chars().next().unwrap().to_uppercase().to_string() + &module_name[1..],
                                           name
            );
            let mut event = Event {
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

            // Search and update the event in all CDDL content
            search_and_update_event(cddl_strings.clone(), &mut event, &mut event_def, module)?;
            events.push(event);
        }
    }

    // Update the event definition with the processed events
    event_def.events = events;
    Ok(event_def)
}

/// Searches for a specific event in CDDL content and updates the event with parsed data
///
/// This function takes an Event struct and searches for its corresponding definition
/// in the CDDL content using the pattern "module.event //". When found, it will
/// update the event with the parsed method and parameter information.
///
/// # Arguments
/// * `cddl_strings` - The array of CDDL content strings to search through
/// * `event` - Mutable reference to the Event struct to update
/// * `event_def` - Mutable reference to the EventDefinition to update
/// * `module` - The module context for parsing
///
/// # Returns
/// Ok(()) if successful, or an error if parsing fails
pub fn search_and_update_event(cddl_strings: Vec<&str>, event: &mut Event, event_def: &mut EventDefinition, module: &mut Module) -> Result<(), Box<dyn std::error::Error>> {
    // Search for event definition using pattern: module_name.name = (
    let method_name = format!("{}.{}", event.module_name, event.name);
    let pattern = format!(r"^{}\s*=\s*\(", regex::escape(&method_name));
    let regex = Regex::new(&pattern)?;

    // Search through all CDDL content strings
    for cddl_content in cddl_strings.clone() {
        let lines: Vec<&str> = cddl_content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            if regex.is_match(line.trim()) {
                // Collect lines between the parentheses first
                let mut paren_count = 0;
                let mut param_lines: Vec<&str> = Vec::new();

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

                                    // Create EventMethods with the actual method value
                                    let rust_method_name = format!("{}{}",
                                        event.module_name.replace("_", "").chars().next().unwrap().to_uppercase().to_string() + &event.module_name[1..],
                                        event.name
                                    );

                                    if !event_def.event_methods.iter().any(|m| m.name == rust_method_name) {
                                        event_def.event_methods.push(EventMethods {
                                            name: format!("{}Method", rust_method_name),
                                            method_attributes: vec![format!(r#"#[serde(rename = "{}")]"#, actual_method_value)],
                                            enum_attributes: vec![
                                                "#[derive(Debug, Clone, Serialize, Deserialize)]".to_string(),
                                                "#[serde(untagged)]".to_string()
                                            ],
                                        });
                                    }

                                    // Parse the parameter content
                                    let event_value = parse_event_parameters(&param_lines, cddl_strings.clone(), module, event_def)?;
                                    if let Some(params_prop) = event.properties.iter_mut().find(|p| p.name == "params") {
                                        params_prop.value = event_value;
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

                return Ok(()); // Found and processed the event, return early
            }
        }
    }

    Ok(())
}

/// Parses event parameter definitions - duplicated from command parsing logic
pub fn parse_event_parameters(event_lines: &[&str], cddl_strings: Vec<&str>, module: &mut Module, event_def: &mut EventDefinition) -> Result<String, Box<dyn std::error::Error>> {
    // First, find the params line in the event definition
    let mut params_type = None;
    for line in event_lines {
        let line = line.trim();

        // Look for params line: "params: session.NewParams"
        if line.starts_with("params:") {
            // Extract the parameter type after "params:"
            if let Some(param_type) = line.strip_prefix("params:").map(|s| s.trim().trim_end_matches(',')) {
                params_type = Some(param_type.to_string());
                break;
            }
        }
    }

    // If we found a parameter type, search for its definition in all CDDL content
    if let Some(param_type) = params_type {
        // Create the parameter struct name from the param_type
        let param_struct_name = if let Some(dot_pos) = param_type.find('.') {
            // Remove module prefix and use just the type name
            param_type[dot_pos + 1..].to_string()
        } else {
            param_type.clone()
        };

        // Check if this ends with "Params" - if so, generate EventParams, otherwise use process_cddl_to_struct
        if param_struct_name.ends_with("Params") {
            // Search for the parameter definition (e.g., "drivers.CreateUserContextParams = {" or "drivers.CreateUserContextParams = (")
            let pattern = format!(r"^{}\s*=\s*[\{{(]", regex::escape(&param_type));
            let regex = Regex::new(&pattern)?;

            for cddl_content in cddl_strings.clone() {
                let lines: Vec<&str> = cddl_content.lines().collect();

                for (line_num, line) in lines.iter().enumerate() {
                    if regex.is_match(line.trim()) {
                        // Found the parameter definition, determine if it uses braces or parentheses
                        let uses_braces = line.trim().contains("= {");
                        let (open_char, close_char) = if uses_braces { ('{', '}') } else { ('(', ')') };

                        // Extract content between delimiters
                        let mut delimiter_count = 0;
                        let mut found_start = false;
                        let mut param_content_lines = Vec::new();

                        for i in line_num..lines.len() {
                            let current_line = lines[i];

                            for ch in current_line.chars() {
                                if ch == open_char {
                                    delimiter_count += 1;
                                    found_start = true;
                                } else if ch == close_char {
                                    delimiter_count -= 1;
                                    if delimiter_count == 0 && found_start {
                                        // Found the end, process the extracted content
                                        let mut content = param_content_lines.join("\n").trim().to_string();
                                        let is_enum = !uses_braces; // If using parentheses, might be an enum

                                        // If using parentheses, wrap content to retain them
                                        if !uses_braces {
                                            content = format!("(\n{}\n)", content);
                                        }

                                        let (mut processed_properties, _) = parser::process_cddl_to_struct(&content, cddl_strings.clone(), module, None)?;

                                        // If is_enum and only one property was generated, check if it's a generated enum type
                                        if is_enum && processed_properties.len() == 1 {
                                            if let Some(idx) = module.types.iter().position(|t| &t.name == &processed_properties[0].value) {
                                                let bidi_type = module.types.remove(idx);
                                                processed_properties = bidi_type.properties;
                                            }
                                        }

                                        // Create EventParams and add to event_def
                                        let event_param = EventParams {
                                            name: param_struct_name.clone(),
                                            properties: processed_properties,
                                            attributes: vec![
                                                "#[derive(Debug, Clone, Serialize, Deserialize)]".to_string(),
                                            ],
                                        };

                                        // Check if this param already exists to avoid duplicates
                                        if !event_def.event_params.iter().any(|p| p.name == param_struct_name) {
                                            event_def.event_params.push(event_param);
                                        }

                                        return Ok(param_struct_name);
                                    }
                                }
                            }

                            // Add the line if we're inside the delimiters (skip the first line)
                            if found_start && delimiter_count > 0 && i > line_num {
                                param_content_lines.push(current_line.trim());
                            }
                        }

                        return Ok(param_struct_name);
                    }
                }
            }

            return Ok(param_struct_name);
        } else {
            // This is a regular type - run it through process_cddl_to_struct and add to module.types
            let pattern = format!(r"^{}\s*=\s*[\{{(]", regex::escape(&param_type));
            let regex = Regex::new(&pattern)?;

            for cddl_content in cddl_strings.clone() {
                let lines: Vec<&str> = cddl_content.lines().collect();

                for (line_num, line) in lines.iter().enumerate() {
                    if regex.is_match(line.trim()) {
                        // Found the type definition, determine if it uses braces or parentheses
                        let uses_braces = line.trim().contains("= {");
                        let (open_char, close_char) = if uses_braces { ('{', '}') } else { ('(', ')') };

                        // Extract content between delimiters
                        let mut delimiter_count = 0;
                        let mut found_start = false;
                        let mut type_content_lines = Vec::new();

                        for i in line_num..lines.len() {
                            let current_line = lines[i];

                            for ch in current_line.chars() {
                                if ch == open_char {
                                    delimiter_count += 1;
                                    found_start = true;
                                } else if ch == close_char {
                                    delimiter_count -= 1;
                                    if delimiter_count == 0 && found_start {
                                        // Found the end, process through process_cddl_to_struct
                                        let mut content = type_content_lines.join("\n").trim().to_string();
                                        let is_enum = !uses_braces; // If using parentheses, might be an enum

                                        // If using parentheses, wrap content to retain them
                                        if !uses_braces {
                                            content = format!("(\n{}\n)", content);
                                        }

                                        let (mut processed_properties, _) = parser::process_cddl_to_struct(&content, cddl_strings.clone(), module, None)?;

                                        // If is_enum and only one property was generated, check if it's a generated enum type
                                        if is_enum && processed_properties.len() == 1 {
                                            if let Some(idx) = module.types.iter().position(|t| &t.name == &processed_properties[0].value) {
                                                let bidi_type = module.types.remove(idx);
                                                processed_properties = bidi_type.properties;
                                            }
                                        }

                                        // Create BidiType and add to module.types
                                        let bidi_type = crate::module::BidiType {
                                            name: param_struct_name.clone(),
                                            properties: processed_properties,
                                            is_enum: false,
                                            is_alias: false,
                                        };

                                        // Check if this type already exists to avoid duplicates
                                        if !module.types.iter().any(|t| t.name == param_struct_name) {
                                            module.types.push(bidi_type);
                                        }

                                        return Ok(param_struct_name);
                                    }
                                }
                            }

                            // Add the line if we're inside the delimiters (skip the first line)
                            if found_start && delimiter_count > 0 && i > line_num {
                                type_content_lines.push(current_line.trim());
                            }
                        }

                        return Ok(param_struct_name);
                    }
                }
            }

            return Ok(param_struct_name);
        }
    }

    Ok(String::new())
}

