use std::{error::Error, fs, path::PathBuf};

// use config::{Config, ConfigError};
use dialoguer::{console::Style, theme::ColorfulTheme, Confirm, Input};

#[derive(Debug, Clone)]
pub struct Settings {

    //pub rename: bool,
    pub image_folder: PathBuf,
    pub input_folder: PathBuf,
    //pub use_sort_folder: bool,
    
}

// const CONFIG_FILE_PATH: &str = "./config/Default.toml";
// const CONFIG_FILE_PREFIX: &str = "./config/";

// fn save_config_to_fs(path: &PathBuf) -> Result<Option<Settings>, ConfigError>
// {
//     let settings = Config::builder()
//     // Add in `./Settings.toml`
//     .add_source(config::File::with_name("./Settings"))
//     // Add in settings from the environment (with a prefix of APP)
//     // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
//     .add_source(config::Environment::with_prefix("APP"))
//     .build()
//     .unwrap();

//     Ok(Some((
//         Settings {

//         }
//     )))
// }

pub fn dialog_config() -> Result<Option<Settings>, Box<dyn Error>> {
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
    
    Ok(Some(Settings {
        //rename,
        image_folder,
        input_folder
        //use_sort_folder,
    }))
}