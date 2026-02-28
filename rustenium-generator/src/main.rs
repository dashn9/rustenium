
use crate::frontend::{bidi::parser::parse_cddl_str, load_file};

mod frontend;
mod backend;

pub fn main() {
    let cddl_str = load_file("./raw/cddl/all.cddl");
    parse_pdl_source(String::from("./raw/pdl/browser_protocol.pdl"));
}