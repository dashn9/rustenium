use std::fs;
use std::path::Path;
use crate::module::Module;

/// Checks if any types exist in module.types
fn has_types_in_module(module: &crate::module::Module) -> bool {
    !module.types.is_empty()
}

/// Checks if any commands/results exist in module
fn has_commands_in_module(module: &crate::module::Module) -> bool {
    module.command_definition.is_some() || module.result_definition.is_some()
}

/// Checks if any events exist in module
fn has_events_in_module(module: &crate::module::Module) -> bool {
    module.event_definition.is_some()
}

/// Extracts module and type from external reference like "browsingContext::BrowsingContext"
/// Returns (module_name, type_name)
fn extract_module_and_type(type_ref: &str) -> Option<(String, String)> {
    if let Some(pos) = type_ref.find("::") {
        let module = type_ref[..pos].to_string();
        let type_name = type_ref[pos + 2..].to_string();
        Some((module, type_name))
    } else {
        None
    }
}

/// Extracts the inner type from generic wrappers like Vec<T>, Option<T>, Box<T>
fn extract_inner_type(type_ref: &str) -> String {
    // Handle Option<T>
    if let Some(inner) = type_ref.strip_prefix("Option<").and_then(|s| s.strip_suffix('>')) {
        return extract_inner_type(inner);
    }

    // Handle Vec<T>
    if let Some(inner) = type_ref.strip_prefix("Vec<").and_then(|s| s.strip_suffix('>')) {
        return extract_inner_type(inner);
    }

    // Handle Box<T>
    if let Some(inner) = type_ref.strip_prefix("Box<").and_then(|s| s.strip_suffix('>')) {
        return extract_inner_type(inner);
    }

    type_ref.to_string()
}

/// Checks if Extensible type is used in properties
fn uses_extensible(properties: &[crate::parser::Property]) -> bool {
    properties.iter().any(|p| {
        let inner_type = extract_inner_type(&p.value);
        inner_type == "Extensible"
    })
}

/// Checks if EmptyParams type is used in properties
fn uses_empty_params(properties: &[crate::parser::Property]) -> bool {
    properties.iter().any(|p| {
        let inner_type = extract_inner_type(&p.value);
        inner_type == "EmptyParams"
    })
}

/// Checks if EmptyResult type is used in properties
fn uses_empty_result(properties: &[crate::parser::Property]) -> bool {
    properties.iter().any(|p| {
        let inner_type = extract_inner_type(&p.value);
        inner_type == "EmptyResult"
    })
}

/// Checks if validation constraints are used in properties
fn uses_validation(properties: &[crate::parser::Property]) -> bool {
    properties.iter().any(|p| {
        p.validation_info.as_ref().map_or(false, |v| !v.constraints.is_empty())
    })
}

/// Collects external type imports from properties
/// Returns Vec of (module_name, type_name) tuples
fn collect_external_types(properties: &[crate::parser::Property]) -> Vec<(String, String)> {
    let mut types = std::collections::HashSet::new();

    for property in properties {
        // First extract the inner type from any generic wrappers
        let inner_type = extract_inner_type(&property.value);

        // Then check if it has an external module reference
        if let Some((module, type_name)) = extract_module_and_type(&inner_type) {
            types.insert((module, type_name));
        }
    }

    let mut result: Vec<(String, String)> = types.into_iter().collect();
    result.sort();
    result
}

/// Generates import statements for external types
fn generate_external_imports(types: &[(String, String)]) -> String {
    let mut output = String::new();

    for (module, type_name) in types {
        let module_snake = to_snake_case(module);
        output.push_str(&format!("use crate::{}::types::{};\n", module_snake, type_name));
    }

    output
}

/// Sanitizes property names that start with special characters or numbers
///
/// # Arguments
/// * `input` - The property name to sanitize
///
/// # Returns
/// The sanitized property name
fn sanitize_property_name(input: &str) -> String {
    if input.is_empty() {
        return input.to_string();
    }

    let first_char = input.chars().next().unwrap();

    // Handle properties starting with '-'
    if first_char == '-' {
        return format!("Negative{}", &input[1..]);
    }

    // Handle properties starting with numbers
    if first_char.is_numeric() {
        let word_equiv = match first_char {
            '0' => "Zero",
            '1' => "One",
            '2' => "Two",
            '3' => "Three",
            '4' => "Four",
            '5' => "Five",
            '6' => "Six",
            '7' => "Seven",
            '8' => "Eight",
            '9' => "Nine",
            _ => "Number",
        };
        return format!("{}{}", word_equiv, &input[1..]);
    }

    input.to_string()
}

/// Converts PascalCase to snake_case
///
/// # Arguments
/// * `input` - The PascalCase string to convert
///
/// # Returns
/// The snake_case version of the string
fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = input.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        if ch.is_uppercase() && i > 0 {
            // Add underscore before uppercase letters, except:
            // - When previous char is uppercase and current is the last char in an acronym
            //   (e.g., in "namespaceURI", don't add underscore before 'U' and 'R')
            let prev_is_upper = chars.get(i - 1).map_or(false, |c| c.is_uppercase());

            // Only add underscore if previous char was not uppercase
            if !prev_is_upper {
                result.push('_');
            }
        }
        result.push(ch.to_lowercase().next().unwrap());
    }

    result
}

