use std::str::Chars;

pub struct Trie
{
    child_nodes: [Option<Box<TrieNode>>; 26],
}

/// `&self` means the method takes an immutable reference.
/// If you need a mutable reference, change it to `&mut self` instead.
impl Trie
{
    pub fn new() -> Self
    {
        Self { child_nodes: [const { None }; 26] }
    }

    pub fn insert(&mut self, word: String)
    {
        if word.is_empty() { return; }
        let word = word.to_ascii_lowercase();
        
        // Recursively add each character if it does not exist in the trie already.
        let mut chars_remaining = word.chars();
        let next_char = chars_remaining.next()
            .expect("word is not empty");
        let index = next_char as usize - 97;

        // Node is present.
        if let Some(child_node) = &mut self.child_nodes[index]
        {
            (*child_node).insert(chars_remaining);
        }
        // Node is not present, so add the rest of the word recursively.
        else
        {
            let new_node = Box::new(TrieNode::new(next_char));
            (*self).child_nodes[index] = Some(new_node);
            (*self).child_nodes[index]
                .as_mut()
                .expect("we know the node is present")
                .insert(chars_remaining);
        }
    }

    // Recursively search through the tree to see if the word exists.
    pub fn search(&self, word: String) -> bool
    {
        if word.is_empty() { return true; }
        let word = word.to_ascii_lowercase();
        
        let mut chars_remaining = word.chars();
        let next_char = chars_remaining.next()
            .expect("word is not empty");
        let index = next_char as usize - 97;

        // Node is present.
        if let Some(child_node) = &self.child_nodes[index]
        {
            // Return the result of search once it has finished.
            (*child_node).search(chars_remaining)
        }
        // Node is not present, so word is not in the trie.
        else
        {
            false
        }
    }

    // TODO
    pub fn starts_with(&self, prefix: String) -> bool
    {
        // Search through the tree for the entire prefix, and then DFS from the final node to
        // reach a word boundary.
        if prefix.is_empty() { return true; }
        let prefix = prefix.to_ascii_lowercase();
        
        // Recursively add each character if it does not exist in the trie already.
        let mut chars_remaining = prefix.chars();
        let next_char = chars_remaining.next()
            .expect("word is not empty");
        let index = next_char as usize - 97;

        // Node is present.
        if let Some(child_node) = &self.child_nodes[index]
        {
            (*child_node).starts_with(chars_remaining)
        }
        // Node is not present, so no word in the trie has this prefix.
        else
        {
            false
        }
    }
}

#[derive(Clone, Debug)]
struct TrieNode
{
    character:        Option<char>,
    is_word_boundary: bool,
    child_nodes:      [Option<Box<TrieNode>>; 26],
}

impl TrieNode
{
    fn new(character: char) -> Self
    {
        Self
        {
            character:        Some(character),
            is_word_boundary: false,
            child_nodes:      [const { None }; 26],
        }
    }

    fn insert(&mut self, mut chars_remaining: Chars<'_>)
    {
        // Recursively add each character if it does not exist in the trie already.
        if let Some(next_char) = chars_remaining.next()
        {
            let index = next_char as usize - 97;

            // Node is present.
            if let Some(child_node) = &mut self.child_nodes[index]
            {
                (*child_node).insert(chars_remaining);
            }
            // Node is not present, so add it.
            else
            {
                let new_node = Box::new(TrieNode::new(next_char));
                (*self).child_nodes[index] = Some(new_node);
                (*self).child_nodes[index]
                    .as_mut()
                    .expect("we know the node is present")
                    .insert(chars_remaining);
            }
        }
        // If the iterator is empty, mark the current cell as a word boundary.
        else
        {
            (*self).is_word_boundary = true;
        }
    }

    fn search(&self, mut chars_remaining: Chars<'_>) -> bool
    {        
        // Search through the trie for the entire word.
        if let Some(next_char) = chars_remaining.next()
        {
            let index = next_char as usize - 97;

            // Node is present.
            if let Some(child_node) = &self.child_nodes[index]
            {
                // Return the result of the recursive call.
                child_node.search(chars_remaining)
            }
            // Node is not present, so word is not in the trie.
            else
            {
                false
            }
        }
        // Ensure that the current node indicates a word boundary.
        else
        {
            if self.is_word_boundary
            {
                true
            }
            else
            {
                false
            }
        }
    }

    fn starts_with(&self, mut chars_remaining: Chars<'_>) -> bool
    {
        // Search through the trie for the entire word.
        if let Some(next_char) = chars_remaining.next()
        {
            let index = next_char as usize - 97;

            // Node is present.
            if let Some(child_node) = &self.child_nodes[index]
            {
                // Return the result of the recursive call.
                child_node.starts_with(chars_remaining)
            }
            // Node is not present, so word is not in the trie.
            else
            {
                false
            }
        }
        // Doesn't matter if this represents a word boundary or not.
        else
        {
            true
        }
    }
}

// Your Trie object will be instantiated and called as such:
// let obj = Trie::new();
// obj.insert(word);
// let ret_2: bool = obj.search(word);
// let ret_3: bool = obj.starts_with(prefix);

