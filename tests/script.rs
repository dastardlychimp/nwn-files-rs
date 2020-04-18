use nwn_files;
use nwn_files::SsfFile;

use std::path::Path;
use std::fs::File;

pub mod helpers;
use helpers::plain_text;

#[test]
fn script() {
    let mut f = File::open(Path::new("./tests/outputs/ssf/c_badger.ssf")).unwrap();
    let parsed = SsfFile::parse_from(&mut f).unwrap();
    plain_text::write_plain_text_file(Path::new("./c_badger.txt"), parsed).unwrap();
}