/// Transforms a default value to its proper Rust representation
///
/// # Arguments
/// * `default_value` - The raw default value from CDDL
/// * `rust_type` - The Rust type this default value is for
///
/// # Returns
/// The transformed default value as a Rust expression
fn transform_default_value(default_value: &str, rust_type: &str) -> String {
    // Handle null -> None
    if default_value == "null" {
        return "None".to_string();
    }

    // Handle string literals (quoted values) - convert to enum variant
    // e.g., "portrait" -> ReturnType::Portrait
    if default_value.starts_with('"') && default_value.ends_with('"') {
        let inner_value = &default_value[1..default_value.len()-1];
        // Convert to PascalCase for enum variant
        let variant = inner_value
            .split(|c| c == '-' || c == '_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<String>();

        return format!("{}::{}", rust_type, variant);
    }

    // For other values, return as-is
    default_value.to_string()
}

/// Cleans up module prefixes from type names
///
/// # Arguments
/// * `type_name` - The type name that might have a module prefix (e.g., "browser.UserContext" or "Vec<browsingContext::BrowsingContext>")
/// * `current_module` - The current module name (e.g., "browser")
///
/// # Returns
/// The cleaned type name without module prefix
fn clean_module_prefix(type_name: &str, current_module: &str) -> String {
    // Handle special "number" type from CDDL
    if type_name == "number" {
        return "serde_json::value::Number".to_string();
    }

    // Handle generics like Vec<module::Type>
    if let Some(inner) = type_name.strip_prefix("Vec<").and_then(|s| s.strip_suffix('>')) {
        let cleaned_inner = clean_module_prefix(inner, current_module);
        return format!("Vec<{}>", cleaned_inner);
    }
    if let Some(inner) = type_name.strip_prefix("Option<").and_then(|s| s.strip_suffix('>')) {
        let cleaned_inner = clean_module_prefix(inner, current_module);
        return format!("Option<{}>", cleaned_inner);
    }
    if let Some(inner) = type_name.strip_prefix("Box<").and_then(|s| s.strip_suffix('>')) {
        let cleaned_inner = clean_module_prefix(inner, current_module);
        return format!("Box<{}>", cleaned_inner);
    }

    // Handle external module reference with ::
    if let Some((_module, type_only)) = extract_module_and_type(type_name) {
        return type_only;
    }

    // Handle same-module reference with .
    if let Some(dot_pos) = type_name.find('.') {
        let module_prefix = &type_name[..dot_pos];
        if module_prefix.to_lowercase() == current_module.to_lowercase() {
            // Same module - remove the prefix
            type_name[dot_pos + 1..].to_string()
        } else {
            // Different module - keep the full name
            type_name.to_string()
        }
    } else {
        // No module prefix - keep as is
        type_name.to_string()
    }
}

pub fn generate_output(modules: &[Module], root_protocol: &crate::module::RootProtocol) -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory
    let output_dir = "../rustenium-bidi-commands/src";
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir)?;
    }

    // Generate root lib.rs
    generate_root_lib(modules, root_protocol, output_dir)?;

    // Generate files for each module
    for module in modules {
        generate_module_files(module, output_dir)?;
    }

    println!("Generated {} modules to {}", modules.len(), output_dir);

    Ok(())
}

