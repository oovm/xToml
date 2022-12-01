use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum XTomlError {
    IoError(std::io::Error),
}


impl Debug for XTomlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for XTomlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for XTomlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self { XTomlError::IoError(e) => { Some(e) } }
    }
}