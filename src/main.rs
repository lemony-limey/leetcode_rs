use leetcode::word_ladder::ladder_length;

fn main()
{
    let begin_word = String::from("hit");
    let end_word = String::from("cog");
    let word_list: Vec<String> = vec![
          "hot".to_string(),
          "dot".to_string(),
          "dog".to_string(),
          "lot".to_string(),
          "log".to_string(),
          "cog".to_string(),
    ];

    ladder_length(
        begin_word,
        end_word,
        word_list,
    );
}