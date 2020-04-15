pub use super::erf_file::ErfFile;

use std::io;
use io::prelude::*;

use crate::types::{
    Version,
    FileType,
    ResourceType,
    ResRef,
    LanguageId,
    Resource,
    StaticByteSize,
    SerializeToBytes,
    Error as MyError,
};

#[derive(Debug)]
pub struct ErfHeader {
    pub version: Version,
    pub file_type: FileType,
    pub language_count: u32,
    pub localized_string_size: u32,
    pub entry_count: u32,
    pub offset_to_localized_string: u32,
    pub offset_to_key_list: u32,
    pub offset_to_resource_list: u32,
    pub build_year: u32,
    pub build_day: u32,
    pub description_str_ref: u32,    
}

impl StaticByteSize for ErfHeader {
    const BYTE_SIZE: usize = 160;
}

impl SerializeToBytes for ErfHeader {
    fn serialize_to<F: Write>(self, writer: &mut F)
        -> Result<(), MyError>
    {
        writer.write(self.file_type.as_str_ref().as_bytes())?;
        writer.write(self.version.as_str_ref().as_bytes())?;
        writer.write(&self.language_count.to_le_bytes())?;
        writer.write(&self.localized_string_size.to_le_bytes())?;
        writer.write(&self.entry_count.to_le_bytes())?;
        writer.write(&self.offset_to_localized_string.to_le_bytes())?;
        writer.write(&self.offset_to_key_list.to_le_bytes())?;
        writer.write(&self.offset_to_resource_list.to_le_bytes())?;
        writer.write(&self.build_year.to_le_bytes())?;
        writer.write(&self.build_day.to_le_bytes())?;
        writer.write(&self.description_str_ref.to_le_bytes())?;
        writer.write(&[0; 116])?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ErfDescription {
    pub language_id: LanguageId,
    pub text: String,
}

impl ErfDescription {
    #[inline]
    pub fn byte_size(&self)
        -> usize
    {
        self.text.len() + 8
    }
}

impl SerializeToBytes for ErfDescription {
    fn serialize_to<F: Write>(self, writer: &mut F)
        -> Result<(), MyError>
    {
        let size = self.text.len() as u32;
        let id = self.language_id as u32 * 2;
        // TODO: language_id needs to be increased by 1 if it is female based.

        writer.write(&size.to_le_bytes())?;
        writer.write(&id.to_le_bytes())?;
        writer.write(&self.text.into_bytes())?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ErfKey {
    pub file_name: ResRef,
    pub resource_id: u32,
    pub resource_type: ResourceType,
}

impl StaticByteSize for ErfKey {
    const BYTE_SIZE: usize = 24;
}

impl SerializeToBytes for ErfKey {
    fn serialize_to<F: Write>(self, writer: &mut F)
        -> Result<(), MyError>
    {
        let ErfKey { file_name, resource_id, resource_type } = self;

        writer.write(&file_name.serialize())?;
        writer.write(&resource_id.to_le_bytes())?;
        writer.write(&(resource_type as u16).to_le_bytes())?;
        writer.write(&[0; 2])?;

        Ok(())
    }    
}

#[derive(Debug)]
pub struct ErfResourceListItem {
    pub offset: u32,
    pub size: u32,
}

impl StaticByteSize for ErfResourceListItem {
    const BYTE_SIZE: usize = 8;
}

impl SerializeToBytes for ErfResourceListItem {
    fn serialize_to<F: Write>(self, writer: &mut F)
        -> Result<(), MyError>
    {
        writer.write(&self.offset.to_le_bytes())?;
        writer.write(&self.size.to_le_bytes())?;
        
        Ok(())
    }    
}

