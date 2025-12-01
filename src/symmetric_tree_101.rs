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

/// Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric
/// around its center).
///
/// Categorised as a BFS question, but DFS makes more sense.
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool
{
    let Some(root) = root else {
        return false;
    };

    let (left, right) = (root.borrow().left.clone(), root.borrow().right.clone());

    is_symmetric_recursive(left, right)
}

fn is_symmetric_recursive(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool
{
    match (left, right)
    {
        (None, None) => { true },
        (Some(left), Some(right)) => {
            if left.borrow().val != right.borrow().val { return false; }

            is_symmetric_recursive(left.borrow().left.clone(), right.borrow().right.clone())
                && is_symmetric_recursive(left.borrow().right.clone(), right.borrow().left.clone())
        },
        _ => { false },
    }
}
