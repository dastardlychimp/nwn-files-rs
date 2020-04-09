use lazy_static::lazy_static;
use regex;
use regex::Regex;

lazy_static! {
    static ref X2DA_REGEX: Regex = Regex::new(r#"(?:"(.*?)")|(?:^|\s)([^"\s]+)"#).unwrap();
}

pub fn parse_string<S: AsRef<str>>(string: S)
    -> Vec<String>
{
    X2DA_REGEX
        .captures_iter(string.as_ref())
        .map(|c| {
            c.get(1)
                .or(c.get(2))
                .unwrap()
                .as_str()
                .to_owned()
        })
        .collect()
}

#[cfg(test)]
mod test
{
    use super::*;
    
    #[test]
    fn parse_single_line() {
        let x2da_string = r#"1    "A small melon"    1    3.141592    "#;
        let expected = vec![
            String::from("1"),
            String::from("A small melon"),
            String::from("1"),
            String::from("3.141592"),
        ];

        let parsed = parse_string(x2da_string);

        assert_eq!(expected, parsed);
    }
}