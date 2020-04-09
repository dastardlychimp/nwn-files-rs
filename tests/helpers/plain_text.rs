use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::fmt::Debug;


#[allow(dead_code)]
pub fn write_plain_text_file<W: Debug, P: AsRef<Path>>(file_path: P, to_write: W) -> std::io::Result<()> {
    let p = Path::new("./tests/outputs/plain_text/").join(file_path);
    let mut f = File::create(p)?;
    write!(f, "{:#?}", to_write)
}