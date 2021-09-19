struct Solution;

use crate::nodes::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(rt) = root {
            if rt.borrow().left.is_none() && rt.borrow().right.is_none() {
                return;
            }

            Solution::flatten(&mut rt.borrow_mut().left);
            Solution::flatten(&mut rt.borrow_mut().right);

            let mut node = rt.clone();

            if rt.borrow().left.is_none() {
                return;
            }

            let l = node.borrow().left.clone().unwrap();
            node = l;

            while !node.borrow().right.is_none() {
                let r = node.borrow().right.clone();
                node = r.unwrap();
            }

            let lft = rt.borrow().left.clone();

            (*node.borrow_mut()).right = rt.borrow().right.clone();
            (*rt.borrow_mut()).right = lft;
            (*rt.borrow_mut()).left = None;
        };
    }
}
