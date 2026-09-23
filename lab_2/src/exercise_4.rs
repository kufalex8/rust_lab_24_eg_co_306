//exercise 2.5
fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),

        // TODO 5
        _ => n.to_string(),
    }
}

pub fn main() {
    for i in 1..=50 {
        println!("{}", fizzbuzz(i));
    }
}
