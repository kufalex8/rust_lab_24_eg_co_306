pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

pub fn is_palindrome(s: &str) -> bool {
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

pub fn run() {
    let celsius = 25.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);

    println!("{}°C = {}°F", celsius, fahrenheit);

    let word = "level";
    println!("Is '{}' a palindrome? {}", word, is_palindrome(word));
}
