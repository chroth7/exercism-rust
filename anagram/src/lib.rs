use std::collections::HashSet;

fn is_anagram(word: &str, candidate: &str) -> bool {
    let mut chars_1: Vec<char> = word.to_lowercase().chars().collect();
    let mut chars_2: Vec<char> = candidate.to_lowercase().chars().collect();
    chars_1.sort_by(|a, b| a.cmp(b));
    chars_2.sort_by(|a, b| a.cmp(b));
    chars_1.eq(&chars_2)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let input_len = word.chars().count();
    possible_anagrams
        .iter()
        .filter(|candidate| {
            // some quick checks before actually checking is_anagram
            candidate.chars().count() == input_len
                && candidate.to_lowercase() != word.to_lowercase()
                && is_anagram(word, candidate)
        })
        .cloned()
        .collect()
}
