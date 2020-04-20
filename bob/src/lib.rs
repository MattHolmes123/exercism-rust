pub fn reply(message: &str) -> &str {
    
    let m = message.trim();

    if m.is_empty() {
        return "Fine. Be that way!"
    }

    let is_question = m.ends_with("?");    
    let has_letters = message.matches(char::is_alphabetic).count() > 0;

    let yelling = has_letters && m.to_uppercase() == m.to_string();

    if is_question && yelling {
        return "Calm down, I know what I'm doing!"
    }

    if is_question {
        return "Sure."
    }

    if yelling {
        return "Whoa, chill out!"
    }
    
    "Whatever."
}
