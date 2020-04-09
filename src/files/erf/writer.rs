use super::types as erf_file_parts;
use erf_file_parts::SerializeErf;

use std::path::Path;
use std::fs::File;
use std::io;
use io::prelude::*;
use io::BufWriter;
use crate::helpers::date;
use crate::types::{
    Resource,
    Error as MyError,
    LanguageId,
    Version,
    FileType,
    NULL_U32,
};

#[derive(Debug)]
pub struct ErfFileBuilder {
    resources: Vec<Resource>,
    descriptions: Vec<erf_file_parts::Description>,
}

impl ErfFileBuilder {
    pub fn new()
        -> Self
    {
        ErfFileBuilder {
            resources: Vec::new(),
            descriptions: Vec::new(),
        }
    }

    pub fn add_description(mut self, language_id: LanguageId, text: String)
        -> Self
    {
        self.descriptions.push(erf_file_parts::Description {
            language_id: language_id,
            text: text,
        });
        self
    }

    pub fn add_resource(mut self, resource: Resource)
        -> Self
    {
        self.resources.push(resource);
        self
    }

    pub fn add_resources(mut self, resources: &mut Vec<Resource>)
        -> Self
    {
        self.resources.append(resources);
        self
    }
    
    pub fn write<P: AsRef<Path>>(self, file_path: P, file_type: FileType)
        -> Result<(), MyError>
    {
        let file_path = file_path.as_ref();
        // if file_path.exists()
        // {
        //     return Err(MyError::PathAlreadyExists(
        //         file_path
        //             .to_string_lossy()
        //             .to_string()
        //     ));
        // }

        let valid_file_types = [FileType::Erf, FileType::Hak, FileType::Mod, FileType::Sav];
        if ! valid_file_types.contains(&file_type)
        {
            return Err(MyError::InvalidFileTypeForErf(file_type));
        }

        let ErfFileBuilder {resources, descriptions} = self;
        
        let entry_count = resources.len();
        let description_count = descriptions.len();
        let key_list_size = erf_file_parts::ErfKey::BYTE_SIZE.unwrap() * entry_count;
        let resource_list_size =  erf_file_parts::ResourceListItem::BYTE_SIZE.unwrap() * entry_count;
        let header_size = erf_file_parts::ErfHeader::BYTE_SIZE.unwrap();
        let language_size = descriptions
            .iter()
            .fold(0, |count, description| count + description.byte_size());
        

        let offset_to_key_list = header_size + language_size;
        let offset_to_resource_list = offset_to_key_list + key_list_size;
        let offset_to_resources = offset_to_resource_list + resource_list_size;

        let header = erf_file_parts::ErfHeader {
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
                erf_file_parts::ErfKey {
                    file_name: resource.name.clone(),
                    resource_id: i as u32,
                    resource_type: resource.resource_type.clone(),
                }
            })
            .collect::<Vec<erf_file_parts::ErfKey>>();

        let resource_list = resources
            .iter()
            .scan(offset_to_resources, |state, resource| {                
                let resource_size = resource.data.len();

                let item = erf_file_parts::ResourceListItem {
                    offset: *state as u32,
                    size: resource_size as u32,
                };

                *state = *state + resource_size;

                Some(item)
            })
            .collect::<Vec<erf_file_parts::ResourceListItem>>();

        let f = File::create(file_path)?;
        let mut writer = BufWriter::new(f);

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
}

