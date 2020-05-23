use std::convert::{TryFrom, From};
use std::str::Utf8Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Version {
    Unknown,
    V1,
    V2,
    V3,
}

impl Version {
    pub fn as_str_ref(&self)
        -> &'static str
    {
        match self {
            Version::V1 => "V1.0",
            Version::V2 => "V2.0",
            Version::V3 => "V3.0",
            Version::Unknown => "",
        }
    }
}

impl From<&str> for Version
{
    fn from(s: &str) -> Version
    {
        match s {
            "V1.0" | "V1  " => Version::V1,
            "V2.0" | "V2  " => Version::V2,
            "V3.0" | "V3  " => Version::V3,
            _ => Version::Unknown,
        }
    }
}

impl TryFrom<&[u8]> for Version
{
    type Error = Utf8Error;
    
    fn try_from(bytes: &[u8])
        -> Result<Version, Self::Error>
    {
        let s = std::str::from_utf8(bytes)?;
        Ok(Version::from(s))
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    
    #[test]
    fn version_as_str_ref()
    {
        let v = Version::V1;

        assert_eq!("V1.0", v.as_str_ref());
    }

    #[test]
    fn version_from_str() {
        assert_eq!(Version::V1, Version::from("V1.0"));
        assert_eq!(Version::V3, Version::from("V3  "));
    }
    
    #[test]
    fn version_from_bytes() {
        let s = "V1.0";
        let bytes = s.as_bytes();
        let v = Version::try_from(bytes).unwrap();
        assert_eq!(Version::V1, v);
    }
}