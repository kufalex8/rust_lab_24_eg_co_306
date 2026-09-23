//execise 1.5
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// TODO 6 Solution
fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

pub fn run() {
    println!("{}", add(3, 7));

    println!("{}", greet("Rustacean"));

    // TODO 6
    println!("factorial(10) = {}", factorial(10));
}
