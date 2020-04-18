use nwn_files;
use nwn_files::TlkFile;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

mod helpers;
use helpers::tlk::{parse_from_path, tlk_file_path};


#[test]
fn write_tlk() {
    let parsed = parse_from_path(tlk_file_path(), false).unwrap();
    let path_tlk = Path::new("./tests/outputs/my_tlk.tlk");

    let mut builder = TlkFile::new();

    builder
        .add_entry(parsed.entries[374].clone())
        .add_entry(parsed.entries[376].clone());
    
    {
        let f = File::create(path_tlk.clone()).unwrap();
        let mut writer = BufWriter::new(f);
        
        builder.write(&mut writer).unwrap();
    }

    let my_parsed = parse_from_path(path_tlk, false).unwrap();

    assert_eq!(2, my_parsed.entries.len());
}
