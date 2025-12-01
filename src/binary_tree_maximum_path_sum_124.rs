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
/// TODO: This probably requires some kind of O(n^3) search.
pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32
{
    let Some(root) = root else {
        return 0;
    };

    0
}
