use crate::nodes::{TreeNode, TreeResult};
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn is_same_tree(p: TreeResult, q: TreeResult) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }

        if p.is_none() ^ q.is_none() {
            return false;
        }

        let p_unr = p.unwrap();
        let q_unr = q.unwrap();
        let p_val = p_unr.borrow();
        let q_val = q_unr.borrow();

        p_val.val == q_val.val
            && Self::is_same_tree(p_val.left.clone(), q_val.left.clone())
            && Self::is_same_tree(p_val.right.clone(), q_val.right.clone())
    }
}

#[test]
fn test_one() {
    Solution::is_same_tree(
        TreeNode::from(vec![Some(1), Some(2), Some(3)]),
        TreeNode::from(vec![Some(1), Some(2), Some(3)]),
    );
}
