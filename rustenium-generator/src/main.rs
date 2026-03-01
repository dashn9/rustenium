
use crate::frontend::{load_file};

mod frontend;
mod backend;

pub fn main() {
    let cddl_str = load_file("./raw/cddl/all.cddl");
}