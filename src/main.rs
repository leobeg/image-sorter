//use std::fs;

mod config;
mod error;
mod filesystem;
mod user_input;
//mod file_index;

use std::{fs};

use chrono::Datelike;
use clap::Parser;
use config::{dialog_config, Settings};
use filesystem::{map_target, move_images_to_sort};
use user_input::{get_numbers, get_string, get_string_with_default};

use crate::filesystem::rename_image;

#[macro_export]
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

#[macro_export]
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

    let config: Settings = dialog_config()
        .expect("Problem with config")
        .ok_or("Config aborted")
        .unwrap();

    // let theme = ColorfulTheme {
    //     values_style: Style::new().yellow().dim(),
    //     ..ColorfulTheme::default()
    // };

    sort_dialog(&config);

    // let files = create_file_list(&config.input_folder).unwrap();

    // if Confirm::with_theme(&theme).with_prompt("Remove sorted files?").interact().unwrap()
    // {
    //     // for file in files
    //     // {
    //     //     //skip_fail!(fs::remove_file(&file.0), format!("Couldn't remove file at {:?}", &file.0));
    //     // }
    // }

    // ToDo: Add config dialoge to add a config. Config should be written to current folder
    // ToDo: Add nicer println statements with console package
    // ToDo: Add more detailed output
    // Concept: After numbers are asigned in sort folder the user can input number ranges to name them. After that the files will get sorted into the corresponding folders

    Ok(())
}

fn sort_dialog(config: &Settings) {
    //let path = Path::new("source/");
    let mut output_index = map_target(&config.image_folder);

    let input_index = move_images_to_sort(&config.input_folder).expect("Something went wrong");

    fs::create_dir_all("./sort").expect("");
    let _ = opener::open("./sort");
    println!("Found {:?} files!", input_index.len());

    let sort: bool = false;

    while !sort {
        println!("");
        
        let image_numbers = skip_fail!(get_numbers("Input image number"), "Invalid Number");

        let folder_input = get_string_with_default(format!("Enter a folder for selected indexes {:?}",image_numbers), "Sonstiges".to_string());
        
        let name_input = get_string(format!("Enter a name for selected indexes {:?}", image_numbers), false);

        for number in image_numbers {

            // Get file from file index

            let file = skip_none!(
                input_index.iter().find(|&x| x.0 == number),
                format!("Image {number} not found. Skipping...")
            );

            // Get file properties for sorting
            let year = file.2.year();
            let month = file.2.month();
            let day = file.2.day();
            let extension = skip_none!(file.1.extension(), format!("Image {number} extension is broken. Skipping..."));
            let extension = String::from(extension.to_str().unwrap());


            // Construct target path
            let target_path = config
                .image_folder
                .join(year.to_string())
                .join(folder_input.to_string());


            let folder = output_index
                .iter_mut()
                .find(|x| x.0.as_os_str() == target_path.as_os_str());


            // Increment index and set the current index
            let mut index: u32 = 1;

            if let Some(folder) = folder {
                
                folder.1 = folder.1 + 1;
                index = folder.1;
            }

            let mut file_name = format!("{:03}_{year}-{:02}-{:02}.{extension}",index, month, day);

            if !name_input.is_empty()
            {
                file_name = format!("{:03}_{year}-{:02}-{:02}-{name_input}.{extension}",index, month, day);
            }

            skip_fail!(
                rename_image(&file.1, &target_path, file_name),
                format!("Couldn't rename image {number}. Skipping...")
            );
            //skip_fail!(fs::remove_file(&file.1), format!("Couldn't remove image {number} from sort folder"));

            if index == 1 {
                output_index.push((target_path, 1));
            }
        }
    }
}
