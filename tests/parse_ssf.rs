use nwn_files;

use std::path::Path;

mod helpers;
use helpers::plain_text;

const SFP: &str = "./tests/samples/psychopath.ssf";

fn ssf_file_path() -> &'static Path {
    return Path::new(SFP);
}

fn parse_default_ssf() -> nwn_files::SsfFile
{
    assert!(ssf_file_path().exists());
    nwn_files::parse_ssf(ssf_file_path()).unwrap()
}

#[test]
fn parse() {
    parse_default_ssf();
}

#[test]
fn parse_to_plain_text() {
    let parsed = parse_default_ssf();
    let plain_text_path = Path::new(".plain_text_ssf.txt");
    plain_text::write_plain_text_file(plain_text_path, parsed).unwrap();
}


#[test]
#[ignore]
fn parse_bad_erf() {
    unimplemented!();
}