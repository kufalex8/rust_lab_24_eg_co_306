use std::io;

pub fn run() {
    println!("Enter an expression (e.g. 3 + 4):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() != 3 {
        println!("Invalid input. Use the format: number operator number");
        return;
    }

    let a: f64 = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid first number.");
            return;
        }
    };

    let b: f64 = match parts[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid second number.");
            return;
        }
    };

    let result = match parts[1] {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                println!("Cannot divide by zero.");
                return;
            }
            a / b
        }
        _ => {
            println!("Unknown operator.");
            return;
        }
    };

    println!("Result: {}", result);
}
