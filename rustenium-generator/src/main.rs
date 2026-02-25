
use crate::frontend::{bidi::parser::parse_cddl_str, load_file};

mod frontend;
mod backend;

pub fn main() {
    let cddl_str = load_file("./raw/all.cddl");
    parse_cddl_str(&cddl_str);
}