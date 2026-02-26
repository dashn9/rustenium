use pdl_compiler::{ast::SourceDatabase, parser::parse_file};

pub fn parse_pdl_source(pdl_source_loc: String) {
    let mut pdl_files = SourceDatabase::new();
    let pdl = parse_file(&mut pdl_files, &pdl_source_loc).unwrap();
    for pdl_declaration in pdl.declarations {
        println!("{:?}", pdl_declaration);
    }
}