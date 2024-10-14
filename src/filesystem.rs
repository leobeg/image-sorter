use std::{ fs::{self, File}, io::BufReader, path::{Path, PathBuf}};

use chrono::{DateTime, Datelike, TimeZone, Utc};

use crate::{error::{ImageParseError, SortError}, image_date_helper};

use exif::{In, Reader, Tag, Exif};

fn read_dir_files(entries: &mut Vec<PathBuf>, base_path: PathBuf)
{
    for entry in fs::read_dir(&base_path).unwrap()
    {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = entry.metadata().unwrap();
        
        if metadata.is_dir()
        {
            read_dir_files(entries, path);
        } else if metadata.is_file() {
            entries.push(path);
        }
        
    }
}

fn walk_dir(base_path: &Path) -> Vec<PathBuf>
{
    let mut entries: Vec<PathBuf> = Vec::new();

    let base_path = base_path.to_owned();
    read_dir_files(&mut entries, base_path);

    //println!("Entries {:?}", entries);

    entries
}

fn read_dir(entries: &mut Vec<PathBuf>, base_path: PathBuf) 
{
    let mut is_top = true;
    for entry in fs::read_dir(&base_path).unwrap()
    {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = entry.metadata().unwrap();
        
        if metadata.is_dir()
        {
            read_dir(entries, path);
            is_top = false;
        } 
    }
    if is_top
    {
        entries.push(base_path);
    }
    
}

pub fn map_target(base_path: &Path) -> Vec<(PathBuf, u32)>
{
    let mut entries: Vec<PathBuf> = Vec::new();

    let base_path = base_path.to_owned();
    read_dir(&mut entries, base_path);

    let mut indexes: Vec<(PathBuf, u32)> = Vec::new();

    for path in entries
    {
        let index = get_last_index(&path);
        indexes.push((path, index))
    }
    
    //println!("Entries {:?}", indexes);

    return indexes;
}

pub fn get_last_index(path: &PathBuf) -> u32
{
    let mut entries: Vec<u32> = Vec::new();

    for entry in fs::read_dir(path).unwrap()
    {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = entry.metadata().unwrap();
        if metadata.is_file() {
            let name = path.file_stem().unwrap().to_str().unwrap();
            if !name.contains("_")
            {
                continue;
            }
            let name: Vec<_> = name.split("_").collect();
            let number: u32 = match name[0].parse() {
                Ok(v) => v,
                Err(_) => continue
            }; 
            entries.push(number);
        }
            
    }

    //println!("Indexes: {:?}", entries);

    if entries.len() == 0
    {
        return 0;
    }

    entries.sort();
    return entries.last().unwrap().clone();


    
}



pub fn create_file_list(path: &PathBuf) -> Result<Vec<(PathBuf, DateTime<Utc>)>, Box<dyn std::error::Error>>
{
    let paths = walk_dir(&path);

    let mut file_list: Vec<(PathBuf, DateTime<Utc>)> = Vec::new();
    let mut skip_counter: u32 = 0;

    for path in paths 
    {
        //let entry = entry?;
        //let path = entry.path();
        let extension = match path.extension()
        {
            Some(v) => v,
            None => continue,
        };

        // If path not a file or not jpg or png skip
        if !path.is_file() || !(extension == "jpg" || extension == "png")
        {
            continue;
        }

        let image_date = match image_date_helper::get_image_date(&path)
        {
            Ok(v) => v,
            Err(e) => 
            { 
                println!("Couldn't get date for image \"{}\"  Error: {}", path.display(), e);
                skip_counter += 1;
                continue; 
            }
        };

            //println!("Year: {}",  image_date.year());
            //println!("Month: {}", image_date.month());
            //println!("Day: {}",   image_date.day());

        file_list.push((path, image_date));
        
    }

    if skip_counter > 0
    {
        println!("Skipped {} file(s) because of errors!", skip_counter);
    }
    

    Ok(file_list)
}

pub fn move_images_to_sort(source_path: &PathBuf) -> Result<Vec<(u32, PathBuf, DateTime<Utc>)>, SortError>
{
    let mut file_list = create_file_list(source_path).map_err(|_| SortError::FileList)?;

    file_list.sort_by_key(|date| date.1);

    let mut file_index: Vec<(u32, PathBuf, DateTime<Utc>)> = Vec::new();
    let mut i: u32 = 1;
    for (path, date) in  file_list {
    
        let year = date.year();
        let month = date.month();
        let day = date.day();
        let extension = match path.extension()
        {
            Some(v) => v,
            None => continue,
        };

        let extension = String::from(extension.to_str().ok_or(SortError::Extension)?);
        
        let target_path = Path::new(format!("sort").as_str()).to_path_buf();
        let copy = copy_image(&path, &target_path, format!("{i}_{year}-{month}-{day}.{extension}")).map_err(|_| SortError::Copy)?;
        file_index.push((i, copy, date));
        i += 1;
    }

    Ok(file_index)
}



pub fn copy_image(base_path: &PathBuf, target_path: &PathBuf, file_name: String) -> Result<PathBuf, std::io::Error>
{
    fs::create_dir_all(&target_path)?;
    let target_path = target_path.join(file_name);
    //println!("Created dir {target_path}");
    fs::copy(base_path, &target_path)?;
    //println!("Copy");
    Ok(target_path)
}

pub fn rename_image(base_path: &PathBuf, target_path: &PathBuf, file_name: String) -> Result<PathBuf, std::io::Error>
{
    fs::create_dir_all(&target_path)?;
    //println!("Created dir {target_path}");
    //let target_path = target_path.clone();
    let target_path = target_path.join(file_name);
    fs::rename(base_path, &target_path)?;
    //println!("Renamed");
    Ok(target_path)
}
