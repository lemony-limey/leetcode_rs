use std::collections::VecDeque;

pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String>
{
    let max_width = max_width as usize;
    let mut words_remaining: VecDeque<String> = VecDeque::from(words);

    // n words + (n-1) spaces must be less than or equal to max_width

    let mut result: Vec<String> = Vec::new();
    let mut words_in_line: Vec<String> = Vec::new();
    let mut characters_in_line = 0;
    // Keep going while we still have words left over.
    while !words_remaining.is_empty()
    {
        let next_word = words_remaining.pop_front()
            .expect("we have already established there is a word remaining in the \
                queue, so this value always exists");

        // If adding this next word would make the line too large, commit the current line
        // and start the next one.
        if ((characters_in_line + next_word.len()) + (words_in_line.len())) > max_width
        {
            // Format and commit line
            result.push(
                add_padding_to_line(
                    words_in_line,
                    characters_in_line as i32,
                    max_width as i32,
                    false
                )
            );

            words_in_line = Vec::new();
            characters_in_line = 0;
        }

        characters_in_line += next_word.len();
        words_in_line.push(
            next_word
        );
    }

    result.push(
        add_padding_to_line(
            words_in_line,
            characters_in_line as i32,
            max_width as i32,
            true
        )
    );

    result
}

fn add_padding_to_line(words: Vec<String>, characters_in_line: i32, max_width: i32, is_last_line: bool) -> String
{
    let max_width_usize = max_width as usize;

    // Handle the case where there's only one word.
    if words.len() == 1
    {
        // return format!("{word:.0$}", max_width, word = words_in_line[0]);
        return format!("{:<0max_width_usize$.*}", max_width_usize, words[0]);
    }
    if is_last_line
    {
        let temp = words.into_iter()
            .reduce(|acc, word| {
                format!("{acc} {word}")
            }).unwrap();
        return format!("{:<0width$}", temp, width = max_width_usize);
    }

    // if we need to fill k spaces, add 1 space in each gap from left
    // to right.
    // Spaces are added from the left, to the right of all existing words
    // except the last one.
    // if n = 3, s = 5, f(0) = 2, f(1) = 2, f(2) = 1
    // Divide and apply equal amount to all, then distribute remainder?
    // How do we know?
    // (5 / 3) to all, but for f(0) we have 5 % 3 left over.
    // We add an extra 1 if (((5 % 3) - i) > 0): (2 - 0 > 0, 2 - 1 > 0, 2 - 2 !> 0)


    let gap_count = words.len() - 1;
    let spaces_needed = max_width - characters_in_line;
    words.into_iter()
        .enumerate()
        .reduce(|(_, acc), (index, word)| {
            (0, format!("{acc}{:>0width$}{word}", " ", width = (
                    (spaces_needed / gap_count as i32) + ((spaces_needed % gap_count as i32) - index as i32 >= 0) as i32
                ) as usize,
            ))
        }).expect("this is not empty").1
}