fn super_find(opener: char, closer: char, subset: &str) -> Option<usize> {
    let mut inner_count = 0;
    let mut index = 0;

    let chars = subset[1..].chars();
    for c in chars {
        index += 1;
        if c == opener {
            inner_count += 1;
        }
        if c == closer && inner_count == 0 {
            return Some(index);
        }
        if c == closer && inner_count > 0 {
            inner_count -= 1;
        }
    }
    None
}

fn find_match_index(first: Option<char>, subset: &str) -> Option<usize> {
    match first {
        Some('[') => super_find('[', ']', subset),
        Some('{') => super_find('{', '}', subset),
        Some('(') => super_find('(', ')', subset),
        _ => None,
    }
}

fn check_two_parts(one: &str, two: &str) -> bool {
    (one.is_empty() || brackets_are_balanced(one)) && (two.is_empty() || brackets_are_balanced(two))
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let subset: String = string
        .chars()
        .filter(|c| ['[', ']', '{', '}', '(', ')'].contains(c))
        .collect();

    // quick checks
    if subset.is_empty() {
        return true;
    }

    if subset.len() % 2 != 0 {
        return false;
    }

    let first = subset.chars().next();
    let last_index = find_match_index(first, &subset);

    if let Some(last_index_checked) = last_index {
        let last = subset.chars().nth(last_index_checked);
        let between = &subset[1..last_index_checked];
        let rest = &subset[last_index_checked + 1..];

        println!(
            "Out: Subset {subset} - between {between} - rest {rest} = {} {}",
            first.unwrap(),
            last.unwrap()
        );
        match (first, last) {
            (Some('['), Some(']')) => check_two_parts(between, rest),
            (Some('{'), Some('}')) => check_two_parts(between, rest),
            (Some('('), Some(')')) => check_two_parts(between, rest),
            (_, _) => false,
        }
    } else {
        false
    }
}
