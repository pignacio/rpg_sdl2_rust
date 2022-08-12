use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::data::font::FontData;
use crate::data::map::MapData;
use crate::data::text::TextLineData;

use crate::error::Error;

pub mod font;
pub mod gfx;
pub mod map;
pub mod text;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameConfig {
    pub font: FontData,
    pub map: MapData,
    pub text_line: Option<TextLineData>
}

impl Data for GameConfig {
    fn reroot(&mut self, base_path: &Path) {
        self.font.reroot(base_path);
        self.map.reroot(base_path);
        self.text_line.reroot(base_path);
    }

}

pub enum Format {
    JSON,
    BINCODE,
}

pub trait Data {
    fn reroot(&mut self, base_path: &Path);
}

impl<T: Data> Data for Option<T> {
    fn reroot(&mut self, base_path: &Path) {
        self.as_mut().map(|data| data.reroot(base_path));
    }
}

fn join_as_string<P1: AsRef<Path>, P2: AsRef<Path>>(base_path: P1, rel_path: P2) -> String {
    base_path.as_ref().join(rel_path).to_str().unwrap().to_owned()
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

pub fn load_file<P: AsRef<Path>, D: DeserializeOwned>(path: P) -> Result<D, Error> {
    let path_ref = path.as_ref();
    do_load_file(path_ref).map_err(|e| e.with_path(path_ref))
}

fn do_load_file<D: DeserializeOwned>(path: &Path) -> Result<D, Error> {
    deserialize(File::open(path)?, format_for_path(path)?)
}

pub fn write_file<P: AsRef<Path>, S: Serialize>(path: P, value: &S) -> Result<(), Error> {
    let path_ref = path.as_ref();
    do_write_file(path_ref, value).map_err(|e| e.with_path(path_ref))
}

fn do_write_file<S: Serialize>(path: &Path, value: &S) -> Result<(), Error> {
    let format = format_for_path(path)?;
    serialize(value, File::create(path)?, format)
}

fn format_for_path(path: &Path) -> Result<Format, Error> {
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .ok_or(Error::Simple { message: format!("Could not find extension for {:?}", path.to_str()) })?;
    println!("Extension is {}", extension);
    match extension.as_ref() {
        "json" => Ok(Format::JSON),
        "bin" => Ok(Format::BINCODE),
        _ => Err(Error::Simple { message: format!("Could not detect format for extension {}", extension) }),
    }
}