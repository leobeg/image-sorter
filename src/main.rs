//use std::fs;

mod error;
mod user_input;
mod filesystem;
mod config;

use chrono::Datelike;
use clap::Parser;
use config::{init_config, Config};
use dialoguer::Input;
use filesystem::{map_target, move_images_to_sort};



use crate::{filesystem::rename_image, user_input::parse_number};

#[macro_export] macro_rules! skip_fail {
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

#[macro_export] macro_rules! skip_none {
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

    let config: Config = init_config().expect("Problem with config").ok_or("Config aborted").unwrap();

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

fn sort_dialog(config: &Config)
{
    //let path = Path::new("source/");
    let mut indexes = map_target(&config.image_folder);

    let file_index = move_images_to_sort(&config.input_folder).expect("Something went wrong");

    let _ = opener::open("./sort");
    println!("File list: {:?}", file_index);

    let sort: bool = false;

    while !sort {
        println!("");
        let number_input = Input::new().with_prompt("Input image number").interact_text().unwrap();
        if number_input == "finish"
        {
            break;
        }
        let image_numbers: Vec<u32> = skip_fail!(parse_number(number_input), "Invalid Number");
        
        let name_input: String = Input::new().with_prompt(format!("Enter a name for selected indexes {:?}", image_numbers)).interact_text().unwrap();

        for number in image_numbers 
        {
            let file = skip_none!(file_index.iter().find(|&x| x.0 == number), format!("Image {number} not found. Skipping..."));

            let year = file.2.year();
            let month =  file.2.month();
            let extension =  file.1.extension().unwrap();

            let extension = String::from(extension.to_str().unwrap());
            let target_path = config.image_folder.join(year.to_string()).join(month.to_string());
            //let target_path = Path::new(format!("source/{year}/{month}").as_str()).to_path_buf();
            let folder = indexes.iter_mut().find(|x| x.0.as_os_str() == target_path.as_os_str());
            
            let mut index: u32 = 1;

            if folder.is_some()
            {
                let folder = folder.unwrap();
                index = folder.1 + 1;
                folder.1 = index;
            }

            let file_name = format!("{:03}-{year}{:02}-{name_input}.{extension}", index, month);
            skip_fail!(rename_image(&file.1, &target_path, file_name), format!("Couldn't rename image {number}. Skipping..."));
            //skip_fail!(fs::remove_file(&file.1), format!("Couldn't remove image {number} from sort folder"));

            if index == 1
            {
                indexes.push((target_path, 1));
            }
        }
    }
}
