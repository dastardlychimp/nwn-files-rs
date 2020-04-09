use std::convert::From;

#[derive(Debug, Copy, Clone,)]
pub enum LanguageId
{
    Unknown = 75666776,
    English = 0,
    French = 1,
    German = 2,
    Italian = 3,
    Spanish = 4,
    Polish = 5,
    Korean = 128,
    ChineseTraditional = 129,
    ChineseSimplified = 130,
    Japanese = 131,
}

impl From<u32> for LanguageId
{
    fn from(value: u32) -> Self
    {
        match value {
            0 => LanguageId::English,
            1 => LanguageId::French,
            2 => LanguageId::German,
            3 => LanguageId::Italian,
            4 => LanguageId::Spanish,
            5 => LanguageId::Polish,
            128 => LanguageId::Korean,
            129 => LanguageId::ChineseTraditional,
            130 => LanguageId::ChineseSimplified,
            121 => LanguageId::Japanese,
            _ => LanguageId::Unknown,
        }
    }
}