use std::fs;
mod type_generator;
mod definitions;
mod output;
mod command_parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let local_cddl = fs::read_to_string("raw/local.cddl")?;
    let remote_cddl = fs::read_to_string("raw/remote.cddl")?;
    
    let cddl_strings = vec![local_cddl.as_str(), remote_cddl.as_str()];
    let modules = type_generator::detect_modules(cddl_strings)?;
    
    output::generate_output(&modules)?;
    
    println!("Generated {} modules to ./output", modules.len());
    
    Ok(())
}
