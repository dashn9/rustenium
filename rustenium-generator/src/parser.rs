use regex::Regex;
use crate::command_parser::{CommandParams, CommandDefinition};
use crate::module::Module;

#[derive(Debug, Clone)]
pub struct Property {
    is_enum: bool,
    is_primitive: bool,
    pub(crate) is_optional: bool,
    pub(crate) name: String,
    pub(crate) value: String,
    attributes: Vec<String>,
}
/// Parses parameter definitions from CDDL content
/// This module handles the complex parsing of WebDriver BiDi command parameters
/// from CDDL definitions, creating reusable parsing functions.

/// Parses parameters from a CDDL command definition between parentheses
/// 
/// # Arguments
/// * `command_lines` - The lines of CDDL content for the command definition
/// * `cddl_strings` - All CDDL content for searching parameter definitions
/// * `module` - The Module struct containing module information
/// * `command_def` - Mutable reference to CommandDefinition to add parsed params
/// 
/// # Returns
/// The parsed parameters as a string representation
pub fn parse_command_parameters(command_lines: &[&str], cddl_strings: Vec<&str>, module: &mut Module, command_def: &mut CommandDefinition) -> Result<String, Box<dyn std::error::Error>> {
    // First, find the params line in the command definition
    let mut params_type = None;
    for line in command_lines {
        let line = line.trim();
        
        // Look for params line: "params: session.NewParameters"
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
        // Search for the parameter definition (e.g., "browser.CreateUserContextParameters = {")
        let pattern = format!(r"^{}\s*=\s*\{{", regex::escape(&param_type));
        let regex = Regex::new(&pattern)?;
        
        for cddl_content in cddl_strings.clone() {
            let lines: Vec<&str> = cddl_content.lines().collect();
            
            for (line_num, line) in lines.iter().enumerate() {
                if regex.is_match(line.trim()) {
                    // Found the parameter definition, extract content between braces
                    let mut brace_count = 0;
                    let mut found_start = false;
                    let mut param_content_lines = Vec::new();
                    
                    for i in line_num..lines.len() {
                        let current_line = lines[i];
                        
                        for ch in current_line.chars() {
                            match ch {
                                '{' => {
                                    brace_count += 1;
                                    found_start = true;
                                }
                                '}' => {
                                    brace_count -= 1;
                                    if brace_count == 0 && found_start {
                                        // Found the end, process the extracted content
                                        let content = param_content_lines.join("\n").trim().to_string();
                                        let processed_struct = process_cddl_to_struct(&content, cddl_strings.clone(), module)?;
                                        return Ok(content);
                                    }
                                }
                                _ => {}
                            }
                        }
                        
                        // Add the line if we're inside the braces (skip the first line)
                        if found_start && brace_count > 0 && i > line_num {
                            param_content_lines.push(current_line.trim());
                        }
                    }
                    
                    return Ok(format!("Found but couldn't extract: {}", param_type));
                }
            }
        }
        
        return Ok(format!("params: {}", param_type));
    }
    
    Ok(String::new())
}

/// Extracts parameter types from CDDL content
/// 
/// # Arguments
/// * `cddl_content` - Raw CDDL content to parse
/// 
/// # Returns
/// A vector of parameter type definitions
pub fn extract_parameter_types(cddl_content: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // TODO: Parse different parameter types from CDDL
    // Handle types like: text, bool, js-uint, custom types, arrays, objects, etc.
    
    Ok(Vec::new()) // Placeholder for now
}

/// Processes CDDL content and converts it to a Rust parameter struct
/// 
/// # Arguments
/// * `cddl_content` - The CDDL content lines to process
/// * `cddl_strings` - All CDDL content for type lookups
/// * `module` - The Module struct containing module information
/// 
/// # Returns
/// Array of Property structs parsed from the CDDL content
pub fn process_cddl_to_struct(cddl_content: &str, cddl_strings: Vec<&str>, module: &mut Module) -> Result<Vec<Property>, Box<dyn std::error::Error>> {
    let mut properties = Vec::new();
    
    // Parse each line in the CDDL body
    for line in cddl_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue; // Skip empty lines and comments
        }
        
        // Parse line like: "? userContext: browser.UserContext,"
        if let Some(property) = parse_cddl_property_line(line, &cddl_strings, module)? {
            properties.push(property);
        }
    }
    
    Ok(properties)
}

