use crate::nodes::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::climb_down(root, 0)
    }

    fn climb_down(root: Option<Rc<RefCell<TreeNode>>>, mut depth: i32) -> i32 {
        if root.is_none() {
            return depth;
        }

        let unr = root.unwrap();
        let bor = unr.borrow();

        depth += 1;

        max(
            Self::climb_down(bor.left.clone(), depth),
            Self::climb_down(bor.right.clone(), depth),
        )
    }
}

#[test]
fn climb_down() {
    let res = Solution::max_depth(TreeNode::from(vec![
        Some(3),
        Some(9),
        Some(20),
        None,
        None,
        Some(15),
        Some(7),
    ]));

    assert_eq!(3, res);
}
