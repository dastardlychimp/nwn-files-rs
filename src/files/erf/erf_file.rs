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
    pub fn parse_from<R: Read + Seek>(reader: &mut R)
        -> Result<Self, MyError>
    {
        // let mut reader = BufReader::new(reader);
        let mut reader = BufReader::new(reader);

        let header = parse_header(&mut reader)?;

        // dbg!("{:?}", &header);
    
        let localized_language_strings =
            parse_localized_language_strings(&mut reader, &header)?;
    
        // dbg!("{:?}", &localized_language_strings);
    
        let key_list = parse_key_list(&mut reader, &header)?;
    
        // dbg!("{:?}", &key_list[0..2]);

        let resource_list_items = parse_resource_list_items(&mut reader, &header)?;

    
        let resources = parse_resources(&mut reader, key_list, &resource_list_items)?;
    
        // dbg!("{:?}", &resources[0..2]);
    
        Ok(ErfFile {
            header: Some(header),
            descriptions: localized_language_strings,
            resources: resources,
        })
    }
}