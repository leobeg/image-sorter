use std::{error::Error, fs, path::PathBuf};

use dialoguer::{console::Style, theme::ColorfulTheme, Confirm, Input};

#[derive(Debug)]
pub struct Config {

    pub rename: bool,
    pub image_folder: PathBuf,
    pub input_folder: PathBuf,
    pub use_sort_folder: bool,
    
}

// fn save_config_to_fs(path: &PathBuf) -> Result<Option<Config>, ConfigError>
// {
//     let file = File::open("foo.txt")?;
//     let mut buf_reader = BufReader::new(file);
//     let mut contents = String::new();
//     buf_reader.read_to_string(&mut contents)?;
//     assert_eq!(contents, "Hello, world!");
//     Ok()
// }

pub fn init_config() -> Result<Option<Config>, Box<dyn Error>> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    println!("Welcome to the setup wizard");

    if !Confirm::with_theme(&theme)
        .with_prompt("Do you want to continue?")
        .interact()?
    {
        return Ok(None);
    }

    let rename = Confirm::with_theme(&theme)
        .with_prompt("Keep files in sort folder?")
        .interact()?;

    let use_sort_folder = Confirm::with_theme(&theme)
        .with_prompt("Use seperate sort folder?")
        .interact()?;

    let image_folder: String = Input::with_theme(&theme)
        .with_prompt("Path to image folder")
        .interact()?;

    let input_folder: String = Input::with_theme(&theme)
        .with_prompt("Path to input folder")
        .interact()?;

    let image_folder: PathBuf = image_folder.into();
    

    if !image_folder.exists()
    {
        println!("The image folder does not exist. Creating?");

        match fs::create_dir(&image_folder) {
            Ok(()) => println!("Success"),
            Err(err) => {
                println!("Could not create folder: {:?}", err);
                return Ok(None);
            }
        } 
    }

    let input_folder: PathBuf = input_folder.into();

    if !input_folder.exists()
    {
        println!("Input folder does not exist. Exiting...");
        return Ok(None);
    }
    
    Ok(Some(Config {
        rename,
        image_folder,
        input_folder,
        use_sort_folder,
    }))
}