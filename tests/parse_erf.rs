use nwn_files;
use nwn_files::ErfFile;

use std::path::Path;
use std::fs::File;


mod helpers;

use helpers::plain_text;

const EFP: &str = "./tests/samples/cep2_core0.hak";

fn erf_file_path() -> &'static Path {
    let path = Path::new(EFP);
    assert!(path.exists());
    path
}

fn erf_reader() -> File {
    File::open(erf_file_path()).unwrap()
}

#[test]
fn parse() {
    let mut file = erf_reader();
    ErfFile::parse_from(&mut file).unwrap();
}

#[test]
fn parse_to_plain_text() {
    let mut file = erf_reader();
    let parsed = ErfFile::parse_from(&mut file).unwrap();
    let plain_text_path = Path::new("./plain_text_erf.txt");
    plain_text::write_plain_text_file(plain_text_path, parsed).unwrap();
}


#[test]
#[ignore]
fn parse_bad_erf() {
    unimplemented!();
}