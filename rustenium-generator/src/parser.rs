use regex::Regex;
use crate::command_parser::{CommandParams, CommandDefinition};
use crate::event_parser::{EventParams, EventDefinition};
use crate::module::Module;
use std::sync::Mutex;
use anyhow::Context;

#[derive(Debug, Clone)]
pub struct DeferredType {
    pub type_name: String,
    pub content: String,
    pub def_type: String,
    pub target_module_name: String,
}

// Global storage for deferred types using Mutex<Vec<DeferredType>>
static DEFERRED_TYPES: std::sync::OnceLock<Mutex<Vec<DeferredType>>> = std::sync::OnceLock::new();

pub fn add_deferred_type(deferred_type: DeferredType) {
    let deferred_types = DEFERRED_TYPES.get_or_init(|| Mutex::new(Vec::new()));
    deferred_types.lock().unwrap().push(deferred_type);
}

pub fn take_deferred_types() -> Vec<DeferredType> {
    let deferred_types = DEFERRED_TYPES.get_or_init(|| Mutex::new(Vec::new()));
    let mut borrowed = deferred_types.lock().unwrap();
    std::mem::take(&mut *borrowed)
}

pub fn process_deferred_types(modules: &mut Vec<crate::module::Module>, cddl_strings: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let deferred_types = take_deferred_types();

    for deferred_type in deferred_types {
        // Find the target module
        if let Some(target_module) = modules.iter_mut().find(|m| m.name.to_lowercase() == deferred_type.target_module_name.to_lowercase()) {
            // Extract just the type name without module prefix
            let clean_type_name = if let Some(dot_pos) = deferred_type.type_name.find('.') {
                deferred_type.type_name[dot_pos + 1..].to_string()
            } else {
                deferred_type.type_name.clone()
            };

            // Use generate_type_if_same_module to properly process the deferred type
            generate_type_if_same_module(&clean_type_name, &deferred_type.content, &deferred_type.def_type, &cddl_strings, target_module);
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct Property {
    pub is_enum: bool,
    pub is_primitive: bool,
    pub(crate) is_optional: bool,
    pub(crate) name: String,
    pub(crate) value: String,
    pub attributes: Vec<String>,
    pub validation_info: Option<ValidationInfo>,
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
pub fn  parse_command_parameters(command_lines: &[&str], cddl_strings: Vec<&str>, module: &mut Module, command_def: &mut CommandDefinition) -> Result<String, Box<dyn std::error::Error>> {
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
        // Search for the parameter definition (e.g., "browser.CreateUserContextParameters = {" or "= TypeAlias")
        let struct_pattern = format!(r"^{}\s*=\s*\{{", regex::escape(&param_type));
        let alias_pattern = format!(r"^{}\s*=\s*(.+)", regex::escape(&param_type));
        let struct_regex = Regex::new(&struct_pattern)?;
        let alias_regex = Regex::new(&alias_pattern)?;

        for cddl_content in cddl_strings.clone() {
            let lines: Vec<&str> = cddl_content.lines().collect();

            for (line_num, line) in lines.iter().enumerate() {
                let line_trimmed = line.trim();

                // Check for struct definition pattern first
                if struct_regex.is_match(line_trimmed) {
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
                                        let (processed_properties, _) = process_cddl_to_struct(&content, cddl_strings.clone(), module, None)?;

                                        // Create the parameter struct name from the param_type
                                        let param_struct_name = if let Some(dot_pos) = param_type.find('.') {
                                            // Remove module prefix and use just the type name
                                            param_type[dot_pos + 1..].to_string()
                                        } else {
                                            param_type.clone()
                                        };

                                        // Create CommandParams and add to command_def
                                        let command_param = crate::command_parser::CommandParams {
                                            name: param_struct_name.clone(),
                                            properties: processed_properties,
                                            attributes: vec![
                                                "#[derive(Debug, Clone, Serialize, Deserialize)]".to_string(),
                                            ],
                                        };

                                        // Check if this param already exists to avoid duplicates
                                        if !command_def.command_params.iter().any(|p| p.name == param_struct_name) {
                                            command_def.command_params.push(command_param);
                                        }
                                        return Ok(param_struct_name);
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

                    return Ok(format!("{}", param_type));
                }
                // Check for alias pattern
                else if let Some(captures) = alias_regex.captures(line_trimmed) {
                    let alias_type = captures[1].trim().to_string();

                    // Process the alias type through process_cddl_to_struct
                    let (mut processed_properties, _) = process_cddl_to_struct(&alias_type, cddl_strings.clone(), module, None)?;

                    // If only one property and it references a generated enum type, extract its properties
                    if processed_properties.len() == 1 {
                        if let Some(idx) = module.types.iter().position(|t| &t.name == &processed_properties[0].value) {
                            let bidi_type = module.types.remove(idx);
                            processed_properties = bidi_type.properties;
                        }
                    }

                    // Create the parameter struct name from the param_type
                    let param_struct_name = if let Some(dot_pos) = param_type.find('.') {
                        // Remove module prefix and use just the type name
                        param_type[dot_pos + 1..].to_string()
                    } else {
                        param_type.clone()
                    };

                    // Create CommandParams and add to command_def
                    let command_param = crate::command_parser::CommandParams {
                        name: param_struct_name.clone(),
                        properties: processed_properties,
                        attributes: vec![
                            "#[derive(Debug, Clone, Serialize, Deserialize)]".to_string(),
                        ],
                    };

                    // Check if this param already exists to avoid duplicates
                    if !command_def.command_params.iter().any(|p| p.name == param_struct_name) {
                        command_def.command_params.push(command_param);
                    }
                    return Ok(param_struct_name);
                }
            }
        }

        return Ok(format!("{}", param_type));
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
/// * `type_name` - Optional type name for context in union generation
///
/// # Returns
/// A tuple of (properties, meta_comment) where meta_comment indicates if a type was generated
pub fn process_cddl_to_struct(cddl_content: &str, cddl_strings: Vec<&str>, module: &mut Module, type_name: Option<&str>) -> Result<(Vec<Property>, Option<String>), Box<dyn std::error::Error>> {
    let mut properties = Vec::new();
    // Check if content is empty or only whitespace - return empty properties for empty structs
    if cddl_content.trim().is_empty() {
        return Ok((properties, None));
    }

    // Preprocess multiline union types - collapse them to single line
    let preprocessed_content = preprocess_multiline_unions(cddl_content);

    // Check if content has no ":" and type_name exists - indicates it's not a struct with properties
    if !preprocessed_content.contains(':') && type_name.is_some() {
        // Concatenate content and pass to parse_cddl_property_line as a fake property
        let fake_line = format!("{}: {}", type_name.unwrap(), preprocessed_content);
        let (property, meta_comment) = parse_cddl_property_line(&fake_line, &cddl_strings, module)?;
        if let Some(property) = property {
            // If parse_cddl_property_line returns a property, it means no custom type was generated
            // If it returns None, it means a custom type was generated and we should return empty
            properties.push(property);
        }
        return Ok((properties, meta_comment));
    }

    // Preprocess content to handle nested inline structs
    let (processed_content, updated_cddl_strings) = preprocess_nested_structs(&preprocessed_content, type_name, cddl_strings, module)?;
    let updated_cddl_refs: Vec<&str> = updated_cddl_strings.iter().map(|s| s.as_str()).collect();

    // Parse each line in the processed CDDL body
    for line in processed_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") || line.starts_with(";") {
            continue; // Skip empty lines and comments
        }

        // Parse line like: "? userContext: browser.UserContext,"
        let (property, meta_comment) = parse_cddl_property_line(line, &updated_cddl_refs, module)?;
        if let Some(property) = property {
            properties.push(property);
        }
    }

    Ok((properties, None))
}

/// Preprocesses multiline union types to collapse them into single lines
///
/// # Arguments
/// * `cddl_content` - The CDDL content to preprocess
///
/// # Returns
/// The content with multiline unions collapsed to single lines
fn preprocess_multiline_unions(cddl_content: &str) -> String {
    let lines: Vec<&str> = cddl_content.lines().collect();
    let mut processed_lines = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Check if line starts with '(' - beginning of a potential multiline union
        if line == "(" {
            let mut union_parts = Vec::new();
            let mut paren_count = 1;
            i += 1; // Move past the opening parenthesis

            // Collect all lines until we find the closing parenthesis
            while i < lines.len() && paren_count > 0 {
                let current_line = lines[i].trim();

                // Count parentheses
                for ch in current_line.chars() {
                    match ch {
                        '(' => paren_count += 1,
                        ')' => paren_count -= 1,
                        _ => {}
                    }
                }

                if paren_count > 0 {
                    // Remove trailing "//" or "//" and clean up the line
                    let cleaned = current_line.trim_end_matches("//").trim_end_matches("/").trim();
                    if !cleaned.is_empty() {
                        union_parts.push(cleaned.to_string());
                    }
                }

                i += 1;
            }

            // Join all parts with " // " and wrap in parentheses
            if !union_parts.is_empty() {
                let collapsed_union = format!("({})", union_parts.join(" // "));
                // Try to flatten inline struct unions
                let flattened = flatten_inline_struct_union(&collapsed_union);
                processed_lines.push(flattened);
            }
        } else {
            // Regular line - just add it
            processed_lines.push(line.to_string());
            i += 1;
        }
    }

    processed_lines.join("\n")
}

/// Flattens inline struct unions into a single property
/// Example: ((coordinates: Type1 / null) // (error: Type2))
/// Becomes: coordinates_error: Type1 // null // Type2
///
/// # Arguments
/// * `union_content` - The union content wrapped in parentheses
///
/// # Returns
/// The flattened property or original content if not applicable
fn flatten_inline_struct_union(union_content: &str) -> String {
    let trimmed = union_content.trim();

    // Check if it's wrapped in parentheses
    if !trimmed.starts_with('(') || !trimmed.ends_with(')') {
        return union_content.to_string();
    }

    // Remove outer parentheses
    let inner = &trimmed[1..trimmed.len()-1];

    // Split by " // " to get union parts
    let parts: Vec<&str> = inner.split(" // ").collect();

    // Check if all parts are inline structs: (property: type)
    let inline_struct_pattern = match Regex::new(r"^\s*\(\s*(\w+)\s*:\s*(.+?)\s*\)\s*$") {
        Ok(pattern) => pattern,
        Err(_) => return union_content.to_string(),
    };

    let mut property_names = Vec::new();
    let mut all_types = Vec::new();

    for part in &parts {
        if let Some(captures) = inline_struct_pattern.captures(part.trim()) {
            let prop_name = captures[1].trim().to_string();
            let prop_type = captures[2].trim().to_string();

            property_names.push(prop_name);

            // If the type itself contains unions, split them
            if prop_type.contains(" / ") {
                let sub_types: Vec<&str> = prop_type.split(" / ").map(|t| t.trim()).collect();
                all_types.extend(sub_types.iter().map(|t| t.to_string()));
            } else {
                all_types.push(prop_type);
            }
        } else {
            // Not all parts are inline structs, return original
            return union_content.to_string();
        }
    }

    // Create combined property name
    let combined_name = property_names.join("_");

    // Create flattened union type
    let flattened_type = all_types.join(" // ");

    // Return as a single property line
    format!("{}: {}", combined_name, flattened_type)
}

/// Preprocesses CDDL content to extract nested inline structs and create separate type definitions
///
/// # Arguments
/// * `cddl_content` - The CDDL content to preprocess
/// * `parent_type_name` - The name of the parent type (e.g., "NewResult")
/// * `cddl_strings` - All CDDL content for type lookups
/// * `module` - The current module for generating types
///
/// # Returns
/// A tuple of (processed_content, updated_cddl_strings) where nested structs are replaced with type references
fn preprocess_nested_structs(cddl_content: &str, parent_type_name: Option<&str>, mut cddl_strings: Vec<&str>, module: &mut Module) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
    let mut processed_lines = Vec::new();
    let mut owned_cddl_strings: Vec<String> = cddl_strings.iter().map(|s| s.to_string()).collect();
    let lines: Vec<&str> = cddl_content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Check if line contains inline struct: "property: {"
        if line.contains(": {") && !line.trim_end().ends_with('}') {
            // Extract property name and handle nested struct
            if let Some(colon_pos) = line.find(": {") {
                let property_part = &line[..colon_pos].trim();
                let property_name = property_part.trim_start_matches('?').trim();

                // Create new type name: module.ParentTypePropertyName (e.g., session.NewResultCapabilities)
                let module_name = &module.name.to_lowercase();
                let parent_name = parent_type_name.unwrap_or("Unknown");
                let new_type_name = format!("{}.{}{}", module_name, parent_name, capitalize_first(property_name));

                // Collect nested content
                let mut nested_lines = Vec::new();
                let mut brace_count = 1; // We already saw the opening brace
                i += 1; // Move past the current line

                while i < lines.len() && brace_count > 0 {
                    let nested_line = lines[i];
                    for ch in nested_line.chars() {
                        match ch {
                            '{' => brace_count += 1,
                            '}' => brace_count -= 1,
                            _ => {}
                        }
                    }

                    if brace_count > 0 {
                        nested_lines.push(nested_line.trim());
                    }
                    i += 1;
                }

                // Create the new type definition
                let nested_content = nested_lines.join("\n");
                let new_type_def = format!("{} = {{\n{}\n}}", new_type_name, nested_content);
                owned_cddl_strings.push(new_type_def);

                // Replace the inline struct with type reference
                let replacement_line = format!("{}: {},", property_part, new_type_name);
                processed_lines.push(replacement_line);

                continue;
            }
        }

        processed_lines.push(line.to_string());
        i += 1;
    }

    // Convert owned strings back to string refs for return
    let updated_cddl_strings: Vec<String> = owned_cddl_strings;
    Ok((processed_lines.join("\n"), updated_cddl_strings))
}

