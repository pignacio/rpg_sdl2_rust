use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameConfig {
    pub character: String,
}

pub enum Format {
    JSON,
    BINCODE,
}


pub fn deserialize<R: Read, D: DeserializeOwned>(data: R, format: Format) -> Result<D, Error> {
    Ok(match format {
        Format::JSON => {
            serde_json::from_reader(data)?
        }
        _ => {
            return Err(Error::Simple { message: "Not supported".to_owned() });
        }
    })
}

pub fn serialize<W: Write, S: Serialize>(value: S, writer: W, format: Format) -> Result<(), Error> {
    match format {
        Format::JSON => {
            serde_json::to_writer(writer, &value)?
        }
        _ => {
            bincode::serialize_into(writer, &value)?
        }
    }
    Ok(())
}

pub fn load_file<P: AsRef<Path>, D: DeserializeOwned>(path: &P) -> Result<D, Error> {
    do_load_file(path).map_err(|e| e.with_path(path))
}

fn do_load_file<P: AsRef<Path>, D: DeserializeOwned>(path: &P) -> Result<D, Error> {
    deserialize(File::open(path.as_ref())?, format_for_path(path)?)
}

pub fn write_file<P: AsRef<Path>, S: Serialize>(path: &P, value: &S) -> Result<(), Error> {
    do_write_file(path, value).map_err(|e| e.with_path(path))
}

fn do_write_file<P: AsRef<Path>, S: Serialize>(path: &P, value: &S) -> Result<(), Error> {
    let format = format_for_path(path)?;
    serialize(value, File::create(path.as_ref())?, format)
}

fn format_for_path<P: AsRef<Path>>(path: P) -> Result<Format, Error> {
    let extension = path.as_ref().extension()
        .and_then(|e| e.to_str())
        .ok_or(Error::Simple { message: format!("Could not find extension for {:?}", path.as_ref().to_str()) })?;
    println!("Extension is {}", extension);
    match extension.as_ref() {
        "json" => Ok(Format::JSON),
        "bin" => Ok(Format::BINCODE),
        _ => Err(Error::Simple { message: format!("Could not detect format for extension {}", extension) }),
    }
}