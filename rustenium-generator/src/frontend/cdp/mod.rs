use crate::frontend::cdp::parser::parse_pdl;

mod dep;
mod error;
pub mod parser;
pub mod resolver;

pub fn pdl_to_cdp<P: AsRef<Path>>(pdl_locations: &[P]) {
    let mut protocols = vec![];

    for (idx, pdl_location) in pdl_locations.iter().enumerate() {
        let pdl = parse_pdl(pdl_location).map_err(|e| Error::new(ErrorKind::Other, e.message))?;

        self.domains
            .extend(pdl.domains.iter().map(|d| (d.name.to_string(), idx)));

        // store enum types
        self.enums.extend(
            pdl.domains
                .iter()
                .flat_map(|d| d.types.iter().filter(|d| d.is_enum()))
                .map(|e| e.raw_name.to_string()),
        );

        protocols.push(pdl);
    }

    compile
}
