mod resref;
mod resource_type;
mod resource;
mod error;
mod language_id;
mod file_type;
mod version;

pub use resref::{ResRef, ResRefError};
pub use resource_type::ResourceType;
pub use resource::Resource;
pub use error::Error;
pub use language_id::LanguageId;
pub use file_type::FileType;
pub use version::Version;
pub use crate::files::tlk::types::{TlkEntry, TlkSound};
pub use crate::files::x2da::types::{X2daRow, X2daItem, X2daError};
pub use crate::files::erf::types::{ErfFile};

use std::io::prelude::*;

#[derive(Debug, Default)]
pub struct SsfEntry {
    pub res_ref: ResRef,
    pub string_ref: Option<u32>,
}

pub const NULL_U32: u32 = u32::max_value();

pub const NULL_STRING: &str = "****";


pub trait StaticByteSize {
    const BYTE_SIZE: usize;
}

pub trait SerializeToBytes {
    fn serialize_to<F: Write>(self, writer: &mut F)
        -> Result<(), Error>;
}

impl<S: SerializeToBytes> SerializeToBytes for Vec<S> {
    fn serialize_to<F: Write>(self, writer: &mut F)
        -> Result<(), Error>
    {
        for item in self {
            item.serialize_to(writer)?;
        }

        Ok(())
    }
}

pub trait DeserializeFromBytes: Sized {
    fn deserialize_from<R: Read>(self, reader: &mut R)
        -> Result<Self, Error>;
}