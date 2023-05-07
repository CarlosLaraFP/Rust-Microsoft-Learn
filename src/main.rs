use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let sentence = "Hello world! This is a beautiful world.";

    fn count_words(text: &str) -> HashMap<&str, u32> {
        let mut counts = HashMap::new();
        for word in text.split_whitespace() {
            let word = word.trim_matches(|c| {
                !char::is_alphabetic(c)
            });
            let count = counts.entry(word).or_default();
            *count += 1;
        }
        counts
    }

    let map = count_words(sentence);

    let mut result = String::new();
    for (key, value) in &map {
        result.push_str(&format!("{}: {}\n", key, value));
    }
    println!("{}", result);
}
