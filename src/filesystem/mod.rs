use std::{path::PathBuf, vec};

use thiserror::Error;

use crate::image::Image;

mod filetree;

#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("failed to access the os filesystem")]
    OSFileSystem,
}

pub struct FileSystem {
    pub input_folder_path: PathBuf,
    pub output_folder_path: PathBuf,
    pub sort_folder_path: PathBuf,
}

impl FileSystem {

    
    pub fn new(input_folder_path: PathBuf, output_folder_path: PathBuf, sort_folder_path: PathBuf) -> Self {
        Self { input_folder_path, output_folder_path, sort_folder_path }
    }
    
    pub fn read_input_dir(&self) -> Result<Vec<Image>, FileSystemError> {
        let paths = Self::get_dir_content(self.input_folder_path.clone())?;
        let mut images: Vec<Image> = vec![];
        let mut skip_counter: u32 = 0;

        for path in paths {
            let image = match Image::from_path_buf(path.clone()) {
                Ok(v) => v,
                Err(err) => {
                    println!(
                        "Couldn't get date for image \"{}\"  Error: {}",
                        path.display(),
                        err
                    );
                    skip_counter += 1;
                    continue;
                }
            };
            images.push(image);
        }

        if skip_counter > 0 {
            println!("Skipped {} file(s) because of errors!", skip_counter);
        }

        Ok(images)
    }

    pub fn get_last_index_of_folder(_folder: String) -> u32 {
        0
    }
}
