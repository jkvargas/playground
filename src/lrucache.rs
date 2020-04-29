// use std::cell::RefCell;
// use std::rc::{Weak, Rc};
// use std::collections::HashMap;
//
// pub struct LinkedListNode<T> {
//     pub next: Option<RefCell<Weak<LinkedListNode<T>>>>,
//     pub previous: Option<RefCell<Weak<LinkedListNode<T>>>>,
//     pub val: T
// }
//
// impl<T> LinkedListNode<T> {
//     pub fn new(val: T) -> Self {
//         LinkedListNode {
//             next: None,
//             previous: None,
//             val: val
//         }
//     }
// }
//
// pub struct LinkedList<T> {
//     pub head: Option<RefCell<Weak<LinkedListNode<T>>>>,
//     pub tail: Option<RefCell<Weak<LinkedListNode<T>>>>
// }
//
// pub struct LruCache {
//     pub map: HashMap<i32, RefCell<Rc<LinkedListNode<(i32, i32)>>>>,
//     pub list: LinkedList<(i32, i32)>
// }
//
// impl<T> LinkedList<T> {
//     pub fn push_front(&mut self, node: RefCell<Weak<LinkedListNode<T>>>) {
//         if let Some(front) = self.head {
//             let borrowed = node.borrow_mut();
//             borrowed.ne
//         }
//
//         self.head = Some(node);
//     }
// }
//
// impl LruCache {
//     pub fn put(&mut self, key: i32, value: i32) {
//         let llnode = RefCell::new(Rc::new(LinkedListNode::new((key, value))));
//         self.map.insert(key, llnode);
//         self.list.push_front(llnode);
//     }
// }