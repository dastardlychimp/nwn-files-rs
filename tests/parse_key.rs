use nwn_files;

use nwn_files::types::{ResourceType};

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub mod helpers;
use helpers::extract;
use helpers::plain_text;

const KFP: &str = "./tests/samples/nwn_base.key";

fn key_file_path() -> &'static Path {
    return Path::new(KFP);
}

#[test]
fn parse() {
    assert!(key_file_path().exists());

    nwn_files::parse_key(key_file_path()).unwrap();
}

#[test]
fn parse_to_plain_text() {
    let plain_text_path = Path::new("./plain_text_nwn_base.txt");

    let parsed = nwn_files::parse_key(key_file_path()).unwrap();
    plain_text::write_plain_text_file(&plain_text_path, parsed).unwrap();
}

#[test]
fn extract_soundsets_2da() {
    let soundset_bytes = extract::bif_resource_by_name(key_file_path(), "soundset");

    let path_soundset = Path::new("./tests/outputs/soundset.2da");

    let mut f = File::create(path_soundset).unwrap();

    f.write_all(&soundset_bytes).unwrap();
}

#[test]
fn extract_ssf() {
    let soundset_bytes = extract::bif_resource_by_name(key_file_path(), "vs_fx0psycm");

    let path_soundset = Path::new("./tests/outputs/psychopath.ssf");

    let mut f = File::create(path_soundset).unwrap();

    f.write_all(&soundset_bytes).unwrap();
}

#[test]
// #[ignore] // TODO: Probably an error with trying to put all the bytes into a single vec.
fn extract_all_ssf() {
    let resources = extract::bif_resources_by_type(key_file_path(), ResourceType::ssf);

    for r in resources {
        let path_string = format!("./tests/outputs/ssf/{}.ssf", *r.name);
        let ssf_path = Path::new(&path_string);

        let mut f = File::create(&ssf_path).unwrap();
        f.write_all(&r.data).unwrap();
        
        assert!(ssf_path.exists());
    }
}

#[test]
#[ignore]
fn parse_bad_key() {
    unimplemented!();
}