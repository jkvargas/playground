use crate::nodes::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;

type TreeNodeType = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() { return false; }

        let r = root.unwrap();
        let r_bor = r.borrow();

        Self::is_mirror(r_bor.left.clone(), r_bor.right.clone())
    }

    fn is_mirror(left: TreeNodeType, right: TreeNodeType) -> bool {
        if left.is_none() && right.is_none() { return true; }
        if left.is_none() ^ right.is_none() { return false; }

        let l = left.unwrap();
        let r = right.unwrap();

        let l_bor = l.borrow();
        let r_bor = r.borrow();

        l_bor.val == r_bor.val && Self::is_mirror(l_bor.left.clone(), r_bor.right.clone()) && Self::is_mirror(l_bor.right.clone(), r_bor.left.clone())
    }
}
