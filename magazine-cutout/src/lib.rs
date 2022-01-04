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

fn check_subset<T>(inner: HashMap<T, usize>, outer: HashMap<T, usize>) -> bool
where
    T: Eq + Hash,
{
    inner
        .iter()
        .all(|(key, count)| outer.get(key).unwrap_or(&0) >= count)
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_words = create_hashmap(magazine);
    let note_words = create_hashmap(note);

    check_subset(note_words, magazine_words)
}
