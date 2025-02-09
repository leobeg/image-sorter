use dialoguer::Input;

use crate::{cli::console_input::{ConsoleAction, ConsoleError, ConsoleInput}, sorter::Sorter};



#[derive(Clone)]
pub struct SortAction {
    pub sorter: Sorter,
}

impl ConsoleAction for SortAction {
    fn prompt(&mut self) -> Result<(), ConsoleError> {
        // Setup sorting
        self.sorter
            .start_sorting()
            .map_err(|_| ConsoleError::InvalidInput)?;

        loop {
            let image_numbers = match Self::get_numbers("Input image number") {
                Ok(v) => v,
                Err(err) => {
                    if err == ConsoleError::InterruptAction {
                        break;
                    } else {
                        println!("Invalid Number");
                        continue;
                    }
                }
            };

            let folder_input = Self::get_string_with_default(
                format!("Enter a folder for selected indexes {:?}", image_numbers),
                "Sonstiges".to_string(),
            );

            let name_input = Self::get_string(
                format!("Enter a name for selected indexes {:?}", image_numbers),
                true,
            );

            self.sorter
                .move_from_sort(image_numbers, folder_input, name_input);
        }

        Ok(())
    }
}

impl SortAction {
    pub fn new(sorter: Sorter) -> Self {
        Self { sorter }
    }

    pub fn get_numbers(prompt: &str) -> Result<Vec<u32>, ConsoleError> {
        let number_input: String = Input::new()
            .with_prompt(prompt)
            .interact_text()
            .map_err(|_| ConsoleError::InvalidInput)?;

        if number_input == "q" {
            return Err(ConsoleError::InterruptAction);
        }

        let image_numbers =
            ConsoleInput::parse_numbers(number_input).map_err(|_| ConsoleError::InvalidInput)?;

        Ok(image_numbers)
    }

    pub fn get_string(prompt: String, allow_empty: bool) -> String {
        Input::new()
            .with_prompt(prompt)
            .allow_empty(allow_empty)
            .interact_text()
            .unwrap()
    }

    pub fn get_string_with_default(prompt: String, prompt_default: String) -> String {
        Input::new()
            .with_prompt(prompt)
            .default(prompt_default)
            .interact_text()
            .unwrap()
    }
}
