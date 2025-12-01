// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode
{
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode
{
    #[inline]
    pub fn new(val: i32) -> Self
    {
        Self
        {
            val,
            left: None,
            right: None
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>
{
    let mut results: Vec<Vec<i32>> = Vec::new();
    let Some(root) = root else {
        return results;
    };

    let mut queue: VecDeque<(usize, Rc<RefCell<TreeNode>>)> = VecDeque::new();

    // Add the root to the first level
    queue.push_back((0, root));

    // BFS
    while let Some((current_level, node)) = queue.pop_front()
    {
        // Check if a corresponding Vec exists in results yet
        if results.len() <= current_level
        {
            results.push(Vec::new());
        }

        // Add the node's value to the array
        let inner_vec_handle = results.get_mut(current_level).unwrap();
        inner_vec_handle.push(node.borrow().val.clone());

        // If the node is Some, add its children with current_level + 1
        let (left, right) = (node.borrow().left.clone(), node.borrow().right.clone());
        match (left, right)
        {
            (Some(left), Some(right)) => {
                queue.push_back((current_level + 1, left));
                queue.push_back((current_level + 1, right));
            },
            (Some(node), None) | (None, Some(node)) => {
                queue.push_back((current_level + 1, node));
            }
            (None, None) => { continue; }
        }
    }

    results
}
