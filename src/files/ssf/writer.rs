use super::types::{ SsfHeader, SerializeSsf };

use crate::types::{
    SsfEntry,
    Version,
    FileType,
    Error as MyError,
    StaticByteSize,
};


use std::io::{Write, BufWriter};


pub struct SsfBuilder {
    entries: Vec<SsfEntry>,
}

impl SsfBuilder {
    pub fn new() -> Self
    {
        SsfBuilder {
            entries: Vec::new()
        }
    }

    pub fn add_entry(&mut self, entry: SsfEntry)
        -> &mut Self
    {
        self.entries.push(entry);
        self
    }

    pub fn write<W: Write>(self, writer: &mut W)
        -> Result<(), MyError>
    {
        let entry_count = self.entries.len();
        
        let entry_size = entry_count * 4;
        
        let table_offset = SsfHeader::BYTE_SIZE;
        let data_offset = table_offset + entry_size;
        
        let header = SsfHeader {
            version: Version::V1,
            file_type: FileType::Ssf,
            entry_count: entry_count as u32,
            table_offset: table_offset as u32,
        };
        
        let entry_table = (0..entry_count)
            .into_iter()
            .map(|i| (data_offset + i * SsfEntry::BYTE_SIZE) as u32);
            
        
        let mut writer = BufWriter::new(writer);

        header.serialize_to(&mut writer)?;
        entry_table
            .map(|num| {
                writer.write(&num.to_le_bytes())
            })
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        self.entries.serialize_to(&mut writer)?;

        writer.flush()?;
            
        Ok(())
    }
}