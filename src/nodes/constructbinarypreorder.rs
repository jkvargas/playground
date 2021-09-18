// https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/

struct Solution;

use crate::nodes::{TreeNode, TreeResult};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> TreeResult {
        if preorder.len() == 0 {
            return None;
        }

        let mut tree_build = TreeBuild::new(preorder[0]);

        for i in 1..preorder.len() {
            tree_build.insert(preorder[i]);
        }

        tree_build.tree
    }
}

struct TreeBuild {
    tree: TreeResult,
}

impl TreeBuild {
    fn new(val: i32) -> Self {
        Self {
            tree: Some(Rc::new(RefCell::new(TreeNode::new(val)))),
        }
    }

    fn try_insert(tree: Option<&Rc<RefCell<TreeNode>>>, val: i32) {
        if let Some(tree) = tree {
            let mut bor = tree.borrow_mut();

            if val > bor.val {
                if bor.right.is_none() {
                    bor.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                } else {
                    Self::try_insert(bor.right.as_ref(), val);
                }
            } else {
                if bor.left.is_none() {
                    bor.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                } else {
                    Self::try_insert(bor.left.as_ref(), val);
                }
            }
        }
    }

    fn insert(&mut self, val: i32) {
        Self::try_insert(self.tree.as_ref(), val);
    }
}

#[test]
fn test_one() {
    let preorder = vec![8, 5, 1, 7, 10, 12];

    let result = Solution::bst_from_preorder(preorder);

    dbg!(&result);
}
