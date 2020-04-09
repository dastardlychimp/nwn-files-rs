use std::path::Path;
use std::convert::TryFrom;

use crate::nwn_files;

use nwn_files::types::{ResourceType, Resource, ResRef};
use nwn_files::BifFile;
use nwn_files::BifFile2;
use nwn_files::BifResource;

#[allow(dead_code)]
pub fn bif_resource_by_index<P: AsRef<Path> + Copy>(file_path: P, idx: usize) -> Vec<u8> {
    let parsed = nwn_files::parse_bif(file_path).unwrap();
    let resource = &parsed.0[idx];

    resource.bytes.clone()
}

#[allow(dead_code)]
pub fn bif_resource_by_name<P: AsRef<Path>, S: AsRef<str>>(file_path: P, name: S) -> Vec<u8>
{
    let parsed = nwn_files::parse_key(file_path).unwrap();

    let (bif_file_name, r) = parsed.0
        .iter()
        .find_map(|biffile| {
            biffile
                .resources
                .iter()
                .find(|r| &r.name == name.as_ref())
                .map(|r| Some((biffile.name.clone(), r)))
        })
        .unwrap()
        .unwrap();
    
    let bif_file_name = Path::new(&bif_file_name)
        .file_name()
        .unwrap();

    let path_bif = Path::new("./tests/samples/").join(bif_file_name);

    assert!(&path_bif.exists());
    
    let parsed = nwn_files::parse_bif(&path_bif).unwrap();

    parsed.0[r.resource_index].bytes.clone()
}

#[allow(dead_code)]
pub fn bif_resources_by_type<P: AsRef<Path>>(file_path: P, resource_type: ResourceType)
    -> Vec<Resource>
{
    let parsed = nwn_files::parse_key(file_path).unwrap();

    let resources = parsed.0
        .iter()
        .filter_map(|biffile| { 
            let resources: Vec<&BifResource> = biffile
                .resources
                .iter()
                .filter(|r| r.resource_type == resource_type)
                .collect();
            
            if resources.len() > 0
            {
                let parsed_bif = parse_bif_file(&biffile);

                let resources: Vec<Resource> = resources
                    .into_iter()
                    .map(|r| {
                        let data = parsed_bif.0[r.resource_index].bytes.clone();

                        Resource {
                            resource_type: r.resource_type.clone(),
                            data: data,
                            name: ResRef::try_from(r.name.clone()).unwrap(),
                        }
                    })
                    .collect();

                Some(resources)

            } else 
            {
                None
            }
        })
        .flatten()
        .collect();

    resources
}

#[allow(dead_code)]
fn parse_bif_file(bf: &BifFile) -> BifFile2 {
    let bif_file_name = Path::new(&bf.name)
        .file_name()
        .unwrap();

    let path_bif = Path::new("./tests/samples/")
        .join(bif_file_name);

    assert!(&path_bif.exists(), format!("{} does not exist.", &path_bif.as_path().to_string_lossy()));

    nwn_files::parse_bif(&path_bif).unwrap()
}