use std::{fs, path::Path};

use parser::parse_pdl;

use crate::{backend::generator::compile_protocols, frontend::cdp::resolver::resolve_pdl};

mod dep;
mod error;
pub mod parser;
pub mod resolver;

pub fn pdl_to_cdp(pdl_locations: &[&Path]) {
    let inputs: Vec<(String, Option<&str>)> = pdl_locations
        .iter()
        .map(|pdl_location| {
            let file_name = pdl_location
                .file_stem()
                .unwrap_or_else(|| {
                    panic!("Failed to read file name for {}", pdl_location.display())
                })
                .to_str();
            let content = fs::read_to_string(pdl_location).unwrap();
            (resolve_pdl(pdl_location, &content).unwrap(), file_name)
        })
        .collect();

    let protocols: Vec<_> = inputs
        .iter()
        .map(|input| {
            let mut protocol = parse_pdl(&input.0).unwrap();
            protocol.name = input.1;
            protocol
        })
        .collect();
    compile_protocols(&protocols).unwrap();
}
