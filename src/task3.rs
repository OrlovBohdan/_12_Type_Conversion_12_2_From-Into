#[test]

/*
use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    // IMPLEMENT from method
}

impl From<num::ParseIntError> for CliError {
    // IMPLEMENT from method
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? automatically converts io::Error to CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() {
    println!("Success!");
}
*/

fn main() {
    // Наприклад, тут можна викликати функцію open_and_parse_file
    match open_and_parse_file("example.txt") {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => match e {
            CliError::IoError(err) => eprintln!("IO Error: {}", err),
            CliError::ParseError(err) => eprintln!("Parse Error: {}", err),
        },
    }

    println!("Success!");
}


use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> Self {
        CliError::ParseError(error)
    }
}

// Функція не використовується, додайте її виклик в main, якщо потрібно
fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    let contents = fs::read_to_string(&file_name)?;
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}






/*
impl From<io::Error> for CliError: Реалізуємо метод from, який приймає об’єкт типу io::Error і повертає значення типу CliError::IoError.
impl From<num::ParseIntError> for CliError: Реалізуємо метод from, який приймає об’єкт типу num::ParseIntError і повертає значення типу CliError::ParseError.
*/