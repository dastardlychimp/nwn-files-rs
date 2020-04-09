use std::convert::{TryFrom, From};
use std::error::Error;
use std::fmt;
use std::ops::{ Deref, DerefMut };

const RES_REF_LENGTH: usize = 16;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ResRef(String);

#[derive(Debug, PartialEq)]
pub enum ResRefError {
    InvalidLengthTooLong,
}

impl fmt::Display for ResRefError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            ResRefError::InvalidLengthTooLong =>
                write!(f, "ResRefError: ResRef needs to be 16 chars or less."),
        }
    }
}

impl Error for ResRefError {}

impl ResRef {
    fn new(s: String) -> Result<Self, ResRefError>
    {
        if s.len() > RES_REF_LENGTH {
            Err(ResRefError::InvalidLengthTooLong)
        } else {
            Ok(ResRef(s))
        }
    }

    pub fn serialize(self) -> Vec<u8>
    {
        let difference = RES_REF_LENGTH - self.0.len();
        let padding = std::iter::repeat(0).take(difference);

        let mut bytes = self.0.into_bytes();
        bytes.extend(padding);

        bytes
    }
}

impl TryFrom<&str> for ResRef {
    type Error = ResRefError;
    
    #[inline]
    fn try_from(s: &str)
        -> Result<ResRef, Self::Error>
    {
        ResRef::new(s.to_owned())
    }
}

impl TryFrom<String> for ResRef {
    type Error = ResRefError;
    
    #[inline]
    fn try_from(s: String)
        -> Result<ResRef, Self::Error>
    {
        ResRef::new(s)
    }
}

impl TryFrom<&[u8]> for ResRef {
    type Error = ResRefError;

    fn try_from(s: &[u8])
        -> Result<ResRef, Self::Error>
    {
        let string = String::from_utf8_lossy(&s)
            .trim_end_matches(char::from(0))
            .to_owned();

        ResRef::new(string)
    }
}

impl Deref for ResRef{
    type Target = String;

    fn deref(&self) 
        -> &Self::Target
    {
        &self.0
    }
}

impl DerefMut for ResRef {
    fn deref_mut(&mut self)
        -> &mut Self::Target
    {
        &mut self.0
    }
}


#[cfg(test)]
mod test
{
    use super::*;
    
    #[test]
    fn res_ref_from_string() {
        let result = ResRef::try_from("a".repeat(16));
        result.unwrap();
    }

    #[test]
    fn res_ref_from_str() {
        let result = ResRef::try_from("blah");
        result.unwrap();
    }

    #[test]
    fn res_ref_too_long() {
        let result = ResRef::try_from("I_am_over_16_chars_long_and_thus_invalid");
        assert_eq!(ResRefError::InvalidLengthTooLong, result.unwrap_err());
    }

    #[test]
    fn modify_res_ref() {
        let mut resref = ResRef::try_from("123").unwrap();
        let new_contents = String::from("Blah");

        *resref = new_contents.clone();

        assert_eq!(ResRef(new_contents), resref);
    }

    #[test]
    fn res_ref_from_bytes() {
        let bytes = vec![97, 98, 99, 0, 0, 0];
        let resref = ResRef::try_from(&bytes[..]).unwrap();

        let expected = ResRef::try_from("abc").unwrap();
        
        assert_eq!(expected, resref);
    }

    #[test]
    fn res_ref_to_bytes() {
        let resref = ResRef::try_from("abc").unwrap();
        let serialized = resref.serialize();

        let expected = [97, 98, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(&expected, serialized.as_slice());
    }
}