fn generate_root_lib(modules: &[Module], root_protocol: &crate::module::RootProtocol, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = String::new();

    // Add imports
    output.push_str("use serde::{Serialize, Deserialize};\n");
    output.push_str("use std::collections::HashMap;\n\n");

    // Declare all modules
    for module in modules {
        let module_snake = to_snake_case(&module.name);
        output.push_str(&format!("pub mod {};\n", module_snake));
    }
    output.push_str("\n");

    // Re-export all definition enums
    for module in modules {
        let module_snake = to_snake_case(&module.name);

        // Explicitly re-export command definition enum
        if let Some(ref cmd_def) = module.command_definition {
            output.push_str(&format!("pub use {}::commands::{};\n",
                module_snake,
                cmd_def.name
            ));
        }

        // Explicitly re-export result definition enum
        if let Some(ref result_def) = module.result_definition {
            output.push_str(&format!("pub use {}::commands::{};\n",
                module_snake,
                result_def.name
            ));
        }

        // Explicitly re-export event definition enum
        if let Some(ref event_def) = module.event_definition {
            output.push_str(&format!("pub use {}::events::{};\n",
                module_snake,
                event_def.name
            ));
        }
    }
    output.push_str("\n");

    // Add deserializer helper functions
    output.push_str("use serde::Deserializer;\n\n");
    output.push_str("fn float_or_int_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>\n");
    output.push_str("where\n");
    output.push_str("    D: Deserializer<'de>,\n");
    output.push_str("{\n");
    output.push_str("    let value = serde_json::Value::deserialize(deserializer)?;\n\n");
    output.push_str("    match value {\n");
    output.push_str("        serde_json::Value::Number(num) => {\n");
    output.push_str("            if let Some(i) = num.as_u64() {\n");
    output.push_str("                Ok(i)\n");
    output.push_str("            } else if let Some(f) = num.as_f64() {\n");
    output.push_str("                Ok(f as u64)\n");
    output.push_str("            } else {\n");
    output.push_str("                Err(serde::de::Error::custom(\"Invalid number\"))\n");
    output.push_str("            }\n");
    output.push_str("        }\n");
    output.push_str("        _ => Err(serde::de::Error::custom(\"Expected a number\")),\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");

    output.push_str("fn option_float_or_int_to_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>\n");
    output.push_str("where\n");
    output.push_str("    D: Deserializer<'de>,\n");
    output.push_str("{\n");
    output.push_str("    let value = serde_json::Value::deserialize(deserializer)?;\n\n");
    output.push_str("    if value.is_null() {\n");
    output.push_str("        return Ok(None);\n");
    output.push_str("    }\n\n");
    output.push_str("    match value {\n");
    output.push_str("        serde_json::Value::Number(num) => {\n");
    output.push_str("            if let Some(i) = num.as_u64() {\n");
    output.push_str("                Ok(Some(i))\n");
    output.push_str("            } else if let Some(f) = num.as_f64() {\n");
    output.push_str("                Ok(Some(f as u64))\n");
    output.push_str("            } else {\n");
    output.push_str("                Err(serde::de::Error::custom(\"Invalid number\"))\n");
    output.push_str("            }\n");
    output.push_str("        }\n");
    output.push_str("        _ => Err(serde::de::Error::custom(\"Expected a number\")),\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");

    output.push_str("fn deserialize_empty_map<'de, D>(deserializer: D) -> Result<Extensible, D::Error>\n");
    output.push_str("where\n");
    output.push_str("    D: serde::Deserializer<'de>,\n");
    output.push_str("{\n");
    output.push_str("    let map = Extensible::deserialize(deserializer)?;\n\n");
    output.push_str("    if map.is_empty() {\n");
    output.push_str("        Ok(map)\n");
    output.push_str("    } else {\n");
    output.push_str("        Err(serde::de::Error::custom(\"expected empty object\"))\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");

    // Generate Extensible type
    output.push_str("pub type Extensible = HashMap<String, serde_json::Value>;\n\n");

    // Generate Command
    output.push_str(&generate_rust_struct("Command", &root_protocol.command.properties, "root"));
    output.push_str("\n\n");

    // Generate CommandData
    output.push_str(&generate_rust_enum("CommandData", &root_protocol.command_data.properties, "root"));
    output.push_str("\n\n");

    // Generate EmptyParams - special case with flattened Extensible
    output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    output.push_str("pub struct EmptyParams {\n");
    output.push_str("    #[serde(flatten, deserialize_with = \"deserialize_empty_map\")]\n");
    output.push_str("    pub extensible: Extensible,\n");
    output.push_str("}\n\n");

    // Generate Message with ErrorResponse before CommandResponse
    let mut message_properties = root_protocol.message.properties.clone();
    // Find and swap ErrorResponse and CommandResponse
    if let (Some(error_idx), Some(cmd_idx)) = (
        message_properties.iter().position(|p| p.value == "ErrorResponse"),
        message_properties.iter().position(|p| p.value == "CommandResponse")
    ) {
        if error_idx > cmd_idx {
            message_properties.swap(error_idx, cmd_idx);
        }
    }
    output.push_str(&generate_rust_enum("Message", &message_properties, "root"));
    output.push_str("\n\n");

    // Generate CommandResponse with deserialize_with on id field
    let mut cmd_response_properties = root_protocol.command_response.properties.clone();
    for property in &mut cmd_response_properties {
        if to_snake_case(&property.name) == "id" {
            property.attributes.push("#[serde(deserialize_with = \"float_or_int_to_u64\")]".to_string());
        }
    }
    output.push_str(&generate_rust_struct("CommandResponse", &cmd_response_properties, "root"));
    output.push_str("\n\n");

    // Generate ErrorResponse with deserialize_with on id field
    let mut error_response_properties = root_protocol.error_response.properties.clone();
    for property in &mut error_response_properties {
        if to_snake_case(&property.name) == "id" {
            property.attributes.push("#[serde(deserialize_with = \"option_float_or_int_to_u64\")]".to_string());
        }
    }
    output.push_str(&generate_rust_struct("ErrorResponse", &error_response_properties, "root"));
    output.push_str("\n\n");

    // Add Display implementation for ErrorResponse
    output.push_str("impl std::fmt::Display for ErrorResponse {\n");
    output.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
    output.push_str("        write!(\n");
    output.push_str("            f,\n");
    output.push_str("            \"Error[{}]: {} (ID: {}){}\",\n");
    output.push_str("            self.error,\n");
    output.push_str("            self.message,\n");
    output.push_str("            self.id.map_or(\"None\".to_string(), |id| id.to_string()),\n");
    output.push_str("            self.stacktrace.as_ref().map_or(\"\".to_string(), |st| format!(\"\\nStacktrace:\\n{}\", st))\n");
    output.push_str("        )\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");

    // Generate ResultData
    output.push_str(&generate_rust_enum("ResultData", &root_protocol.result_data.properties, "root"));
    output.push_str("\n\n");

    // Generate EmptyResult - special case with flattened Extensible
    output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    output.push_str("pub struct EmptyResult {\n");
    output.push_str("    #[serde(flatten, deserialize_with = \"deserialize_empty_map\")]\n");
    output.push_str("    pub extensible: Extensible,\n");
    output.push_str("}\n\n");

    // Generate Event
    output.push_str(&generate_rust_struct("Event", &root_protocol.event.properties, "root"));
    output.push_str("\n\n");

    // Generate EventData
    output.push_str(&generate_rust_enum("EventData", &root_protocol.event_data.properties, "root"));
    output.push_str("\n\n");

    // Generate ErrorCode
    output.push_str(&generate_rust_enum("ErrorCode", &root_protocol.error_code.properties, "root"));
    output.push_str("\n\n");

    // Add Display implementation for ErrorCode
    output.push_str("impl std::fmt::Display for ErrorCode {\n");
    output.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
    output.push_str("        match self {\n");
    for property in &root_protocol.error_code.properties {
        let variant_name = &property.name;
        // Convert PascalCase to "lowercase with spaces" (e.g., InvalidArgument -> invalid argument)
        let display_str = to_snake_case(variant_name).replace('_', " ");
        output.push_str(&format!("            ErrorCode::{} => write!(f, \"{}\"),\n", variant_name, display_str));
    }
    output.push_str("        }\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");

    // Generate additional types (like SuccessEnum, ErrorEnum, EventEnum)
    for bidi_type in &root_protocol.additional_types {
        if bidi_type.is_enum {
            output.push_str(&generate_rust_enum(&bidi_type.name, &bidi_type.properties, "root"));
        } else if bidi_type.is_alias {
            output.push_str(&generate_type_alias(&bidi_type.name, &bidi_type.properties, "root"));
        } else {
            output.push_str(&generate_rust_struct(&bidi_type.name, &bidi_type.properties, "root"));
        }
        output.push_str("\n\n");
    }

    // Write to lib.rs
    fs::write(format!("{}/lib.rs", output_dir), output)?;

    Ok(())
}

fn generate_module_files(module: &Module, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create module directory with snake_case name
    let module_dir_name = to_snake_case(&module.name);
    let module_dir = format!("{}/{}", output_dir, module_dir_name);
    fs::create_dir_all(&module_dir)?;
    
    // Create mod.rs
    let mut mod_content = String::new();
    
    // Create command file if definition exists
    if let Some(ref cmd_def) = module.command_definition {
        let command_content = generate_commands_file(cmd_def, module);
        fs::write(format!("{}/commands.rs", module_dir), command_content)?;
        mod_content.push_str("pub mod commands;\n");
    }
    
    // Create event file if definition exists
    if let Some(ref event_def) = module.event_definition {
        let events_content = generate_events_file(event_def, module);
        fs::write(format!("{}/events.rs", module_dir), events_content)?;
        mod_content.push_str("pub mod events;\n");
    }
    
    // Create types file with generated types
    let types_content = generate_types_file(module);
    fs::write(format!("{}/types.rs", module_dir), types_content)?;
    mod_content.push_str("pub mod types;\n");
    
    // Write mod.rs
    fs::write(format!("{}/mod.rs", module_dir), mod_content)?;
    
    Ok(())
}

