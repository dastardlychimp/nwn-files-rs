use std::io::prelude::*;
use std::mem;

use crate::types::{
    Version,
    FileType,
    LanguageId,
    ResRef,
    StaticByteSize,
    SerializeToBytes,
    Error as MyError,
};

#[derive(Debug)]
pub struct TlkHeader {
    pub file_type: FileType,
    pub version: Version,
    pub language_id: LanguageId,
    pub string_count: usize,
    pub string_offset: usize,
}

impl StaticByteSize for TlkHeader {
    const BYTE_SIZE: usize = 20;
}

impl SerializeToBytes for TlkHeader {
    fn serialize_to<F: Write>(self, writer: &mut F)
        -> Result<(), MyError>
    {
        writer.write(self.file_type.as_str_ref().as_bytes())?;
        writer.write(self.version.as_str_ref().as_bytes())?;
        writer.write(&(self.language_id as u32).to_le_bytes())?;
        writer.write(&(self.string_count as u32).to_le_bytes())?;
        writer.write(&(self.string_offset as u32).to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum TlkFlags {
    TextPresent =  1,
    SoundPresent = 2,
    SoundLengthPresent = 4,
}

impl TlkFlags {
    fn u8_from_tlk_entry(entry: &TlkEntry) -> u8
    {
        let mut flags = 0;

        if ! entry.string.is_empty() {
            flags += TlkFlags::TextPresent as u8;
        }

        if entry.sound.is_some() {
            flags += TlkFlags::SoundPresent as u8;

            if entry.sound.as_ref().unwrap().sound_length.is_some() {
                flags += TlkFlags::SoundLengthPresent as u8;
            }
        }

        flags
    }

}
#[derive(Debug)]
pub struct TlkData {
    pub flags: u8,
    pub sound_res_ref: ResRef,
    pub offset_to_string: usize,
    pub string_size: usize,
    pub sound_length: f32,
}

impl SerializeToBytes for TlkData {
    fn serialize_to<F: Write>(self, writer: &mut F)
        -> Result<(), MyError>
    {
        writer.write(&(self.flags as u32).to_le_bytes())?;
        writer.write(&self.sound_res_ref.serialize())?;
        writer.write(&[0; 8])?;
        writer.write(&(self.offset_to_string as u32).to_le_bytes())?;
        writer.write(&(self.string_size as u32).to_le_bytes())?;
        writer.write(&self.sound_length.to_le_bytes())?; 

        Ok(())
    }
}

impl StaticByteSize for TlkData {
    const BYTE_SIZE: usize = 40;
}

#[derive(Debug)]
pub struct TlkFile {
    pub entries: Vec<TlkEntry>,
}

#[derive(Debug, Clone)]
pub struct TlkSound {
    pub res_ref: ResRef,
    pub sound_length: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct TlkEntry {
    pub string: String,
    pub sound: Option<TlkSound>
}

impl SerializeToBytes for Vec<TlkEntry>
{
    fn serialize_to<F: Write>(mut self, writer: &mut F)
        -> Result<(), MyError>
    {
        let string_data_list = self
            .iter_mut()
            .scan(0, |state, entry| {
                let flags = TlkFlags::u8_from_tlk_entry(&entry);
                let offset_to_string = *state;
                let string_size = entry.string.len();

                let sound = mem::replace(&mut entry.sound, None);
                
                let sound_length = sound
                    .as_ref()
                    .and_then(|s| s.sound_length)
                    .unwrap_or(0.0);

                let sound_res_ref = sound
                    .map(|s| s.res_ref)
                    .unwrap_or(ResRef::default());
                

                *state = *state + string_size;

                Some(TlkData {
                    flags,
                    sound_res_ref,
                    string_size,
                    offset_to_string,
                    sound_length,
                })
            })
            .collect::<Vec<TlkData>>();

        string_data_list.serialize_to(writer)?;

        for entry in self {
            write!(writer, "{}", entry.string)?;
        }
            
        Ok(())
    }
}