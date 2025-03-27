use std::{fs, path::PathBuf};

use chrono::{DateTime, Utc};
use image_date::ImageParseError;
use thiserror::Error;

use crate::cli::console_input::ConsoleInput;

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
    #[error("there is already a image with the same name")]
    DuplicateName,
}

#[derive(Clone, Debug)]
pub struct Image {
    pub path: PathBuf,
    pub extension: String,
    pub date: DateTime<Utc>,
    pub file_name: Option<String>,
}

impl Image {
    pub fn from_path_buf(path: PathBuf) -> Result<Self, ImageError> {
        let extension = path.extension().ok_or(ImageError::ExtensionMissing)?;
        let extension = extension
            .to_os_string()
            .into_string()
            .map_err(|_| ImageError::ExtensionMissing)?;

        // If path not a file or not jpg or png skip
        if !path.is_file() || !(extension == "jpg" || extension == "png") {
            return Err(ImageError::NotAnImage);
        }

        let image_date = Self::get_image_date(&path).map_err(ImageError::InvalidDate)?;

        Ok(Self {
            path,
            extension,
            date: image_date,
            file_name: None,
        })
    }

    pub fn copy_image(
        &self,
        destination: &PathBuf,
        file_name: String,
    ) -> Result<Image, ImageError> {
        fs::create_dir_all(destination).map_err(|_| ImageError::OSFileSystem)?;
        let mut image = self.clone();

        image.path = destination.join(&file_name);
        image.file_name = Some(file_name);

        fs::copy(&self.path, &image.path).map_err(|_| ImageError::OSFileSystem)?;

        Ok(image)
    }

    pub fn move_image(
        mut self,
        destination: &PathBuf,
        file_name: String,
    ) -> Result<Image, ImageError> {
        fs::create_dir_all(destination).map_err(|_| ImageError::OSFileSystem)?;

        let target_path = destination.join(&file_name);

        if target_path.exists() {
            if !ConsoleInput::confirm_action("Do you want to overwrite the existing image?").unwrap() {
                return Err(ImageError::DuplicateName);
            }
        }

        fs::rename(&self.path, &target_path).map_err(|_| ImageError::OSFileSystem)?;

        self.path = target_path;
        self.file_name = Some(file_name);

        Ok(self)
    }
}
