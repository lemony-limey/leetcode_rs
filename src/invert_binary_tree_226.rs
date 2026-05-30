use std::{cell::RefCell, fmt, rc::Rc};

/// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode
{
    pub val:   i32,
    pub left:  Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode
{
    #[inline]
    pub fn new(val: i32) -> Self
    {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

impl fmt::Display for TreeNode
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match (&self.left, &self.right)
        {
            (None, None) => write!(f, "{}", self.val),
            (Some(node), None) | (None, Some(node)) => write!(f, "{}:{}", self.val, node.borrow()),
            (Some(left), Some(right)) =>
            {
                write!(f, "{}:{}:{}", self.val, left.borrow(), right.borrow())
            },
        }
    }
}

// Test cases:
// root = [4, 2, 7, 1, 3, 6, 9]
// Output = [4, 7, 2, 9, 6, 3, 1]
//
// root = [2, 1, 3]
// Output = [2, 3, 1]
//
// root = []
// Output = []
//
// Assumptions:
//     - No guarantees that tree is full.
//
// Probably DFS and switch nodes around on the way back up.
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>
{
    // Handle the case where the tree is empty.
    if root.is_none()
    {
        return None;
    }

    // Extract the TreeNode from the Option.
    let root = root?;

    // Match the left and right nodes, then modify the current node accordingly.
    let (left, right) = (root.borrow().left.clone(), root.borrow().right.clone());
    match (left, right)
    {
        // Leaf node
        (None, None) => Some(root),
        // Right child node will become new left child node.
        (None, Some(node)) =>
        {
            let new_left = invert_tree(Some(node));
            (*root.borrow_mut()).right = None;
            (*root.borrow_mut()).left = new_left;

            Some(root)
        },
        // Left child node
        (Some(node), None) =>
        {
            let new_right = invert_tree(Some(node));
            (*root.borrow_mut()).right = new_right;
            (*root.borrow_mut()).left = None;
            Some(root)
        },
        // Both child nodes.
        (Some(left), Some(right)) =>
        {
            // Modify the left and right nodes of the root.
            // Recursively swap the left and right nodes.
            let new_right = invert_tree(Some(left));
            let new_left = invert_tree(Some(right));

            // Swap the positions of the nodes.
            (*root.borrow_mut()).right = new_right;
            (*root.borrow_mut()).left = new_left;

            Some(root)
        },
    }
}
