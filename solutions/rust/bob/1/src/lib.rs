pub fn reply(message: &str) -> &str {
    if is_empty(message) {
        return "Fine. Be that way!";
    }

    if is_all_caps(message) && is_a_question(message) {
        return "Calm down, I know what I'm doing!";
    }

    if is_all_caps(message) {
        return "Whoa, chill out!";
    }

    if is_a_question(message) {
        return "Sure.";
    }

    "Whatever."
}

fn is_empty(message: &str) -> bool {
    message.trim().is_empty()
}

fn is_all_caps(message: &str) -> bool {
    message.chars().any(|c| c.is_alphabetic())
        && message
            .chars()
            .filter(|&c| c.is_alphabetic())
            .all(|c| c.is_uppercase())
}

fn is_a_question(message: &str) -> bool {
    message.trim_end().ends_with('?')
}
