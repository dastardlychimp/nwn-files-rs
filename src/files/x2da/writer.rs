use crate::types::{
    Error as MyError,
    NULL_STRING,
};

use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::convert::From;

use super::types::{
    X2daError,
    X2daRow,
    X2daHeader,
    X2daColumns,
    X2daFile,
};

#[derive(Debug)]
pub struct X2daBuilder<T>
    where T: X2daRow
{
    rows: Vec<T>,
    config: X2daBuilderConfig,
    columns: Option<Vec<String>>,
    header: X2daHeader,
}

impl<T> X2daBuilder<T>
    where T: X2daRow
{
    pub fn new() -> Self
    {
        X2daBuilder {
            rows: Vec::new(),
            config: X2daBuilderConfig::default(),
            columns: None,
            header: X2daHeader::default(),
        }
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
        Self::validate_row(&item)?;
        self.rows.push(item);
        Ok(self)
    }

    pub fn write<F: Write>(&mut self, writer: &mut F)
        -> Result<(), MyError>
    {
        if self.columns.is_none()
        {
            Err(X2daError::X2daBuildWithoutColumns)?;
        }

        let mut writer = BufWriter::new(writer);

        let string_rows = self.rows
            .iter()
            .map(|r| {
                r.to_row()
                    .as_ref()
                    .iter()
                    .map(|item| item
                        .as_ref()
                        .map(|i| i.serialize_to_string())
                        .unwrap_or(String::from(NULL_STRING))
                    )
                    .collect()
            })
            .collect::<Vec<Vec<String>>>();
            
        dbg!("{:?}", &string_rows);

        let max_lengths = self.max_lengths(&string_rows);

        self.write_header(&mut writer)?;
        self.write_columns(&mut writer, &max_lengths)?;

        string_rows
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                self.write_row(&mut writer, &max_lengths, idx, row)
            })
            .collect::<Result<Vec<_>, io::Error>>()?;

        dbg!("{:?}", max_lengths);

        Ok(())
    }

    fn validate_row(row: &T)
        -> Result<(), X2daError>
    {
        row.to_row()
            .as_ref()
            .iter()
            .map(|item| item
                .as_ref()
                .map(|i| i.validate())
                .unwrap_or(Ok(()))
            )
            .collect::<Result<Vec<_>, X2daError>>()?;

        Ok(())
    }

    fn write_row<F: Write>(
        &self,
        writer: &mut F,
        max_lengths: &Vec<usize>,
        idx: usize,
        row: &Vec<String>
    )
        -> Result<(), io::Error>
    {
        write!(writer, "{:<width$}", idx, width = self.config.spacing_length + 1)?;
        
        row
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let padding = self.config.spacing_length + max_lengths[i];
                write!(writer, "{:width$}", item, width = padding)
            })
            .collect::<Result<Vec<_>, io::Error>>()?;

        write!(writer, "\n")?;

        Ok(())
    }

    fn write_columns<F: Write>(
        &self,
        writer: &mut F,
        max_lengths: &Vec<usize>
    )
        -> Result<(), io::Error>
    {
        write!(writer, "{:<width$}", "", width = self.config.spacing_length + 1)?;
        
        self.columns
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, col)| {
                let padding = self.config.spacing_length + max_lengths[i];
                write!(writer, "{:width$}", col, width = padding)
            })
            .collect::<Result<Vec<_>, io::Error>>()?;

        write!(writer, "\n")?;

        Ok(())
    }

    fn write_header<F: Write>(&self, writer: &mut F)
        -> Result<(), io::Error>
    {
        write!(
            writer,
            "{}{}\n\n",
            self.header.file_type.as_str_ref(),
            self.header.version.as_str_ref()
        )?;

        Ok(())
    }

    fn max_lengths(&self, string_rows: &Vec<Vec<String>>) -> Vec<usize>
    {
        (0..T::SIZE)
            .into_iter()
            .map(|i| {
                string_rows
                    .iter()
                    .fold(0, |state, row| {
                        match row[i].len() {
                            l if l > state => l,
                            _ => state,
                        }
                    })
                    .max(self.columns.as_ref().unwrap()[i].len())
            })
            .collect()
    }
}

