use std::path::Path;

#[derive(Debug)]
pub enum Error {
    Simple { message: String },
    Pathed { path: String, err: Box<Error> },
    SerdeJson { err: serde_json::Error },
    Bincode { err: bincode::Error },
    Io { err: std::io::Error },
}

impl Error {
    pub fn with_path<P: AsRef<Path>>(self, path: &P) -> Error {
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

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJson { err }
    }
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Self {
        Error::Bincode { err }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io { err }
    }
}

