use leetcode::word_ladder_127::ladder_length;
use leetcode::word_search_79::exist;

fn main()
{
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];

    let result = exist(board, "ABCCED".to_string());
    println!("result: {}", result);
}