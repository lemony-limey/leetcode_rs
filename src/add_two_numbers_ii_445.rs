use std::collections::VecDeque;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode
{
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode
{
    #[inline]
    fn new(val: i32) -> Self
    {
        Self {
          next: None,
          val
        }
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>
{
    // Add the contents of each list to the corresponding stack.
    let mut l1_stack: VecDeque<i32> = get_stack_from_list(&l1);
    let mut l2_stack: VecDeque<i32> = get_stack_from_list(&l2);
    
    let mut new_list: Option<Box<ListNode>> = None;

    loop
    {
        match (l1_stack.pop_front(), l2_stack.pop_front())
        {
            (Some(value1), Some(value2)) => {
                
            },
            (Some(value), None) => {
                
            },
            (None, Some(value)) => {
                
            },
            (None, None) => return new_list,
        }
    }

    // TODO
    todo!()
}

// pub fn get_list_len(list: &Option<Box<ListNode>>) -> usize
// {
//     let mut current_node = list;
//     let mut counter = 0;
//
//     while current_node.is_some()
//     {
//         counter += 1;
//         current_node = &current_node
//             .as_ref()
//             .unwrap()
//             .next;
//     }
//
//     counter
// }

fn get_stack_from_list(list: &Option<Box<ListNode>>) -> VecDeque<i32>
{
    let mut current_node = list;
    let mut stack = VecDeque::new();

    while current_node.is_some()
    {
        stack.push_front(
            current_node
                .as_ref()
                .unwrap()
                .val
        );
        current_node = &current_node
            .as_ref()
            .unwrap()
            .next;
    }

    stack
}

