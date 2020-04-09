use nwn_files;

use std::path::Path;
use std::convert::TryFrom;
use std::fs::File;

use nwn_files::SsfBuilder;
use nwn_files::types::{SsfEntry, ResRef};

mod helpers;

const SFP: &str = "./tests/outputs/ssf/000-my-blah.ssf";

fn ssf_file_path() -> &'static Path {
    return Path::new(SFP);
}


#[test]
fn write_ssf() {
    let mut ssf_array = Vec::with_capacity(30);
    ssf_array.resize_with(30, Default::default);
    let res_ref = ResRef::try_from("organ_finale").unwrap();

    ssf_array[0] = SsfEntry {
        res_ref: res_ref.clone(),
        string_ref: None,
    };

    let mut builder = SsfBuilder::new();

    ssf_array
        .into_iter()
        .for_each(|entry| {
            builder.add_entry(entry);
        });

    let mut f = File::create(ssf_file_path()).unwrap();

    builder.write(&mut f).unwrap();

    let parsed = nwn_files::parse_ssf(ssf_file_path()).unwrap();

    assert_eq!(30, parsed.0.len());
    assert_eq!(res_ref, parsed.0[0].res_ref);
}
