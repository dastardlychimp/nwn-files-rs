pub(crate) mod types;
pub mod writer;
pub mod parser;
mod regex_parser;


#[cfg(test)]
mod data {
    use super::types::{
        X2daError,
        X2daRow,
        X2daItem,
    };
    
    #[derive(Debug, PartialEq)]
    pub struct Sample2da {
        pub a_text: Option<String>,
        pub a_u32: Option<u32>,
        pub a_f32: Option<f32>,
    }

    impl Sample2da {
        pub fn new(a_text: String, a_u32: u32, a_f32: f32)
            -> Self
        {
            Sample2da {
                a_text: Some(a_text),
                a_u32: Some(a_u32),
                a_f32: Some(a_f32),
            }
        }
    }

    impl X2daRow for Sample2da {
        const SIZE: usize = 3;
        
        type Row = [Option<Box<dyn X2daItem>>; 3];
        
        fn to_row(&self) -> Self::Row
        {
            [
                self.a_text.to_owned().map(X2daItem::boxed),
                self.a_u32.to_owned().map(X2daItem::boxed),
                self.a_f32.to_owned().map(X2daItem::boxed),
            ]
        }

        fn from_strings(mut strings: Vec<Option<String>>)
            -> Result<Self, X2daError>
        {
            let a_text = strings.remove(0);
            let a_u32 = strings.remove(0)
                .map(|v| v.parse::<u32>())
                .transpose()
                .or(Err(X2daError::InvalidTableItem))?;

            let a_f32 = strings.remove(0)
                .map(|v| v.parse::<f32>())
                .transpose()
                .or(Err(X2daError::InvalidTableItem))?;

            Ok(Sample2da {
                a_text: a_text,
                a_u32: a_u32,
                a_f32: a_f32,
            })
        }
    }
    
    pub fn x2da_file_string() -> &'static str
    {
r#"
2DA V2.0

     Melon              num  numf
0    "Cantelope"        0    3.0         
1    "A small melon"    1    3.141592    
2    "Watermelon"       2    100.1

"#.trim_start_matches("\n") 
    }

    pub fn x2da_sample_rows() -> Vec<Sample2da>
    {
        vec![
            Sample2da {
                a_text: Some(String::from("Cantelope")),
                a_u32: Some(0),
                a_f32: Some(3.0),
            },
            Sample2da {
                a_text: Some(String::from("A small melon")),
                a_u32: Some(1),
                a_f32: Some(3.141592),
            },
            Sample2da {
                a_text: Some(String::from("Watermelon")),
                a_u32: Some(2),
                a_f32: Some(100.1),
            },
        ]
    }
}