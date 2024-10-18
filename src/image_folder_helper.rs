use std::{fs, path::PathBuf};

use chrono::{DateTime, Utc};

use crate::{error::ImageParseError, filesystem::rename_image, image_date_helper::get_image_date, skip_fail, skip_none};

#[derive(Debug, Clone, PartialEq)]
struct SortedImage {
    index: u32,
    date: DateTime<Utc>,
    path: PathBuf
    
}


pub fn resort_folder_index(path: &PathBuf) -> Result<(), ImageParseError>
{
    check_folder(path);

    // if entries.len() == 0
    // {
    //     return 0;
    // }

    // entries.sort();
    Ok(())
}

fn check_folder(path: &PathBuf) -> Result<(), ImageParseError>
{
    let mut entries: Vec<SortedImage> = Vec::new();

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

            let date = get_image_date(&path)?;
            entries.push(SortedImage { index: number, date: date, path: path });
        }
            
    }

    let old_entries = entries.clone();

    println!("Indexes: {:?}", entries);

    entries.sort_by_key(|item| item.date);

    if entries == old_entries
    {
        println!("No changes to reorder");
        return Ok(());
    }

    for (new_index, item) in entries.iter_mut().enumerate()
    {
        let name = item.path.file_name().unwrap().to_str().unwrap();

        // Reconstruct the image path with the new index
        let file_name = format!("tmp_{}", name);

        let new_path = skip_none!(item.path.parent(), "Parrent directory not found").to_path_buf();

        let path = skip_fail!(
            rename_image(&item.path, &new_path, file_name),
            format!("Couldn't rename image. Skipping...")
        );

        item.path = path;
    }

    // Update the indices based on the new sorted order
    for (new_index, item) in entries.iter_mut().enumerate() {
        let index = (new_index as u32) + 1;

        let name = item.path.file_name().unwrap().to_str().unwrap();
        let name = name.replace("tmp_", "");
        // Split the image path into index and the rest (date + extension)
        let parts: Vec<&str> = name.splitn(2, '_').collect();

        // Reconstruct the image path with the new index
        let file_name = format!("{:03}_{}", index, parts[1]);
        let file_debug = file_name.clone();

        let new_path = skip_none!(item.path.parent(), "Parrent directory not found").to_path_buf();

        skip_fail!(
            rename_image(&item.path, &new_path, file_name),
            format!("Couldn't rename image. Skipping...")
        );

        println!("{:?}", new_path.join(file_debug));
    }
    
    
    


    //println!("Indexes: {:?}", entries);

    Ok(())
}