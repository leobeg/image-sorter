use std::{fs, path::PathBuf};

use dialoguer::{console::Style, theme::ColorfulTheme, Input};

use crate::{cli::console_input::ConsoleError, config::Config};


pub struct ConfigWizard;

impl ConfigWizard {
    pub fn prompt() -> Result<Config, ConsoleError> {
        let theme = ColorfulTheme {
            values_style: Style::new().yellow().dim(),
            ..ColorfulTheme::default()
        };
        println!("Welcome to the setup wizard");
    
        let output_folder_input: String = Input::with_theme(&theme)
            .with_prompt("Path to output folder")
            .interact().map_err(|_| ConsoleError::InvalidInput)?;
    
        let input_folder_input: String = Input::with_theme(&theme)
            .with_prompt("Path to input folder")
            .interact().map_err(|_| ConsoleError::InvalidInput)?;
    
        let output_folder: PathBuf = output_folder_input.clone().into();
        let input_folder: PathBuf = input_folder_input.clone().into();
    
        if !output_folder.exists() {
            println!("The output folder does not exist. Creating...");
    
            match fs::create_dir(&output_folder) {
                Ok(()) => println!("Success"),
                Err(err) => {
                    println!("Could not create folder: {:?}", err);
                    return Err(ConsoleError::InvalidInput);
                }
            }
        }

        if !input_folder.exists() {
            println!("The input folder does not exist. Creating...");
    
            match fs::create_dir(&input_folder) {
                Ok(()) => println!("Success"),
                Err(err) => {
                    println!("Could not create folder: {:?}", err);
                    return Err(ConsoleError::InvalidInput);
                }
            }
        }

        let config = Config {
            //rename,
            input_folder: input_folder_input,
            output_folder: output_folder_input,
            reorder_after_sort: false //use_sort_folder,
        };

        config.save();
    
        Ok(config)
    }
}