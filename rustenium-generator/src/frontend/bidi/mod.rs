use std::{fs, path::Path};

use crate::backend::base_types::DomainDirection;

pub mod parser;

pub fn cddl_to_bidi(cddl_locations: &[(&Path, DomainDirection)], out_dir: &Path) {
    let inputs: Vec<(String, DomainDirection)> = cddl_locations
        .iter()
        .map(|(p, dir)| {
            let content = fs::read_to_string(p).unwrap();
            (content, *dir)
        })
        .collect();

    let input_refs: Vec<(&str, DomainDirection)> = inputs
        .iter()
        .map(|(s, dir)| (s.as_str(), *dir))
        .collect();

    let protocol = parser::parse_cddl(&input_refs);

    let mut generator = crate::backend::generator::Generator::default();
    generator.out_dir(out_dir);
    generator.compile_protocols(&[protocol]).unwrap();
}
