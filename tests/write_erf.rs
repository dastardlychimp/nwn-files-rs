use nwn_files;

use std::path::Path;
use std::convert::TryFrom;
use std::fs::File;

use nwn_files::types::{Resource, ResourceType, ResRef, FileType};
use nwn_files::ErfFile;

mod helpers;


#[test]
fn write_wav_file_to_erf() {
    let path = Path::new("./tests/outputs/erf_wave.erf");
    let resource_path = Path::new("./tests/samples/organfinale.wav");

    let data = helpers::file::read_file_to_vec(&resource_path).unwrap();

    let name = ResRef::try_from("organ_wav").unwrap();

    let resource = Resource {
        name: name.clone(),
        data: data,
        resource_type: ResourceType::wav,
    };

    let resource_2 = Resource {
        name: ResRef::try_from("organ_wav_2").unwrap(),
        ..resource.clone()
    };

    let mut f = File::create(path).unwrap();

    ErfFile::new()
        .add_resource(resource)
        .add_resource(resource_2)
        .write(&mut f, FileType::Erf)
        .unwrap();
    

    let mut parsed_file = File::open(&path).unwrap();
    let parsed = ErfFile::parse_from(&mut parsed_file).unwrap();

    assert_eq!(name, parsed.resources[0].name);
}
