use std::fmt;
use std::error::Error;

use super::regex_parser;

use crate::types::{
    Version,
    FileType,
};

#[derive(Debug, PartialEq)]
pub enum X2daError
{
    X2daItemContainsQuotes,
    X2daRowNotEnoughValues,
    X2daRowTooManyValues,
    X2daWrongNumberColumns(usize, usize),
    X2daColumnsOnlyAlphaAndUnderscore,
    X2daWriteWithoutHeader,
    X2daWriteWithoutColumns,
    InvalidTableItem
}

impl fmt::Display for X2daError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)
        -> fmt::Result
    {
        match self {
            X2daError::X2daItemContainsQuotes =>
                write!(f, "Strings within X2da's can't contain quotes."),
            X2daError::X2daRowNotEnoughValues =>
                write!(f, "A row in the X2da did't contain enough columns."),
            X2daError::X2daRowTooManyValues =>
                write!(f, "A row in the X2da had too many columns"),
            X2daError::X2daWrongNumberColumns(expected, found) =>
                write!(f, "Found {} columns, expected {}", found, expected),
            X2daError::X2daColumnsOnlyAlphaAndUnderscore =>
                write!(f, "X2da columns can only contain lowercase/uppercase letters and underscores."),
            X2daError::X2daWriteWithoutColumns =>
                write!(f, "X2da can't be build without columns defined."),
            X2daError::X2daWriteWithoutHeader =>
                write!(f, "X2da can't be written without a header created."),
            X2daError::InvalidTableItem =>
                write!(f, "X2da contained a tableitem that couldn't be parsed successfully."),
        }
    }
}

impl Error for X2daError {}

pub trait X2daRow
{
    const SIZE: usize;
    
    type Row: AsRef<[Option<Box<dyn X2daItem>>]>;

    fn to_row(&self) -> Self::Row;

    fn from_strings(strings: Vec<Option<String>>)
        -> Result<Self, X2daError>
        where Self: Sized;

    fn from_line<S: AsRef<str>>(line: S)
        -> Result<Self, X2daError>
        where Self: Sized
    {
        let mut strings = regex_parser::parse_string(line);
        
        match strings.len()
        {   
            l if l < Self::SIZE + 1 => Err(X2daError::X2daRowNotEnoughValues),
            l if l > Self::SIZE + 1 => Err(X2daError::X2daRowTooManyValues),
            _ => {
                strings.remove(0);
                let optional_strings = strings
                    .into_iter()
                    .map(|s| match s.as_ref() {
                        "****" => None,
                        _ => Some(s)
                    })
                    .collect();

                Self::from_strings(optional_strings)
            }
        }
    }
}

pub trait X2daItem
{
    #[inline]
    fn validate(&self)
        -> Result<(), X2daError>
    {
        Ok(())
    }

    fn serialize_to_string(&self) -> String;

    fn boxed(self) -> Box<dyn X2daItem>
        where Self: Sized + 'static,
    {
        Box::new(self)
    }
}

impl X2daItem for String
{
    fn validate(&self)
        -> Result<(), X2daError>
    {
        match self.contains("\"") {
            true => Err(X2daError::X2daItemContainsQuotes),
            false => Ok(())
        }
    }

    #[inline]
    fn serialize_to_string(&self)
        -> String
    {
        format!("\"{}\"", self)
    }
}

impl X2daItem for u32 {
    #[inline]
    fn serialize_to_string(&self)
        -> String
    {
        self.to_string()
    }
}

impl X2daItem for f32 {
    fn serialize_to_string(&self)
        -> String
    {
        format!("{:?}", self)
    }
}

pub struct X2daColumns {}

impl X2daColumns {
    pub fn validate<T: X2daRow>(columns: &Vec<String>)
        -> Result<(), X2daError>
    {
        if columns.len() != T::SIZE {
            return Err(X2daError::X2daWrongNumberColumns(T::SIZE, columns.len()));
        }

        let valid_chars = columns
            .iter()
            .all(|col| {
                col
                    .chars()
                    .all(|c| c.is_alphabetic() || c == '_')
            });

        if ! valid_chars {
            return Err(X2daError::X2daColumnsOnlyAlphaAndUnderscore);
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct X2daHeader {
    pub version: Version,
    pub file_type: FileType,
}

impl Default for X2daHeader {
    fn default() -> Self
    {
        X2daHeader {
            version: Version::V2,
            file_type: FileType::X2da,
        }
    }
}

#[derive(Debug)]
pub struct X2daBuilderConfig
{
    pub spacing_length: usize,
}

impl Default for X2daBuilderConfig
{
    fn default() -> Self
    {
        X2daBuilderConfig {
            spacing_length: 4
        }
    }
}