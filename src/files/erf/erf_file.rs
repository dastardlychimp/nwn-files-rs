use std::io::prelude::*;
use std::io::BufReader;

use super::types::*;

use crate::types::{
    Resource,
    Error as MyError,
};

use super::parser::*;

#[derive(Debug)]
pub struct ErfFile {
    pub header: Option<ErfHeader>,
    pub descriptions: Vec<ErfDescription>,
    pub resources: Vec<Resource>,
}

impl ErfFile {
    pub fn parse_from<R: Read>(reader: &mut R)
        -> Result<Self, MyError>
    {
        // let mut reader = BufReader::new(reader);
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes);

        let header = parse_header(&mut bytes);

        // dbg!("{:?}", &header);
    
        let localized_language_strings =
            parse_localized_language_strings(&mut bytes, &header);
    
        // dbg!("{:?}", &localized_language_strings);
    
        let key_list = parse_key_list(&mut bytes, &header);
    
        // dbg!("{:?}", &key_list[0..2]);
    
        let resources = parse_resources(&mut bytes, &header, &key_list);
    
        // dbg!("{:?}", &resources[0..2]);
    
        Ok(ErfFile {
            header: Some(header),
            descriptions: localized_language_strings,
            resources: resources,
        })
    }
}