/// Parses a single CDDL property line and returns a Property struct
/// 
/// # Arguments
/// * `line` - A single line of CDDL content (e.g., "? userContext: browser.UserContext,")
/// * `cddl_strings` - All CDDL content for type lookups
/// * `current_module` - Current module being processed
/// 
/// # Returns
/// Optional Property if the line contains a valid property definition
fn parse_cddl_property_line(line: &str, cddl_strings: &[&str], current_module: &mut Module) -> Result<Option<Property>, Box<dyn std::error::Error>> {
    // Pattern for: [?] propertyName: type[,]
    let property_pattern = Regex::new(r"^\s*(\??\s*)(\w+):\s*(.+?)(?:,\s*)?$")?;

    // TODO: Move enum to the type or property holder, for context
    // browser.SetClientWindowStateParameters = {
    //     clientWindow: browser.ClientWindow,
    //     (browser.ClientWindowNamedState // browser.ClientWindowRectState)
    // }
    // the second property should be an untagged serde
    if let Some(captures) = property_pattern.captures(line) {
        let optional_marker = captures[1].trim();
        let property_name = captures[2].trim().to_string();
        let property_type = captures[3].trim().trim_end_matches(',').to_string();
        
        let is_optional = optional_marker.contains('?');
        let (rust_type, is_primitive) = convert_cddl_type_to_rust(&property_type, cddl_strings, current_module);
        
        return Ok(Some(Property {
            is_enum: false, // TODO: Detect enums later
            is_primitive,
            is_optional,
            name: property_name,
            value: rust_type,
            attributes: Vec::new(), // TODO: Add attributes if needed
        }));
    }
    
    Ok(None)
}

/// Searches for a type definition across all CDDL content
/// 
/// # Arguments
/// * `type_name` - The type name to search for (e.g., "session.CapabilityRequest", "ErrorCode")
/// * `cddl_strings` - All CDDL content to search through
/// * `current_module` - The current module being processed for context
/// 
/// # Returns
/// Option containing the found type definition content if found
fn find_type_definition(type_name: &str, cddl_strings: &[&str], current_module: &Module) -> Option<String> {
    // Try direct match first (e.g., "session.CapabilityRequest = {")
    let direct_pattern = format!(r"^{}\s*=\s*[\{{\(]", regex::escape(type_name));
    if let Ok(regex) = Regex::new(&direct_pattern) {
        for cddl_content in cddl_strings {
            for line in cddl_content.lines() {
                if regex.is_match(line.trim()) {
                    return Some(format!("Found type definition: {}", type_name));
                }
            }
        }
    }
    
    // If no module prefix, try with current module prefix
    if !type_name.contains('.') {
        let prefixed_type = format!("{}.{}", current_module.name.to_lowercase(), type_name);
        return find_type_definition(&prefixed_type, cddl_strings, current_module);
    }
    
    None
}

/// Checks if a custom type belongs to the current module
/// 
/// # Arguments
/// * `type_name` - The type name to check (e.g., "session.CapabilityRequest")
/// * `current_module` - Current module being processed
/// 
/// # Returns
/// True if the type belongs to the current module
fn is_same_module_type(type_name: &str, current_module: &Module) -> bool {
    if let Some(dot_pos) = type_name.find('.') {
        let module_prefix = &type_name[..dot_pos];
        return module_prefix.to_lowercase() == current_module.name.to_lowercase();
    }
    // If no module prefix, assume it belongs to current module
    true
}

/// Generates Rust code for a custom type if it belongs to the same module
/// 
/// # Arguments
/// * `type_name` - The full type name (e.g., "session.CapabilityRequest")
/// * `content` - The CDDL definition content
/// * `def_type` - The definition type ("struct", "enum", "tuple", "alias")
/// * `cddl_strings` - All CDDL content for nested type resolution
/// * `current_module` - Mutable reference to current module being processed
fn generate_type_if_same_module(type_name: &str, content: &str, def_type: &str, cddl_strings: &[&str], current_module: &mut Module) {
    // Check if same module
    if !is_same_module_type(type_name, current_module) {
        return;
    }
    
    // Extract clean type name without module prefix
    let clean_name = if let Some(dot_pos) = type_name.find('.') {
        &type_name[dot_pos + 1..]
    } else {
        type_name
    };
    
    // Check if type already exists in module
    if current_module.types.iter().any(|t| t.name == clean_name) {
        return; // Type already exists, don't generate again
    }
    
    // Use existing process_cddl_to_struct for recursive parsing
    if let Ok(properties) = process_cddl_to_struct(content, cddl_strings.to_vec(), current_module) {
        // Create BidiType with the properties and store in module
        current_module.types.push(crate::module::BidiType {
            name: clean_name.to_string(),
            properties,
            raw: content.to_string(),
        });
    }
}

