mod helpers;
mod files;

use files::{bif, key, ssf, x2da, tlk};
use std::path::Path;
use helpers::file::read_file_to_vec;

pub mod types;
pub use bif::parse as parse_bif;

pub use ssf::ssf_file::SsfFile;
pub use ssf::writer::SsfBuilder;
pub use x2da::x2da_file::X2daFile;
pub use key::{BifFile, BifResource};
pub use bif::BifFile2;
pub use tlk::tlk_file::TlkFile;

pub use types::{
    ErfFile
};


pub fn parse_key<P: AsRef<Path>>(file_path: P)
    -> Result<key::KeyFile2, std::io::Error>
{
    let bytes = read_file_to_vec(file_path)?;
    key::parse(bytes)
}

