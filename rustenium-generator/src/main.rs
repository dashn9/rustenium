use std::fs;
mod module;
mod extractor;
mod output;
mod command_parser;
mod event_parser;
mod parser;

// mod generated_output;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let all_cddl = fs::read_to_string("raw/all.cddl")?;
    let local_cddl = fs::read_to_string("raw/local.cddl")?;
    let remote_cddl = fs::read_to_string("raw/remote.cddl")?;

    let cddl_strings = vec![all_cddl.as_str(), local_cddl.as_str(), remote_cddl.as_str()];
    let root_protocol = module::detect_root_protocol(cddl_strings.clone())?;
    let modules = module::detect_modules(cddl_strings)?;

    output::generate_output(&modules, &root_protocol)?;

    println!("Generated {} modules to ./output", modules.len());

    Ok(())
}
