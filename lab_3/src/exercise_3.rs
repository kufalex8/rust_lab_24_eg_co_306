use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    OutOfRange { value: i32, min: i32, max: i32 },
    EmptyInput,
    DivisibleByZero,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "Parse error: {}", e),

            AppError::OutOfRange { value, min, max } => {
                write!(f, "{} is not in [{}, {}]", value, min, max)
            }

            AppError::EmptyInput => {
                write!(f, "Input was empty")
            }

            AppError::DivisibleByZero => {
                write!(f, "Division by zero is not allowed")
            }
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

fn parse_and_validate(s: &str, min: i32, max: i32) -> Result<i32, AppError> {
    if s.is_empty() {
        return Err(AppError::EmptyInput);
    }

    let n: i32 = s.trim().parse()?;

    if n < min || n > max {
        return Err(AppError::OutOfRange { value: n, min, max });
    }

    Ok(n)
}

// TODO 4
fn safe_div(a: i32, b: i32) -> Result<i32, AppError> {
    if b == 0 {
        return Err(AppError::DivisibleByZero);
    }

    Ok(a / b)
}

pub fn main() {
    let test_cases = vec!["42", "101", "abc", "", "-5"];

    for case in test_cases {
        match parse_and_validate(case, 0, 100) {
            Ok(n) => println!("Valid: {}", n),
            Err(e) => println!("Error for '{}': {}", case, e),
        }
    }

    println!();

    match safe_div(20, 4) {
        Ok(result) => println!("20 / 4 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match safe_div(20, 0) {
        Ok(result) => println!("20 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