/// Capitalizes the first letter of a string
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

/// Parses a single CDDL property line and returns a Property struct
///
/// # Arguments
/// * `line` - A single line of CDDL content (e.g., "? userContext: browser.UserContext,")
/// * `cddl_strings` - All CDDL content for type lookups
/// * `current_module` - Current module being processed
///
/// # Returns
/// A tuple of (Optional Property, generated_type_marker) where generated_type_marker is "generated_type" if a type was created
fn parse_cddl_property_line(line: &str, cddl_strings: &[&str], current_module: &mut Module) -> Result<(Option<Property>, Option<String>), Box<dyn std::error::Error>> {
    // Convert multiline input to single line by replacing newlines with spaces and cleaning up extra spaces
    let single_line = line.lines().collect::<Vec<&str>>().join(" ").trim().to_string();

    // Pattern for: [?] propertyName: type[,]
    let property_pattern = Regex::new(r"^\s*(\??\s*)(\w+):\s*(.+?)(?:,\s*)?$")?;

    // TODO: Move enum to the type or property holder, for context
    // browser.SetClientWindowStateParameters = {
    //     clientWindow: browser.ClientWindow,
    //     (browser.ClientWindowNamedState // browser.ClientWindowRectState)
    // }
    // the second property should be an untagged serde
    if let Some(captures) = property_pattern.captures(&single_line) {
        let optional_marker = captures[1].trim();
        let property_name = captures[2].trim().to_string();
        let property_type = captures[3].trim().trim_end_matches(',').to_string();
        
        let is_optional_marker = optional_marker.contains('?');
        let (rust_type, is_primitive, validation_info, meta_comment) = convert_cddl_type_to_rust(&property_type, cddl_strings, current_module, Some(property_name.as_str()));

        // If the property has a default value, it shouldn't be  optional
        let is_optional = is_optional_marker &&
            validation_info.as_ref().map_or(true, |v| v.default_value.is_none());

        let mut attributes = vec![format!(r#"#[serde(rename = "{}")]"#, property_name)];

        return Ok((Some(Property {
            is_enum: false,
            is_primitive,
            is_optional,
            name: property_name,
            value: rust_type,
            attributes,
            validation_info,
        }), meta_comment));
    }

    // Check for enum pattern: just a simple word
    let enum_pattern = Regex::new(r"^\s*(\??\s*)(.+?)(?:,\s*)?$")?;
    if let Some(captures) = enum_pattern.captures(&single_line) {
        let optional_marker = captures[1].trim();
        let enum_type = captures[2].trim().to_string();
        let (rust_type, is_primitive, validation_info, meta_comment) = convert_cddl_type_to_rust(&enum_type, cddl_strings, current_module, None);

        return Ok((Some(Property {
            is_enum: true,
            is_primitive,
            is_optional: optional_marker.contains('?'),
            name: rust_type.clone(),
            value: rust_type,
            attributes: Vec::new(),
            validation_info,
        }), meta_comment));
    }

    Ok((None, None))
}

/// Finds and extracts the content between braces for a type definition
///
/// # Arguments
/// * `type_name` - The type name to search for (e.g., "session.CapabilityRequest", "ErrorCode")
/// * `cddl_strings` - All CDDL content to search through
/// * `current_module` - The current module being processed for context
///
/// # Returns
/// Option containing tuple of (content, def_type) if found
pub fn find_and_extract_type_content(type_name: &str, cddl_strings: &[&str], current_module: &Module) -> Option<(String, String)> {
    // Try direct match first (e.g., "session.CapabilityRequest = {")
    let direct_pattern = format!(r"^{}\s*=\s*\{{", regex::escape(type_name));
    if let Ok(regex) = Regex::new(&direct_pattern) {
        for cddl_content in cddl_strings {
            let lines: Vec<&str> = cddl_content.lines().collect();

            for (line_num, line) in lines.iter().enumerate() {
                if regex.is_match(line.trim()) {
                    // Found the type definition, extract content between braces
                    let mut brace_count = 0;
                    let mut found_start = false;
                    let mut content_lines = Vec::new();

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
                                        // Found the end, return the extracted content
                                        return Some((content_lines.join("\n").trim().to_string(), "struct".to_string()));
                                    }
                                }
                                _ => {}
                            }
                        }

                        // Add the line if we're inside the braces (skip the first line)
                        if found_start && brace_count > 0 && i > line_num {
                            content_lines.push(current_line.trim());
                        }
                    }
                }
            }
        }
    }

    // Try parentheses pattern (e.g., "browsingContext.BaseNavigationInfo = (")
    let paren_pattern = format!(r"^{}\s*=\s*\(", regex::escape(type_name));
    if let Ok(regex) = Regex::new(&paren_pattern) {
        for cddl_content in cddl_strings {
            let lines: Vec<&str> = cddl_content.lines().collect();

            for (line_num, line) in lines.iter().enumerate() {
                if regex.is_match(line.trim()) {
                    // Found the type definition, extract content between parentheses
                    let mut paren_count = 0;
                    let mut found_start = false;
                    let mut content_lines = Vec::new();

                    for i in line_num..lines.len() {
                        let current_line = lines[i];

                        for ch in current_line.chars() {
                            match ch {
                                '(' => {
                                    paren_count += 1;
                                    found_start = true;
                                }
                                ')' => {
                                    paren_count -= 1;
                                    if paren_count == 0 && found_start {
                                        // Found the end, return the extracted content
                                        return Some((content_lines.join("\n").trim().to_string(), "union".to_string()));
                                    }
                                }
                                _ => {}
                            }
                        }

                        // Add the line if we're inside the parentheses (skip the first line)
                        if found_start && paren_count > 0 && i > line_num {
                            content_lines.push(current_line.trim());
                        }
                    }
                }
            }
        }
    }

    // Try object assignment pattern with inline content (e.g., "emulation.ScreenOrientation = { natural: ..., type: ... }")
    let object_inline_pattern = format!(r"^{}\s*=\s*\{{(.+)\}}", regex::escape(type_name));
    if let Ok(regex) = Regex::new(&object_inline_pattern) {
        for cddl_content in cddl_strings {
            for line in cddl_content.lines() {
                if let Some(captures) = regex.captures(line.trim()) {
                    // Extract the content between braces from inline definition
                    return Some((captures[1].trim().to_string(), "struct".to_string()));
                }
            }
        }
    }

    // Try string literal pattern (e.g., 'emulation.ForcedColorsModeTheme = "light" / "dark"')
    let literal_pattern = format!(r#"^{}\s*=\s*(.+?)\;?$"#, regex::escape(type_name));
    if let Ok(regex) = Regex::new(&literal_pattern) {
        for cddl_content in cddl_strings {
            let lines: Vec<&str> = cddl_content.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                if let Some(captures) = regex.captures(line.trim()) {
                    let mut content = captures[1].trim().to_string();
                    let mut idx = i;

                    while content.ends_with('/') && idx + 1 < lines.len() {
                        idx += 1;
                        content.push(' ');
                        content.push_str(lines[idx].trim());
                    }

                    return Some((content, "alias".to_string()));
                }
            }
        }
    }

    // If no module prefix, try with current module prefix
    if !type_name.contains('.') {
        let prefixed_type = format!("{}.{}", current_module.name.to_lowercase(), type_name);
        return find_and_extract_type_content(&prefixed_type, cddl_strings, current_module);
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
///
/// # Returns
/// The clean type name if generated, None if not in same module or already exists
fn generate_type_if_same_module(type_name: &str, content: &str, def_type: &str, cddl_strings: &[&str], current_module: &mut Module) -> Option<String> {
    // Check if same module
    if !is_same_module_type(type_name, current_module) {
        return None;
    }
    // Extract clean type name without module prefix
    let clean_name = if let Some(dot_pos) = type_name.find('.') {
        &type_name[dot_pos + 1..]
    } else {
        type_name
    };

    // Skip generating Extensible type - it's defined in root module
    if clean_name == "Extensible" {
        return Some(clean_name.to_string());
    }

    // Check if type already exists in module
    if current_module.types.iter().any(|t| t.name == clean_name) {
        return Some(clean_name.to_string()); // Type already exists, return the name
    }

    // Check if type is currently being processed (cycle detection)
    if current_module.pending_types.contains(clean_name) {
        return Some(clean_name.to_string());
    }

    // Add to pending types before processing
    current_module.pending_types.insert(clean_name.to_string());

    // Use existing process_cddl_to_struct for recursive parsing
    let result = if let Ok((mut properties, meta_comment)) = process_cddl_to_struct(content, cddl_strings.to_vec(), current_module, Some(clean_name)) {
        // Check if properties length is 1 and meta_comment indicates type was generated
        if properties.len() == 1 && meta_comment == Some("generated_property".to_string()) {
            // Find the BidiType using the property value, extract its properties, and delete it
            let property_value = &properties[0].value;
            if let Some(idx) = current_module.types.iter().position(|t| &t.name == property_value) {
                let bidi_type = current_module.types.remove(idx);
                properties = bidi_type.properties;
            }
        }

        let properties_len = properties.len();

        // Determine if this is truly an alias or an enum
        // Single quoted literal strings like `network.CollectorType = "blob"` should be treated as enums,
        // not as type aliases. This allows proper enum generation with unit variants.
        let content_is_quoted = content.trim().starts_with('"') && content.trim().ends_with('"');
        let is_enum = content_is_quoted && properties_len == 1;
        let is_alias = def_type == "alias"
            && properties_len < 2
            && !content_is_quoted;

        // Create BidiType with the properties and store in module
        current_module.types.push(crate::module::BidiType {
            name: clean_name.to_string(),
            properties,
            is_enum,
            is_alias,
        });

        Some(clean_name.to_string())
    } else {
        None
    };

    // Remove from pending types after processing
    current_module.pending_types.remove(clean_name);

    result
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
/// A tuple of (rust_type_name, generated_type_name) where generated_type_name is Some if a new type was created
fn parse_custom_type(type_name: &str, cddl_strings: &[&str], current_module: &mut Module, struct_name: Option<&str>, property_name: Option<&str>) -> (String, Option<String>) {
    // Check for validation patterns like "(float .ge 0.0) .default 1.0"
    if let Some((base_type, _validation_info)) = parse_validation_pattern(type_name) {
        // For now, just return the base type - validation info will be handled in convert_cddl_type_to_rust
        let (rust_type, _, _) = convert_basic_cddl_type(&base_type, cddl_strings, current_module, property_name);
        return (rust_type, None);
    }
    
    // Handle union types with / or // separator
    if type_name.contains(" // ") || type_name.contains(" / ") {
        // Remove surrounding parentheses if present
        let cleaned_type_name = if type_name.starts_with('(') && type_name.ends_with(')') {
            &type_name[1..type_name.len()-1]
        } else {
            type_name
        };

        // Break into parts - handle both // and / separators
        let mut parts: Vec<String> = if cleaned_type_name.contains(" // ") {
            cleaned_type_name.split(" // ").map(|s| s.trim().to_string()).collect()
        } else {
            cleaned_type_name.split(" / ").map(|s| s.trim().to_string()).collect()
        };
        
        // Check if null is present
        let has_null = parts.iter().any(|part| part == "null");
        
        // Remove all nulls
        parts.retain(|part| part != "null");

        // Process remaining types through convert_basic_cddl_type
        let processed_types: Vec<(String, Option<String>, Vec<String>)> = parts
            .into_iter()
            .map(|part| {
                // Strip braces if present (e.g., "{ script.DateLocalValue }" -> "script.DateLocalValue")
                let cleaned_part = if part.trim().starts_with('{') && part.trim().ends_with('}') {
                    let inner = &part.trim()[1..part.trim().len()-1];
                    inner.trim().to_string()
                } else {
                    part.trim().to_string()
                };

                // Check if part is a literal string
                if cleaned_part.starts_with('"') && cleaned_part.ends_with('"') {
                    let literal_value = &cleaned_part[1..cleaned_part.len()-1]; // Remove quotes
                    let variant_name = to_pascal_case(literal_value);
                    let attributes = vec![format!(r#"#[serde(rename = "{}")]"#, literal_value)];

                    (variant_name, None, attributes)
                } else {
                    let (rust_type, _, _) = convert_basic_cddl_type(&cleaned_part, cddl_strings, current_module, property_name);
                    (rust_type.clone(), Some(rust_type), Vec::new()) // No attributes for non-literals
                }
            })
            .collect();
        
        // Determine final type
        return match processed_types.len() {
            0 => ("()".to_string(), None), // Only null was present
            1 => {
                let (base_type, _, _) = &processed_types[0];
                let result_type = if has_null {
                    format!("Option<{}>", base_type)
                } else {
                    base_type.clone()
                };
                (result_type, None)
            }
            _ => {


                // Multiple non-null types - create enum using struct and property names
                let enum_name_temp = match (struct_name, property_name) {
                    (Some(s), Some(p)) => format!("{}{}", s, p),
                    (Some(s), None) => format!("{}Union", s),
                    (None, Some(p)) => format!("{}Union", p),
                    (None, None) => {
                        // Combine all type names when no struct or property name is present
                        let type_names: Vec<String> = processed_types
                            .iter()
                            .map(|(variant_name, _, _)| variant_name.clone())
                            .collect();
                        format!("{}Union", type_names.join(""))
                    },
                };

                let enum_name = to_pascal_case(enum_name_temp.as_str());

                // Create enum variants from processed types
                let properties: Vec<Property> = processed_types
                    .iter()
                    .map(|(variant_name, variant_value, attributes)| Property {
                        is_enum: true,
                        is_primitive: false,
                        is_optional: false,
                        name: to_pascal_case(variant_name.as_str()),
                        value: if let Some(value) = variant_value {
                            value.clone()
                        } else {
                            "UNIT_VARIANT".to_string() // Special marker for unit variants
                        },
                        attributes: attributes.clone(),
                        validation_info: None,
                    })
                    .collect();

                // Create and add BidiType to module
                current_module.types.push(crate::module::BidiType {
                    name: enum_name.clone(),
                    properties,
                    is_enum: true,
                    is_alias: false,
                });

                let result_type = if has_null {
                    format!("Option<{}>", enum_name)
                } else {
                    enum_name.clone()
                };
                (result_type, Some(String::from("generated_property")))
            }
        };
    }
    
    // Handle single literal strings (quoted values)
    if type_name.starts_with('"') && type_name.ends_with('"') {
        let literal_value = &type_name[1..type_name.len()-1]; // Remove quotes
        let enum_name = format!("{}Enum", to_pascal_case(literal_value));

        // Check if type already exists before adding
        if !current_module.types.iter().any(|t| t.name == enum_name) {
            let properties = vec![Property {
                is_enum: true,
                is_primitive: false,
                is_optional: false,
                name: to_pascal_case(literal_value),
                value: "UNIT_VARIANT".to_string(),
                attributes: vec![format!(r#"#[serde(rename = "{}")]"#, literal_value)],
                validation_info: None,
            }];

            current_module.types.push(crate::module::BidiType {
                name: enum_name.clone(),
                properties,
                is_enum: true,
                is_alias: false,
            });
        }
        return (enum_name.clone(), Some(String::from("generated_property")));
    }

    // // Check if this type has already been processed to prevent regeneration
    // if current_module.types.iter().any(|t| t.name == type_name) {
    //     let result_type = if let Some(dot_pos) = type_name.find('.') {
    //         let type_name_clean = &type_name[dot_pos + 1..];
    //         type_name_clean.to_string()
    //     } else {
    //         type_name.to_string()
    //     };
    //     return (result_type, None);
    // }

    // Check if this type belongs to the current module
    if current_module.name != "root" && is_same_module_type(type_name, current_module) {
        // First find the actual type definition content
        if let Some((type_content, def_type)) = find_and_extract_type_content(type_name, cddl_strings, current_module) {

            // Generate type if needed
            if let Some(generated_name) = generate_type_if_same_module(type_name, &type_content, &def_type, cddl_strings, current_module) {
                return (generated_name, None);
            }
        }

        // Return only the type name (without module prefix)
        let result_type = if let Some(dot_pos) = type_name.find('.') {
            type_name[dot_pos + 1..].to_string()
        } else {
            type_name.to_string()
        };
        (result_type, None)
    } else {
        // External module type - try to find and store for later processing
        if current_module.name != "root" {
            if let Some((type_content, def_type)) = find_and_extract_type_content(type_name, cddl_strings, current_module) {
                // Extract the target module name
                let target_module_name = if let Some(dot_pos) = type_name.find('.') {
                    type_name[..dot_pos].to_string()
                } else {
                    "unknown".to_string()
                };

                // Check if this type is already deferred to prevent duplicates
                let deferred_types = DEFERRED_TYPES.get_or_init(|| Mutex::new(Vec::new()));
                let already_deferred = deferred_types.lock().unwrap().iter().any(|dt| dt.type_name == type_name);

                if !already_deferred {
                    // Store the deferred type for later processing
                    let deferred_type = DeferredType {
                        type_name: type_name.to_string(),
                        content: type_content,
                        def_type,
                        target_module_name,
                    };
                    add_deferred_type(deferred_type);
                }
            }
        }

        // Return module::type format for external modules
        let result_type = if let Some(dot_pos) = type_name.find('.') {
            let module_name = &type_name[..dot_pos];
            let type_name_clean = &type_name[dot_pos + 1..];
            format!("{}::{}", module_name, type_name_clean)
        } else {
            type_name.to_string()
        };
        (result_type, None)
    }
}


/// Converts basic CDDL primitive types to Rust types
///
/// # Arguments
/// * `cddl_type` - The CDDL type string
/// * `cddl_strings` - All CDDL content for type lookups
/// * `current_module` - Current module being processed
/// * `property_name` - Optional property name for context in union generation
///
/// # Returns
/// A tuple of (rust_type, is_primitive, meta_comment)
fn convert_basic_cddl_type(cddl_type: &str, cddl_strings: &[&str], current_module: &mut Module, property_name: Option<&str>) -> (String, bool, Option<String>) {
    match cddl_type {
        "text" => ("String".to_string(), true, None),
        "bool" => ("bool".to_string(), true, None),
        "false" => ("bool".to_string(), false, None),
        "js-uint" => ("u64".to_string(), true, None),
        "js-int" => ("i64".to_string(), true, None),
        "float" => ("f64".to_string(), true, None),
        "null" => ("Option<()>".to_string(), false, None), // null is typically represented as Option
        _ => {
                // Parse the custom type (this function will handle generation if same module)
                let (parsed_type, meta_comment) = parse_custom_type(cddl_type, cddl_strings, current_module, None, property_name);
                (parsed_type, false, meta_comment)
        }
    }
}

/// Replaces numeric range notation with Rust type and extracts validation info
/// Example: "(0.0..1.5707963267948966) .default 0.0" -> "(f64) .default 0.0" + ValidationInfo
///
/// # Arguments
/// * `input` - The string potentially containing a range
///
/// # Returns
/// Option containing (modified_string, ValidationInfo) if a range was found and replaced
fn parse_numeric_range(input: &str) -> Option<(String, ValidationInfo)> {
    // Simple check: does it contain .. or ...?
    if !input.contains("..") {
        return None;
    }

    // Find the range pattern: number..number or number...number
    let range_pattern = Regex::new(r"(-?\d+(?:\.\d+)?)\.{2,3}(-?\d+(?:\.\d+)?)").ok()?;

    if let Some(captures) = range_pattern.captures(input) {
        let min_val = captures[1].to_string();
        let max_val = captures[2].to_string();
        let full_match = captures.get(0)?.as_str();

        // Determine if it's a float or integer range
        let is_float = min_val.contains('.') || max_val.contains('.');
        let rust_type = if is_float { "f64" } else { "i64" };

        // Create validation info with range constraints
        let mut validation_info = ValidationInfo::new();
        validation_info.constraints.push(ConstraintInfo {
            constraint_type: "ge".to_string(),
            value: min_val,
        });
        validation_info.constraints.push(ConstraintInfo {
            constraint_type: "le".to_string(),
            value: max_val,
        });

        // Replace the range with the Rust type
        let modified_string = input.replace(full_match, rust_type);

        return Some((modified_string, validation_info));
    }

    None
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

/// Parses type expressions with modifiers like "bool .default false" or "(float .ge 0.0) .default 1.0"
/// Also handles range types with defaults like "(0.1..2.0) .default 1.0"
///
/// # Arguments
/// * `type_name` - The full type expression to parse
///
/// # Returns
/// Option containing (base_type, ValidationInfo) if modifiers are found
fn parse_validation_pattern(type_name: &str) -> Option<(String, ValidationInfo)> {
    let input = type_name.trim();
    let mut validation_info = ValidationInfo::new();
    let mut has_modifiers = false;

    // Look for .default and extract it
    if let Some(default_pos) = input.find(".default ") {
        let after_default = &input[default_pos + 9..]; // Skip ".default "
        let default_value = after_default.split_whitespace().next().unwrap_or("").to_string();
        if !default_value.is_empty() {
            validation_info.default_value = Some(default_value);
            has_modifiers = true;
        }
    }

    // Look for constraints (.ge, .le, .gt, .lt)
    for constraint_type in &[".ge ", ".le ", ".gt ", ".lt "] {
        if let Some(constraint_pos) = input.find(constraint_type) {
            let after_constraint = &input[constraint_pos + constraint_type.len()..];
            let constraint_value = after_constraint.split_whitespace().next().unwrap_or("").trim_end_matches(')').to_string();
            if !constraint_value.is_empty() {
                validation_info.constraints.push(ConstraintInfo {
                    constraint_type: constraint_type.trim().trim_start_matches('.').to_string(),
                    value: constraint_value,
                });
                has_modifiers = true;
            }
        }
    }

    if !has_modifiers {
        return None;
    }

    // Extract base type - first word plus any union parts (/ or //)
    let first_word = input.split_whitespace()
        .next()
        .unwrap_or("")
        .trim_start_matches('(')
        .trim_end_matches(')')
        .to_string();

    // Check if there's a union separator after the first word
    let mut base_type = first_word.clone();

    // Look for / or // after the first word
    if let Some(union_pos) = input.find(" // ").or_else(|| input.find(" / ")) {
        // Find what comes after the separator (before any .default or constraint)
        let after_separator = &input[union_pos..];

        // Extract the next word after / or //
        let separator_and_word = if after_separator.starts_with(" // ") {
            after_separator.split_whitespace().take(2).collect::<Vec<_>>().join(" ")
        } else {
            after_separator.split_whitespace().take(2).collect::<Vec<_>>().join(" ")
        };

        base_type = format!("{} {}", first_word, separator_and_word.trim_end_matches(')').to_string());
    }

    if base_type.is_empty() {
        None
    } else {
        Some((base_type, validation_info))
    }
}


/// Information about validation constraints and defaults
#[derive(Debug, Clone)]
pub struct ValidationInfo {
    pub constraints: Vec<ConstraintInfo>,
    pub default_value: Option<String>,
}

impl ValidationInfo {
    fn new() -> Self {
        Self {
            constraints: Vec::new(),
            default_value: None,
        }
    }
}

/// Information about a single constraint
#[derive(Debug, Clone)]
pub struct ConstraintInfo {
    pub constraint_type: String, // "ge", "le", "gt", "lt"
    pub value: String,
}


/// Converts a CDDL type to a Rust type, handling arrays, tuples, validation patterns, and basic types
///
/// # Arguments
/// * `cddl_type` - The CDDL type string
/// * `cddl_strings` - All CDDL content for type lookups
/// * `current_module` - Current module being processed
/// * `property_name` - Optional property name for context in union generation
///
/// # Returns
/// A tuple of (rust_type, is_primitive, validation_info, meta_comment_on_type)
fn convert_cddl_type_to_rust(cddl_type: &str, cddl_strings: &[&str], current_module: &mut Module, property_name: Option<&str>) -> (String, bool, Option<ValidationInfo>, Option<String>) {
    let mut trimmed = cddl_type.trim().trim_end_matches(';').trim().to_string();
    let mut range_validation: Option<ValidationInfo> = None;

    // First check for numeric range types and replace them
    if let Some((replaced_string, validation_info)) = parse_numeric_range(&trimmed) {
        trimmed = replaced_string;
        range_validation = Some(validation_info);
    }

    // Check for validation patterns like "(float .ge 0.0) .default 1.0"
    if let Some((base_type, mut validation_info)) = parse_validation_pattern(&trimmed) {
        // Merge range validation if present
        if let Some(range_val) = range_validation {
            for constraint in range_val.constraints {
                validation_info.constraints.push(constraint);
            }
        }

        let (rust_type, is_primitive, _, meta_comment) = convert_cddl_type_to_rust(&base_type, cddl_strings, current_module, property_name);
        return (rust_type, is_primitive, Some(validation_info), meta_comment);
    }

    // Return range validation if we only had a range (no other modifiers)
    if let Some(validation_info) = range_validation {
        // Extract just the type from trimmed (strip parentheses if present)
        let rust_type = trimmed.trim().trim_start_matches('(').trim_end_matches(')').trim().to_string();
        return (rust_type, true, Some(validation_info), None);
    }

    // Check if it's a bracketed type [...]
    if let Some(inner_content) = extract_bracket_content(&trimmed) {
        // Check for arrays: [+type] or [*type]
        if let Ok(array_pattern) = Regex::new(r"^\s*[\+\*]\s*(.+)$") {
            if let Some(captures) = array_pattern.captures(&inner_content) {
                let inner_type = captures[1].trim();

                // Recursively convert the inner type
                let (rust_inner_type, _, _, meta_comment) = convert_cddl_type_to_rust(inner_type, cddl_strings, current_module, property_name);

                // Arrays are not primitive
                return (format!("Vec<{}>", rust_inner_type), false, None, meta_comment);
            }
        }

        // Check for tuples by splitting on top-level commas
        let parts = split_respecting_brackets(&inner_content);
        if parts.len() > 1 {
            // It's a tuple - convert each part
            let tuple_types: Vec<String> = parts
                .into_iter()
                .map(|part| {
                    let (rust_type, _, _, _) = convert_cddl_type_to_rust(&part, cddl_strings, current_module, property_name);
                    rust_type
                })
                .collect();

            // Tuples are not primitive
            return (format!("({})", tuple_types.join(", ")), false, None, None);
        } else if parts.len() == 1 {
            // Single item in brackets - could be a single-element array or just grouped
            // For now, treat as the inner type
            let (rust_type, is_primitive, validation_info, meta_comment) = convert_cddl_type_to_rust(&parts[0], cddl_strings, current_module, property_name);
            return (rust_type, is_primitive, validation_info, meta_comment);
        }
    }

    // Fall back to basic type conversion
    let (rust_type, is_primitive, meta_comment) = convert_basic_cddl_type(&trimmed, cddl_strings, current_module, property_name);
    (rust_type, is_primitive, None, meta_comment)
}

/// Converts a string to PascalCase
///
/// # Arguments
/// * `input` - The input string to convert
///
/// # Returns
/// The PascalCase version of the input string
fn to_pascal_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true; // Start with capitalizing the first character
    let mut is_first_char = true;

    for ch in input.chars() {
        // Preserve leading '-' sign
        if ch == '-' && is_first_char {
            result.push(ch);
            is_first_char = false;
            capitalize_next = true;
        } else if ch == ' ' || ch == '-' || ch == '_' {
            is_first_char = false;
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_uppercase().next().unwrap_or(ch));
            capitalize_next = false;
            is_first_char = false;
        } else {
            result.push(ch);
            is_first_char = false;
        }
    }

    result
}

