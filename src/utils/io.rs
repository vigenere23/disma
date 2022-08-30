use std::{fs, path::Path};

use serde::{de::DeserializeOwned, Serialize};
use yaml_merge_keys::merge_keys;
use yaml_rust::{YamlEmitter, YamlLoader};

fn serialize_json<T: Serialize>(object: &T, file_path: &Path) {
    let file_content = serde_json::to_string_pretty(object).unwrap();
    fs::write(file_path, format!("{}\n", file_content)).unwrap();
}

fn deserialize_json<T: DeserializeOwned>(file_path: &Path) -> T {
    let file_content = fs::read_to_string(file_path).unwrap();
    serde_json::from_str(&file_content).unwrap()
}

fn serialize_yaml<T: Serialize + ?Sized>(object: &T, file_path: &Path) {
    let file_content = serde_yaml::to_string(object).unwrap();
    fs::write(file_path, file_content).unwrap();
}

fn deserialize_yaml<T: DeserializeOwned>(file_path: &Path) -> T {
    let file_content = fs::read_to_string(file_path).unwrap();

    let yaml_content = YamlLoader::load_from_str(&file_content).unwrap().remove(0);
    let merged_yaml_content = merge_keys(yaml_content).unwrap();

    let mut merge_yaml_str = String::new();
    YamlEmitter::new(&mut merge_yaml_str)
        .dump(&merged_yaml_content)
        .unwrap();

    serde_yaml::from_str(&merge_yaml_str).unwrap()
}

pub struct Serializer();

impl Serializer {
    pub fn serialize<T: Serialize + ?Sized>(&self, object: &T, file_path: &Path) {
        match file_path.extension() {
            Some(extension) => match extension.to_str() {
                Some("json") => serialize_json(&object, file_path),
                Some("yml") | Some("yaml") => serialize_yaml(object, file_path),
                Some(extension) => panic!("No serializer available for extension {extension}. Supported file types are JSON and YAML."),
                _ => panic!("Invalid characters found in filename."),
            },
            None => panic!("File must have an extension."),
        }
    }
}

pub struct Deserializer();

impl Deserializer {
    pub fn deserialize<T: DeserializeOwned>(&self, file_path: &Path) -> T {
        match file_path.extension() {
            Some(extension) => match extension.to_str() {
                Some("json") => deserialize_json(file_path),
                Some("yml") | Some("yaml") => deserialize_yaml(file_path),
                Some(extension) => panic!("No serializer available for extension {extension}. Supported file types are JSON and YAML."),
                _ => panic!("Invalid characters found in filename."),
            },
            None => panic!("File must have an extension."),
        }
    }
}
