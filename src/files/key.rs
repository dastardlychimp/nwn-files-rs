use crate::helpers::conversion::*;
use crate::types::*;

#[derive(Debug)]
pub struct KeyHeader {
    version: Version,
    file_type: FileType,
    bif_count: u32,
    key_count: u32,
    offset_file_table: u32,
    offset_key_table: u32,
    build_years: u32,
    build_days: u32,
}

#[derive(Debug)]
pub struct TableEntry {
    file_size: u32,
    file_name_offset: u32,
    file_name_size: u16,
    drives: u16,
}

#[derive(Debug)]
pub struct KeyEntry {
    name: String,
    resource_type: u16,
    id: u32,
}

#[derive(Debug)]
pub struct KeyFile {
    header: KeyHeader,
    table_entries: Vec<TableEntry>,
    file_names: Vec<String>,
    key_entries: Vec<KeyEntry>,
}

#[derive(Debug)]
pub struct KeyFile2(pub Vec<BifFile>);

#[derive(Debug)]
pub struct BifFile {
    pub name: String,
    pub resources: Vec<BifResource>
}

#[derive(Debug)]
pub struct BifResource {
    pub resource_index: usize,
    pub resource_type: ResourceType,
    pub name: String,
}


pub fn parse(bytes: Vec<u8>) -> std::io::Result<KeyFile2> {
    let header = parse_key_header(&bytes);

    // dbg!("{:?}", &header);

    let entries = parse_table_entries(&bytes, &header);

    // dbg!("{:?}", &entries);

    let file_names = parse_file_names(&bytes, &entries);
    
    // dbg!("{:?}", &file_names);
    
    let key_table = parse_key_entries(&bytes, &header);

    let bif_files = file_names
        .into_iter()
        .enumerate()
        .map(|(i, name)| {
            let resources = key_table
                .iter()
                .filter(|k| k.id >> 20 == i as u32)
                .map(|k| {
                    let resource_type = ResourceType::from(k.resource_type);                
                    BifResource {
                        name: k.name.trim_end_matches("\u{0}").to_string(),
                        resource_index: 0xFFFFF & k.id as usize,
                        resource_type: resource_type,
                    }
                })
                .collect();

            Ok(BifFile {
                name: name,
                resources: resources,
            })
        })
        .collect::<Result<Vec<BifFile>, std::io::Error>>()?;

    Ok(KeyFile2(bif_files))
}

fn parse_key_header(bytes: &Vec<u8>) -> KeyHeader {
    let _file_type = String::from_utf8_lossy(&bytes[0..4]);
    let _file_version = String::from_utf8_lossy(&bytes[4..8]);

    let header = KeyHeader {
        version: Version::V1,
        file_type: FileType::Key,
        bif_count: u32_from_bytes(&bytes[8..12]),
        key_count: u32_from_bytes(&bytes[12..16]),
        offset_file_table: u32_from_bytes(&bytes[16..20]),
        offset_key_table: u32_from_bytes(&bytes[20..24]),
        build_years: u32_from_bytes(&bytes[24..28]),
        build_days: u32_from_bytes(&bytes[28..32]),
    };

    header
}

fn parse_file_names(bytes: &Vec<u8>, entries: &Vec<TableEntry>) -> Vec<String>
{
    entries
        .into_iter()
        .map(|entry| {
            let o = entry.file_name_offset as usize;
            String::from_utf8_lossy(
                &bytes[o..o + entry.file_name_size as usize]
            ).to_string()
        })
        .collect()
}

fn parse_table_entries(bytes: &Vec<u8>, header: &KeyHeader) -> Vec<TableEntry> {
    let offset_table_entries = 64;

    (0..header.bif_count as usize)
        .into_iter()
        .map(|i| {
            let o = i * 12 + offset_table_entries;

            TableEntry {
                file_size: u32_from_bytes(&bytes[o..o+4]),
                file_name_offset: u32_from_bytes(&bytes[o+4..o+8]),
                file_name_size: u16_from_bytes(&bytes[o+8..o+10]),
                drives: u16_from_bytes(&bytes[o+10..o+12]),
            }
        })
        .collect()
}


fn parse_key_entries(bytes: &Vec<u8>, header: &KeyHeader) -> Vec<KeyEntry> {
    let offset_key_table = header.offset_key_table as u64;
    const KEY_TABLE_SIZE: usize = 22;

    (0..header.key_count as usize)
        .into_iter()
        .map(|i| {
            let o = offset_key_table as usize + KEY_TABLE_SIZE * i;

            KeyEntry {
                name: String::from_utf8_lossy(&bytes[o..o+16]).into_owned(),
                resource_type: u16_from_bytes(&bytes[o+16..o+18]),
                id: u32_from_bytes(&bytes[o+18..o+22]),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

}
