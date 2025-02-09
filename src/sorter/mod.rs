use std::{collections::HashMap, path::PathBuf};

use chrono::Datelike;
use thiserror::Error;

use crate::{
    filesystem::{FileSystem, FileSystemError},
    image::Image,
    skip_fail, skip_none,
};

#[derive(Error, Debug)]
pub enum SortingError {
    #[error("could not read images from input")]
    ReadError,
    #[error("filesystem error")]
    FSError(FileSystemError),
}

#[derive(Clone)]
pub struct Sorter {
    pub filesystem: FileSystem,
    pub output_indexes: HashMap<PathBuf, u32>,
    pub sorting_indexes: HashMap<u32, Image>,
}

impl Sorter {
    pub fn new(filesystem: FileSystem) -> Self {
        Self {
            filesystem,
            output_indexes: HashMap::new(),
            sorting_indexes: HashMap::new(),
        }
    }

    pub fn start_sorting(&mut self) -> Result<(), SortingError> {
        self.sorting_indexes = self
            .filesystem
            .move_images_to_sort()
            .map_err(|_| SortingError::ReadError)?;

        self.filesystem.open_sort().map_err(SortingError::FSError)?;

        println!("Found {:?} files!", self.sorting_indexes.len());

        self.output_indexes = self
            .filesystem
            .create_output_index_list()
            .map_err(SortingError::FSError)?;
        Ok(())
    }

    pub fn move_from_sort(&mut self, indexes: Vec<u32>, folder_name: String, name: String) {
        for index in indexes {
            let image = skip_none!(
                self.sorting_indexes.remove(&index),
                format!("Image {index} not found. Skipping...")
            );

            let target_path = self
                .filesystem
                .output_folder_path
                .join(image.date.year().to_string())
                .join(&folder_name);

            let folder_index = self.output_indexes.get_mut(&target_path);

            let folder_index = match folder_index {
                Some(folder_index) => {
                    *folder_index += 1;
                    *folder_index
                }
                None => 1u32,
            };

            let file_name: String = if !name.is_empty() {
                format!(
                    "{:03}_{}-{:02}-{:02}-{name}.{}",
                    folder_index,
                    image.date.year(),
                    image.date.month(),
                    image.date.day(),
                    image.extension
                )
            } else {
                format!(
                    "{:03}_{}-{:02}-{:02}.{}",
                    folder_index,
                    image.date.year(),
                    image.date.month(),
                    image.date.day(),
                    image.extension
                )
            };

            skip_fail!(
                image.move_image(&target_path, file_name),
                format!("Couldn't rename image {index}. Skipping...")
            );

            // Insert the path to indexes
            if folder_index == 1 {
                self.output_indexes.insert(target_path, folder_index);
            }
        }
    }
}
