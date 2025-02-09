use std::path::Path;

use thiserror::Error;

use crate::{config::Config, filesystem::FileSystem, sorter::Sorter};

use super::actions::sort_action::SortAction;



#[derive(Error, Debug, PartialEq)]
pub enum ConsoleError {
    #[error("user input was invalid")]
    InvalidInput,
    #[error("user input was aborted")]
    InterruptAction,
    #[error("action was not set up yet")]
    ActionNotSetup,
}

pub trait ConsoleAction {
    fn prompt(&mut self) -> Result<(), ConsoleError>;
}

#[derive(Hash, PartialEq, Eq)]
pub enum UserAction {
    Sorting,
}

pub struct ConsoleInput {
    //pub current_state: Option<UserAction>,
    pub config: Config,
    pub sort_action: Option<SortAction>,
}

impl ConsoleInput {
    pub fn prompt_user_action(&mut self, action: UserAction) -> Result<(), ConsoleError> {
        match action {
            UserAction::Sorting => {
                if self.sort_action.is_none() {
                    let config = self.config.clone();
                    let input_folder = Path::new(config.input_folder.as_str()).to_path_buf();
                    let output_folder = Path::new(config.output_folder.as_str()).to_path_buf();

                    let filesystem = FileSystem::new(
                        input_folder,
                        output_folder,
                        Path::new("./sort").to_path_buf(),
                    );
                    let sorter = Sorter::new(filesystem);
                    let mut sort_action = SortAction::new(sorter);
                    sort_action.prompt()?;
                    self.sort_action = Some(sort_action);
                    return Ok(());
                }
                self.sort_action.as_mut().ok_or(ConsoleError::ActionNotSetup)?.prompt()?;
            }
        };
        Ok(())
    }

    pub fn new(config: Config) -> Self {
        Self {
            config,
            sort_action: None
        }
    }


    pub fn parse_numbers(number_input: String) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        let mut image_numbers: Vec<u32> = Vec::new();

        let parts: Vec<_> = number_input.split(",").collect();
        for part in parts {
            let numbers: Vec<_> = part.split("-").collect();
            let number1: u32 = numbers[0].parse()?;

            let number2: u32 = if numbers.len() == 1 {
                number1
            } else {
                numbers[1].parse()?
            };

            for n in number1..=number2 {
                image_numbers.push(n);
            }
        }

        Ok(image_numbers)
    }
}