fn generate_command_method(method: &crate::command_parser::CommandMethods) -> String {
    let mut output = String::new();

    // Add enum attributes
    for attribute in &method.enum_attributes {
        output.push_str(&format!("{}\n", attribute));
    }

    // Generate method enum
    output.push_str(&format!("pub enum {} {{\n", method.name));

    // Add method attributes
    for attribute in &method.method_attributes {
        output.push_str(&format!("    {}\n", attribute));
    }

    // Add the method variant as unit variant
    let method_variant = method.name.replace("Method", "");
    output.push_str(&format!("    {},\n", method_variant));

    output.push_str("}");
    output
}

fn generate_command_param(param: &crate::command_parser::CommandParams, module: &Module) -> String {
    let mut output = String::new();
    let mut default_functions = String::new();

    // Clone properties and add serde flatten to enum properties when appropriate
    let mut modified_properties: Vec<crate::parser::Property> = param.properties.to_vec();
    let all_enum_properties = param.properties.iter().all(|p| p.is_enum);

    for property in &mut modified_properties {
        if property.is_enum {
            if param.properties.len() == 1 && all_enum_properties {
                // Single enum property case
                property.attributes.push("#[serde(flatten)]".to_string());
            } else if !all_enum_properties {
                // Mixed properties case - flatten enum properties
                property.attributes.push("#[serde(flatten)]".to_string());
            }
        }
    }

    // Generate default functions first (they need to come before the struct)
    for property in &modified_properties {
        if let Some(validation_info) = &property.validation_info {
            if let Some(default_value) = &validation_info.default_value {
                let function_name = format!("{}_default_{}", to_snake_case(&param.name), to_snake_case(&property.name));
                let cleaned_type = clean_module_prefix(&property.value, &module.name);

                let return_type = if property.is_optional {
                    format!("Option<{}>", cleaned_type)
                } else {
                    cleaned_type.clone()
                };

                let transformed_value = transform_default_value(default_value, &cleaned_type);
                let return_value = if property.is_optional {
                    format!("Some({})", transformed_value)
                } else {
                    transformed_value
                };

                default_functions.push_str(&format!(
                    "fn {}() -> {} {{\n    {}\n}}\n\n",
                    function_name, return_type, return_value
                ));
            }
        }
    }

    // Add default functions to output
    if !default_functions.is_empty() {
        output.push_str(&default_functions);
    }

    // Check if validation is used
    let has_validation = uses_validation(&modified_properties);

    // Add parameter struct attributes
    for attribute in &param.attributes {
        // Extend derive attribute to include Validate if needed
        if has_validation && attribute.starts_with("#[derive(") {
            let extended = attribute.replace(")]\n", ", Validate)]\n").replace(")]", ", Validate)]");
            output.push_str(&format!("{}\n", extended));
        } else {
            output.push_str(&format!("{}\n", attribute));
        }
    }

    // Generate parameter struct
    output.push_str(&format!("pub struct {} {{\n", param.name));

    // Add properties
    for property in &modified_properties {
        // Add property attributes
        for attr in &property.attributes {
            output.push_str(&format!("    {}\n", attr));
        }

        // Add validation-based attributes
        if let Some(validation_info) = &property.validation_info {
            // Add validation constraints - each constraint gets its own #[validate(...)] attribute
            for constraint in &validation_info.constraints {
                match constraint.constraint_type.as_str() {
                    "ge" => {
                        output.push_str(&format!("    #[validate(minimum = {})]\n", constraint.value));
                    },
                    "le" => {
                        output.push_str(&format!("    #[validate(maximum = {})]\n", constraint.value));
                    },
                    "gt" => {
                        // For greater than, we need to add a small epsilon
                        if let Ok(val) = constraint.value.parse::<f64>() {
                            output.push_str(&format!("    #[validate(minimum = {})]\n", val + f64::EPSILON));
                        } else {
                            output.push_str(&format!("    #[validate(minimum = {})]\n", constraint.value));
                        }
                    },
                    "lt" => {
                        // For less than, we need to subtract a small epsilon
                        if let Ok(val) = constraint.value.parse::<f64>() {
                            output.push_str(&format!("    #[validate(maximum = {})]\n", val - f64::EPSILON));
                        } else {
                            output.push_str(&format!("    #[validate(maximum = {})]\n", constraint.value));
                        }
                    },
                    _ => {} // Unknown constraint type
                }
            }

            // Add default value attribute
            if let Some(_default_value) = &validation_info.default_value {
                let function_name = format!("{}_default_{}", to_snake_case(&param.name), to_snake_case(&property.name));
                output.push_str(&format!("    #[serde(default = \"{}\")]\n", function_name));
            }
        }

        // Clean up property type - remove module prefix if it's the same module
        let cleaned_type = clean_module_prefix(&property.value, &module.name);

        // Add field
        let field_type = if property.is_optional {
            format!("Option<{}>", cleaned_type)
        } else {
            cleaned_type
        };

        // Snakify property name and handle "type" keyword
        let mut field_name = to_snake_case(&property.name);
        if field_name == "type" {
            field_name = "r#type".to_string();
        }

        output.push_str(&format!("    pub {}: {},\n", field_name, field_type));
    }

    output.push_str("}");
    output
}

