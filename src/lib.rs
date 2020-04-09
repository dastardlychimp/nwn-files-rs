mod helpers;
mod files;

use files::{bif, key, erf, ssf, x2da, tlk};
use std::path::Path;
use helpers::file::read_file_to_vec;

pub mod types;
pub use bif::parse as parse_bif;

pub use ssf::parser::SsfFile;
pub use ssf::writer::SsfBuilder;
pub use x2da::parser::parse;
pub use x2da::writer::X2daBuilder;
pub use erf::writer::ErfFileBuilder;
pub use key::{BifFile, BifResource};
pub use bif::BifFile2;
pub use tlk::types::TlkFile;
pub use tlk::writer::TlkBuilder;

use std::io::BufReader;
use std::fs::File;


pub fn parse_key<P: AsRef<Path>>(file_path: P)
    -> Result<key::KeyFile2, std::io::Error>
{
    let bytes = read_file_to_vec(file_path)?;
    key::parse(bytes)
}

pub fn parse_erf<P: AsRef<Path>>(file_path: P) -> Result<erf::types::ErfFile, std::io::Error> {
    let bytes = read_file_to_vec(file_path)?;
    erf::parser::parse(bytes)
}

pub fn parse_ssf<P: AsRef<Path>>(file_path: P) -> Result<SsfFile, std::io::Error> {
    let bytes = read_file_to_vec(file_path)?;
    ssf::parser::parse(bytes)
}

pub fn parse_tlk<P: AsRef<Path>>(file_path: P) -> Result<TlkFile, types::Error> {
    let f = File::open(file_path.as_ref()).unwrap();
    let mut r = BufReader::new(f);
    tlk::parser::parse(&mut r)
}
