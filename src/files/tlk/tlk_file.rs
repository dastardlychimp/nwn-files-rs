use super::parser;

use crate::types::{
    Version,
    FileType,
    LanguageId,
    Error as MyError,
    StaticByteSize,
    SerializeToBytes,
};

use super::types::{
    TlkHeader,
    TlkEntry,
    TlkData,
};

use std::io::prelude::*;

const ALT_ENTRY: usize = 0x01000000;

#[derive(Debug)]
pub struct TlkFile {
    pub header: Option<TlkHeader>,
    pub entries: Vec<TlkEntry>,
    pub language_id: LanguageId,
    pub entry_count: usize,
    pub alternative: bool,
}

impl TlkFile {
    pub fn new()
        -> Self
    {
        TlkFile {
            header: None,
            entries: Vec::new(),
            language_id: LanguageId::English,
            entry_count: 0,
            alternative: true,
        }
    }
    
    pub fn parse_from<R: BufRead + Seek>(reader: &mut R, alternative: bool)
        -> Result<Self, MyError>
    {
        parser::parse(reader, alternative)
    }


    pub fn next_id(&mut self) -> usize
    {
        if self.alternative {
            self.entry_count + ALT_ENTRY
        } else {
            self.entry_count
        }
    }
    
    pub fn add_entry(&mut self, entry: TlkEntry)
        -> &mut Self
    {
        self.entry_count += 1;
        self.entries.push(entry);
        self
    }

    pub fn add_entries(&mut self, mut entries: Vec<TlkEntry>)
        -> &mut Self
    {
        self.entry_count += entries.len();
        self.entries.append(&mut entries);
        self
    }

    pub fn write<W: Write>(self, writer: &mut W)
        -> Result<(), MyError>
    {
        self.write_header(writer)?;
        self.entries.serialize_to(writer)?;

        Ok(())
    }

    fn write_header<W: Write>(&self, writer: &mut W)
        -> Result<(), MyError>
    {
        let string_count = self.entries.len();
        let string_offset = TlkHeader::BYTE_SIZE + TlkData::BYTE_SIZE * string_count;

        let header = TlkHeader {
            version: Version::V3,
            file_type: FileType::Tlk,
            language_id: self.language_id,
            string_count: string_count,
            string_offset: string_offset,
        };

        header.serialize_to(writer)?;

        Ok(())
    }
}