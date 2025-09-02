use regex::Regex;
use crate::command_parser::{CommandParams, CommandDefinition};
use crate::module::Module;

#[derive!(Debug, Clone)]
pub struct Property {
    is_enum: bool,
    is_primitive: bool,
    is_optional: bool,
    name: String,
    value: String,
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
pub fn parse_command_parameters(command_lines: &[&str], cddl_strings: Vec<&str>, module: &Module, command_def: &mut CommandDefinition) -> Result<String, Box<dyn std::error::Error>> {
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
pub fn process_cddl_to_struct(cddl_content: &str, cddl_strings: Vec<&str>, module: &Module) -> Result<Vec<Property>, Box<dyn std::error::Error>> {
    let mut properties = Vec::new();
    
    // Parse each line in the CDDL body
    for line in cddl_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue; // Skip empty lines and comments
        }
        
        // Parse line like: "? userContext: browser.UserContext,"
        if let Some(property) = parse_cddl_property_line(line)? {
            properties.push(property);
        }
    }
    
    Ok(properties)
}

/// Parses a single CDDL property line and returns a Property struct
/// 
/// # Arguments
/// * `line` - A single line of CDDL content (e.g., "? userContext: browser.UserContext,")
/// 
/// # Returns
/// Optional Property if the line contains a valid property definition
fn parse_cddl_property_line(line: &str) -> Result<Option<Property>, Box<dyn std::error::Error>> {
    // Pattern for: [?] propertyName: type[,]
    let property_pattern = Regex::new(r"^\s*(\??\s*)(\w+):\s*(.+?)(?:,\s*)?$")?;
    
    if let Some(captures) = property_pattern.captures(line) {
        let optional_marker = captures[1].trim();
        let property_name = captures[2].trim().to_string();
        let property_type = captures[3].trim().trim_end_matches(',').to_string();
        
        let is_optional = optional_marker.contains('?');
        let (rust_type, is_primitive) = convert_cddl_type_to_rust(&property_type);
        
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

/// Converts a CDDL type to a Rust type
/// 
/// # Arguments
/// * `cddl_type` - The CDDL type string
/// 
/// # Returns
/// A tuple of (rust_type, is_primitive)
fn convert_cddl_type_to_rust(cddl_type: &str) -> (String, bool) {
    // First, check if it's an array
    if let Some(array_type) = detect_and_convert_array(cddl_type) {
        return array_type;
    }
    
    // Then check primitive types
    match cddl_type {
        "text" => ("String".to_string(), true),
        "bool" => ("bool".to_string(), true),
        "js-uint" => ("u64".to_string(), true),
        "js-int" => ("i64".to_string(), true),
        "float" => ("f64".to_string(), true),
        _ => {
            // Custom types, objects - keep as-is
            (cddl_type.to_string(), false)
        }
    }
}

/// Detects and converts CDDL array types to Rust Vec types
/// 
/// # Arguments
/// * `cddl_type` - The CDDL type string to check
/// 
/// # Returns
/// Optional tuple of (rust_type, is_primitive) if it's an array
fn detect_and_convert_array(cddl_type: &str) -> Option<(String, bool)> {
    // Pattern for arrays: [+type] or [*type] 
    let array_pattern = Regex::new(r"^\s*\[\s*[\+\*]\s*(.+?)\s*\]\s*$").ok()?;
    
    if let Some(captures) = array_pattern.captures(cddl_type) {
        let inner_type = captures[1].trim();
        
        // Recursively convert the inner type (handles nested arrays)
        let (rust_inner_type, _) = convert_cddl_type_to_rust(inner_type);
        
        // Arrays are not primitive, even if they contain primitives
        return Some((format!("Vec<{}>", rust_inner_type), false));
    }
    
    None
}