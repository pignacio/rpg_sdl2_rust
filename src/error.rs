use std::path::Path;

#[derive(Debug)]
pub enum Error {
    Simple { message: String },
    Standard { err: Box<dyn std::error::Error> },
    Pathed { path: String, err: Box<Error> },
    SerdeJson { err: serde_json::Error },
    Bincode { err: bincode::Error },
    Io { err: std::io::Error },
}

impl Error {
    pub fn simple<T: AsRef<str>>(message: T) -> Self {
        Error::Simple { message: message.as_ref().to_owned() }
    }

    pub fn with_path<P: AsRef<Path>>(self, path: P) -> Error {
        match path.as_ref().to_str() {
            Some(p) => Error::Pathed { path: p.to_owned(), err: Box::new(self) },
            None => self,
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::Simple { message }
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

macro_rules! from_impl {
    ($t:ty) => {
        impl From<$t> for Error {
            fn from(err: $t) -> Self {
                Error::Standard { err: Box::new(err) }
            }
        }
    }
}

from_impl![serde_json::Error];
from_impl![bincode::Error];
from_impl![std::io::Error];
from_impl![sdl2::render::TextureValueError];
from_impl![sdl2::render::TargetRenderError];
from_impl![sdl2::ttf::FontError];
