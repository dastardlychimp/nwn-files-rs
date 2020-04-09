use std::io;
use io::prelude::*;

use crate::types::{
    Version,
    FileType,
    SsfEntry,
    NULL_U32,
    StaticByteSize,
};

pub trait SerializeSsf {
    fn serialize_to<F: Write>(self, writer: &mut F)
        -> io::Result<()>;
}

impl<C: SerializeSsf> SerializeSsf for Vec<C>
{
    fn serialize_to<F: Write>(self, writer: &mut F)
        -> io::Result<()>
    {
        self
            .into_iter()
            .map(|c| c.serialize_to(writer))
            .collect::<Result<Vec<_>, io::Error>>()?;
        
        Ok(())
    }
}

#[derive(Debug)]
pub struct SsfHeader {
    pub version: Version,
    pub file_type: FileType,
    pub entry_count: u32,
    pub table_offset: u32,
}

impl SerializeSsf for SsfHeader
{
    fn serialize_to<F: Write>(self, writer: &mut F)
        -> io::Result<()>
    {
        writer.write(self.version.as_str_ref().as_bytes())?;
        writer.write(self.file_type.as_str_ref().as_bytes())?;
        writer.write(&self.entry_count.to_le_bytes())?;
        writer.write(&self.table_offset.to_le_bytes())?;
        writer.write(&[0; 24])?;

        Ok(())
    }
}

impl StaticByteSize for SsfHeader
{
    const BYTE_SIZE: usize = 40;
}

impl SerializeSsf for SsfEntry
{
    fn serialize_to<F: Write>(self, writer: &mut F)
        -> io::Result<()>
    {
        writer.write(&self.res_ref.serialize())?;

        let str_ref = self.string_ref
            .unwrap_or(NULL_U32);

        writer.write(&str_ref.to_le_bytes())?;

        Ok(())
    }
}

impl StaticByteSize for SsfEntry
{
    const BYTE_SIZE: usize = 20;
}