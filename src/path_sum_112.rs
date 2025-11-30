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

/// Given the root of a binary tree and an integer targetSum, return true if the tree
/// has a root-to-leaf path such that adding up all the values along the path equals targetSum.
//
// A leaf is a node with no children.
///
/// Basically, if at a leaf, check the current sum.
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool
{
    let Some(root) = root else {
        return false;
    };

    has_path_sum_recursive(root, 0, target_sum)
}

fn has_path_sum_recursive(next_node: Rc<RefCell<TreeNode>>, mut current_sum: i32, target_sum: i32) -> bool
{
    current_sum += next_node.borrow().val;

    // If both left and right are none, check if current sum plus next node sum equals target sum
    match (next_node.borrow().left.clone(), next_node.borrow().right.clone())
    {
        (None, None) => {
            // Base case: this is a leaf node. Check if the sums are equal.
            current_sum == target_sum
        },
        (Some(left), Some(right)) => {
            // Return true if one of them is true.
            has_path_sum_recursive(left, current_sum, target_sum)
                || has_path_sum_recursive(right, current_sum, target_sum)
        },
        (Some(node), _) | (_, Some(node)) => {
            has_path_sum_recursive(node, current_sum, target_sum)
        },
    }
}
