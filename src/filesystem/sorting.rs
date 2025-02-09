use std::{collections::HashMap, fs, path::PathBuf};

use chrono::Datelike;

use crate::image::Image;

use super::{FileSystem, FileSystemError};

impl FileSystem {
    pub fn move_images_to_sort(&self) -> Result<HashMap<u32, Image>, FileSystemError> {
        let mut images = self.read_images_from_input_dir()?;
        images.sort_by_key(|image| image.date);

        let mut index_map: HashMap<u32, Image> = HashMap::new();
        let mut index: u32 = 1;
        for image in images {
            let filename = format!(
                "{index}_{}-{}-{}.{}",
                image.date.year(),
                image.date.month(),
                image.date.day(),
                image.extension
            );
            let image_in_sort = image
                .copy_image(&self.sort_folder_path, filename)
                .map_err(|_| FileSystemError::OSFileSystem)?;
            index_map.insert(index, image_in_sort);
            index += 1;
        }

        Ok(index_map)
    }

    pub fn create_output_index_list(&self) -> Result<HashMap<PathBuf, u32>, FileSystemError> {
        let directories = Self::get_directories(&self.output_folder_path)?;
        let mut index_map: HashMap<PathBuf, u32> = HashMap::new();
        for dir in directories {
            let index = Self::get_last_index_of_folder(&dir)?;
            index_map.insert(dir, index);
        }

        Ok(index_map)
    }

    fn get_last_index_of_folder(folder: &PathBuf) -> Result<u32, FileSystemError> {
        let mut indexes: Vec<u32> = Vec::new();
        let entries = fs::read_dir(folder).map_err(|_| FileSystemError::OSFileSystem)?;

        for entry in entries {
            let entry = entry.map_err(|_| FileSystemError::OSFileSystem)?;
            let metadata = entry
                .metadata()
                .map_err(|_| FileSystemError::OSFileSystem)?;

            // Skip dirs
            if !metadata.is_file() {
                continue;
            }

            let path = entry.path();
            let name = path
                .file_stem()
                .ok_or(FileSystemError::OSFileSystem)?
                .to_str()
                .ok_or(FileSystemError::OSFileSystem)?;

            // Skip files not containing the delimiter
            if !name.contains("_") {
                continue;
            }

            let name: Vec<_> = name.split("_").collect();
            let number: u32 = match name[0].parse() {
                Ok(v) => v,
                Err(_) => continue,
            };
            indexes.push(number);
        }

        if indexes.is_empty() {
            return Ok(0);
        }

        indexes.sort();

        Ok(*indexes.last().unwrap())
    }
}
