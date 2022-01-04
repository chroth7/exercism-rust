use std::collections::HashMap;
use std::hash::Hash;

// inspired by https://exercism.org/tracks/rust/exercises/magazine-cutout/solutions/surfingtomchen

fn create_hashmap<'a, T>(vec: &'a [&T]) -> HashMap<&'a &'a T, i32>
where
    T: Eq + Hash + ?Sized,
{
    vec.iter().fold(HashMap::new(), |mut acc, key| {
        *acc.entry(key).or_insert(0) += 1;
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
