use std::{cell::RefCell, ops::Deref, rc::Rc};

use leetcode::{invert_binary_tree_226::invert_tree, jump_game_ii_45};

fn main()
{
    // let board = vec![
    //     vec!['A', 'B', 'C', 'E'],
    //     vec!['S', 'F', 'C', 'S'],
    //     vec!['A', 'D', 'E', 'E'],
    // ];
    //
    // let result = exist(board, "ABCCED".to_string());
    // println!("result: {}", result);

    // let test_case = vec![
    //     "This",
    //     "is",
    //     "an",
    //     "example",
    //     "of",
    //     "text",
    //     "justification.",
    // ]
    // .iter()
    // .map(|val| val.to_string())
    // .collect();
    // let result = full_justify(test_case, 16);
    // println!("result: {:#?}", result);

    // Test cases:
    //
    // root = [2, 1, 3]
    // Output = [2, 3, 1]
    // let mut root = leetcode::invert_binary_tree_226::TreeNode::new(2);
    // let left = Rc::new(RefCell::new(leetcode::invert_binary_tree_226::TreeNode::new(1)));
    // let right = Rc::new(RefCell::new(leetcode::invert_binary_tree_226::TreeNode::new(3)));
    // root.left = Some(left);
    // root.right = Some(right);
    // let root = Rc::new(RefCell::new(root));
    // let result = invert_tree(Some(root));
    //
    // root = []
    // Output = []

    // root = [4, 2, 7, 1, 3, 6, 9]
    // Output = [4, 7, 2, 9, 6, 3, 1]
    // let two = Rc::new(RefCell::new(
    //     leetcode::invert_binary_tree_226::TreeNode::new(2),
    // ));
    // let one = Rc::new(RefCell::new(
    //     leetcode::invert_binary_tree_226::TreeNode::new(1),
    // ));
    // let three = Rc::new(RefCell::new(
    //     leetcode::invert_binary_tree_226::TreeNode::new(3),
    // ));
    // (*two.borrow_mut()).left = Some(one);
    // (*two.borrow_mut()).right = Some(three);


    // let seven = Rc::new(RefCell::new(
    //     leetcode::invert_binary_tree_226::TreeNode::new(7),
    // ));
    // let six = Rc::new(RefCell::new(
    //     leetcode::invert_binary_tree_226::TreeNode::new(6),
    // ));
    // let nine = Rc::new(RefCell::new(
    //     leetcode::invert_binary_tree_226::TreeNode::new(9),
    // ));
    // (*seven.borrow_mut()).left = Some(six);
    // (*seven.borrow_mut()).right = Some(nine);

    // let four = Rc::new(RefCell::new(
    //     leetcode::invert_binary_tree_226::TreeNode::new(4),
    // ));
    // (*four.borrow_mut()).left = Some(two);
    // (*four.borrow_mut()).right = Some(seven);

    // let root = four;
    // let result = invert_tree(Some(root));

    // let word = "hello".to_string();
    // let prefix = "hell".to_string();
    // let mut obj = leetcode::implement_trie_208::Trie::new();
    // obj.insert(word.clone());
    // let ret_2: bool = obj.search(word);
    // let ret_3: bool = obj.starts_with(prefix);
    // // println!("result: {:#?}", result);
    // println!("is hello in trie? (expected true): {}\nis there a word beginning with hell? (expected true): {}", ret_2, ret_3);

    let nums: Vec<i32> = vec![10,9,8,7,6,5,4,3,2,1,1,0];
    let result = jump_game_ii_45::jump(nums);
    println!("result: {}", result);
}
