// The compiler needs to know which input reference
// the output reference is tied to.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

// Struct holding a reference — must be annotated
struct Important<'a> {
    content: &'a str,
}

impl<'a> Important<'a> {
    fn summarise(&self) -> &str {
        &self.content[..self.content.len().min(80)]
    }
}

// TODO 1 Solution
fn first_sentence(text: &str) -> &str {
    match text.find('.') {
        Some(index) => &text[..index],
        None => text,
    }
}

pub fn main() {
    let s1 = String::from("long string is long");
    let result;

    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("Longest: {}", result);
    }

    // println!("{}", result); // Would fail — s2 is dropped!

    let article = String::from("Rust 2024 edition brings many improvements...");
    let imp = Important { content: &article };
    println!("Summary: {}", imp.summarise());

    // TODO 1 Test
    let text1 = "Rust is fast. It is also safe.";
    println!("First sentence: {}", first_sentence(text1));

    let text2 = "No dot in this sentence";
    println!("First sentence: {}", first_sentence(text2));
}
