use std::fs;
use std::path::Path;
use crate::module::Module;

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
        fs::write(format!("{}/events.rs", module_dir), &event_def.content)?;
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

fn generate_commands_file(cmd_def: &crate::command_parser::CommandDefinition, module: &Module) -> String {
    let mut output = String::new();
    
    // Add header comment
    output.push_str("// Generated commands for module\n\n");
    
    // Generate the main command enum first
    output.push_str(&generate_command_enum(cmd_def));
    output.push_str("\n\n");
    
    // Generate individual command structs
    for command in &cmd_def.commands {
        output.push_str(&generate_command_struct(command));
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
        
        // Add field
        let field_type = if property.is_optional {
            format!("Option<{}>", property.value)
        } else {
            property.value.clone()
        };
        
        output.push_str(&format!("    pub {}: {},\n", property.name, field_type));
    }
    
    output.push_str("}");
    output
}

fn generate_types_file(module: &Module) -> String {
    let mut output = String::new();
    
    // Add header comment
    output.push_str("// Generated types for module\n\n");
    
    // Generate each type
    for bidi_type in &module.types {
        output.push_str(&generate_rust_struct(&bidi_type.name, &bidi_type.properties));
        output.push_str("\n\n");
    }
    
    // If no types, add placeholder comment
    if module.types.is_empty() {
        output.push_str("// No types generated for this module\n");
    }
    
    output
}

fn generate_rust_struct(name: &str, properties: &[crate::parser::Property]) -> String {
    let mut output = String::new();
    
    // Add derive attributes
    output.push_str("#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]\n");
    output.push_str(&format!("pub struct {} {{\n", name));
    
    // Add properties
    for property in properties {
        let field_type = if property.is_optional {
            format!("Option<{}>", property.value)
        } else {
            property.value.clone()
        };
        
        output.push_str(&format!("    pub {}: {},\n", property.name, field_type));
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
        // Add variant attributes if any
        for attribute in &command.attributes {
            output.push_str(&format!("    {}\n", attribute));
        }
        
        // Add variant - use command name as both variant and type like Close(Close)
        output.push_str(&format!("    {}({}),\n", command.name, command.name));
    }
    
    output.push_str("}\n");
    output
}