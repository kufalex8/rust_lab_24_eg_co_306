// 4.3 Exercise B — HashMap

use std::collections::HashMap;
pub fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        let clean: String = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();

        if !clean.is_empty() {
            *freq.entry(clean).or_insert(0) += 1;
        }
    }
    freq
}

// TODO 3: Return the top n words by frequency, sorted descending
fn top_n(freq: &HashMap<String, usize>, n: usize) -> Vec<(&String, &usize)> {
    let mut items: Vec<(&String, &usize)> = freq.iter().collect();
    // Sort descending by count; if equal, sort alphabetically by word
    items.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    items.into_iter().take(n).collect()
}

pub fn run() {
    println!("--- Exercise B: HashMap ---");
    let text =
        "the quick brown fox jumps over the lazy dog the fox was very quick and the dog was lazy";
    let freq = word_frequency(text);
    println!("Word frequencies: {:?}", freq);

    println!("\nTop 5 words:");
    for (word, count) in top_n(&freq, 5) {
        println!("{:>10} : {}", word, count);
    }
    println!();
}
