use std::convert::From;

#[derive(Debug, PartialEq, Clone)]
pub enum FileType {
    Unknown,
    Key,
    Bif,
    Erf,
    Ssf,
    Mod,
    Sav,
    Hak,
    X2da,
    Tlk,
}

impl FileType {
    pub fn as_str_ref(&self) -> &'static str
    {
        match self {
            FileType::Erf => "ERF ",
            FileType::Bif => "BIF ",
            FileType::Key => "KEY ",
            FileType::Ssf => "SSF ",
            FileType::Mod => "MOD ",
            FileType::Sav => "SAV ",
            FileType::Hak => "HAK ",
            FileType::X2da => "2DA ",
            FileType::Tlk => "TLK ",
            FileType::Unknown => "",
        }
    }
}

impl From<&str> for FileType {
    fn from(s: &str) -> FileType
    {
        match s {
            "ERF " => FileType::Erf,
            "BIF " => FileType::Bif,
            "KEY " => FileType::Key,
            "SSF " => FileType::Ssf,
            "MOD " => FileType::Mod,
            "SAV " => FileType::Sav,
            "HAK " => FileType::Hak,
            "2DA " => FileType::X2da,
            "TLK " => FileType::Tlk,
            _ => FileType::Unknown,
        }
    }
}


#[cfg(test)]
mod tests
{
    use super::*;
    
    #[test]
    fn str_from_file_type()
    {
        let ft = FileType::Erf;

        assert_eq!("ERF ", ft.as_str_ref());
    }

    #[test]
    fn file_type_from_str() {
        assert_eq!(FileType::Erf, FileType::from("ERF "));
        assert_eq!(FileType::Ssf, FileType::from("SSF "));
    }
}