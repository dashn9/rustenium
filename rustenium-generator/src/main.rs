
use std::path::Path;

use crate::frontend::{cdp::pdl_to_cdp, load_file};

mod frontend;
mod backend;

pub fn main() {
    // let cddl_str = load_file("./raw/cddl/all.cddl");
    pdl_to_cdp(&[Path::new("./raw/pdl/js_protocol.pdl"), Path::new("./raw/pdl/browser_protocol.pdl")]);
}