#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// so it seems `windows` is cleaner, but that's maybe easier to read.
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let a_in_b = is_sublist(_first_list, _second_list);
    let b_in_a = is_sublist(_second_list, _first_list);
    match (a_in_b, b_in_a) {
        (true, true) => Comparison::Equal,
        (false, true) => Comparison::Superlist,
        (true, false) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}

fn is_sublist<T: PartialEq>(_first_list: &[T], mut _second_list: &[T]) -> bool {
    // some quick checks
    if _first_list.is_empty() {
        return true;
    }
    if _first_list.len() > _second_list.len() {
        return false;
    }

    // full check
    while !_second_list.is_empty() {
        if _second_list.starts_with(_first_list) {
            return true;
        }
        _second_list = &_second_list[1..]
    }
    false
}
