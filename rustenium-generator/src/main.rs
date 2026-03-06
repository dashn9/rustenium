use std::path::Path;

use crate::frontend::bidi::cddl_to_bidi;
use crate::frontend::bidi::parser::Origin;
use crate::frontend::cdp::pdl_to_cdp;

mod backend;
mod frontend;

pub fn main() {
    let cdp_out_dir =
        std::env::var("CDP_OUT_DIR").expect("CDP_OUT_DIR environment variable must be set");

    let bidi_out_dir =
        std::env::var("BIDI_OUT_DIR").expect("BIDI_OUT_DIR environment variable must be set");

    // pdl_to_cdp(
    //     &[
    //         Path::new("./raw/pdl/js_protocol.pdl"),
    //         Path::new("./raw/pdl/browser_protocol.pdl"),
    //     ],
    //     Path::new(&cdp_out_dir),
    // );

    cddl_to_bidi(
        &[
            (Path::new("./raw/cddl/remote.cddl"), Origin::Remote),
            (Path::new("./raw/cddl/local.cddl"), Origin::Local),
        ],
        Path::new(&bidi_out_dir),
    );
}
