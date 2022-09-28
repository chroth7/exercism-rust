use std::iter::{once, zip};

fn verse(one: &str, two: &str) -> String {
    format!("For want of a {one} the {two} was lost.\n")
}

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        // list.windows(2) is probably even cleaner than zip
        Some(first) => zip(list, &list[1..])
            .map(|(one, two)| verse(one, two))
            .chain(once(format!("And all for the want of a {first}.")))
            .collect(),
    }
}
