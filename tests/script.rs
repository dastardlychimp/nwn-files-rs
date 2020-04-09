use nwn_files;

use std::path::Path;

pub mod helpers;
use helpers::plain_text;

#[test]
fn script() {
    let parsed = nwn_files::parse_ssf(Path::new("./tests/outputs/ssf/c_badger.ssf")).unwrap();
    plain_text::write_plain_text_file(Path::new("./c_badger.txt"), parsed).unwrap();
}