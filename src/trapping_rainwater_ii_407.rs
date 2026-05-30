use std::{
    cmp::{Ordering, Reverse, max},
    collections::BinaryHeap,
};

#[derive(Eq, PartialEq)]
struct Cell
{
    pub height: i32,
    pub row:    i32,
    pub column: i32,
}

impl Ord for Cell
{
    fn cmp(&self, other: &Self) -> Ordering { self.height.cmp(&other.height) }
}

impl PartialOrd for Cell
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(&other)) }
}

pub fn trapping_rainwater_ii(mut height_map: Vec<Vec<i32>>) -> i32
{
    let (rows, columns) = (height_map.len(), height_map[0].len());

    // 1. Add the border to the heap
    let mut heap = BinaryHeap::new();
    for row in 0..rows
    {
        for column in 0..columns
        {
            if row == 0 || row == rows - 1 || column == 0 || column == columns - 1
            {
                heap.push(Reverse(Cell {
                    height: height_map[row][column],
                    row:    row as i32,
                    column: column as i32,
                }));
                height_map[row][column] = -1;
            }
        }
    }

    // 2. Search the neighbouring nodes
    let mut total_water = 0;
    let mut max_height = -1;

    while !heap.is_empty()
    {
        let Reverse(current_cell) = heap.pop().expect("could not pop from heap");
        let (height, row, column) = (current_cell.height, current_cell.row, current_cell.column);

        max_height = max(max_height, height);
        total_water += max_height - height;

        let neighbours = [
            (row + 1, column),
            (row - 1, column),
            (row, column + 1),
            (row, column - 1),
        ];

        // Add unvisited neighbouring cells to the heap
        for (nr, nc) in neighbours
        {
            if nr < 0
                || nc < 0
                || nr as usize == rows
                || nc as usize == columns
                || height_map[nr as usize][nc as usize] == -1
            {
                continue;
            }

            heap.push(Reverse(Cell {
                height: height_map[nr as usize][nc as usize],
                row:    nr,
                column: nc,
            }));
            height_map[nr as usize][nc as usize] = -1;
        }
    }

    total_water
}
