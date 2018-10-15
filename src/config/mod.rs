use config::model::Config;
use core::*;
use serde_yaml as yaml;
use std::fs::File;
use std::path::Path;

mod model;
mod build;

#[derive(Debug, Fail)]
pub enum ConfigError {
    #[fail(display = "IO Error: {}", error)]
    IoError { #[cause] error: std::io::Error },

    #[fail(display = "YAML Error: {}", error)]
    YamlError { #[cause] error: yaml::Error },
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::IoError { error }
    }
}

impl From<yaml::Error> for ConfigError {
    fn from(error: yaml::Error) -> Self {
        ConfigError::YamlError { error }
    }
}

pub fn load<P>(path: P) -> Result<Config, ConfigError> where P: AsRef<Path> {
    let file = File::open(path)?;
    let config = yaml::from_reader(file)?;

    return Ok(config);
}

impl Into<Box<Node>> for Config {
    fn into(self) -> Box<Node> {
        build::Builder::from_config(&self)
    }
}
