use crate::types::{
    Error as MyError,
    NULL_STRING,
};

use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::convert::From;


use super::x2da_file::X2daFile;

use super::types::{
    X2daError,
    X2daRow,
    X2daHeader,
    X2daBuilderConfig,
};

pub fn validate_row<T: X2daRow>(row: &T)
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

pub fn write<F, T>(
    x2da_file: &mut X2daFile<T>, 
    writer: &mut F,
    config: X2daBuilderConfig
)
    -> Result<(), MyError>
    where F: Write, T: X2daRow
{
    if x2da_file.columns.is_none()
    {
        Err(X2daError::X2daWriteWithoutColumns)?;
    }

    if x2da_file.header.is_none()
    {
        Err(X2daError::X2daWriteWithoutHeader)?;
    }

    let mut writer = BufWriter::new(writer);

    let string_rows = x2da_file.rows
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
        
    // dbg!("{:?}", &string_rows);

    let max_lengths = max_lengths(&x2da_file, &string_rows);
    let columns = x2da_file.columns.as_ref().unwrap();
    let header = x2da_file.header.as_ref().unwrap();

    write_header(&mut writer, header)?;
    write_columns(&mut writer, &config, columns, &max_lengths)?;

    string_rows
        .iter()
        .enumerate()
        .map(|(idx, row)| {
            write_row(&mut writer, &config, &max_lengths, idx, row)
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    // dbg!("{:?}", max_lengths);

    Ok(())
}


fn write_row<F>(
    writer: &mut F,
    config: &X2daBuilderConfig,
    max_lengths: &Vec<usize>,
    idx: usize,
    row: &Vec<String>
)
    -> Result<(), io::Error>
    where F: Write
{
    let spacing = config.spacing_length;
    write!(writer, "{:<width$}", idx, width = spacing + 1)?;
    
    row
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let padding = spacing + max_lengths[i];
            write!(writer, "{:width$}", item, width = padding)
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    write!(writer, "\n")?;

    Ok(())
}

fn write_columns<F>(
    writer: &mut F,
    config: &X2daBuilderConfig,
    columns: &Vec<String>,
    max_lengths: &Vec<usize>
)
    -> Result<(), io::Error>
    where F: Write
{
    write!(writer, "{:<width$}", "", width = config.spacing_length + 1)?;
    
    columns
        .iter()
        .enumerate()
        .map(|(i, col)| {
            let padding = config.spacing_length + max_lengths[i];
            write!(writer, "{:width$}", col, width = padding)
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    write!(writer, "\n")?;

    Ok(())
}

fn write_header<F>( writer: &mut F, header: &X2daHeader)
    -> Result<(), io::Error>
    where F: Write,
{
    write!(
        writer,
        "{}{}\n\n",
        header.file_type.as_str_ref(),
        header.version.as_str_ref()
    )?;

    Ok(())
}

fn max_lengths<T>(
    x2da_file: &X2daFile<T>,
    string_rows: &Vec<Vec<String>>
)
    -> Vec<usize>
    where T: X2daRow
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
                .max(x2da_file.columns.as_ref().unwrap()[i].len())
        })
        .collect()
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

        let e = X2daFile::<Sample2da>::new()
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

        let e = X2daFile::<Sample2da>::new()
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

        let e = X2daFile::new()
            .add_row(my_2da_row)
            .unwrap_err();

        assert_eq!(X2daError::X2daItemContainsQuotes, e);
    }

    #[test]
    fn build_without_columns() {
        let mut c = Cursor::new(Vec::new());
        
        let mut rows = x2da_sample_rows();

        let e = X2daFile::new()
            .add_row(rows.remove(0))
            .unwrap()
            .add_row(rows.remove(0))
            .unwrap()
            .add_row(rows.remove(0))
            .unwrap()
            .write(&mut c)
            .unwrap_err();

        match e {
            MyError::X2daError(X2daError::X2daWriteWithoutColumns) => assert!(true),
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

    X2daFile::new()
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

        X2daFile::new()
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