fn generate_commands_file(cmd_def: &crate::command_parser::CommandDefinition, module: &Module) -> String {
    let mut output = String::new();

    // Add header comment
    output.push_str("// Generated commands for module\n\n");

    // Add imports for serde
    output.push_str("use serde::{Serialize, Deserialize};\n");

    // Collect external type references
    let mut all_properties = Vec::new();
    for param in &cmd_def.command_params {
        all_properties.extend(param.properties.clone());
    }
    for command in &cmd_def.commands {
        all_properties.extend(command.properties.clone());
    }
    if let Some(result_def) = &module.result_definition {
        for result in &result_def.results {
            all_properties.extend(result.properties.clone());
        }
    }

    let external_types = collect_external_types(&all_properties);
    let external_imports = generate_external_imports(&external_types);
    output.push_str(&external_imports);

    // Import Extensible from root if used
    if uses_extensible(&all_properties) {
        output.push_str("use crate::Extensible;\n");
    }

    // Import EmptyParams from root if used
    if uses_empty_params(&all_properties) {
        output.push_str("use crate::EmptyParams;\n");
    }

    // Import EmptyResult from root if used
    if uses_empty_result(&all_properties) {
        output.push_str("use crate::EmptyResult;\n");
    }

    // Import serde_valid if validation is used
    if uses_validation(&all_properties) {
        output.push_str("use serde_valid::Validate;\n");
    }

    // Import from types if module has types
    if has_types_in_module(module) {
        output.push_str("use super::types::*;\n");
    }
    output.push_str("\n");

    // Generate the main command enum first
    output.push_str(&generate_command_enum(cmd_def));
    output.push_str("\n\n");

    // Generate command methods after the enum definition
    for method in &cmd_def.command_methods {
        output.push_str(&generate_command_method(method));
        output.push_str("\n\n");
    }

    // Generate command params after methods
    for param in &cmd_def.command_params {
        // Check if all properties are enums - if so, generate as enum instead of struct
        let all_enum_properties = param.properties.iter().all(|p| p.is_enum);

        if all_enum_properties && !param.properties.is_empty() {
            output.push_str(&generate_rust_enum(&param.name, &param.properties, &module.name));
        } else {
            output.push_str(&generate_command_param(param, module));
        }
        output.push_str("\n\n");
    }

    // Generate individual command structs
    for command in &cmd_def.commands {
        output.push_str(&generate_command_struct(command));
        output.push_str("\n\n");
    }

    // Generate result enum and structs if result definition exists
    if let Some(ref result_def) = module.result_definition {
        output.push_str("// Generated results\n\n");
        output.push_str(&generate_result_enum(result_def));
        output.push_str("\n\n");

        // Generate individual result structs
        for result in &result_def.results {
            // Check if all properties are enums - if so, generate as enum instead of struct
            let all_enum_properties = result.properties.iter().all(|p| p.is_enum);

            if all_enum_properties && !result.properties.is_empty() {
                output.push_str(&generate_rust_enum(&result.name, &result.properties, &result.module_name));
            } else {
                output.push_str(&generate_result_struct(result, module));
            }
            output.push_str("\n\n");
        }
    }

    output
}

fn generate_events_file(event_def: &crate::event_parser::EventDefinition, module: &Module) -> String {
    let mut output = String::new();

    // Add header comment
    output.push_str("// Generated events for module\n\n");

    // Add imports for serde
    output.push_str("use serde::{Serialize, Deserialize};\n");

    // Collect external type references
    let mut all_properties = Vec::new();
    for param in &event_def.event_params {
        all_properties.extend(param.properties.clone());
    }
    for event in &event_def.events {
        all_properties.extend(event.properties.clone());
    }

    let external_types = collect_external_types(&all_properties);
    let external_imports = generate_external_imports(&external_types);
    output.push_str(&external_imports);

    // Import Extensible from root if used
    if uses_extensible(&all_properties) {
        output.push_str("use crate::Extensible;\n");
    }

    // Import EmptyParams from root if used
    if uses_empty_params(&all_properties) {
        output.push_str("use crate::EmptyParams;\n");
    }

    // Import EmptyResult from root if used
    if uses_empty_result(&all_properties) {
        output.push_str("use crate::EmptyResult;\n");
    }

    // Import serde_valid if validation is used
    if uses_validation(&all_properties) {
        output.push_str("use serde_valid::Validate;\n");
    }

    // Import from types if module has types
    if has_types_in_module(module) {
        output.push_str("use super::types::*;\n");
    }
    output.push_str("\n");

    // Generate the main event enum first
    output.push_str(&generate_event_enum(event_def));
    output.push_str("\n\n");

    // Generate event methods after the enum definition
    for method in &event_def.event_methods {
        output.push_str(&generate_event_method(method));
        output.push_str("\n\n");
    }

    // Generate event params after methods
    for param in &event_def.event_params {
        output.push_str(&generate_event_param(param, module));
        output.push_str("\n\n");
    }

    // Generate individual event structs
    for event in &event_def.events {
        output.push_str(&generate_event_struct_new(event));
        output.push_str("\n\n");
    }

    output
}

fn generate_command_struct(command: &crate::command_parser::Command) -> String {
    let mut output = String::new();

    // Check if validation is used
    let has_validation = uses_validation(&command.properties);

    // Add command attributes
    for attribute in &command.attributes {
        // Extend derive attribute to include Validate if needed
        if has_validation && attribute.starts_with("#[derive(") {
            let extended = attribute.replace(")]\n", ", Validate)]\n").replace(")]", ", Validate)]");
            output.push_str(&format!("{}\n", extended));
        } else {
            output.push_str(&format!("{}\n", attribute));
        }
    }

    output.push_str(&format!("pub struct {} {{\n", command.name));

    // Add properties with their attributes
    for property in &command.properties {
        // Add property attributes
        for attr in &property.attributes {
            output.push_str(&format!("    {}\n", attr));
        }

        // Clean up property type - remove module prefix if it's the same module
        let cleaned_type = clean_module_prefix(&property.value, &command.module_name);

        // Add field
        let field_type = if property.is_optional {
            format!("Option<{}>", cleaned_type)
        } else {
            cleaned_type
        };

        // Snakify property name and handle "type" keyword
        let mut field_name = to_snake_case(&property.name);
        if field_name == "type" {
            field_name = "r#type".to_string();
        }

        output.push_str(&format!("    pub {}: {},\n", field_name, field_type));
    }

    output.push_str("}");
    output
}

