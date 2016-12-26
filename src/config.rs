extern crate toml;

use std;
use std::io::prelude::*;
use std::fs::File;


#[derive(Debug)]
pub enum Error {
    FileReadError,
    TomlError
}

impl From<std::io::Error> for Error {
    fn from(_err: std::io::Error) -> Error {
        Error::FileReadError
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub image: String,
    pub version: String,
    pub root: String,
}

impl Config {
    pub fn new() -> Result<Config, Error> {
        // @TODO do something about this path
        let mut path = std::env::current_dir()
            .unwrap()
            .as_path()
            .join("config.toml");
        println!("{:?}", path);
        let mut file = File::open(path)?;

        let mut s = String::new();
        file.read_to_string(&mut s)?;

        match toml::decode_str::<Config>(&s) {
            Some(c) => Ok(c),
            None => Err(Error::TomlError)
        }
    }
}
