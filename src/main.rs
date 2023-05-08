use std::collections::HashMap;
use rayon::prelude::*;

fn main() {
    let sentence = "Hello world? This is a _beautiful_ world. !This is my home!";

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

    let result = map
        .iter()
        .map(|(key, value)| format!("{}: {}", key, value))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", result);

    fn sum_of_squares(input: &[i32]) -> i32 {
        input
            .iter()
            .map(|&i| i * i)
            .sum()
    }

    let slice = &[1, 2, 3];
    let expected = &14;
    // Shadowing results in a memory leak? I think Rust is smart enough to know, otherwise it would not compile.
    let result = &sum_of_squares(slice);
    assert!(result.eq(expected));

    fn sum_of_squares_par(input: &[i32]) -> i32 {
        input
            .par_iter()
            .map(|&i| i * i)
            .sum()
    }

    let par_result_commutative = &sum_of_squares_par(slice);

    assert!(par_result_commutative.eq(expected));
}
