pub fn solve_sudoku(board: &mut Vec<Vec<char>>)
{
    // 1. Go through every cell and make a list of all potential candidates.
    let mut candidates: [[Option<u16>; 9]; 9] = [[None; 9]; 9];

    for (i, row) in board.iter().enumerate()
    {
        for (j, cell) in row.iter().enumerate()
        {
            
        }
    }

    // 2. Solve (potentially with backtracking?)
    
}
