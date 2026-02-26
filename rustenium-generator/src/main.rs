
use crate::frontend::{bidi::parser::parse_cddl_str, cdp::parser::parse_pdl_source, load_file};

mod frontend;
mod backend;

pub fn main() {
    let cddl_str = load_file("./raw/cddl/all.cddl");
    // parse_cddl_str(&cddl_str);
    parse_pdl_source(String::from("./raw/pdl/browser_protocol.pdl"));
}