pub fn raindrops(n: u32) -> String {
    let is_factor = |factor| n % factor == 0;
    let mut result = String::new();

    let mut push_sound = |letter| {
        result.push_str("Pl");
        result.push_str(letter);
        result.push_str("ng");
    };

    if is_factor(3) {
        push_sound("i");
    }
    if is_factor(5) {
        push_sound("a");
    }
    if is_factor(7) {
        push_sound("o");
    }
    if !result.is_empty() {
        return result;
    }
    n.to_string()
}
