use std::{
    fs::{self, read_dir},
    path::PathBuf,
};

use super::{FileSystem, FileSystemError};

impl FileSystem {
    fn read_dir_files(base_path: PathBuf) -> Result<Vec<PathBuf>, FileSystemError> {
        let mut entries: Vec<PathBuf> = Vec::new();
        for entry in fs::read_dir(&base_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let metadata = entry.metadata().unwrap();

            if metadata.is_dir() {
                entries.append(&mut Self::read_dir_files(path)?);
            } else if metadata.is_file() {
                entries.push(path);
            }
        }
        Ok(entries)
    }

    pub fn get_dir_content(path: PathBuf) -> Result<Vec<PathBuf>, FileSystemError> {
        let entries: Vec<PathBuf> = Self::read_dir_files(path)?;
        Ok(entries)
    }
}
