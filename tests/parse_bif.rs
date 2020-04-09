use nwn_files;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub mod helpers;

use helpers::extract;
use helpers::plain_text;

const BFP: &str = "./tests/samples/en_voicesets.bif";

fn bif_file_path() -> &'static Path {
    return Path::new(BFP);
}

#[test]
#[ignore]
fn parse() {
    nwn_files::parse_bif(bif_file_path()).unwrap();

    assert!(false);
}

#[test]
#[ignore] // TODO: Parse to plain text is writing all the bytes on a single line which takes forever.
fn parse_to_plain_text() {
    let parsed = nwn_files::parse_bif(bif_file_path()).unwrap();
    let plain_text_path = Path::new("./en_voicesets.txt");
    plain_text::write_plain_text_file(plain_text_path, parsed).unwrap();

    assert!(plain_text_path.exists());
}


#[test]
fn test_extract_wave_file() {
    let sound_path = Path::new("./tests/outputs/extract_voiceset_sound.wav");

    let buff = extract::bif_resource_by_index(bif_file_path(), 4001);

    let mut f = File::create(&sound_path).unwrap();
    f.write_all(&buff).unwrap();
}


#[test]
fn extract_2da_from_bif() {
    let p = Path::new("./tests/samples/base_2da.bif");
    let parsed = nwn_files::parse_bif(&p).unwrap();

    for (i, r) in parsed.0.iter().enumerate().take(3) {
        let path_string = format!("./tests/outputs/2da/{}.2da", i);
        let plain_text_path = Path::new(&path_string);

        let mut f = File::create(&plain_text_path).unwrap();
        f.write_all(&r.bytes).unwrap();

        assert!(plain_text_path.exists());
    }
}


#[test]
#[ignore]
fn parse_bad_bif() {
    unimplemented!();
}