fn generate_types_file(module: &Module) -> String {
    let mut output = String::new();

    // Add header comment
    output.push_str("// Generated types for module\n\n");

    // Add imports for serde
    output.push_str("use serde::{Serialize, Deserialize};\n");

    // Collect external type references
    let mut all_properties = Vec::new();
    for bidi_type in &module.types {
        all_properties.extend(bidi_type.properties.clone());
    }

    let external_types = collect_external_types(&all_properties);
    let external_imports = generate_external_imports(&external_types);
    output.push_str(&external_imports);

    // Import Extensible from root if used
    if uses_extensible(&all_properties) {
        output.push_str("use crate::Extensible;\n");
    }

    // Import EmptyParams from root if used
    if uses_empty_params(&all_properties) {
        output.push_str("use crate::EmptyParams;\n");
    }

    // Import EmptyResult from root if used
    if uses_empty_result(&all_properties) {
        output.push_str("use crate::EmptyResult;\n");
    }

    // Import serde_valid if validation is used
    if uses_validation(&all_properties) {
        output.push_str("use serde_valid::Validate;\n");
    }

    output.push_str("\n");

    // Generate each type (skip Extensible, EmptyParams, and EmptyResult as they're generated in lib.rs)
    for bidi_type in &module.types {
        // Skip types that are generated in root lib.rs
        if bidi_type.name == "Extensible" || bidi_type.name == "EmptyParams" || bidi_type.name == "EmptyResult" {
            continue;
        }

        if bidi_type.is_enum {
            // Force enum generation if BidiType is marked as enum
            output.push_str(&generate_rust_enum(&bidi_type.name, &bidi_type.properties, &module.name));
        } else if bidi_type.is_alias {
            // Generate type alias
            output.push_str(&generate_type_alias(&bidi_type.name, &bidi_type.properties, &module.name));
        } else {
            output.push_str(&generate_rust_struct(&bidi_type.name, &bidi_type.properties, &module.name));
        }
        output.push_str("\n\n");
    }

    // If no types, add placeholder comment
    if module.types.is_empty() {
        output.push_str("// No types generated for this module\n");
    }

    output
}

fn generate_rust_struct(name: &str, properties: &[crate::parser::Property], module_name: &str) -> String {
    let mut output = String::new();

    // Check if this should be an enum instead of a struct
    let has_enum_properties = properties.iter().any(|p| p.is_enum);
    let all_enum_properties = properties.iter().all(|p| p.is_enum);

    // Clone properties and add serde flatten to enum properties when appropriate
    let mut modified_properties: Vec<crate::parser::Property> = properties.to_vec();

    for property in &mut modified_properties {
        if property.is_enum {
            if properties.len() == 1 && all_enum_properties {
                // Single enum property case
                property.attributes.push("#[serde(flatten)]".to_string());
            } else if !all_enum_properties {
                // Mixed properties case - flatten enum properties
                property.attributes.push("#[serde(flatten)]".to_string());
            }
        }
    }

    if has_enum_properties && properties.len() > 1 && all_enum_properties {
        // Generate enum with variants only if ALL properties are enums
        generate_rust_enum(name, &modified_properties, module_name)
    } else if properties.is_empty() {
        // Generate unit struct for empty types
        output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
        output.push_str(&format!("pub struct {};\n", name));
        output
    } else {
        // Generate regular struct
        generate_rust_regular_struct(name, &modified_properties, module_name)
    }
}

fn generate_rust_regular_struct(name: &str, properties: &[crate::parser::Property], module_name: &str) -> String {
    let mut output = String::new();
    let mut default_functions = String::new();

    // Generate default functions first (they need to come before the struct)
    for property in properties {
        if let Some(validation_info) = &property.validation_info {
            if let Some(default_value) = &validation_info.default_value {
                let function_name = format!("{}_default_{}", to_snake_case(name), to_snake_case(&property.name));
                let cleaned_type = clean_module_prefix(&property.value, module_name);

                let return_type = if property.is_optional {
                    format!("Option<{}>", cleaned_type)
                } else {
                    cleaned_type.clone()
                };

                let transformed_value = transform_default_value(default_value, &cleaned_type);
                let return_value = if property.is_optional || (return_type.contains("Option<") && transformed_value != "None") {
                    format!("Some({})", transformed_value)
                } else {
                    transformed_value
                };

                default_functions.push_str(&format!(
                    "fn {}() -> {} {{\n    {}\n}}\n\n",
                    function_name, return_type, return_value
                ));
            }
        }
    }

    // Add default functions to output
    if !default_functions.is_empty() {
        output.push_str(&default_functions);
    }

    // Check if validation is used
    let has_validation = uses_validation(properties);

    // Add derive attributes
    if has_validation {
        output.push_str("#[derive(Debug, Clone, Serialize, Deserialize, Validate)]\n");
    } else {
        output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    }
    output.push_str(&format!("pub struct {} {{\n", name));

    // Add properties
    for property in properties {
        // Add property attributes
        for attr in &property.attributes {
            output.push_str(&format!("    {}\n", attr));
        }

        // Add validation-based attributes
        if let Some(validation_info) = &property.validation_info {
            // Add validation constraints - each constraint gets its own #[validate(...)] attribute
            for constraint in &validation_info.constraints {
                match constraint.constraint_type.as_str() {
                    "ge" => {
                        output.push_str(&format!("    #[validate(minimum = {})]\n", constraint.value));
                    },
                    "le" => {
                        output.push_str(&format!("    #[validate(maximum = {})]\n", constraint.value));
                    },
                    "gt" => {
                        // For greater than, we need to add a small epsilon
                        if let Ok(val) = constraint.value.parse::<f64>() {
                            output.push_str(&format!("    #[validate(minimum = {})]\n", val + f64::EPSILON));
                        } else {
                            output.push_str(&format!("    #[validate(minimum = {})]\n", constraint.value));
                        }
                    },
                    "lt" => {
                        // For less than, we need to subtract a small epsilon
                        if let Ok(val) = constraint.value.parse::<f64>() {
                            output.push_str(&format!("    #[validate(maximum = {})]\n", val - f64::EPSILON));
                        } else {
                            output.push_str(&format!("    #[validate(maximum = {})]\n", constraint.value));
                        }
                    },
                    _ => {} // Unknown constraint type
                }
            }

            // Add default value attribute
            if let Some(_default_value) = &validation_info.default_value {
                let function_name = format!("{}_default_{}", to_snake_case(name), to_snake_case(&property.name));
                output.push_str(&format!("    #[serde(default = \"{}\")]\n", function_name));
            }
        }

        // Clean up property type - remove module prefix if it's the same module
        let cleaned_type = clean_module_prefix(&property.value, module_name);

        let field_type = if property.is_optional {
            format!("Option<{}>", cleaned_type)
        } else {
            cleaned_type
        };

        // Snakify property name and handle "type" keyword
        let mut field_name = to_snake_case(&property.name);
        if field_name == "type" {
            field_name = "r#type".to_string();
        }

        output.push_str(&format!("    pub {}: {},\n", field_name, field_type));
    }

    output.push_str("}");
    output
}

