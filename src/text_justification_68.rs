pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String>
{
    let mut words_remaining = words;

    // n words + (n-1) spaces must be less than or equal to max_width
    // if we need to fill k spaces, add 1 space in each gap from left
    // to right.
    // Spaces are added from the left, to the right of all existing words
    // except the last one.

    // if n = 3, s = 5, f(0) = 2, f(1) = 2, f(2) = 1
    // Divide and apply equal amount to all, then distribute remainder?
    // How do we know?
    // (5 / 3) to all, but for f(0) we have 5 % 3 left over.
    // We add an extra 1 if 5 % 3 - i > 0 (2 - 0 > 0, 2 - 1 > 0, 2 - 2 !> 0)

    // Keep going while we still have words left over.
    while !words_remaining.is_empty()
    {

    }

    todo!()
}