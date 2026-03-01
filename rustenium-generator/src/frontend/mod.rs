pub mod bidi;
pub mod cdp;

use std::{fs::File, io::Read};

pub fn load_file(url: &str) -> String {
    let mut file = File::open(&url).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    file_contents
}