fn generate_type_alias(name: &str, properties: &[crate::parser::Property], module_name: &str) -> String {
    let mut output = String::new();

    // For alias types, there should be exactly one property that represents the aliased type
    if let Some(first_property) = properties.first() {
        let cleaned_type = clean_module_prefix(&first_property.value, module_name);
        output.push_str(&format!("pub type {} = {};\n", name, cleaned_type));
    } else {
        // Fallback to unit type if no properties
        output.push_str(&format!("pub type {} = ();\n", name));
    }

    output
}

fn generate_rust_enum(name: &str, properties: &[crate::parser::Property], module_name: &str) -> String {
    let mut output = String::new();

    // Add derive attributes
    output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");

    // Check if we need untagged serde for union types
    // ErrorCode, SuccessEnum, ErrorEnum, and EventEnum should not be untagged
    if name != "ErrorCode" && name != "SuccessEnum" && name != "ErrorEnum" && name != "EventEnum" {
        output.push_str("#[serde(untagged)]\n");
    }

    output.push_str(&format!("pub enum {} {{\n", name));

    // Add enum variants
    for property in properties {
        // Add variant attributes
        for attr in &property.attributes {
            output.push_str(&format!("    {}\n", attr));
        }

        // Clean up property type - remove module prefix if it's the same module
        let cleaned_type = clean_module_prefix(&property.value, module_name);

        // Sanitize the variant name
        let sanitized_variant_name = sanitize_property_name(&property.name);

        if property.is_enum {
            // Enum variant
            if cleaned_type == "UNIT_VARIANT" {
                // Unit variant (no value)
                output.push_str(&format!("    {},\n", sanitized_variant_name));
            } else {
                // Tuple variant (with value)
                output.push_str(&format!("    {}({}),\n", sanitized_variant_name, cleaned_type));
            }
        } else {
            // Regular variant with data
            output.push_str(&format!("    {}({}),\n", sanitized_variant_name, cleaned_type));
        }
    }

    output.push_str("}");
    output
}

fn generate_command_enum(cmd_def: &crate::command_parser::CommandDefinition) -> String {
    let mut output = String::new();
    
    // Add attributes
    for attribute in &cmd_def.attributes {
        output.push_str(&format!("{}\n", attribute));
    }
    
    // Add enum declaration
    output.push_str(&format!("pub enum {} {{\n", cmd_def.name));
    
    // Add enum variants
    for command in &cmd_def.commands {
        // Add variant - use command name as both variant and type like Close(Close)
        output.push_str(&format!("    {}({}),\n", command.name, command.name));
    }
    
    output.push_str("}\n");
    output
}

fn generate_result_enum(result_def: &crate::command_parser::ResultDefinition) -> String {
    let mut output = String::new();

    // Add attributes
    for attribute in &result_def.attributes {
        output.push_str(&format!("{}\n", attribute));
    }

    // Add enum declaration
    output.push_str(&format!("pub enum {} {{\n", result_def.name));

    // Add enum variants
    for result in &result_def.results {
        // Add variant - use result name as both variant and type like CreateUserContextResult(CreateUserContextResult)
        output.push_str(&format!("    {}({}),\n", result.name, result.name));
    }

    output.push_str("}\n");
    output
}

fn generate_event_enum(event_def: &crate::event_parser::EventDefinition) -> String {
    let mut output = String::new();

    // Add attributes
    for attribute in &event_def.attributes {
        output.push_str(&format!("{}\n", attribute));
    }

    // Add enum declaration
    output.push_str(&format!("pub enum {} {{\n", event_def.name));

    // Add enum variants
    for event in &event_def.events {
        // Add variant - use event name as both variant and type like ContextCreated(ContextCreated)
        output.push_str(&format!("    {}({}),\n", event.name, event.name));
    }

    output.push_str("}\n");
    output
}

fn generate_result_struct(result: &crate::command_parser::BidiResult, module: &Module) -> String {
    let mut output = String::new();

    // Handle type aliases differently from structs
    if result.is_alias {
        // Generate type alias: pub type ResultName = AliasedType;
        let cleaned_type = clean_module_prefix(&result.content, &result.module_name);
        output.push_str(&format!("pub type {} = {};\n", result.name, cleaned_type));
        return output;
    }

    // Clone properties and add serde flatten to enum properties when appropriate
    let mut modified_properties: Vec<crate::parser::Property> = result.properties.to_vec();
    let all_enum_properties = result.properties.iter().all(|p| p.is_enum);

    for property in &mut modified_properties {
        if property.is_enum {
            if result.properties.len() == 1 && all_enum_properties {
                // Single enum property case
                property.attributes.push("#[serde(flatten)]".to_string());
            } else if !all_enum_properties {
                // Mixed properties case - flatten enum properties
                property.attributes.push("#[serde(flatten)]".to_string());
            }
        }
    }

    // Generate regular struct
    // Check if validation is used
    let has_validation = uses_validation(&modified_properties);

    // Add result attributes
    for attribute in &result.attributes {
        // Extend derive attribute to include Validate if needed
        if has_validation && attribute.starts_with("#[derive(") {
            let extended = attribute.replace(")]\n", ", Validate)]\n").replace(")]", ", Validate)]");
            output.push_str(&format!("{}\n", extended));
        } else {
            output.push_str(&format!("{}\n", attribute));
        }
    }

    output.push_str(&format!("pub struct {} {{\n", result.name));

    // Add properties with their attributes
    for property in &modified_properties {
        // Add property attributes
        for attr in &property.attributes {
            output.push_str(&format!("    {}\n", attr));
        }

        // Clean up property type - remove module prefix if it's the same module
        let cleaned_type = clean_module_prefix(&property.value, &result.module_name);

        // Add field
        let field_type = if property.is_optional {
            format!("Option<{}>", cleaned_type)
        } else {
            cleaned_type
        };

        // Snakify property name and handle "type" keyword
        let mut field_name = to_snake_case(&property.name);
        if field_name == "type" {
            field_name = "r#type".to_string();
        }

        output.push_str(&format!("    pub {}: {},\n", field_name, field_type));
    }

    output.push_str("}");
    output
}

fn generate_event_method(method: &crate::event_parser::EventMethods) -> String {
    let mut output = String::new();

    // Add enum attributes
    for attribute in &method.enum_attributes {
        output.push_str(&format!("{}\n", attribute));
    }

    // Generate method enum
    output.push_str(&format!("pub enum {} {{\n", method.name));

    // Add method attributes
    for attribute in &method.method_attributes {
        output.push_str(&format!("    {}\n", attribute));
    }

    // Add the method variant as unit variant
    let method_variant = method.name.replace("Method", "");
    output.push_str(&format!("    {},\n", method_variant));

    output.push_str("}");
    output
}

