use std::io::prelude::*;

use crate::types::{
    Error as MyError,
};

use super::x2da_file::X2daFile;
use super::types::{
    X2daError,
    X2daRow,
    X2daColumns,
    X2daHeader,
};

pub fn parse<T: X2daRow, F: Seek + BufRead>(reader: F)
    -> Result<X2daFile<T>, MyError>
{
    
    let lines = reader
        .lines()
        .collect::<std::io::Result<Vec<_>>>()?;

    let columns = str::split_whitespace(&lines[2])
        .map(|s| String::from(s))
        .collect();

    X2daColumns::validate::<T>(&columns)?;
    
    let rows = lines[3..]
        .iter()
        .filter(|l| l.as_str() != "")
        .map(T::from_line)
        .collect::<Result<Vec<T>, X2daError>>()?;

    Ok(X2daFile {
        header: Some(X2daHeader::default()),
        rows: rows,
        columns: Some(columns),
    })
}


#[cfg(test)]
mod test
{
    use super::*;
    use super::super::data::{
        Sample2da,
        x2da_file_string,
        x2da_sample_rows
    };
    use std::io::Cursor;
    
    #[test]
    fn parse_x2da_file_string() {

        let expected_rows = x2da_sample_rows();
        let expected_columns = vec![
            String::from("Melon"),
            String::from("num"),
            String::from("numf"),
        ];

        let file_string = x2da_file_string();

        let bytes = String::from(file_string).into_bytes();
        let c = Cursor::new(bytes);

        let file = parse::<Sample2da, _>(c).unwrap();

        assert_eq!(expected_rows, file.rows);
        assert_eq!(Some(expected_columns), file.columns);
    }

    #[test]
    fn parse_x2da_string() {

        let x2da_string = r#"1    "A small melon"    1    3.141592    "#;

        let expected = Sample2da::new(
            String::from("A small melon"),
            1,
            3.141592,
        );

        let row = Sample2da::from_line(x2da_string).unwrap();

        assert_eq!(expected, row);
    }

    #[test]
    fn parse_x2da_string_no_quotes() {
        let x2da_string = r#"0    Cantelope        0    3.0     "#;

        let expected = Sample2da::new(
            String::from("Cantelope"),
            0,
            3.0,
        );

        let row = Sample2da::from_line(x2da_string).unwrap();

        assert_eq!(expected, row)
    }

    #[test]
    fn parse_x2da_string_single_space() {
        let x2da_string = r#"0 Cantelope 0 3.0"#;

        let expected = Sample2da::new(
            String::from("Cantelope"),
            0,
            3.0,
        );

        let row = Sample2da::from_line(x2da_string).unwrap();

        assert_eq!(expected, row)
    }

    #[test]
    fn parse_x2da_string_with_nulls() {
        let x2da_string = r#"0    ****    ****  3.0"#;

        let expected = Sample2da {
            a_text: None,
            a_u32: None,
            a_f32: Some(3.0),
        };
        
        let row = Sample2da::from_line(x2da_string).unwrap();

        assert_eq!(expected, row);
    }
}