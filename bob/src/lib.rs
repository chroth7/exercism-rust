pub fn reply(message: &str) -> &str {
    let is_question = message.trim().ends_with('?');
    let is_yell = message.to_uppercase() == message && message.to_lowercase() != message;
    let has_content = !message.trim().is_empty();

    match (is_question, is_yell, has_content) {
        (_, _, false) => "Fine. Be that way!",
        (false, false, _) => "Whatever.",
        (true, false, _) => "Sure.",
        (false, true, _) => "Whoa, chill out!",
        (true, true, _) => "Calm down, I know what I'm doing!",
    }
}
