use std::collections::{HashMap, HashSet, VecDeque};

const ALPHABET_SIZE: usize = 26;

// The index of the child node represents the alphabetical character.
struct TrieNode
{
    is_word_boundary: bool,
    children: HashMap<char, Option<Box<TrieNode>>>,
}

impl TrieNode
{
    pub fn new() -> Self
    {
        Self
        {
            is_word_boundary: false,
            children: HashMap::with_capacity(26),
        }
    }

    pub fn add_word(&mut self, word: &mut VecDeque<char>)
    {
        // If there are still characters left, this is not the end of the word.
        if let Some(next_char) = word.pop_front()
        {
            if let Some(node) = self.children.get_mut(&next_char)
            {
                if let Some(node) = node
                {
                    node.add_word(word);
                }
            }
            // Node doesn't already exist as a child node, so add it.
            else
            {
                self.children.insert(
                    next_char,
                    Some( Box::new( TrieNode::new() ) )
                );
            }
        }
        // If this node is the end of the word, mark it as a word boundary.
        else
        {
            self.is_word_boundary = true;
        }
    }
}

// 212. Word Search II
// A Trie is used with DFS.
pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String>
{
    let mut found = HashSet::with_capacity(words.len());

    let mut visited: HashSet<(i8, i8)> = HashSet::new();
    let words: HashMap<char, String> = words
        .into_iter()
        .map(|word| {
            let first_char = word.chars().nth(0).unwrap();
            (first_char, word)
        })
        .collect();

    for row in 0..board.len()
    {
        for column in 0..board[row].len()
        {

        }
    }

    // Once this point is reached, the search space has been exhausted
    found.into_iter().collect()
}


