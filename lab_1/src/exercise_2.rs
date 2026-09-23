//execise 1.4
pub fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("s2 = {}", s2);

    let s3 = String::from("world");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);

    let s5 = String::from("Rust is great");

    let length = calculate_length(&s5);

    println!("\"{}\" has {} characters", s5, length);

    // TODO 5
    let word = first_word(&s5);
    println!("First word: {}", word);
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

//  Solution for TODO 5
fn first_word(s: &str) -> &str {
    if let Some(pos) = s.find(' ') {
        &s[..pos]
    } else {
        s
    }
}
