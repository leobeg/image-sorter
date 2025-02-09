use std::{
    fs::{self},
    path::PathBuf,
};

use super::{FileSystem, FileSystemError};

impl FileSystem {
    pub fn get_dir_content(base_path: PathBuf) -> Result<Vec<PathBuf>, FileSystemError> {
        let mut entries: Vec<PathBuf> = Vec::new();
        for entry in fs::read_dir(&base_path).map_err(|_| FileSystemError::OSFileSystem)? {
            let entry = entry.map_err(|_| FileSystemError::OSFileSystem)?;
            let path = entry.path();
            let metadata = entry
                .metadata()
                .map_err(|_| FileSystemError::OSFileSystem)?;

            if metadata.is_dir() {
                entries.append(&mut Self::get_dir_content(path)?);
            } else if metadata.is_file() {
                entries.push(path);
            }
        }
        Ok(entries)
    }

    pub fn get_directories(base_path: &PathBuf) -> Result<Vec<PathBuf>, FileSystemError> {
        let mut entries: Vec<PathBuf> = Vec::new();
        let mut is_top = true;
        for entry in fs::read_dir(base_path).map_err(|_| FileSystemError::OSFileSystem)? {
            let entry = entry.map_err(|_| FileSystemError::OSFileSystem)?;
            let path = entry.path();
            let metadata = entry
                .metadata()
                .map_err(|_| FileSystemError::OSFileSystem)?;

            if metadata.is_dir() {
                entries.append(&mut Self::get_directories(&path)?);
                is_top = false;
            }
        }
        if is_top {
            entries.push(base_path.clone());
        }

        Ok(entries)
    }
}
