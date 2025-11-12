use std::path::PathBuf;

use serde::{Deserialize, Serialize};

fn main() {
    let mut args = std::env::args();
    // Unwrapping is OK here, as UTF-8 Strings can always be converted to PathBufs
    let Some(path) = args.nth(1).map(PathBuf::from) else {
        eprintln!("Please specify the input path");
        return;
    };
    // Unwrapping is Ok as `path` was created from UTF-8 string, and so is the extension
    let extension = path.extension().map(|o| o.to_str().unwrap());
    let file_contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            // `path` was created from an UTF-8 string, so can be converted to one
            eprintln!(
                "Error reading file at path {}: {}",
                path.to_str().unwrap(),
                e
            );
            return;
        }
    };

    let config: Config = if extension.unwrap() == "json" {
        deserialize_config(&JsonDeserializer::new(), &file_contents).unwrap()
    } else {
        deserialize_config(&YmlDeserializer::new(), &file_contents).unwrap()
    };

    println!("Config was: {config:?}");
}

fn deserialize_config<'a>(
    deserializer: &dyn ConfigDeserializer,
    contents: &'a str,
) -> Result<Config<'a>, Error> {
    deserializer.deserialize(contents)
}

/// An imaginary config file
#[derive(Serialize, Deserialize, Debug)]
pub struct Config<'a> {
    port: u16,
    base_url: &'a str,
    s3_path: &'a str,
    database_url: &'a str,
}

#[derive(Debug)]
/// Config deserialization error
pub enum Error {
    /// Something went wrong deserializing JSON
    Json(serde_json::Error),
    /// Something went wrong deserializing YAML
    Yaml(serde_yaml::Error),
}

// Had to rename this bc I didn't like the original name..
// Original name = `DeserializeConfig`
trait ConfigDeserializer {
    /// Deserialize the contents into a `Config`
    fn deserialize<'a>(&self, contents: &'a str) -> Result<Config<'a>, Error>;
}

struct JsonDeserializer {}

impl JsonDeserializer {
    fn new() -> Self {
        Self {}
    }
}

impl ConfigDeserializer for JsonDeserializer {
    fn deserialize<'a>(&self, contents: &'a str) -> Result<Config<'a>, Error> {
        match serde_json::from_str(contents) {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::Json(e)),
        }
    }
}

struct YmlDeserializer {}

impl YmlDeserializer {
    fn new() -> Self {
        Self {}
    }
}

impl ConfigDeserializer for YmlDeserializer {
    fn deserialize<'a>(&self, contents: &'a str) -> Result<Config<'a>, Error> {
        match serde_yaml::from_str(contents) {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::Yaml(e)),
        }
    }
}
