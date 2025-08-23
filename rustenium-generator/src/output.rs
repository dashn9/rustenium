use std::fs;
use std::path::Path;
use crate::type_generator::Module;
use crate::command_parser::{parse_command_definition, ParsedCommandDefinition, ParsedCommandDefinitionCommand};

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
        let mut command_content = String::new();
        
        // Parse command definition and generate enum
        match parse_command_definition(cmd_def) {
            Ok(parsed_cmd) => {
                command_content.push_str(&generate_command_enum(&parsed_cmd));
            }
            Err(_) => {
                // Fallback to original content if parsing fails
                command_content.push_str(&cmd_def.content);
            }
        }
        
        // Add result definition to commands if it exists
        if let Some(ref result_def) = module.result_definition {
            command_content.push_str("\n\n");
            command_content.push_str(&result_def.content);
        }
        
        fs::write(format!("{}/commands.rs", module_dir), command_content)?;
        mod_content.push_str("pub mod commands;\n");
    }
    
    // Create event file if definition exists
    if let Some(ref event_def) = module.event_definition {
        fs::write(format!("{}/events.rs", module_dir), &event_def.content)?;
        mod_content.push_str("pub mod events;\n");
    }
    
    // Create types file (always present)
    fs::write(format!("{}/types.rs", module_dir), "// Module types\n")?;
    mod_content.push_str("pub mod types;\n");
    
    // Write mod.rs
    fs::write(format!("{}/mod.rs", module_dir), mod_content)?;
    
    Ok(())
}

fn generate_command_enum(parsed_cmd: &ParsedCommandDefinition) -> String {
    let mut output = String::new();
    
    // Add attributes
    for attribute in &parsed_cmd.attributes {
        output.push_str(&format!("{}\n", attribute));
    }
    
    // Add enum declaration
    output.push_str(&format!("pub enum {} {{\n", parsed_cmd.value));
    
    // Add enum variants
    for command in &parsed_cmd.commands {
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