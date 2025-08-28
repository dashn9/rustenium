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
        let mut command_content = String::new();
        
        // Generate enum from pre-parsed command definition
        command_content.push_str(&generate_command_enum(cmd_def));
        
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