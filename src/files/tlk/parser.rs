use std::io;
use io::{BufRead, Seek};
use std::convert::TryFrom;

use crate::types::{
    Error as MyError,
    FileType,
    LanguageId,
    Version,
    ResRef,
};

use super::tlk_file::TlkFile;

use super::types::{
    TlkHeader,
    TlkData,
    TlkFlags,
    TlkSound,
    TlkEntry,
};

use crate::helpers::reader::ReaderExt;

pub fn parse<R>(reader: &mut R, alternative: bool)
    -> Result<TlkFile, MyError>
    where R: BufRead + Seek
{
    let header = parse_header(reader)?;

    let string_data_table = parse_string_data_table(reader, &header)?;
        
    let strings = parse_strings(reader, &header, &string_data_table)?;
    
    let tlk_entries = string_data_table
        .iter()
        .zip(strings)
        .map(|(dt, string)| {
            let flag_sound = TlkFlags::SoundPresent as u8;
            let flag_sound_length = TlkFlags::SoundLengthPresent as u8;
            let sound_present = dt.flags & flag_sound == flag_sound;
            let sound_length_present = dt.flags & flag_sound_length == flag_sound_length;

            let sound = if sound_present {
                let sound_length = if sound_length_present {
                    Some(dt.sound_length)
                } else {
                    None
                };

                Some(TlkSound {
                    res_ref: dt.sound_res_ref.clone(),
                    sound_length: sound_length,
                })
            } else {
                None
            };
            
            TlkEntry {
                string,
                sound,
            }
        })
        .collect::<Vec<TlkEntry>>();

    
    Ok(TlkFile {
        entry_count:  tlk_entries.len(),
        language_id: header.language_id.clone(),
        header: Some(header),
        entries: tlk_entries,
        alternative: alternative,
    })
}

fn parse_header<R>(reader: &mut R)
    -> Result<TlkHeader, io::Error>
    where R: BufRead + Seek
{
    let file_type = FileType::from(reader.read_bytes_to_string(4)?.as_str());
    let version = Version::from(reader.read_bytes_to_string(4)?.as_str());
    let language_id = LanguageId::from(reader.read_u32()?);
    let string_count = reader.read_u32()? as usize;
    let string_offset = reader.read_u32()? as usize;
    
    Ok(TlkHeader {
        file_type,
        version,
        language_id,
        string_count,
        string_offset,
    })
}

fn parse_string_data_table<R>(reader: &mut R, header: &TlkHeader)
    -> Result<Vec<TlkData>, MyError>
    where R: BufRead + Seek
{
    let data = (0..header.string_count)
        .map(|_| {
            let flags = reader.read_u32()? as u8;
            let sound_res_ref = ResRef::try_from(reader.read_bytes_to_string(16)?)?;
            reader.seek_from_current(8)?;
            let offset_to_string = reader.read_u32()? as usize;
            let string_size = reader.read_u32()? as usize;
            let sound_length = reader.read_f32()?;

            Ok(TlkData {
                flags,
                sound_res_ref,
                offset_to_string,
                string_size,
                sound_length,
            })
        })
        .collect::<Result<Vec<_>, MyError>>()?;

    Ok(data)
}

fn parse_strings<R>(reader: &mut R, header: &TlkHeader, data_table: &Vec<TlkData>)
    -> Result<Vec<String>, MyError>
    where R: BufRead + Seek
{

    let data = data_table
        .iter()
        .map(|dt| {
            let offset = header.string_offset + dt.offset_to_string;
            reader.seek_from_start(offset as u64)?;
            reader.read_bytes_to_string(dt.string_size)
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(data)
}