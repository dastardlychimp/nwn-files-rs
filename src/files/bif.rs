use crate::types::*;
use crate::helpers::conversion::*;
use std::path::Path;
use crate::helpers::file::{ read_file_to_vec };

#[derive(Debug)]
pub struct BifHeader {
    pub version: Version,
    pub file_type: FileType,
    pub variable_resource_count: u32,
    pub fixed_resource_count: u32,
    pub variable_table_offset: u32,
}

#[derive(Debug)]
pub struct VariableResource {
    pub id: u32,
    pub resource_offset: u32,
    pub resource_size: u32,
    pub resource_type: u32,
}

#[derive(Debug)]
pub struct BifFile {
    pub header: BifHeader,
    pub variable_resources: Vec<VariableResource>
}

#[derive(Debug)]
pub struct BifFile2(pub Vec<Resource>);

#[derive(Debug)]
pub struct Resource {
    pub bytes: Vec<u8>,
    pub resource_type: ResourceType,
}

pub fn parse<P: AsRef<Path>>(file_path: P) -> std::io::Result<BifFile2>  {
    let bytes = read_file_to_vec(file_path)?;

    let header = parse_header(&bytes);
    // dbg!("{:?}", &header);
    
    let variable_resources = parse_variable_resources(&bytes, &header);
    // dbg!("{:?}", &variable_resources);

    let resources = variable_resources
        .into_iter()
        .map(|vr| {
            let o = vr.resource_offset as usize;
            let bytes = Vec::from(&bytes[o..o+vr.resource_size as usize]);
            
            Resource {
                bytes: bytes,
                resource_type: ResourceType::from(vr.resource_type),
            }
        })
        .collect();

    Ok(BifFile2(resources))
}

fn parse_header(bytes: &Vec<u8>) -> BifHeader {
    BifHeader {
        file_type: FileType::Bif,
        version: Version::V1,
        variable_resource_count: u32_from_bytes(&bytes[8..12]),
        fixed_resource_count: u32_from_bytes(&bytes[12..16]),
        variable_table_offset: u32_from_bytes(&bytes[16..20]),
    }
}

fn parse_variable_resources(bytes: &Vec<u8>, header: &BifHeader) -> Vec<VariableResource> {
    const VARIABLE_RESOURCE_SIZE: usize = 16;
    let index_start = header.variable_table_offset as usize;
    let resource_count = header.variable_resource_count as usize;

    (0..resource_count)
        .into_iter()
        .map(|i| {
            let index = index_start + VARIABLE_RESOURCE_SIZE * i;

            VariableResource {
                id: u32_from_bytes(&bytes[index..index + 4]),
                resource_offset: u32_from_bytes(&bytes[index+4..index+8]),
                resource_size: u32_from_bytes(&bytes[index+8..index+12]),
                resource_type: u32_from_bytes(&bytes[index+12..index+16]),
            }
        })
        .collect()
}

