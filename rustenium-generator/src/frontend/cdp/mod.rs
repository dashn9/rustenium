use std::{
    fs, io::{Error, ErrorKind}, ops::Deref, path::{Path, PathBuf}
};

use parser::parse_pdl;

use crate::frontend::cdp::resolver::resolve_pdl;

mod dep;
mod error;
pub mod parser;
pub mod resolver;

pub fn pdl_to_cdp(pdl_locations: &[PathBuf]) {
    let mut protocols = vec![];

    let mut inputs = vec![];
    for (idx, pdl_location) in pdl_locations.iter().enumerate() {
        let content = fs::read_to_string(pdl_location)?;
        let resolved = resolve_pdl(pdl_location, &content)
            .map_err(|e| Error::new(ErrorKind::Other, e.message))?;
        let protocol =
            parse_pdl(&resolved).map_err(|e| Error::new(ErrorKind::Other, e.message))?;

        protocols.push(protocol);
    }
}
