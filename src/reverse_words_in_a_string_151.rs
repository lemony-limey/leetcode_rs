pub fn reverse_words(s: String) -> String
{
    s.trim()
        .split_whitespace()
        .fold(String::new(), |acc, word| format!("{acc} {word}"))
        .trim()
        .to_string()
}
