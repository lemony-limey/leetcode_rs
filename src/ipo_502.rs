use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// w - Initial capital
/// k - The maximum number of projects
/// profits - The profits you will make for each project i
/// capital - The capital threshold you must match or exceed to do project i and
///     gain the profit.
pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32
{
    // Capital is initially equal to w
    let mut current_capital = w;
    let mut projects_remaining = k;

    let mut capital_and_profits = capital.into_iter()
        .zip(profits.into_iter())
        .map(|x| Reverse(x))
        .collect::<BinaryHeap<Reverse<(i32, i32)>>>();
    let mut project_profits = BinaryHeap::with_capacity(capital_and_profits.len());

    while projects_remaining > 0
    {
        // Add any newly available projects to the heap.
        while let Some(Reverse((capital, profit))) = capital_and_profits.peek()
        {
            if *capital <= current_capital
            {
                project_profits.push(*profit);
                capital_and_profits.pop();
            }
            else
            {
                break;
            }
        }

        // Add the next greatest project's profit to the capital
        if let Some(profit) = project_profits.pop()
        {
            current_capital += profit;
        }
        else
        {
            break;
        }
        projects_remaining -= 1;
    }

    current_capital
}