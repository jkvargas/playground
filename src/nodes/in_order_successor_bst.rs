// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use crate::nodes::{TreeNode, TreeResult};
use std::cell::RefCell;
use std::rc::Rc;
// https://leetcode.com/problems/inorder-successor-in-bst/
struct Solution;

// from the internet
//
// impl Solution {
//     pub fn inorder_successor(
//         mut root: Option<Rc<RefCell<TreeNode>>>,
//         p: Option<Rc<RefCell<TreeNode>>>,
//     ) -> Option<Rc<RefCell<TreeNode>>> {
//         let mut successor: Option<Rc<RefCell<TreeNode>>> = None;
//         let mut current = root;
//         let p_val = p.as_ref().unwrap().borrow().val;
//         while current.is_some() {
//             if p_val >= current.as_ref().unwrap().borrow().val {
//                 current = current.unwrap().borrow().right.clone();
//             } else {
//                 successor = current.clone();
//                 current = current.unwrap().borrow().left.clone();
//             }
//         }
//
//         successor
//     }
// }

impl Solution {
    pub fn inorder_successor(root: TreeResult, p: TreeResult) -> TreeResult {
        if root.is_none() {
            return None;
        }

        let mut sucessor = Solution::inorder_successor(
            root.as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .map_or(None, |x| Some(x.clone())),
            p.as_ref().map_or(None, |x| Some(x.clone())),
        );

        if sucessor.is_some() {
            return sucessor;
        }

        if root.as_ref().unwrap().borrow().val > p.as_ref().unwrap().borrow().val {
            return root;
        }

        Self::inorder_successor(
            root.as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .map_or(None, |x| Some(x.clone())),
            p.as_ref().map_or(None, |x| Some(x.clone())),
        )
    }
}
