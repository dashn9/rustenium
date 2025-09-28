use std::fs;
use std::path::Path;
use crate::module::Module;

/// Converts PascalCase to snake_case
///
/// # Arguments
/// * `input` - The PascalCase string to convert
///
/// # Returns
/// The snake_case version of the string
fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_uppercase() && !result.is_empty() {
            result.push('_');
        }
        result.push(ch.to_lowercase().to_string().chars().next().unwrap());
    }

    result
}

/// Cleans up module prefixes from type names if they belong to the same module
///
/// # Arguments
/// * `type_name` - The type name that might have a module prefix (e.g., "browser.UserContext")
/// * `current_module` - The current module name (e.g., "browser")
///
/// # Returns
/// The cleaned type name without module prefix if it's the same module
fn clean_module_prefix(type_name: &str, current_module: &str) -> String {
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

pub fn generate_output(modules: &[Module]) -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory
    let output_dir = "./output";
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir)?;
    }
    
    // Generate files for each module
    for module in modules {
        generate_module_files(module, output_dir)?;
    }
    
    Ok(())
}

fn generate_module_files(module: &Module, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create module directory
    let module_dir = format!("{}/{}", output_dir, module.name);
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

    // Add the method variant in format method(method)
    let method_variant = method.name.replace("Method", "");
    output.push_str(&format!("    {}({}),\n", method_variant, method_variant));

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
                let function_name = format!("{}_default_{}", to_snake_case(&param.name), property.name);
                let cleaned_type = clean_module_prefix(&property.value, &module.name);

                let return_type = if property.is_optional {
                    format!("Option<{}>", cleaned_type)
                } else {
                    cleaned_type.clone()
                };

                let return_value = if property.is_optional {
                    format!("Some({})", default_value)
                } else {
                    default_value.clone()
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

    // Add parameter struct attributes
    for attribute in &param.attributes {
        output.push_str(&format!("{}\n", attribute));
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
            // Add validation constraints
            if !validation_info.constraints.is_empty() {
                let mut constraints = Vec::new();
                for constraint in &validation_info.constraints {
                    match constraint.constraint_type.as_str() {
                        "ge" => constraints.push(format!("min = {}", constraint.value)),
                        "le" => constraints.push(format!("max = {}", constraint.value)),
                        "gt" => {
                            // For greater than, we need to add a small epsilon
                            if let Ok(val) = constraint.value.parse::<f64>() {
                                constraints.push(format!("min = {}", val + f64::EPSILON));
                            } else {
                                constraints.push(format!("min = {}", constraint.value));
                            }
                        },
                        "lt" => {
                            // For less than, we need to subtract a small epsilon
                            if let Ok(val) = constraint.value.parse::<f64>() {
                                constraints.push(format!("max = {}", val - f64::EPSILON));
                            } else {
                                constraints.push(format!("max = {}", constraint.value));
                            }
                        },
                        _ => {} // Unknown constraint type
                    }
                }
                if !constraints.is_empty() {
                    output.push_str(&format!("    #[validate(range({}))]\n", constraints.join(", ")));
                }
            }

            // Add default value attribute
            if let Some(_default_value) = &validation_info.default_value {
                let function_name = format!("{}_default_{}", to_snake_case(&param.name), property.name);
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
        output.push_str(&generate_command_param(param, module));
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
            output.push_str(&generate_result_struct(result, module));
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
    output.push_str("use serde::{Serialize, Deserialize};\n\n");

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

    // Add command attributes
    for attribute in &command.attributes {
        output.push_str(&format!("{}\n", attribute));
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
    output.push_str("use serde::{Serialize, Deserialize};\n\n");

    // Generate each type
    for bidi_type in &module.types {
        if bidi_type.is_enum {
            // Force enum generation if BidiType is marked as enum
            output.push_str(&generate_rust_enum(&bidi_type.name, &bidi_type.properties, &module.name));
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
                let function_name = format!("{}_default_{}", to_snake_case(name), property.name);
                let cleaned_type = clean_module_prefix(&property.value, module_name);

                let return_type = if property.is_optional {
                    format!("Option<{}>", cleaned_type)
                } else {
                    cleaned_type.clone()
                };

                let return_value = if property.is_optional {
                    format!("Some({})", default_value)
                } else {
                    default_value.clone()
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

    // Add derive attributes
    output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    output.push_str(&format!("pub struct {} {{\n", name));

    // Add properties
    for property in properties {
        // Add property attributes
        for attr in &property.attributes {
            output.push_str(&format!("    {}\n", attr));
        }

        // Add validation-based attributes
        if let Some(validation_info) = &property.validation_info {
            // Add validation constraints
            if !validation_info.constraints.is_empty() {
                let mut constraints = Vec::new();
                for constraint in &validation_info.constraints {
                    match constraint.constraint_type.as_str() {
                        "ge" => constraints.push(format!("min = {}", constraint.value)),
                        "le" => constraints.push(format!("max = {}", constraint.value)),
                        "gt" => {
                            // For greater than, we need to add a small epsilon
                            if let Ok(val) = constraint.value.parse::<f64>() {
                                constraints.push(format!("min = {}", val + f64::EPSILON));
                            } else {
                                constraints.push(format!("min = {}", constraint.value));
                            }
                        },
                        "lt" => {
                            // For less than, we need to subtract a small epsilon
                            if let Ok(val) = constraint.value.parse::<f64>() {
                                constraints.push(format!("max = {}", val - f64::EPSILON));
                            } else {
                                constraints.push(format!("max = {}", constraint.value));
                            }
                        },
                        _ => {} // Unknown constraint type
                    }
                }
                if !constraints.is_empty() {
                    output.push_str(&format!("    #[validate(range({}))]\n", constraints.join(", ")));
                }
            }

            // Add default value attribute
            if let Some(_default_value) = &validation_info.default_value {
                let function_name = format!("{}_default_{}", to_snake_case(name), property.name);
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

fn generate_rust_enum(name: &str, properties: &[crate::parser::Property], module_name: &str) -> String {
    let mut output = String::new();

    // Add derive attributes
    output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");

    // Check if we need untagged serde for union types
    let is_union = properties.iter().any(|p| p.attributes.iter().any(|a| a.contains("serde(rename")));
    if is_union {
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

        if property.is_enum {
            // Enum variant
            if cleaned_type == "UNIT_VARIANT" {
                // Unit variant (no value)
                output.push_str(&format!("    {},\n", property.name));
            } else {
                // Tuple variant (with value)
                output.push_str(&format!("    {}({}),\n", property.name, cleaned_type));
            }
        } else {
            // Regular variant with data
            output.push_str(&format!("    {}({}),\n", property.name, cleaned_type));
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
    // Add result attributes
    for attribute in &result.attributes {
        output.push_str(&format!("{}\n", attribute));
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

    // Add the method variant in format method(method)
    let method_variant = method.name.replace("Method", "");
    output.push_str(&format!("    {}({}),\n", method_variant, method_variant));

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
                let function_name = format!("{}_default_{}", to_snake_case(&param.name), property.name);
                let cleaned_type = clean_module_prefix(&property.value, &module.name);

                let return_type = if property.is_optional {
                    format!("Option<{}>", cleaned_type)
                } else {
                    cleaned_type.clone()
                };

                let return_value = if property.is_optional {
                    format!("Some({})", default_value)
                } else {
                    default_value.clone()
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

    // Add parameter struct attributes
    for attribute in &param.attributes {
        output.push_str(&format!("{}\n", attribute));
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
            // Add validation constraints
            if !validation_info.constraints.is_empty() {
                let mut constraints = Vec::new();
                for constraint in &validation_info.constraints {
                    match constraint.constraint_type.as_str() {
                        "ge" => constraints.push(format!("min = {}", constraint.value)),
                        "le" => constraints.push(format!("max = {}", constraint.value)),
                        "gt" => {
                            // For greater than, we need to add a small epsilon
                            if let Ok(val) = constraint.value.parse::<f64>() {
                                constraints.push(format!("min = {}", val + f64::EPSILON));
                            } else {
                                constraints.push(format!("min = {}", constraint.value));
                            }
                        },
                        "lt" => {
                            // For less than, we need to subtract a small epsilon
                            if let Ok(val) = constraint.value.parse::<f64>() {
                                constraints.push(format!("max = {}", val - f64::EPSILON));
                            } else {
                                constraints.push(format!("max = {}", constraint.value));
                            }
                        },
                        _ => {} // Unknown constraint type
                    }
                }
                if !constraints.is_empty() {
                    output.push_str(&format!("    #[validate(range({}))]\n", constraints.join(", ")));
                }
            }

            // Add default value attribute
            if let Some(_default_value) = &validation_info.default_value {
                let function_name = format!("{}_default_{}", to_snake_case(&param.name), property.name);
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

    // Add event attributes
    for attribute in &event.attributes {
        output.push_str(&format!("{}\n", attribute));
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