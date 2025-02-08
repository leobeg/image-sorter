use std::path::PathBuf;

use chrono::{DateTime, Utc};
use image_date::ImageParseError;
use thiserror::Error;

mod image_date;

#[derive(Error, Debug)]
pub enum ImageError {
    #[error("failed to access the os filesystem")]
    OSFileSystem,
    #[error("image has no extension")]
    ExtensionMissing,
    #[error("provided path is not an image")]
    NotAnImage,
    #[error("could not get date")]
    InvalidDate(ImageParseError),
}

pub struct Image {
    pub path: PathBuf,
    pub date: DateTime<Utc>,
}

impl Image {
    pub fn from_path_buf(path: PathBuf) -> Result<Self, ImageError> {
        let extension = path.extension().ok_or(ImageError::ExtensionMissing)?;

        // If path not a file or not jpg or png skip
        if !path.is_file() || !(extension == "jpg" || extension == "png") {
            return Err(ImageError::NotAnImage);
        }

        let image_date = Self::get_image_date(&path).map_err(|err| ImageError::InvalidDate(err))?;

        Ok(Self {
            path,
            date: image_date,
        })
    }
}
