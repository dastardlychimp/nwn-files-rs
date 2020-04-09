use crate::types::{
    Error as MyError,
    Version,
    FileType,
    LanguageId,
    StaticByteSize,
    SerializeToBytes,
};

use super::types::{
    TlkEntry,
    TlkHeader,
    TlkData,
};

use std::io::{Write};

pub struct TlkBuilder {
    entries: Vec<TlkEntry>,
    language_id: LanguageId,
}

impl TlkBuilder {
    pub fn new() -> Self
    {
        TlkBuilder {
            language_id: LanguageId::English,
            entries: Vec::new(),
        }
    }
    
    pub fn add_entry(&mut self, entry: TlkEntry)
        -> &mut Self
    {
        self.entries.push(entry);
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