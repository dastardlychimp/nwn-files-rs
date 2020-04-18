use nwn_files;

use std::path::Path;

mod helpers;

use helpers::plain_text;
use helpers::tlk::*;

#[test]
fn parse() {
   parse_from_path(tlk_file_path(), false).unwrap();
}

#[test]
fn parse_to_plain_text() {
    let parsed = parse_from_path(tlk_file_path(), false).unwrap();
    let plain_text_path = Path::new("./plain_text_tlk.txt");
    plain_text::write_plain_text_file(plain_text_path, parsed).unwrap();
}


#[test]
#[ignore]
fn parse_bad_erf() {
    unimplemented!();
}