use std::io::prelude::*;

use crate::types::{
    Error as MyError,
};

use super::writer;
use super::parser;

use super::types::{
    X2daRow,
    X2daBuilderConfig,
    X2daHeader,
    X2daError,
    X2daColumns,
};

#[derive(Debug)]
pub struct X2daFile<T>
    where T: X2daRow
{
    pub rows: Vec<T>,
    pub columns: Option<Vec<String>>,
    pub header: Option<X2daHeader>,
}

impl<T> X2daFile<T>
    where T: X2daRow
{
    pub fn new() -> Self
    {
        X2daFile {
            rows: Vec::new(),
            columns: None,
            header: Some(X2daHeader::default()),
        }
    }

    pub fn parse_from<R: Seek + BufRead>(reader: &mut R)
        -> Result<Self, MyError>
    {
        parser::parse(reader)
    }

    pub fn set_columns(&mut self, columns: Vec<String>)
        -> Result<&mut Self, X2daError>
    {
        X2daColumns::validate::<T>(&columns)?;
        self.columns = Some(columns);
        Ok(self)
    }

    pub fn add_row(&mut self, item: T)
        -> Result<&mut Self, X2daError>
    {
        writer::validate_row(&item)?;
        self.rows.push(item);
        Ok(self)
    }

    pub fn write_with_config<W: Write>(&mut self, config: X2daBuilderConfig, writer: &mut W)
        -> Result<(), MyError>
    {
        writer::write(self, writer, config)?;

        Ok(())
    }

    pub fn write<W: Write>(&mut self, writer: &mut W)
        -> Result<(), MyError>
    {
        self.write_with_config(X2daBuilderConfig::default(), writer)
    }
}
