#![allow(non_snake_case)]

use serde::Deserialize;

use crate::requirements::ReqType;

#[derive(Deserialize, Debug, Clone)]
pub struct FileWorld {
    Areas: Vec<FileArea>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileArea {
    Name: String,
    Locations: Vec<FileLocation>,
    Exits: Vec<FileExit>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileLocation {
    Name: String,
    Category: Vec<String>,
    Needs: Vec<FileRequirement>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileExit {
    ConnectedArea: String,
    Needs: ReqType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileRequirement {
    reqType: ReqType,
    args: Vec<NestedRequirement>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum NestedRequirement {
    // This is more of a Union, but serde hates unions.
    RawString(String),
    InnerReq(Vec<FileRequirement>),
}


pub mod json {
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    use crate::file::FileWorld;

    fn read_json_world(json_file: Box<Path>) -> FileWorld {
        let file = File::open(json_file).unwrap();
        let reader = BufReader::new(file);
        return serde_json::from_reader(reader).unwrap();
    }
}

pub mod yaml {
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    use crate::file::FileWorld;

    fn read_yml_world(yml_file: Box<Path>) -> FileWorld {
        let file = File::open(yml_file).unwrap();
        let reader = BufReader::new(file);
        return serde_yaml::from_reader(reader).unwrap();
    }
}
