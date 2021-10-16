use crate::nodes::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }

        let mut queue_right = Vec::new();
        let mut queue_left = Vec::new();

        let r = root.unwrap();
        let r_bor = r.borrow();

        queue_right.push(r_bor.right.clone());
        queue_left.push(r_bor.left.clone());

        while !queue_left.is_empty() && !queue_right.is_empty() {
            let mut right_vec = Vec::new();
            let mut left_vec = Vec::new();

            let right_pop = queue_right.pop();
            let left_pop = queue_left.pop();

            if let Some(rp) = right_pop {}

            if let Some(lp) = left_pop {}
        }

        queue_left.is_empty() && queue_right.is_empty()
    }
}
