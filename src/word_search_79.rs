use std::collections::HashSet;

// 79. Word Search
// A DFS approach is taken for this problem.
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool
{
    let (rows, columns) = (board.len(), board[0].len());
    let word = word.chars().collect::<Vec<char>>();

    let mut visited = HashSet::with_capacity(word.len());

    for i in 0..rows
    {
        for j in 0..columns
        {
            // Look for a cell that matches the first character.
            if board[i][j] == word[0] && dfs(&board, i as i32, j as i32, 0, &word, &mut visited)
            {
                return true;
            }
        }
    }

    // If we reach this point, the search space has been exhausted
    false
}

fn dfs(
    board: &Vec<Vec<char>>,
    row: i32,
    column: i32,
    position: usize,
    word: &Vec<char>,
    visited: &mut HashSet<(i32, i32)>,
) -> bool
{
    if position == word.len()
    {
        return true;
    }
    if visited.contains(&(row, column))
        || row < 0
        || row as usize >= board.len()
        || column < 0
        || column as usize >= board[row as usize].len()
        || board[row as usize][column as usize] != word[position as usize]
    {
        return false;
    }

    // Mark the current position as visited.
    visited.insert((row, column));
    let result = dfs(board, row + 1, column, position + 1, word, visited)
        || dfs(board, row - 1, column, position + 1, word, visited)
        || dfs(board, row, column + 1, position + 1, word, visited)
        || dfs(board, row, column - 1, position + 1, word, visited);

    // We then remove the current cell from the visited set before backtracking occurs.
    visited.remove(&(row, column));

    result
}
