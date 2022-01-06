use std::collections::HashMap;
use std::hash::Hash;

// inspired by https://exercism.org/tracks/rust/exercises/magazine-cutout/solutions/surfingtomchen

type Counter<T> = HashMap<T, usize>;

// for reference, this is the short signature without the `Item` generic type.
// But I kinda find the long form more readable, for now...
// fn create_hashmap<T>(vec: &[impl ToOwned<Owned = T>]) -> Counter<T>

fn count<Key, Item>(vec: &[Item]) -> Counter<Key>
where
    Key: Eq + Hash,
    Item: ToOwned<Owned = Key>,
{
    vec.iter().fold(HashMap::new(), |mut acc, key| {
        *acc.entry(key.to_owned()).or_insert(0) += 1;
        acc
    })
}

fn check_subset<Key>(inner: Counter<Key>, outer: Counter<Key>) -> bool
where
    Key: Eq + Hash,
{
    inner
        .iter()
        .all(|(key, count)| outer.get(key).unwrap_or(&0) >= count)
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_words = count(magazine);
    let note_words = count(note);

    check_subset(note_words, magazine_words)
}

#[cfg(test)]
mod tests {
    use crate::count;

    #[test]
    fn string_slices() {
        let slices = ["foo", "bar", "foo"];
        let hashmap = count(&slices);

        assert_eq!(hashmap.get("foo"), Some(&2usize));
    }

    #[test]
    fn strings() {
        let strings = [
            String::from("foo"),
            String::from("bar"),
            String::from("foo"),
            String::from("foo"),
        ];
        let hashmap = count(&strings);

        assert_eq!(hashmap.get("foo"), Some(&3usize));
    }
}