fn generate_event_param(param: &crate::event_parser::EventParams, module: &Module) -> String {
    let mut output = String::new();
    let mut default_functions = String::new();

    // Clone properties and add serde flatten to enum properties when appropriate
    let mut modified_properties: Vec<crate::parser::Property> = param.properties.to_vec();
    let all_enum_properties = param.properties.iter().all(|p| p.is_enum);

    for property in &mut modified_properties {
        if property.is_enum {
            if param.properties.len() == 1 && all_enum_properties {
                // Single enum property case
                property.attributes.push("#[serde(flatten)]".to_string());
            } else if !all_enum_properties {
                // Mixed properties case - flatten enum properties
                property.attributes.push("#[serde(flatten)]".to_string());
            }
        }
    }

    // Generate default functions first (they need to come before the struct)
    for property in &modified_properties {
        if property.name.contains("union") {
            println!("{}", property.name)
        }
        if let Some(validation_info) = &property.validation_info {
            if let Some(default_value) = &validation_info.default_value {
                let function_name = format!("{}_default_{}", to_snake_case(&param.name), to_snake_case(&property.name));
                let cleaned_type = clean_module_prefix(&property.value, &module.name);

                let return_type = if property.is_optional {
                    format!("Option<{}>", cleaned_type)
                } else {
                    cleaned_type.clone()
                };

                let transformed_value = transform_default_value(default_value, &cleaned_type);
                let return_value = if property.is_optional {
                    format!("Some({})", transformed_value)
                } else {
                    transformed_value
                };

                default_functions.push_str(&format!(
                    "fn {}() -> {} {{\n    {}\n}}\n\n",
                    function_name, return_type, return_value
                ));
            }
        }
    }

    // Add default functions to output
    if !default_functions.is_empty() {
        output.push_str(&default_functions);
    }

    // Check if validation is used
    let has_validation = uses_validation(&modified_properties);

    // Add parameter struct attributes
    for attribute in &param.attributes {
        // Extend derive attribute to include Validate if needed
        if has_validation && attribute.starts_with("#[derive(") {
            let extended = attribute.replace(")]\n", ", Validate)]\n").replace(")]", ", Validate)]");
            output.push_str(&format!("{}\n", extended));
        } else {
            output.push_str(&format!("{}\n", attribute));
        }
    }

    // Generate parameter struct
    output.push_str(&format!("pub struct {} {{\n", param.name));

    // Add properties
    for property in &modified_properties {
        // Add property attributes
        for attr in &property.attributes {
            output.push_str(&format!("    {}\n", attr));
        }

        // Add validation-based attributes
        if let Some(validation_info) = &property.validation_info {
            // Add validation constraints - each constraint gets its own #[validate(...)] attribute
            for constraint in &validation_info.constraints {
                match constraint.constraint_type.as_str() {
                    "ge" => {
                        output.push_str(&format!("    #[validate(minimum = {})]\n", constraint.value));
                    },
                    "le" => {
                        output.push_str(&format!("    #[validate(maximum = {})]\n", constraint.value));
                    },
                    "gt" => {
                        // For greater than, we need to add a small epsilon
                        if let Ok(val) = constraint.value.parse::<f64>() {
                            output.push_str(&format!("    #[validate(minimum = {})]\n", val + f64::EPSILON));
                        } else {
                            output.push_str(&format!("    #[validate(minimum = {})]\n", constraint.value));
                        }
                    },
                    "lt" => {
                        // For less than, we need to subtract a small epsilon
                        if let Ok(val) = constraint.value.parse::<f64>() {
                            output.push_str(&format!("    #[validate(maximum = {})]\n", val - f64::EPSILON));
                        } else {
                            output.push_str(&format!("    #[validate(maximum = {})]\n", constraint.value));
                        }
                    },
                    _ => {} // Unknown constraint type
                }
            }

            // Add default value attribute
            if let Some(_default_value) = &validation_info.default_value {
                let function_name = format!("{}_default_{}", to_snake_case(&param.name), to_snake_case(&property.name));
                output.push_str(&format!("    #[serde(default = \"{}\")]\n", function_name));
            }
        }

        // Clean up property type - remove module prefix if it's the same module
        let cleaned_type = clean_module_prefix(&property.value, &module.name);

        // Add field
        let field_type = if property.is_optional {
            format!("Option<{}>", cleaned_type)
        } else {
            cleaned_type
        };

        // Snakify property name and handle "type" keyword
        let mut field_name = to_snake_case(&property.name);
        if field_name == "type" {
            field_name = "r#type".to_string();
        }

        output.push_str(&format!("    pub {}: {},\n", field_name, field_type));
    }

    output.push_str("}");
    output
}

fn generate_event_struct_new(event: &crate::event_parser::Event) -> String {
    let mut output = String::new();

    // Check if validation is used
    let has_validation = uses_validation(&event.properties);

    // Add event attributes
    for attribute in &event.attributes {
        // Extend derive attribute to include Validate if needed
        if has_validation && attribute.starts_with("#[derive(") {
            let extended = attribute.replace(")]\n", ", Validate)]\n").replace(")]", ", Validate)]");
            output.push_str(&format!("{}\n", extended));
        } else {
            output.push_str(&format!("{}\n", attribute));
        }
    }

    output.push_str(&format!("pub struct {} {{\n", event.name));

    // Add properties with their attributes
    for property in &event.properties {
        // Add property attributes
        for attr in &property.attributes {
            output.push_str(&format!("    {}\n", attr));
        }

        // Clean up property type - remove module prefix if it's the same module
        let cleaned_type = clean_module_prefix(&property.value, &event.module_name);

        // Add field
        let field_type = if property.is_optional {
            format!("Option<{}>", cleaned_type)
        } else {
            cleaned_type
        };

        // Snakify property name and handle "type" keyword
        let mut field_name = to_snake_case(&property.name);
        if field_name == "type" {
            field_name = "r#type".to_string();
        }

        output.push_str(&format!("    pub {}: {},\n", field_name, field_type));
    }

    output.push_str("}");
    output
}