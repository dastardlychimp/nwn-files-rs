use nwn_files;

use std::path::Path;

mod helpers;

use helpers::plain_text;

const TFP: &str = "./tests/samples/dialog.tlk";

fn tlk_file_path() -> &'static Path {
    return Path::new(TFP);
}

#[test]
fn parse() {
    assert!(tlk_file_path().exists());

    nwn_files::parse_tlk(tlk_file_path()).unwrap();
}

#[test]
fn parse_to_plain_text() {
    let parsed = nwn_files::parse_tlk(tlk_file_path()).unwrap();
    let plain_text_path = Path::new("./plain_text_tlk.txt");
    plain_text::write_plain_text_file(plain_text_path, parsed).unwrap();
}


#[test]
#[ignore]
fn parse_bad_erf() {
    unimplemented!();
}