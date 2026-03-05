use std::path::Path;

use crate::frontend::cdp::pdl_to_cdp;

mod frontend;
mod backend;

pub fn main() {
    let cdp_out_dir = std::env::var("CDP_OUT_DIR")
        .expect("CDP_OUT_DIR environment variable must be set");
    
    let bidi_out_dir = std::env::var("BIDI_OUT_DIR")
        .expect("BIDI_OUT_DIR environment variable must be set");

    cddl_to_bidi(
        &[
            Path::new("./.pdl"),
            Path::new("./raw/pdl/browser_protocol.pdl"),
        ],
        Path::new(&cdp_out_dir),
    );

    pdl_to_cdp(
        &[
            Path::new("./raw/pdl/js_protocol.pdl"),
            Path::new("./raw/pdl/browser_protocol.pdl"),
        ],
        Path::new(&cdp_out_dir),
    );
}