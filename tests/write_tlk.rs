use nwn_files;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use nwn_files::TlkBuilder;

mod helpers;

const TFP: &str = "./tests/samples/dialog.tlk";

fn tlk_file_path() -> &'static Path {
    return Path::new(TFP);
}

#[test]
fn write_tlk() {
    let parsed = nwn_files::parse_tlk(tlk_file_path()).unwrap();
    let path_tlk = Path::new("./tests/outputs/my_tlk.tlk");

    let mut builder = TlkBuilder::new();

    builder
        .add_entry(parsed.entries[374].clone())
        .add_entry(parsed.entries[376].clone());
    
    {
        let f = File::create(path_tlk.clone()).unwrap();
        let mut writer = BufWriter::new(f);
        
        builder.write(&mut writer).unwrap();
    }

    let my_parsed = nwn_files::parse_tlk(path_tlk).unwrap();

    assert_eq!(2, my_parsed.entries.len());
}
