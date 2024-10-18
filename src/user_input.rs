
use dialoguer::Input;

pub fn parse_number(number_input: String) -> Result<Vec<u32>, Box<dyn std::error::Error>>
{
    let mut image_numbers: Vec<u32> = Vec::new();

    let parts: Vec<_> = number_input.split(",").collect();
    for part in parts
    {
        let numbers: Vec<_> = part.split("-").collect();
        let number1: u32 =  numbers[0].parse()?;

        let number2: u32;
        if numbers.len() == 1
        {
            number2 = number1;
        } else {
            number2 = numbers[1].parse()?;
        }
        

        for n in number1..=number2
        {
            image_numbers.push(n);
        }
    }

    Ok(image_numbers)

}

pub fn get_numbers(prompt: &str) -> Result<Vec<u32>, Box<dyn std::error::Error>>
{
    let number_input = Input::new()
            .with_prompt(prompt)
            .interact_text()
            .unwrap();
    
    let image_numbers = parse_number(number_input);

    return image_numbers;
}

pub fn get_string(prompt: String, allow_empty: bool) -> String
{
    Input::new()
        .with_prompt(prompt)
        .allow_empty(allow_empty)
        .interact_text()
        .unwrap()
}

pub fn get_string_with_default(prompt: String, prompt_default: String) -> String
{
    Input::new()
        .with_prompt(prompt)
        .default(prompt_default)
        .interact_text()
        .unwrap()
}
