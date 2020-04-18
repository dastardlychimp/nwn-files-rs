use std::io::prelude::*;

use super::parser;

use super::types::{
    SsfHeader,
    SsfEntry,
};

use crate::types::{
    Error as MyError,
};

#[derive(Debug)]
pub struct SsfFile {
    pub header: Option<SsfHeader>,
    pub entries: Vec<SsfEntry>,
}

impl SsfFile {
    pub fn parse_from<R: Read>(reader: &mut R)
        -> Result<Self, MyError>
    {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        parser::parse(bytes)
    }
}