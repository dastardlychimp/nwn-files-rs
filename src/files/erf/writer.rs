use std::io;
use io::prelude::*;
use io::BufWriter;
use std::mem;
use crate::helpers::date;
use crate::types::{
    Error as MyError,
    Version,
    FileType,
    NULL_U32,
    StaticByteSize,
    SerializeToBytes,
};

use super::types::{
    ErfFile,
    ErfHeader,
    ErfKey,
    ErfResourceListItem,
};


pub fn write<W: Write>(erf_file: &mut ErfFile, writer: &mut W, file_type: FileType)
    -> Result<(), MyError>
{
    let descriptions = mem::replace(&mut erf_file.descriptions, Vec::new());
    let resources = mem::replace(&mut erf_file.resources, Vec::new());
    
    let entry_count = resources.len();
    let description_count = descriptions.len();
    let key_list_size = ErfKey::BYTE_SIZE * entry_count;
    let resource_list_size =  ErfResourceListItem::BYTE_SIZE * entry_count;
    let header_size = ErfHeader::BYTE_SIZE;
    let language_size = descriptions
        .iter()
        .fold(0, |count, description| count + description.byte_size());
    

    let offset_to_key_list = header_size + language_size;
    let offset_to_resource_list = offset_to_key_list + key_list_size;
    let offset_to_resources = offset_to_resource_list + resource_list_size;

    let header = ErfHeader {
        version: Version::V1,
        file_type: file_type,
        language_count: description_count as u32,
        localized_string_size: language_size as u32,
        entry_count: entry_count as u32,
        offset_to_localized_string: header_size as u32,
        offset_to_key_list: offset_to_key_list as u32,
        offset_to_resource_list: offset_to_resource_list as u32,
        build_day: date::days_since_jan_1(),
        build_year: date::years_since_1990(),
        description_str_ref: NULL_U32,
    };

    let key_list = resources
        .iter()
        .enumerate()
        .map(|(i, resource)| {
            ErfKey {
                file_name: resource.name.clone(),
                resource_id: i as u32,
                resource_type: resource.resource_type.clone(),
            }
        })
        .collect::<Vec<ErfKey>>();

    let resource_list = resources
        .iter()
        .scan(offset_to_resources, |state, resource| {                
            let resource_size = resource.data.len();

            let item = ErfResourceListItem {
                offset: *state as u32,
                size: resource_size as u32,
            };

            *state = *state + resource_size;

            Some(item)
        })
        .collect::<Vec<ErfResourceListItem>>();

    let mut writer = BufWriter::new(writer);

    header.serialize_to(&mut writer)?;
    descriptions.serialize_to(&mut writer)?;
    key_list.serialize_to(&mut writer)?;
    resource_list.serialize_to(&mut writer)?;

    resources
        .iter()
        .map(|resource| {
            writer.write(&resource.data)
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    writer.flush()?;

    Ok(())
}