/// Parses a custom CDDL type and handles generation if it's in the same module
/// 
/// # Arguments
/// * `type_name` - The custom type name to parse
/// * `cddl_strings` - All CDDL content for lookup
/// * `current_module` - Current module being processed
/// * `struct_name` - Optional struct name for context
/// * `property_name` - Optional property name for context
/// 
/// # Returns
/// The Rust type name to use for this custom type
fn parse_custom_type(type_name: &str, cddl_strings: &[&str], current_module: &mut Module, struct_name: Option<&str>, property_name: Option<&str>) -> String {
    // Handle single literal strings (quoted values)
    if type_name.starts_with('"') && type_name.ends_with('"') {
        let literal_value = &type_name[1..type_name.len()-1]; // Remove quotes
        let enum_name = format!("{}Enum", literal_value.replace(" ", "").replace("-", ""));
        
        let properties = vec![Property {
            is_enum: true,
            is_primitive: false,
            is_optional: false,
            name: enum_name.clone(),
            value: enum_name.clone(),
            attributes: vec![format!(r#"#[serde(rename = "{}")]"#, literal_value)],
        }];
        
        current_module.types.push(crate::module::BidiType {
            name: enum_name.clone(),
            properties,
            raw: type_name.to_string(),
        });
        
        return enum_name;
    }
    
    // Handle union types with / or // separator
    if type_name.contains(" // ") || type_name.contains(" / ") {
        // Break into parts - handle both // and / separators
        let mut parts: Vec<String> = if type_name.contains(" // ") {
            type_name.split(" // ").map(|s| s.trim().to_string()).collect()
        } else {
            type_name.split(" / ").map(|s| s.trim().to_string()).collect()
        };
        
        // Check if null is present
        let has_null = parts.iter().any(|part| part == "null");
        
        // Remove all nulls
        parts.retain(|part| part != "null");
        
        // Process remaining types through convert_basic_cddl_type
        let processed_types: Vec<(String, Vec<String>)> = parts
            .into_iter()
            .map(|part| {
                // Check if part is a literal string
                if part.starts_with('"') && part.ends_with('"') {
                    let literal_value = &part[1..part.len()-1]; // Remove quotes
                    let variant_name = literal_value.replace(" ", "").replace("-", "");
                    let attributes = vec![format!(r#"#[serde(rename = "{}")]"#, literal_value)];
                    (variant_name, attributes)
                } else {
                    let (rust_type, _) = convert_basic_cddl_type(&part, cddl_strings, current_module);
                    (rust_type, Vec::new()) // No attributes for non-literals
                }
            })
            .collect();
        
        // Determine final type
        return match processed_types.len() {
            0 => "()".to_string(), // Only null was present
            1 => {
                let (base_type, _) = &processed_types[0];
                if has_null {
                    format!("Option<{}>", base_type)
                } else {
                    base_type.clone()
                }
            }
            _ => {
                // Multiple non-null types - create enum using struct and property names
                let enum_name = match (struct_name, property_name) {
                    (Some(s), Some(p)) => format!("{}{}", s, p),
                    (Some(s), None) => format!("{}Union", s),
                    (None, Some(p)) => format!("{}Union", p),
                    (None, None) => format!("Union{}", current_module.types.len()),
                };
                
                // Create enum variants from processed types
                let properties: Vec<Property> = processed_types
                    .iter()
                    .map(|(rust_type, attributes)| Property {
                        is_enum: true,
                        is_primitive: false,
                        is_optional: false,
                        name: format!("{}{}", enum_name, rust_type),
                        value: format!("{}{}", enum_name, rust_type),
                        attributes: attributes.clone(),
                    })
                    .collect();
                
                // Create and add BidiType to module
                current_module.types.push(crate::module::BidiType {
                    name: enum_name.clone(),
                    properties,
                    raw: type_name.to_string(),
                });
                
                if has_null {
                    format!("Option<{}>", enum_name)
                } else {
                    enum_name
                }
            }
        };
    }
    
    // Check if this type belongs to the current module
    if is_same_module_type(type_name, current_module) {
        // TODO: Generate type if needed
        
        // Return only the type name (without module prefix)
        if let Some(dot_pos) = type_name.find('.') {
            type_name[dot_pos + 1..].to_string()
        } else {
            type_name.to_string()
        }
    } else {
        // Return module::type format for external modules
        if let Some(dot_pos) = type_name.find('.') {
            let module_name = &type_name[..dot_pos];
            let type_name_clean = &type_name[dot_pos + 1..];
            format!("{}::{}", module_name, type_name_clean)
        } else {
            type_name.to_string()
        }
    }
}

/// Checks if a type is a custom type that should be looked up in CDDL
/// 
/// # Arguments
/// * `type_name` - The type name to check
/// * `cddl_strings` - All CDDL content for lookup
/// * `current_module` - Current module being processed
/// 
/// # Returns
/// True if this is a custom type that exists in CDDL definitions
fn is_custom_type(type_name: &str, cddl_strings: &[&str], current_module: &Module) -> bool {
    // Check if it contains a module prefix (e.g., "session.CapabilityRequest")
    if type_name.contains('.') {
        return find_type_definition(type_name, cddl_strings, current_module).is_some();
    }
    
    // Check if it exists as a global type or with current module prefix
    find_type_definition(type_name, cddl_strings, current_module).is_some()
}

/// Converts basic CDDL primitive types to Rust types
/// 
/// # Arguments
/// * `cddl_type` - The CDDL type string
/// * `cddl_strings` - All CDDL content for type lookups
/// * `current_module` - Current module being processed
/// 
/// # Returns
/// A tuple of (rust_type, is_primitive)
fn convert_basic_cddl_type(cddl_type: &str, cddl_strings: &[&str], current_module: &mut Module) -> (String, bool) {
    match cddl_type {
        "text" => ("String".to_string(), true),
        "bool" => ("bool".to_string(), true),
        "js-uint" => ("u64".to_string(), true),
        "js-int" => ("i64".to_string(), true),
        "float" => ("f64".to_string(), true),
        "null" => ("Option<()>".to_string(), false), // null is typically represented as Option
        _ => {
            // Check if it's a custom type that exists in CDDL
            if is_custom_type(cddl_type, cddl_strings, current_module) {
                // Parse the custom type (this function will handle generation if same module)
                let parsed_type = parse_custom_type(cddl_type, cddl_strings, current_module, None, None);
                (parsed_type, false)
            } else {
                // Unknown type - keep as-is but mark as non-primitive
                (cddl_type.to_string(), false)
            }
        }
    }
}

/// Splits a string by commas while respecting nested brackets
/// 
/// # Arguments
/// * `input` - The string to split
/// 
/// # Returns
/// Vector of strings split by top-level commas only
fn split_respecting_brackets(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut bracket_depth = 0;
    
    for ch in input.chars() {
        match ch {
            '[' => {
                bracket_depth += 1;
                current.push(ch);
            }
            ']' => {
                bracket_depth -= 1;
                current.push(ch);
            }
            ',' if bracket_depth == 0 => {
                result.push(current.trim().to_string());
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }
    
    if !current.is_empty() {
        result.push(current.trim().to_string());
    }
    
    result
}

/// Extracts content from within brackets, handling nested structures
/// 
/// # Arguments
/// * `input` - The bracketed string (e.g., "[content]")
/// 
/// # Returns
/// The content inside the outermost brackets
fn extract_bracket_content(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
        return None;
    }
    
    // Remove outer brackets
    let content = &trimmed[1..trimmed.len()-1];
    Some(content.trim().to_string())
}

/// Converts a CDDL type to a Rust type, handling arrays, tuples, and basic types
/// 
/// # Arguments
/// * `cddl_type` - The CDDL type string
/// * `cddl_strings` - All CDDL content for type lookups
/// * `current_module` - Current module being processed
/// 
/// # Returns
/// A tuple of (rust_type, is_primitive)
fn convert_cddl_type_to_rust(cddl_type: &str, cddl_strings: &[&str], current_module: &mut Module) -> (String, bool) {
    let trimmed = cddl_type.trim();
    
    // Check if it's a bracketed type [...]
    if let Some(inner_content) = extract_bracket_content(trimmed) {
        // Check for arrays: [+type] or [*type]
        if let Ok(array_pattern) = Regex::new(r"^\s*[\+\*]\s*(.+)$") {
            if let Some(captures) = array_pattern.captures(&inner_content) {
                let inner_type = captures[1].trim();
                
                // Recursively convert the inner type
                let (rust_inner_type, _) = convert_cddl_type_to_rust(inner_type, cddl_strings, current_module);
                
                // Arrays are not primitive
                return (format!("Vec<{}>", rust_inner_type), false);
            }
        }
        
        // Check for tuples by splitting on top-level commas
        let parts = split_respecting_brackets(&inner_content);
        if parts.len() > 1 {
            // It's a tuple - convert each part
            let tuple_types: Vec<String> = parts
                .into_iter()
                .map(|part| {
                    let (rust_type, _) = convert_cddl_type_to_rust(&part, cddl_strings, current_module);
                    rust_type
                })
                .collect();
            
            // Tuples are not primitive
            return (format!("({})", tuple_types.join(", ")), false);
        } else if parts.len() == 1 {
            // Single item in brackets - could be a single-element array or just grouped
            // For now, treat as the inner type
            let (rust_type, is_primitive) = convert_cddl_type_to_rust(&parts[0], cddl_strings, current_module);
            return (rust_type, is_primitive);
        }
    }
    
    // Fall back to basic type conversion
    convert_basic_cddl_type(trimmed, cddl_strings, current_module)
}

    