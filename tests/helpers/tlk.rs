
use nwn_files;
use nwn_files::TlkFile;
use nwn_files::types::Error as MyError;

use std::io::BufReader;
use std::path::Path;
use std::fs::File;

const TFP: &str = "./tests/samples/dialog.tlk";

#[allow(dead_code)]
pub fn tlk_file_path() -> &'static Path {
    return Path::new(TFP);
}

#[allow(dead_code)]
pub fn parse_from_path<P: AsRef<Path>>(path: P, alternative: bool) -> Result<TlkFile, MyError> {
    assert!(path.as_ref().exists());

    let f = File::open(path).unwrap();
    let mut reader = BufReader::new(f);
    TlkFile::parse_from(&mut reader, alternative)
}