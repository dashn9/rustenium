use std::{fs, path::Path};

use parser::Origin;

pub mod parser;

pub fn cddl_to_bidi(cddl_locations: &[(&Path, Origin)], out_dir: &Path) {
    let inputs: Vec<(String, Origin)> = cddl_locations
        .iter()
        .map(|(p, origin)| {
            let content = fs::read_to_string(p).unwrap();
            (content, *origin)
        })
        .collect();

    let input_refs: Vec<(&str, Origin)> = inputs
        .iter()
        .map(|(s, origin)| (s.as_str(), *origin))
        .collect();

    let protocol = parser::parse_cddl(&input_refs);

    let mut generator = crate::backend::generator::Generator::default();
    generator.out_dir(out_dir);
    generator.compile_protocols(&[protocol]).unwrap();
}
