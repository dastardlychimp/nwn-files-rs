use crate::helpers::conversion::*;
use crate::types::*;
use crate::types::{
    Error as MyError,
};

use std::convert::TryFrom;

use super::ssf_file::SsfFile;
use super::types::{
    SsfHeader,
};

pub fn parse(bytes: Vec<u8>)
    -> Result<SsfFile, MyError>
{
    let header = parse_ssf_header(&bytes);
    // dbg!("{:?}", &header);

    let entry_offsets = parse_entry_table(&bytes, &header);
    // dbg!("{:?}", &entry_offsets);

    let entries = parse_entries(&bytes, &entry_offsets);
    // dbg!("{:?}", &entries);

    Ok(SsfFile {
        header: Some(header),
        entries: entries
    })
}

fn parse_ssf_header(bytes: &Vec<u8>) -> SsfHeader {
    SsfHeader {
        version: Version::V1,
        file_type: FileType::Ssf,
        entry_count: u32_from_bytes(&bytes[8..12]),
        table_offset: u32_from_bytes(&bytes[12..16]),
    }
}

fn parse_entry_table(bytes: &Vec<u8>, header: &SsfHeader)
    -> Vec<usize>
{
    (0..header.entry_count as usize)
        .into_iter()
        .map(|i| {
            let o = i * 4 + header.table_offset as usize;
            u32_from_bytes(&bytes[o..o+4]) as usize
        })
        .collect()
}

fn parse_entries(bytes: &Vec<u8>, entry_offsets: &Vec<usize>)
    -> Vec<SsfEntry>
{
    entry_offsets
        .into_iter()
        .map(|offset| {
            let str_ref = match u32_from_bytes(&bytes[offset+16..offset+20]) {
                NULL_U32 => None,
                t => Some(t),
            };

            SsfEntry {
                res_ref: ResRef::try_from(&bytes[*offset..*offset+16]).unwrap(),
                string_ref: str_ref,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

}
