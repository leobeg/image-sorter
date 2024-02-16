use std::{ fs::{self, File}, io::BufReader, path::{Path, PathBuf}};

use chrono::{DateTime, Datelike, TimeZone, Utc};

use crate::error::{ImageParseError, SortError};

use exif::{In, Reader, Tag, Exif};

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
            if !name.contains("-")
            {
                continue;
            }
            let name: Vec<_> = name.split("-").collect();
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



pub fn create_file_list() -> Result<Vec<(PathBuf, DateTime<Utc>)>, Box<dyn std::error::Error>>
{
    let paths = fs::read_dir("./")?;

    let mut file_list: Vec<(PathBuf, DateTime<Utc>)> = Vec::new();

    for entry in paths 
    {
        let entry = entry?;
        let path = entry.path();
        let extension = match path.extension()
        {
            Some(v) => v,
            None => continue,
        };

        if !path.is_file()
        {
            continue;
        }
        
        //println!("Name: {}", path.display());
        if extension == "jpg" || extension == "png" 
        {
            let file_name = path.to_str().ok_or("err").expect("msg");
            let image_date = get_image_date(file_name).expect("wad");
            // let imageDate = match imageDate {
            //     Ok(date) => date,
            //     Err(e) => {
            //         println!("Sadly your image is trash")
            //         cont;
            //     }
            // };

            // println!("Date: {}", image_date);
            let image_date = image_date.replace(" ", ":");
            let image_dates = image_date.split(":").collect::<Vec<_>>();
            
            let dt = Utc.with_ymd_and_hms(image_dates[0].parse::<i32>().unwrap(), image_dates[1].parse::<u32>().unwrap(), image_dates[2].parse::<u32>().unwrap(), image_dates[3].parse::<u32>().unwrap(), image_dates[4].parse::<u32>().unwrap(), image_dates[5].parse::<u32>().unwrap()).unwrap();

            // println!("Year: {}", image_dates[0]);
            // println!("Month: {}", image_dates[1]);
            // println!("Day: {}", image_dates[2]);

            file_list.push((path, dt));
        }
        
    }

    Ok(file_list)
}

pub fn move_images_to_sort() -> Result<Vec<(u32, PathBuf, DateTime<Utc>)>, SortError>
{
    let mut file_list = create_file_list().map_err(|_| SortError::FileList)?;

    file_list.sort_by_key(|date| date.1);

    let mut file_index: Vec<(u32, PathBuf, DateTime<Utc>)> = Vec::new();
    let mut i: u32 = 1;
    for (path, date) in  file_list {
        

        let year = date.year();
        let month = date.month();
        let extension = match path.extension()
        {
            Some(v) => v,
            None => continue,
        };

        let extension = String::from(extension.to_str().ok_or(SortError::Extension)?);
        
        let target_path = Path::new(format!("sort").as_str()).to_path_buf();
        let copy = copy_image(&path, &target_path, format!("{i}-{year}-{month}.{extension}")).map_err(|_| SortError::Copy)?;
        file_index.push((i, copy, date));
        i += 1;
    }

    Ok(file_index)
}

pub fn get_image_date(path: &str) -> Result<String, ImageParseError>
{
    let file = File::open(&path).map_err(|_| ImageParseError::FileOpen)?;
    let exif: Exif = Reader::new().read_from_container(&mut BufReader::new(&file)).map_err(|_| ImageParseError::Exif)?;

    let field: String = match exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) 
    {
        Some(v) => v.display_value().to_string(),
        None => return Err(ImageParseError::NoTag)
    };

    let field = field.replace("-", ":");

    Ok(field)
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
