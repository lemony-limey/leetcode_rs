use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>>
{
    score.sort_by_key(|row| Reverse(row[k as usize]));

    score
}