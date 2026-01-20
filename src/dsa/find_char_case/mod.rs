pub fn find_char_case(ch: char) -> String {
    if ch >= 'A' && ch <= 'Z' {
        return "Uppercase!".to_string();
    } else if ch >= 'a' && ch <= 'z' {
        return "Lowercase!".to_string();
    } else {
        return "Invalid Character!".to_string();
    }
}
