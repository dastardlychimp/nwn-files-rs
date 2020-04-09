use super::types::{
    ErfHeader,
    ErfKey,
    Description,
    ErfFile,
};

use crate::types::{
    ResourceType,
    Version,
    FileType,
    Resource,
    ResRef,
    LanguageId,
};
use crate::helpers::conversion::*;

use std::convert::TryFrom;

pub fn parse(bytes: Vec<u8>) -> std::io::Result<ErfFile> {
    let header = parse_header(&bytes);

    dbg!("{:?}", &header);

    let localized_language_strings =
        parse_localized_language_strings(&bytes, &header);

    dbg!("{:?}", &localized_language_strings);

    let key_list = parse_key_list(&bytes, &header);

    dbg!("{:?}", &key_list[0..2]);

    let resources = parse_resources(&bytes, &header, &key_list);

    dbg!("{:?}", &resources[0..2]);

    Ok(ErfFile {
        header: header,
        resources: resources,
    })
}

fn parse_header(bytes: &Vec<u8>) -> ErfHeader {
    ErfHeader {
        version: Version::V1,
        file_type: FileType::Erf,
        language_count: u32_from_bytes(&bytes[8..12]),
        localized_string_size: u32_from_bytes(&bytes[12..16]),
        entry_count: u32_from_bytes(&bytes[16..20]),
        offset_to_localized_string: u32_from_bytes(&bytes[20..24]),
        offset_to_key_list: u32_from_bytes(&bytes[24..28]),
        offset_to_resource_list: u32_from_bytes(&bytes[28..32]),
        build_year: u32_from_bytes(&bytes[32..36]),
        build_day: u32_from_bytes(&bytes[36..40]),
        description_str_ref: u32_from_bytes(&bytes[40..44]),
    }
}

fn parse_localized_language_strings(bytes: &Vec<u8>, header: &ErfHeader)
    -> Vec<Description>
{
    let mut index = header.offset_to_localized_string as usize;

    (0..header.language_count)
        .into_iter()
        .map(|_| {
            let id = u32_from_bytes(&bytes[index..index+4]);
            let size = u32_from_bytes(&bytes[index+4..index+8]);
            let size_usize = size as usize;
            let text = String::from_utf8_lossy(&bytes[index+8..index+8+size_usize]).to_string();

            index += 8 + size_usize;

            Description {
                language_id: LanguageId::from(id),
                text: text,
            }
        })
        .collect()
}

fn parse_key_list(bytes: &Vec<u8>, header: &ErfHeader)
    -> Vec<ErfKey>
{
    let mut index = header.offset_to_key_list as usize;

    (0..header.entry_count)
        .into_iter()
        .map(|_| {
            let key = ErfKey {
                file_name: ResRef::try_from(&bytes[index..index+16]).unwrap(),
                resource_id: u32_from_bytes(&bytes[index+16..index+20]),
                resource_type: ResourceType::from(u16_from_bytes(&bytes[index+20..index+22])),
            };

            index += 24;
            
            key
        })
        .collect()
}

fn parse_resources(bytes: &Vec<u8>, header: &ErfHeader, key_list: &Vec<ErfKey>)
    -> Vec<Resource>
{
    key_list
        .iter()
        .enumerate()
        .map(|(i, key)| {
            let o = i * 8 + header.offset_to_resource_list as usize;
            let offset = u32_from_bytes(&bytes[o..o+4]);
            let size = u32_from_bytes(&bytes[o+4..o+8]);
            let offset_usize = offset as usize;
            let data = Vec::from(&bytes[offset_usize..offset_usize + size as usize]);

            Resource {
                name: key.file_name.clone(),
                resource_type: key.resource_type.clone(),
                data: data,
            }
        })
        .collect()
}