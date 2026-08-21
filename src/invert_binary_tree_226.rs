use std::{cell::RefCell, rc::Rc};

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

// Test cases:
// root = [4, 2, 7, 1, 3, 6, 9]
// Output = [4, 7, 2, 9, 6, 3, 1]
//
// root = [2, 1, 3]
// Output = [2, 3, 1]
//
// root = []
// Output = []
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>
{
    // Extract the TreeNode from the Option, returning None if root is None also.
    let root = root?;

    // Match the left and right nodes, then modify the current node accordingly.
    // Epic life lesson: if you tried to use the stuff below for left and right directly in the
    // tuple of the match statement it wouldn't compile, because then root itself is immutably
    // borrowed twice in the temporary expression of the match statement, and so it cannot then
    // be moved while it's still being borrowed because the temporary expression remains in scope
    // for the entire duration of the match statement (and so root remains borrowed).
    // Since the stuff below is wrapped in Rc (a Reference-Counted pointer), cloning the handles
    // to the left and right child nodes of root is ok because it simply increments the reference
    // count and so you can get multiple ownership. At this point after the handles are cloned the
    // root.borrow() temporary value is dropped and so root is not borrowed by the time we enter
    // the match statement, and so it can be returned from the function as Some(root) within a
    // branch of the match statement, transferring ownership, which would not be possible if root
    // was still borrowed because the borrow checker would be like what the hell are you an
    // absolute daftie?
    let (left, right) = (root.borrow().left.clone(), root.borrow().right.clone());
    match (left, right)
    {
        // The root is a leaf node, and so has no child nodes by definition.
        (None, None) => Some(root),
        // Right child node will become new left child node.
        (None, Some(node)) =>
        {
            let new_left = invert_tree(Some(node));
            // borrow_mut() effectively transforms this from Rc<RefCell<TreeNode>>
            // to &mut TreeNode (technically not quite but basically yes, the RefCell is how
            // we get a mutable reference because it causes the borrow to be checked at runtime
            // not compile time, so while it will still compile even if you try to borrow it
            // mutably twice at the same time (sacrilege) it'll still panic at runtime if it's
            // wrong, and so it won't cause UB, which is why it's considered safe). RefCell itself
            // does incur a runtime cost though. Rc probably does but I don't know if it's
            // as hefty.
            // 
            // The root.borrow_mut() part is dereferenced here to be on the safe side, but I don't
            // think it always has to be done, and it definitely doesn't in this case (I tested
            // it to be sure).
            // I always do it because if TreeNode was Copy (which it can't be because it contains
            // smart pointers, but if it contained only primitive types or other structs/enums
            // that derived Copy, then it could be) then you might be
            // making changes on a copy of the variable's value instead of the actual thing itself,
            // meaning that the changes may not be happening to the correct thing, and thus appear
            // like they have no effect (this actually happened to my cache simulator for
            // CS4202 P1 and then again to my branch predictor, hence why I always dereference
            // explicitly now).
            // For this to make sense, consider the primitive types like u16, i32, usize, etc,
            // these are all Copy. If I have a variable that is 2, for example, adding 1 to it
            // will not add 1 to every variable that contains an instance of 2 ever because that
            // would be crazy. If I assign its value to a new variable, the value is COPIED, hence
            // the name:
            // 
            // let mut a: usize = 2;
            // let b = a; // (b = 2. "b" is not a pointer to the value held in "a", it just contains
            //            // a copy of the value in "a" because usize derives Copy)
            // a += 1;  // When the value in "a" is modified, this change is only seen by "a",
            //          // because "b"s value is totally separate.
            // assert_eq!(a, 3);
            // assert_eq!(b, 2);
            //
            // Now consider some struct (like for representing a cache) that contains instances
            // of another struct (e.g., cache line metadata):
            //
            // let mut cache = Cache {
            //     cache_line: CacheLine { tag: 5, index: 10, offset: 16 },
            // };
            //
            // If cache is Copy (and thus so is CacheLine), then the following operation will not
            // apply changes to the cache's cache line...
            //
            // cache.cache_line.tag = 20;
            //
            // ...because all the values are Copy, so this is just a local change. For the changes
            // to persist, the following would work...
            //
            // (*cache).cache_line.tag = 20;
            //
            // ...as this would write the data back to the specific cache line instance. At least,
            // this is how it makes sense in my head. Maybe the situation in my architecture
            // practical was slightly different but this was the gist of it.
            //
            // The parenthesis around *root.borrow_mut() are necessary to indicate that it is the
            // root.borrow_mut() part that is to be dereferenced, otherwise it seems to be the
            // thing at the end that it applies to:
            //
            // (*root.borrow_mut()).left = None -> "root" is dereferenced and root.left is set
            //                                     to None.
            // *root.borrow_mut().left = None   -> "left" is dereferenced (but it isn't actually
            //                                     because the program won't compile lol, because
            //                                     whatever the type of "left" is cannot be
            //                                     dereferenced (it is Option<Rc<RefCell<TreeNode>))
            (*root.borrow_mut()).right = None;
            (*root.borrow_mut()).left = new_left;

            Some(root)
        },
        // Left child node will become new right child node.
        (Some(node), None) =>
        {
            let new_right = invert_tree(Some(node));
            (*root.borrow_mut()).right = new_right;
            (*root.borrow_mut()).left = None;
            Some(root)
        },
        // Both child nodes swap.
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
