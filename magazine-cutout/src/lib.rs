use std::collections::HashMap;
use std::hash::Hash;

// inspired by https://exercism.org/tracks/rust/exercises/magazine-cutout/solutions/surfingtomchen

fn create_hashmap<T>(vec: &[T]) -> HashMap<T, usize>
where
    T: Eq + Hash + Copy,
{
    vec.iter().fold(HashMap::new(), |mut acc, key| {
        *acc.entry(*key).or_insert(0) += 1;
        acc
    })
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_words = create_hashmap(magazine);
    let note_words = create_hashmap(note);

    note_words
        .iter()
        .all(|(word, count)| magazine_words.get(word).unwrap_or(&0) >= count)
}
