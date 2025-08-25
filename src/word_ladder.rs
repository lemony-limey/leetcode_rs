use std::collections::{HashMap, HashSet, VecDeque};

pub fn ladder_length(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32
{
    // Exit early if the starting word is not in the word list
    if !word_list.contains(&end_word) { return 0 }

    // Build the adjacency list
    let mut neighbours: HashMap<String, Vec<String>> = HashMap::new();
    word_list.push(begin_word.clone());

    for word in word_list
    {
        for j in 0..word.len()
        {
            // let first_half = &word[0..j].clone
            let pattern = format!("{}*{}", &word[0..j], &word[j + 1..]);

            if !neighbours.contains_key(&pattern)
            {
                neighbours.insert(pattern.clone(), vec![word.clone()]);
            }
            else
            {
                let inner_vec = neighbours
                    .get_mut(&pattern)
                    .expect("could not access inner vec");

                inner_vec.push(word.clone());
            }
        }
    }

    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(begin_word.clone());
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(begin_word.clone());
    let mut result = 1;

    // Perform BFS
    while !queue.is_empty()
    {
        for _ in 0..queue.len()
        {
            let Some(word) = queue.pop_front() else {
                eprintln!("bfs: Could not pop from queue");
                return 0
            };

            if word.eq(&end_word)
            {
                return result
            }

            for j in 0..word.len()
            {
                let pattern = format!("{}*{}", &word[0..j], &word[j + 1..]);

                // Add any unvisited nodes to the BFS queue.
                for neighbouring_word in neighbours.get(&pattern).unwrap()
                {
                    if !visited.contains(neighbouring_word)
                    {
                        visited.insert(neighbouring_word.clone());
                        queue.push_back(neighbouring_word.clone());
                    }
                }
            }
        }

        result += 1;
    }

    eprintln!("Could not find a path: returning default value of 0");
    0
}