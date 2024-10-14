// error.rs

use std::fmt::{self};

#[derive(Debug)]
pub enum ImageParseError {
    FileOpen,
    Exif,
    NoTag,
    InvalidName
}

impl std::error::Error for ImageParseError {}

impl fmt::Display for ImageParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageParseError::FileOpen => write!(f, "FileOpen"),
            ImageParseError::Exif => write!(f, "Exif"),
            ImageParseError::NoTag => write!(f, "NoTag"),
            ImageParseError::InvalidName => write!(f, "InvalidName"),
        }
    }
}

#[derive(Debug)]
pub enum SortError {
    Extension,
    Copy,
    FileList,
}

impl std::error::Error for SortError {}

impl fmt::Display for SortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SortError::Extension => write!(f, "Extension Error"),
            SortError::Copy => write!(f, "Copy Error"),
            SortError::FileList => write!(f, "FileList Error"),
        }
    }
}


