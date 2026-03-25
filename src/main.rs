use leetcode::text_justification_68::full_justify;
use leetcode::word_ladder_127::ladder_length;
use leetcode::word_search_79::exist;

fn main()
{
    // let board = vec![
    //     vec!['A', 'B', 'C', 'E'],
    //     vec!['S', 'F', 'C', 'S'],
    //     vec!['A', 'D', 'E', 'E'],
    // ];
    //
    // let result = exist(board, "ABCCED".to_string());
    // println!("result: {}", result);

    let test_case = vec!["This", "is", "an", "example", "of", "text", "justification."]
        .iter()
        .map(|val| val.to_string())
        .collect();
    let result = full_justify(test_case, 16);
    println!("result: {:#?}", result);
}