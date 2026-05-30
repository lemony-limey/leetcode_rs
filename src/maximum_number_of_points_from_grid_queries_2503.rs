use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
};

#[derive(PartialOrd, Eq, PartialEq)]
struct Cell
{
    pub value:  i32,
    pub row:    i32,
    pub column: i32,
}

impl Ord for Cell
{
    fn cmp(&self, other: &Self) -> Ordering { self.value.cmp(&other.value) }
}

pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32>
{
    if grid.is_empty() || queries.is_empty()
    {
        return Vec::new();
    }

    let (rows, columns) = (grid.len(), grid[0].len());

    // Less repetitive work if we compute answer in ascending order
    // of query size, but we still need to remember initial array position,
    // so an index is added with enumerate().
    let mut queries: Vec<(i32, usize)> = queries
        .iter()
        .enumerate()
        .map(|(a, b): (usize, &i32)| (*b, a))
        .collect();
    queries.sort();

    let mut heap: BinaryHeap<Reverse<Cell>> = BinaryHeap::new();
    push_cell_to_heap(&mut heap, grid[0][0], 0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    let mut results: Vec<i32> = vec![0; queries.capacity()];
    let mut points = 0;

    for (limit, index) in queries
    {
        while !heap.is_empty() && heap.peek().unwrap().0.value < limit
        {
            let Some(Reverse(cell)) = heap.pop()
            else
            {
                panic!("couldn't pop from the heap");
            };

            let Cell {
                value: _value,
                row,
                column,
            } = cell;
            points += 1;
            let neighbours = [
                (row + 1, column),
                (row - 1, column),
                (row, column + 1),
                (row, column - 1),
            ];

            for (nr, nc) in neighbours
            {
                if (0 <= nr)
                    && (nr < rows as i32)
                    && (0 <= nc)
                    && (nc < columns as i32)
                    && !visited.contains(&(nr, nc))
                {
                    push_cell_to_heap(&mut heap, grid[nr as usize][nc as usize], nr, nc);
                    visited.insert((nr, nc));
                }
            }
        }

        results[index] = points;
    }

    results
}

fn push_cell_to_heap(heap: &mut BinaryHeap<Reverse<Cell>>, value: i32, row: i32, column: i32)
{
    heap.push(Reverse(Cell { value, row, column }));
}
