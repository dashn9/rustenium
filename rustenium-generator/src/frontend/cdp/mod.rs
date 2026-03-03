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
    let inputs: Vec<String> = pdl_locations
        .iter()
        .map(|pdl_location| {
            let content = fs::read_to_string(pdl_location).unwrap();
            resolve_pdl(pdl_location, &content).unwrap()
        })
        .collect();

    let protocols: Vec<_> = inputs.iter().map(|input| parse_pdl(input).unwrap()).collect();
    compile_protocols(&protocols).unwrap();
}
