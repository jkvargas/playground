use std::rc::Rc;
use std::cell::RefCell;
use crate::nodes::TreeNode;

pub struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        Self::go_though(&root, l, r)
    }

    fn go_though(root: &Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        if let Some(node) = root {
            let borrowed = node.borrow();

            return if borrowed.val >= l && borrowed.val <= r {
                borrowed.val
            } else {
                0
            } +
                Self::go_though(&borrowed.left, l, r) +
                Self::go_though(&borrowed.right, l, r);
        }

        0
    }
}