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
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

/// Can perform DFS and check that all of the compared nodes are the same.
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool
{
    let mut queue_1_p = VecDeque::new();
    let mut queue_2_q = VecDeque::new();

    // If both are empty, they are the same
    if p.is_none() && q.is_none()
    {
        return true;
    }

    // Add stuff before loop begins
    queue_1_p.push_front(p);
    queue_2_q.push_front(q);

    // Perform DFS
    while !queue_1_p.is_empty() || !queue_2_q.is_empty()
    {
        let (current_p, current_q) = (queue_1_p.pop_front(), queue_2_q.pop_front());

        // Check if any values were popped off the stack.
        match (current_p, current_q)
        {
            (Some(p), Some(q)) =>
            {
                // Match the inner values
                // If both were in the stack, they should both

                match (p, q)
                {
                    (Some(p), Some(q)) =>
                    {
                        // Check that the values of each node are the same
                        if p.borrow().val != q.borrow().val
                        {
                            return false;
                        }

                        // Add all of the values of each to the queue
                        queue_1_p.push_front(p.borrow_mut().left.clone());
                        queue_1_p.push_front(p.borrow_mut().right.clone());
                        queue_2_q.push_front(q.borrow_mut().left.clone());
                        queue_2_q.push_front(q.borrow_mut().right.clone());
                    },
                    (None, None) =>
                    {
                        continue;
                    },
                    (_, _) =>
                    {
                        return false;
                    },
                }
            },
            (None, None) =>
            {
                continue;
            },
            (_, _) =>
            {
                return false;
            },
        }
    }

    // If we've successfully completed DFS up to this point, the
    // graphs are the same.
    true
}
