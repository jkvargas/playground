use crate::nodes::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

pub struct Max {
    max: i32
}

impl Max {
    pub fn new(init_val: i32) -> Self {
        Self {
            max: init_val
        }
    }

    pub fn diameter_of_binary_tree(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        self.go_through(&root);
        self.max - 1
    }

    fn go_through(&mut self, root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node_borrow = &node.borrow();

            dbg!(&node_borrow.val);
            let max_left = self.go_through(&node_borrow.left);
            let max_right = self.go_through(&node_borrow.right);

            self.max = std::cmp::max(self.max, max_left + max_right + 1);
            return std::cmp::max(max_left, max_right) + 1;
        }

        return 0;
    }
}

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut out_max = Max::new(1);

        out_max.diameter_of_binary_tree(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diameter_of_binary_tree_1() {
        let node = TreeNode::from(vec![
            Some(4),
            Some(-7),
            Some(-3),
            None,
            None,
            Some(-9),
            Some(-3),
            Some(9),
            Some(-7),
            Some(-4),
            None,
            Some(6),
            None,
            Some(-6),
            Some(-6),
            None,
            None,
            Some(0),
            Some(6),
            Some(5),
            None,
            Some(9),
            None,
            None,
            Some(-1),
            Some(-4),
            None,
            None,
            None,
            Some(-2),
        ]);

        let result = Solution::diameter_of_binary_tree(node);

        assert_eq!(result, 7);
    }
}
