use std::path::Path;

use crate::frontend::cdp::pdl_to_cdp;

mod frontend;
mod backend;

pub fn main() {
    let cdp_out_dir = std::env::var("CDP_OUT_DIR")
        .expect("CDP_OUT_DIR environment variable must be set");

    pdl_to_cdp(
        &[
            Path::new("./raw/pdl/js_protocol.pdl"),
            Path::new("./raw/pdl/browser_protocol.pdl"),
        ],
        Path::new(&cdp_out_dir),
    );
}