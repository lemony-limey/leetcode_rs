pub fn is_palindrome(s: String) -> bool
{
    let s: String = s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let s_forwards = s.chars();
    let s_backwards = s.chars().rev();

    for (a, b) in s_forwards.zip(s_backwards)
    {
        if a != b
        {
            return false;
        }
    }

    true
}
