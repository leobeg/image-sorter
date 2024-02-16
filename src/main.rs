//use std::fs;

mod error;
mod user_input;
mod filesystem;

use std::{fs, path::Path};

use chrono::Datelike;
use clap::Parser;
use filesystem::{create_file_list, move_images_to_sort, map_target};


use crate::{filesystem::rename_image, user_input::{get_input, parse_number}};

macro_rules! skip_fail {
    ($res:expr, $error_msg:expr) => {
        match $res {
            Ok(val) => val,
            Err(_) => {
                println!("Error: {}", $error_msg);
                continue;
            }
        }
    };
}

macro_rules! skip_none {
    ($res:expr, $error_msg:expr) => {
        match $res {
            Some(val) => val,
            None => {
                println!("Error: {}", $error_msg);
                continue;
            }
        }
    };
}

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,

    /// The path for the file to read
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    //let args = Cli::parse();

    let path = Path::new("source/");
    let mut indexes = map_target(path);

    let file_index = move_images_to_sort().expect("Something went wrong");

    //println!("File list: {:?}", file_index);

    let sort: bool = false;

    while !sort {
        println!("");
        let number_input = get_input("Input image number: ".to_string());
        let image_numbers: Vec<u32> = skip_fail!(parse_number(number_input), "Invalid Number");
        
        let name_input = get_input(format!("Enter a name for selected indexes {:?}: ", image_numbers));

        for number in image_numbers 
        {
            let file = skip_none!(file_index.iter().find(|&x| x.0 == number), format!("Image {number} not found. Skipping..."));

            let year = file.2.year();
            let month =  file.2.month();
            let extension =  file.1.extension().unwrap();

            let extension = String::from(extension.to_str().unwrap());
            let target_path = Path::new(format!("source/{year}/{month}").as_str()).to_path_buf();
            let folder = indexes.iter_mut().find(|x| x.0.as_os_str() == target_path.as_os_str());
            
            let mut index: u32 = 1;

            if folder.is_some()
            {
                let folder = folder.unwrap();
                index = folder.1 + 1;
                folder.1 = index;
            }

            let file_name = format!("{:03}-{year}{:02}-{name_input}.{extension}", index, month);
            let _copy = skip_fail!(rename_image(&file.1, &target_path, file_name), format!("Couldn't rename image {number}. Skipping..."));
            let _rm = skip_fail!(fs::remove_file(&file.1), format!("Couldn't remove image {number} from sort folder"));

            if index == 1
            {
                indexes.push((target_path, 1));
            }
        }

        

        
    }

    //let input = get_input("Input image number: ");


    // ToDo: Implement sort with year month day before assening numbers
    // Concept: After numbers are asigned in sort folder the user can input number ranges to name them. After that the files will get sorted into the corresponding folders


    Ok(())
}

