use std::io::{self, stdout, Write};

pub fn get_input(prompt: String) -> String {
    print!("{}",prompt);
    stdout().flush().expect("");
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    //println!("{}", input.to_string());
    input.trim().to_string()
}

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
