use std::{
    fs, path::{Path}
};

use parser::parse_pdl;

use crate::{backend::generator::compile_protocols, frontend::cdp::resolver::resolve_pdl};

mod dep;
mod error;
pub mod parser;
pub mod resolver;

pub fn pdl_to_cdp(pdl_locations: &[&Path]) {
    let mut protocols = vec![];

    for (idx, pdl_location) in pdl_locations.iter().enumerate() {
        let content = fs::read_to_string(pdl_location).unwrap();
        let resolved = resolve_pdl(pdl_location, &content).unwrap();
        let protocol = parse_pdl(resolved).unwrap();

        protocols.push(protocol);
    }
    compile_protocols(&protocols);
}
