// use std::cell::RefCell;
// use std::rc::{Rc, Weak};
//
// pub struct LinkedListNode {
//     pub next: Option<RefCell<Weak<LinkedListNode>>>,
//     pub previous: Option<RefCell<Weak<LinkedListNode>>>,
//     pub val: i32
// }
//
// pub struct LinkedList {
//     pub head: Option<RefCell<Weak<LinkedListNode>>>,
//     pub tail: Option<RefCell<Weak<LinkedListNode>>>
// }
//
// impl LinkedList {
//     pub fn push_front(&mut self, node: RefCell<Weak<LinkedListNode>>) {
//         if let Some(front) = self.head {
//             let borrowed = node.borrow_mut();
//
//             borrowed. next = Some(front);
//         }
//
//         self.head = Some(node);
//     }
// }