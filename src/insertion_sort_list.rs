// // Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
//
// struct Solution;
//
// impl Solution {
//     pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         let mut dummy = Box::new(ListNode::new(i32::MIN));
//
//         let mut tail = &mut dummy;
//         while let Some(node) = head.take() {
//             if tail.next.is_none() {
//                 tail.next = Some(node);
//                 tail = tail.next.as_mut().unwrap();
//             } else {
//
//             }
//
//             head = node.next;
//         }
//
//         None
//     }
// }
//
// fn temp(node: Option<Box<ListNode>>, prev: i32) -> i32 {
//     if let Some(item) = &node {
//         if item.val < prev {
//             return item.val
//         } else {
//             let we_got = temp(item.next, item.val);
//             if we_got < item.val {
//                 return item.val;
//             }
//         }
//     }
// }