#[derive(Debug)]
struct X2daBuilderConfig
{
    spacing_length: usize,
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

impl<T: X2daRow> From<X2daFile<T>> for X2daBuilder<T>
{
    fn from(file: X2daFile<T>)
        -> Self
    {
        let X2daFile {
            rows,
            columns,
            header
        } = file;
        
        X2daBuilder {
            config: X2daBuilderConfig::default(),
            rows: rows,
            columns: Some(columns),
            header: header,
        }
    }
}


#[cfg(test)]
mod test
{
    // #TODO: Error with floating point numbers ending with 5.
    use super::*;
    use std::io::Cursor;
    use super::super::data::{
        Sample2da,
        x2da_sample_rows
    };

    #[test]
    fn x2da_item_string_contains_quotes() {
        let my_2da_row = Sample2da {
            a_text: Some(String::from("Blah\"Blah\"")),
            a_u32: Some(0),
            a_f32: Some(0.0),
        };
        
        let row = my_2da_row.to_row();

        assert_eq!(X2daError::X2daItemContainsQuotes, row[0].as_ref().unwrap().validate().unwrap_err())
    }


    #[test]
    fn x2da_build_invalid_columns_too_long() {
        let cols = vec![
            String::from("blah"),
            String::from("nah"),
            String::from("Cah"),
            String::from("dah"),
        ];

        let e = X2daBuilder::<Sample2da>::new()
            .set_columns(cols)
            .unwrap_err();

        assert_eq!(X2daError::X2daWrongNumberColumns(3, 4), e);
    }

    #[test]
    fn x2da_build_invalid_columns_characters() {
        let cols = vec![
            String::from("blah"),
            String::from("nah"),
            String::from("Cah3"),
        ];

        let e = X2daBuilder::<Sample2da>::new()
            .set_columns(cols)
            .unwrap_err();

        assert_eq!(X2daError::X2daColumnsOnlyAlphaAndUnderscore, e);    
    }

    #[test]
    fn x2da_build_invalid_item() {
        let my_2da_row = Sample2da {
            a_text: Some(String::from("Blah\"Blah\"")),
            a_u32: Some(0),
            a_f32: Some(0.0),
        };

        let e = X2daBuilder::new()
            .add_row(my_2da_row)
            .unwrap_err();

        assert_eq!(X2daError::X2daItemContainsQuotes, e);
    }

    #[test]
    fn build_without_columns() {
        let mut c = Cursor::new(Vec::new());
        
        let mut rows = x2da_sample_rows();

        let e = X2daBuilder::new()
            .add_row(rows.remove(0))
            .unwrap()
            .add_row(rows.remove(0))
            .unwrap()
            .add_row(rows.remove(0))
            .unwrap()
            .write(&mut c)
            .unwrap_err();

        match e {
            MyError::X2daError(X2daError::X2daBuildWithoutColumns) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn x2da_build_to_string_with_nulls() {
        let expected =
r#"
2DA V2.0

     Melon              num     num_float    
0    "Cantelope"        0       3.0          
1    "A small melon"    1       3.141592     
2    ****               ****    ****         
"#;

    let mut c = Cursor::new(Vec::new());

    let cols = vec![
        String::from("Melon"),
        String::from("num"),
        String::from("num_float"),
    ];

    let mut rows = x2da_sample_rows();

    rows[2] = Sample2da {
        a_text: None,
        a_u32: None,
        a_f32: None,
    };

    X2daBuilder::new()
        .set_columns(cols)
        .unwrap()
        .add_row(rows.remove(0))
        .unwrap()
        .add_row(rows.remove(0))
        .unwrap()
        .add_row(rows.remove(0))
        .unwrap()
        .write(&mut c)
        .unwrap();

    let expected = expected.trim_start_matches("\n");

    let s = String::from_utf8(c.into_inner()).unwrap();

    assert_eq!(expected, s);
    }

    #[test]
    fn x2da_build_to_string() {
        let expected =
r#"
2DA V2.0

     Melon              num    num_float    
0    "Cantelope"        0      3.0          
1    "A small melon"    1      3.141592     
2    "Watermelon"       2      100.1        
"#;
        let mut c = Cursor::new(Vec::new());

        let cols = vec![
            String::from("Melon"),
            String::from("num"),
            String::from("num_float"),
        ];

        let mut rows = x2da_sample_rows();

        X2daBuilder::new()
            .set_columns(cols)
            .unwrap()
            .add_row(rows.remove(0))
            .unwrap()
            .add_row(rows.remove(0))
            .unwrap()
            .add_row(rows.remove(0))
            .unwrap()
            .write(&mut c)
            .unwrap();

        let expected = expected.trim_start_matches("\n");

        let s = String::from_utf8(c.into_inner()).unwrap();
        
        assert_eq!(expected, s);
    }
}
