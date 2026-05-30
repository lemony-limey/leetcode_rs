pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool
{
    // We know the board is 9x9, and that 1-9 should only appear once
    // in each row, column and grid square.

    let mut seen = [false; 9];

    // Do all rows
    for row in 0..9
    {
        seen = [false; 9];

        for value in &board[row]
        {
            if value.is_numeric()
            {
                let value = value.to_digit(10).expect("already checked it's a number") as usize;

                // If value has already been seen, we have a duplicate
                if seen[value - 1]
                {
                    return false;
                }
                else
                {
                    seen[value - 1] = true;
                }
            }
        }
    }

    // Do all columns
    for column in 0..9
    {
        seen = [false; 9];

        for row in 0..9
        {
            let value = board[row][column];
            if value.is_numeric()
            {
                let value = board[row][column]
                    .to_digit(10)
                    .expect("already checked it's a number") as usize;

                // If value has already been seen, we have a duplicate
                if seen[value - 1]
                {
                    return false;
                }
                else
                {
                    seen[value - 1] = true;
                }
            }
        }
    }

    // Do all grid squares
    for grid_square in 0..9
    {
        let starting_x = grid_square % 3;
        let starting_y = grid_square / 3;

        seen = [false; 9];

        for x_offset in 0..3
        {
            for y_offset in 0..3
            {
                let x = (3 * starting_x) + x_offset;
                let y = (3 * starting_y) + y_offset;
                let value = &board[x][y];
                if value.is_numeric()
                {
                    let value = board[x][y]
                        .to_digit(10)
                        .expect("already checked it's a number")
                        as usize;

                    // If value has already been seen, we have a duplicate
                    if seen[value - 1]
                    {
                        return false;
                    }
                    else
                    {
                        seen[value - 1] = true;
                    }
                }
            }
        }
    }

    true
}
