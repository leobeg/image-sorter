//use std::fs;

mod cli;
mod config;
mod filesystem;
mod image;
mod macros;
mod sorter;

//mod file_index;

use cli::{actions::config_wizard_action::ConfigWizardAction, console_input::{ConsoleInput, UserAction}};
use config::Config;


struct ImageSorter {
    pub console_input: ConsoleInput,
}

impl ImageSorter {
    pub fn new() -> Self {

        let config = Config::load();

        let config = if config.is_some() {
            config.unwrap()
        } else {
            ConfigWizardAction::prompt().expect("No valid config")
        };

        Self {
            console_input: ConsoleInput::new(config),
        }
    }

    pub fn start(&mut self) {
        loop {
            let console_result = self.console_input.prompt_user_action(UserAction::Sorting);

            if let Err(console_err) = console_result {
                println!("Console error: {}", console_err)
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    //let args = Cli::parse();

    

    let mut imager_sorter = ImageSorter::new();

    imager_sorter.start();

    // ToDo: Add config dialoge to add a config. Config should be written to current folder
    // ToDo: Add nicer println statements with console package
    // ToDo: Add more detailed output
    // Concept: After numbers are asigned in sort folder the user can input number ranges to name them. After that the files will get sorted into the corresponding folders

    Ok(())
}
