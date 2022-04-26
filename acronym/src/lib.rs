struct Foldy {
    previous_character: char,
    result: String,
}

impl Foldy {
    fn new(c: char, r: String) -> Foldy {
        Foldy {
            previous_character: c,
            result: r,
        }
    }

    fn is_candidate(c: char) -> bool {
        c.is_alphabetic() && c != ' '
    }

    fn has_good_predecessor(c: char, pred: char) -> bool {
        pred == ' ' || pred == '-' || pred == '_' || pred.is_lowercase() && c.is_uppercase()
    }
}

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .chars()
        .fold(Foldy::new(' ', "".to_string()), |acc, c| {
            if Foldy::is_candidate(c) && Foldy::has_good_predecessor(c, acc.previous_character) {
                Foldy::new(c, format!("{}{}", acc.result, c.to_uppercase()))
            } else {
                Foldy::new(c, acc.result)
            }
        })
        .result
}
