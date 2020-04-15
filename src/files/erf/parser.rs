use std::io::prelude::*;

use super::types::{
    ErfHeader,
    ErfKey,
    ErfDescription,
    ErfResourceListItem,
};

use crate::types::{
    ResourceType,
    Version,
    FileType,
    Resource,
    ResRef,
    LanguageId,
    StaticByteSize,
    Error as MyError,
};
use crate::helpers::reader::ReaderExt;

use std::convert::TryFrom;

pub fn parse_header<R>(reader: &mut R)
    -> Result<ErfHeader, MyError>
    where R: BufRead + Seek
{
    let file_type = FileType::from(reader.read_bytes_to_string(4)?.as_str());
    let version = Version::from(reader.read_bytes_to_string(4)?.as_str());
    let language_count = reader.read_u32()?;
    let localized_string_size = reader.read_u32()?;
    let entry_count = reader.read_u32()?;
    let offset_to_localized_string = reader.read_u32()?;
    let offset_to_key_list = reader.read_u32()?;
    let offset_to_resource_list = reader.read_u32()?;
    let build_year = reader.read_u32()?;
    let build_day = reader.read_u32()?;
    let description_str_ref = reader.read_u32()?;

    reader.seek_from_current(116)?;

    Ok(ErfHeader {
        version,
        file_type,
        language_count,
        localized_string_size,
        entry_count,
        offset_to_localized_string,
        offset_to_key_list,
        offset_to_resource_list,
        build_year,
        build_day,
        description_str_ref,
    })
}

pub fn parse_localized_language_strings<R>(
    reader: &mut R,
    header: &ErfHeader
)
    -> Result<Vec<ErfDescription>, MyError>
    where R: BufRead + Seek
{
    reader.seek_from_start(header.offset_to_localized_string as u64)?;

    (0..header.language_count)
        .into_iter()
        .map(|_| {
            let id = reader.read_u32()?;
            let size = reader.read_u32()?;
            let text = reader.read_bytes_to_string(size as usize)?;

            Ok(ErfDescription {
                language_id: LanguageId::from(id),
                text: text,
            })
        })
        .collect::<Result<Vec<_>, MyError>>()
}

pub fn parse_key_list<R>(reader: &mut R, header: &ErfHeader)
-> Result<Vec<ErfKey>, MyError>
    where R: BufRead + Seek
{
    reader.seek_from_start(header.offset_to_key_list as u64)?;

    (0..header.entry_count)
        .into_iter()
        .map(|_| {
            let file_name = reader.read_bytes(16)?;
            let file_name = ResRef::try_from(file_name.as_slice())?;

            let resource_id = reader.read_u32()?;
            let resource_type = ResourceType::from(reader.read_u16()?);

            reader.seek_from_current(2)?;
            
            Ok(ErfKey {
                file_name,
                resource_id,
                resource_type,
            })
        })
        .collect::<Result<Vec<_>, MyError>>()
}

pub fn parse_resource_list_items<R>(reader: &mut R, header: &ErfHeader)
    -> Result<Vec<ErfResourceListItem>, MyError>
    where R: BufRead + Seek
{
    reader.seek_from_start(header.offset_to_resource_list as u64)?;
    
    (0..header.entry_count)
        .into_iter()
        .map(|_| {
            let offset = reader.read_u32()?;
            let size = reader.read_u32()?;

            Ok(ErfResourceListItem {
                offset,
                size,
            })
        })
        .collect::<Result<Vec<_>, MyError>>()
}

pub fn parse_resources<R>(
    reader: &mut R,
    key_list: Vec<ErfKey>,
    resource_list_items: &Vec<ErfResourceListItem>
)
    -> Result<Vec<Resource>, MyError>
    where R: BufRead + Seek
{        
    key_list
        .into_iter()
        .zip(resource_list_items)
        .map(|(key, rli)| {
            reader.seek_from_start(rli.offset as u64)?;
            
            let ErfKey {
                file_name,
                resource_type,
                ..
            } = key;

            let data = reader.read_bytes(rli.size as usize)?;

            Ok(Resource {
                name: file_name,
                resource_type: resource_type,
                data: data,
            })
        })
        .collect::<Result<Vec<_>, MyError>>()
}